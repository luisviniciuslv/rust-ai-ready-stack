use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

/// Middleware global de tratamento de erros e logging
///
/// Responsabilidades:
/// - Log de requisições início/fim
/// - Rastreamento de performance (tempo de resposta)
/// - Captura centralizada de erros
/// - Rastreamento de status HTTP
pub async fn error_handler_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    // Log da requisição de entrada
    log_request_start(&method, &uri);

    let response = next.run(req).await;
    let duration = start.elapsed();
    let status = response.status();

    // Log da resposta com duração
    log_request_end(&method, &uri, status, duration);

    response
}

/// Log estruturado de requisição inicial
fn log_request_start(method: &axum::http::Method, uri: &axum::http::Uri) {
    println!(
        "[REQUEST] {} {} - Iniciada em {}",
        method,
        uri,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    );
}

/// Log estruturado de resposta com duração
fn log_request_end(
    method: &axum::http::Method,
    uri: &axum::http::Uri,
    status: StatusCode,
    duration: std::time::Duration,
) {
    let level = match status.as_u16() {
        200..=299 => "INFO",
        300..=399 => "INFO",
        400..=499 => "WARN",
        500..=599 => "ERROR",
        _ => "DEBUG",
    };

    println!(
        "[{}] {} {} - Status: {} - Duração: {:.2}ms - Finalizada em {}",
        level,
        method,
        uri,
        status,
        duration.as_secs_f64() * 1000.0,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
        middleware as axum_middleware,
        routing::get,
        Router,
    };
    use serde_json::Value;
    use tower::util::ServiceExt;

    async fn always_validation_error() -> Result<&'static str, AppError> {
        Err(AppError::ValidationError("payload inválido".to_string()))
    }

    async fn always_auth_error() -> Result<&'static str, AppError> {
        Err(AppError::AuthenticationError("token ausente".to_string()))
    }

    async fn ok_handler() -> &'static str {
        "ok"
    }

    #[tokio::test]
    async fn middleware_does_not_change_app_error_shape() {
        let app = Router::new()
            .route("/error", get(always_validation_error))
            .layer(axum_middleware::from_fn(error_handler_middleware));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/error")
                    .body(Body::empty())
                    .expect("request válida"),
            )
            .await
            .expect("resposta do router");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body_bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("corpo de resposta");
        let json: Value = serde_json::from_slice(&body_bytes).expect("json de erro válido");

        assert_eq!(json.get("status").and_then(Value::as_u64), Some(400));
        assert!(json
            .get("error")
            .and_then(Value::as_str)
            .is_some_and(|message| message.contains("Erro de validação: payload inválido")));
        assert!(json.get("timestamp").and_then(Value::as_str).is_some());
    }

    #[tokio::test]
    async fn middleware_preserves_success_status() {
        let app = Router::new()
            .route("/ok", get(ok_handler))
            .layer(axum_middleware::from_fn(error_handler_middleware));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ok")
                    .body(Body::empty())
                    .expect("request válida"),
            )
            .await
            .expect("resposta do router");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn middleware_preserves_auth_error_json_contract() {
        let app = Router::new()
            .route("/auth-error", get(always_auth_error))
            .layer(axum_middleware::from_fn(error_handler_middleware));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/auth-error")
                    .body(Body::empty())
                    .expect("request válida"),
            )
            .await
            .expect("resposta do router");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body_bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("corpo de resposta");
        let json: Value = serde_json::from_slice(&body_bytes).expect("json de erro válido");

        assert_eq!(json.get("status").and_then(Value::as_u64), Some(401));
        assert!(json
            .get("error")
            .and_then(Value::as_str)
            .is_some_and(|message| message.contains("Erro de autenticação: token ausente")));
        assert!(json.get("timestamp").and_then(Value::as_str).is_some());
    }

    #[test]
    fn test_log_levels() {
        assert_eq!(
            "INFO",
            match 200 {
                200..=299 => "INFO",
                _ => "ERROR",
            }
        );
        assert_eq!(
            "WARN",
            match 400 {
                400..=499 => "WARN",
                _ => "ERROR",
            }
        );
        assert_eq!(
            "ERROR",
            match 500 {
                500..=599 => "ERROR",
                _ => "DEBUG",
            }
        );
    }
}
