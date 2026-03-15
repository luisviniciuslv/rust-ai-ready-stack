mod adapters;
mod application;
mod domain;
mod endpoints;
mod error;
mod middleware;
mod state;
mod templates;
mod utils;

use crate::adapters::google::{GoogleOAuthAuthorizationUrlBuilder, ReqwestGoogleOAuthProvider};
use crate::adapters::http::AxumSessionCookieService;
use crate::adapters::jwt::JwtIdentityService;
use crate::adapters::lancedb::LanceDbRepo;
use crate::adapters::mongodb::mongo_repo::MongoRepo;
use crate::domain::ports::{
    DocumentRepository, GoogleOAuthProvider, IdentityService, OAuthAuthorizationUrlBuilder,
    SessionCookieService, UserRepository,
};
use crate::endpoints::auth::{
    get_user_profile::get_user_profile, google_oauth::google_oauth,
    google_oauth::google_oauth_callback, sign_out::sign_out,
};
use crate::endpoints::chat::{
    chat_conversation_endpoint::chat_conversation, ingest_endpoint::ingest_document,
};
use crate::middleware::error_handler_middleware;
use crate::middleware::require_jwt_cookie::require_jwt_cookie;
use crate::state::{AppState, UseCases};
use anyhow::Context;
use async_openai::Client;
use axum::{
    http::{
        header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use axum_cookie::prelude::CookieLayer;
use dotenv::dotenv;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{catch_panic::CatchPanicLayer, cors::CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Iniciando servidor");
    dotenv().ok();

    let url_frontend = std::env::var("URL_FRONTEND").unwrap_or("http://localhost:3000".to_string());
    let frontend_origin = url_frontend
        .parse::<HeaderValue>()
        .with_context(|| format!("URL_FRONTEND inválida para header CORS: {}", url_frontend))?;

    let cors = CorsLayer::new()
        .allow_origin(frontend_origin)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    let vector_db = LanceDbRepo::new("data/lancedb_store", "documents_table")
        .await
        .context("Falha ao inicializar LanceDB")?;

    let openai_client = Client::new();

    let google_client_id =
        std::env::var("GOOGLE_CLIENT_ID").context("GOOGLE_CLIENT_ID deve estar no .env")?;
    let google_client_secret =
        std::env::var("GOOGLE_CLIENT_SECRET").context("GOOGLE_CLIENT_SECRET deve estar no .env")?;

    let mongo_uri = std::env::var("MONGODB_URI").context("MONGODB_URI deve estar no .env")?;
    let mongo_repo = MongoRepo::new(&mongo_uri, "rust_ai_ready_stack_db")
        .await
        .context("Falha ao conectar ao MongoDB")?;

    mongo_repo
        .ensure_indexes()
        .await
        .context("Falha ao garantir os índices do MongoDB")?;

    let user_repo: Arc<dyn UserRepository> = Arc::new(mongo_repo);
    let document_repo: Arc<dyn DocumentRepository> = Arc::new(vector_db);

    let google_oauth_provider: Arc<dyn GoogleOAuthProvider> = Arc::new(
        ReqwestGoogleOAuthProvider::new(google_client_id.clone(), google_client_secret),
    );

    let oauth_authorization_url_builder: Arc<dyn OAuthAuthorizationUrlBuilder> =
        Arc::new(GoogleOAuthAuthorizationUrlBuilder::new(google_client_id));

    let jwt_secret = std::env::var("JWT_SECRET").context("JWT_SECRET deve estar no .env")?;
    let identity_service: Arc<dyn IdentityService> = Arc::new(JwtIdentityService::new(jwt_secret));

    let cookie_domain =
        std::env::var("COOKIE_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let session_cookie_service: Arc<dyn SessionCookieService> =
        Arc::new(AxumSessionCookieService::new(cookie_domain));

    let use_cases = UseCases::new(
        user_repo,
        document_repo,
        google_oauth_provider,
        Arc::clone(&identity_service),
        oauth_authorization_url_builder,
        session_cookie_service,
        openai_client,
    );

    let shared_state = Arc::new(AppState::new(use_cases, identity_service));

    let protected_routes = Router::new()
        .route("/chat", post(chat_conversation))
        .route("/profile", get(get_user_profile))
        .layer(axum_middleware::from_fn_with_state(
            shared_state.clone(),
            require_jwt_cookie,
        ));

    let public_routes = Router::new()
        .route("/sign-out", post(sign_out))
        .route("/auth", get(google_oauth))
        .route("/auth/callback", get(google_oauth_callback))
        .route("/ingest", post(ingest_document));

    let app = Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(CookieLayer::default())
        .layer(cors)
        .layer(CatchPanicLayer::new())
        .layer(axum_middleware::from_fn(error_handler_middleware))
        .with_state(shared_state);

    let port = std::env::var("PORT").unwrap_or("5555".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap_or(5555)));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Falha ao realizar bind no endereço {}", addr))?;

    axum::serve(listener, app)
        .await
        .context("Falha ao executar servidor HTTP")?;

    Ok(())
}
