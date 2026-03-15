use crate::domain::errors::DomainError;
use crate::error::AppError;
use mongodb::error::{Error as MongoError, ErrorKind, WriteFailure};

pub(crate) mod example_repository;
pub(crate) mod models;
pub(crate) mod mongo_repo;
pub(crate) mod user_repository;

fn is_duplicate_key_error(err: &MongoError) -> bool {
    matches!(
        err.kind.as_ref(),
        ErrorKind::Write(WriteFailure::WriteError(e)) if e.code == 11000
    )
}

pub(crate) fn to_domain_error(err: MongoError) -> DomainError {
    if is_duplicate_key_error(&err) {
        DomainError::Conflict("Registro duplicado".to_string())
    } else {
        DomainError::BusinessRuleViolation("Falha ao acessar persistência de dados".to_string())
    }
}

pub(crate) fn to_app_error(err: MongoError) -> AppError {
    if is_duplicate_key_error(&err) {
        AppError::Conflict("Registro duplicado".to_string())
    } else {
        AppError::InternalError("Falha ao acessar persistência de dados".to_string())
    }
}
