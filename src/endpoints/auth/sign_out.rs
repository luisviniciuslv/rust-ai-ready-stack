use std::sync::Arc;

use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderValue},
    response::IntoResponse,
};

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

pub async fn sign_out(State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let cookie = state.use_cases.sign_out.execute();
    let cookie = HeaderValue::from_str(&cookie)
        .map_err(|e| AppError::InternalError(format!("Cookie inválido: {}", e)))?;
    Ok([(SET_COOKIE, cookie)])
}
