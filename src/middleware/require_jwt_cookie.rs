use std::sync::Arc;

use axum::body::Body;
use axum::extract::Request;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::State, middleware::Next};

use crate::error::AppError;
use crate::state::AppState;

fn extract_token_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookie_header| {
            cookie_header
                .split(';')
                .map(|s| s.trim())
                .find_map(|cookie| {
                    if cookie.starts_with("token=") {
                        Some(cookie.trim_start_matches("token=").to_string())
                    } else {
                        None
                    }
                })
        })
}

pub async fn require_jwt_cookie(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> axum::response::Response {
    let token_opt = extract_token_cookie(req.headers());
    if let Some(token) = token_opt {
        match state.identity_service.validate_token(&token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                next.run(req).await
            }
            Err(_) => AppError::AuthenticationError("Token inválido ou expirado".to_string())
                .into_response(),
        }
    } else {
        AppError::AuthenticationError("Token inválido ou expirado".to_string()).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::extract_token_cookie;
    use axum::http::{header::COOKIE, HeaderMap, HeaderValue};

    #[test]
    fn extract_token_cookie_returns_token_when_present() {
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_static("theme=dark; token=abc123; session=xyz"),
        );

        assert_eq!(extract_token_cookie(&headers).as_deref(), Some("abc123"));
    }

    #[test]
    fn extract_token_cookie_returns_none_when_absent() {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, HeaderValue::from_static("theme=dark; session=xyz"));

        assert_eq!(extract_token_cookie(&headers), None);
    }
}
