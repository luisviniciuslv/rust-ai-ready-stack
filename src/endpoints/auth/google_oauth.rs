use axum::http::{header::SET_COOKIE, HeaderValue};
use axum::response::Redirect;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::env;
use std::sync::Arc;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct GoogleOAuthCallback {
    pub code: String,
}

pub async fn google_oauth(State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let url_backend = env::var("URL_BACKEND").unwrap_or("http://localhost:5555".to_string());
    let redirect_url = state
        .use_cases
        .build_google_oauth_url
        .execute(&url_backend)?;

    Ok(Redirect::permanent(&redirect_url))
}

pub async fn google_oauth_callback(
    State(state): State<Arc<AppState>>,
    query: Query<GoogleOAuthCallback>,
) -> AppResult<impl IntoResponse> {
    let url_backend = env::var("URL_BACKEND").unwrap_or("http://localhost:5555".to_string());
    let url_frontend = env::var("URL_FRONTEND").unwrap_or("http://localhost:3000".to_string());

    let auth_result = state
        .use_cases
        .authorize_google_user
        .execute(&query.code, &url_backend, &url_frontend)
        .await?;

    if let Some(cookie) = auth_result.auth_cookie {
        let set_cookie = HeaderValue::from_str(&cookie)
            .map_err(|e| AppError::InternalError(format!("Cookie inválido: {}", e)))?;
        return Ok((
            [(SET_COOKIE, set_cookie)],
            Redirect::to(&auth_result.redirect_url),
        )
            .into_response());
    }

    Ok(Redirect::to(&auth_result.redirect_url).into_response())
}
