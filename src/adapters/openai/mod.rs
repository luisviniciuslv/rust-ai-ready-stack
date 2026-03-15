use crate::domain::errors::DomainError;

pub mod embedding_generator;

pub use embedding_generator::OpenAiEmbeddingGenerator;

pub(crate) fn to_domain_error<E>(_: E) -> DomainError
where
    E: std::fmt::Display,
{
    DomainError::VectorDbError("Falha ao gerar embeddings".to_string())
}
