use thiserror::Error;

/// Erros de domínio - Sem dependências de infraestrutura
#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Email inválido: {0}")]
    InvalidEmail(String),

    #[error("Domínio inválido: {0}")]
    InvalidDomain(String),

    #[error("ID inválido: {0}")]
    InvalidId(String),

    #[error("Recurso não encontrado: {0}")]
    NotFound(String),

    #[error("Conflito: {0}")]
    Conflict(String),

    #[error("Não autorizado: {0}")]
    Unauthorized(String),

    #[error("Permissão negada: {0}")]
    Forbidden(String),

    #[error("Validação falhou: {0}")]
    ValidationError(String),

    #[error("Erro de negócio: {0}")]
    BusinessRuleViolation(String),

    #[error("Dados inválidos: {0}")]
    InvalidData(String),

    #[error("Erro no banco de dados vetorial: {0}")]
    VectorDbError(String),
}
