use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

use crate::domain::errors::DomainError;

/// Enum centralizado para todos os erros da aplicação
/// Implementa o padrão thiserror para mensagens de erro limpas
#[derive(Error, Debug)]
pub enum AppError {
    /// Erro ao processar requisição OpenAI
    #[error("Erro ao processar OpenAI: {0}")]
    OpenAiError(String),

    /// Erro ao acessar banco de dados vetorial
    #[error("Erro no banco de dados vetorial: {0}")]
    VectorDbError(String),

    /// Recurso não encontrado
    #[error("Recurso não encontrado: {0}")]
    NotFound(String),

    /// Erro de validação de entrada
    #[error("Erro de validação: {0}")]
    ValidationError(String),

    /// Erro de autenticação (JWT, cookies, etc)
    #[error("Erro de autenticação: {0}")]
    AuthenticationError(String),

    /// Erro de autorização (permissões insuficientes)
    #[error("Erro de autorização: {0}")]
    AuthorizationError(String),

    /// Erro genérico interno do servidor
    #[error("Erro interno do servidor: {0}")]
    InternalError(String),

    /// Erro ao serializar/desserializar JSON
    #[error("Erro ao processar JSON: {0}")]
    JsonError(String),

    /// Erro ao fazer requisição HTTP externa
    #[error("Erro ao fazer requisição HTTP: {0}")]
    HttpError(String),

    #[error("Erro de requisição inválida: {0}")]
    BadRequest(String),

    #[error("Erro de conflito: {0}")]
    Conflict(String),
}

impl AppError {
    /// Retorna o status HTTP apropriado para este erro
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,

            AppError::ValidationError(_)
            | AppError::OpenAiError(_)
            | AppError::VectorDbError(_)
            | AppError::HttpError(_)
            | AppError::JsonError(_)
            | AppError::InternalError(_)
            | AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

/// Implementa a conversão de DomainError para AppError
/// Centraliza o mapeamento entre camada de domínio e aplicação
impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::InvalidEmail(msg) => {
                AppError::ValidationError(format!("Email inválido: {}", msg))
            }
            DomainError::InvalidDomain(msg) => {
                AppError::ValidationError(format!("Domínio inválido: {}", msg))
            }
            DomainError::InvalidId(msg) => {
                AppError::ValidationError(format!("ID inválido: {}", msg))
            }
            DomainError::NotFound(msg) => AppError::NotFound(msg),
            DomainError::Conflict(msg) => AppError::Conflict(msg),
            DomainError::Unauthorized(msg) => AppError::AuthenticationError(msg),
            DomainError::Forbidden(msg) => AppError::AuthorizationError(msg),
            DomainError::ValidationError(msg) => AppError::ValidationError(msg),
            DomainError::BusinessRuleViolation(msg) => {
                AppError::BadRequest(format!("Violação de regra de negócio: {}", msg))
            }
            DomainError::InvalidData(msg) => {
                AppError::BadRequest(format!("Dados inválidos: {}", msg))
            }
            DomainError::VectorDbError(msg) => AppError::VectorDbError(msg),
        }
    }
}

/// Type alias para Result usando AppError
pub type AppResult<T> = Result<T, AppError>;

/// Implementa IntoResponse para que o Axum saiba como converter AppError em resposta HTTP
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_message = self.to_string();

        // Log do erro (será usado pelo middleware global)
        eprintln!("[ERROR] {}: {}", status, error_message);

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));

        (status, body).into_response()
    }
}

// Conversões automáticas de erros comuns para AppError
// Essas implementações permitem usar ? com esses tipos

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::JsonError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            AppError::NotFound("test".into()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::ValidationError("test".into()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::AuthenticationError("test".into()).status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::AuthorizationError("test".into()).status_code(),
            StatusCode::FORBIDDEN
        );
    }
}
