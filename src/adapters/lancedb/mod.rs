use crate::domain::errors::DomainError;

pub mod document_repository;
pub mod lancedb_store;

pub use lancedb_store::LanceDbRepo;

pub(crate) fn to_domain_error(_: anyhow::Error) -> DomainError {
    DomainError::VectorDbError("Falha ao acessar banco de dados vetorial".to_string())
}
