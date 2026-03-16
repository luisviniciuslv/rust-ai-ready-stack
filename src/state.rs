use crate::{
    adapters::openai::OpenAiEmbeddingGenerator,
    application::use_cases::{
        auth::{
            AuthorizeGoogleUserUseCase, BuildGoogleOAuthUrlUseCase, GetUserProfileUseCase,
            LocalSignInConfig, LocalSignInUseCase, SignOutUseCase,
        },
        chat::{ChatConversationUseCase, IngestDocumentUseCase},
    },
    domain::ports::{
        DocumentRepository, GoogleOAuthProvider, IdentityService, OAuthAuthorizationUrlBuilder,
        SessionCookieService, UserRepository,
    },
};
use async_openai::{config::OpenAIConfig, Client};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct UseCases {
    pub build_google_oauth_url: BuildGoogleOAuthUrlUseCase,
    pub authorize_google_user: AuthorizeGoogleUserUseCase,
    pub local_sign_in: LocalSignInUseCase,
    pub chat_conversation: ChatConversationUseCase,
    pub ingest_document: IngestDocumentUseCase,
    pub sign_out: SignOutUseCase,
    pub get_user_profile: GetUserProfileUseCase,
}

pub struct AppState {
    pub conversations: RwLock<HashMap<String, crate::domain::entities::Conversation>>,
    pub use_cases: UseCases,
    pub identity_service: Arc<dyn IdentityService>,
}

impl UseCases {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        document_repo: Arc<dyn DocumentRepository>,
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        identity_service: Arc<dyn IdentityService>,
        oauth_authorization_url_builder: Arc<dyn OAuthAuthorizationUrlBuilder>,
        session_cookie_service: Arc<dyn SessionCookieService>,
        local_sign_in_config: LocalSignInConfig,
        require_google_verified_email: bool,
        allowed_google_email_domains: Vec<String>,
        openai_client: Client<OpenAIConfig>,
    ) -> Self {
        Self {
            build_google_oauth_url: BuildGoogleOAuthUrlUseCase::new(Arc::clone(
                &oauth_authorization_url_builder,
            )),
            authorize_google_user: AuthorizeGoogleUserUseCase::new(
                Arc::clone(&google_oauth_provider),
                Arc::clone(&identity_service),
                Arc::clone(&session_cookie_service),
                require_google_verified_email,
                allowed_google_email_domains,
            ),
            local_sign_in: LocalSignInUseCase::new(
                Arc::clone(&identity_service),
                Arc::clone(&session_cookie_service),
                local_sign_in_config,
            ),
            chat_conversation: ChatConversationUseCase::new_openai(
                openai_client.clone(),
                Arc::clone(&document_repo),
                Arc::new(OpenAiEmbeddingGenerator::new(openai_client.clone())),
            ),
            ingest_document: IngestDocumentUseCase::new(
                Arc::clone(&document_repo),
                Arc::new(OpenAiEmbeddingGenerator::new(openai_client.clone())),
            ),
            sign_out: SignOutUseCase::new(Arc::clone(&session_cookie_service)),
            get_user_profile: GetUserProfileUseCase::new(Arc::clone(&user_repo)),
        }
    }
}

impl AppState {
    pub fn new(use_cases: UseCases, identity_service: Arc<dyn IdentityService>) -> Self {
        Self {
            conversations: RwLock::new(HashMap::new()),
            use_cases,
            identity_service,
        }
    }
}
