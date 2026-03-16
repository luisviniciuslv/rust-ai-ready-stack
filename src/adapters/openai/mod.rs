use crate::domain::errors::DomainError;

pub mod embedding_generator;
pub mod rag_chat_provider;

pub use embedding_generator::OpenAiEmbeddingGenerator;
pub use rag_chat_provider::OpenAiRagChatProvider;

pub(crate) fn to_domain_error<E>(_: E) -> DomainError
where
    E: std::fmt::Display,
{
    DomainError::VectorDbError("Falha ao gerar embeddings".to_string())
}
