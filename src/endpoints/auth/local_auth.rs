use std::sync::Arc;

use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

#[derive(Deserialize)]
pub struct LocalAuthRequest {
    pub email: String,
    pub password: String,
}

pub async fn local_auth_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LocalAuthRequest>,
) -> AppResult<impl IntoResponse> {
    let result = state
        .use_cases
        .local_sign_in
        .execute(payload.email.as_str(), payload.password.as_str())?;

    let cookie = HeaderValue::from_str(&result.auth_cookie)
        .map_err(|e| AppError::InternalError(format!("Cookie inválido: {}", e)))?;

    Ok((StatusCode::NO_CONTENT, [(SET_COOKIE, cookie)]))
}
