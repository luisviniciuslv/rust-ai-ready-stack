use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Porta de saída para geração de embeddings
#[async_trait]
pub trait EmbeddingGenerator: Send + Sync {
    async fn generate_embeddings_batch(
        &self,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>, DomainError>;
}
