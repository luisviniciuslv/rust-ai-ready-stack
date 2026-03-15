use crate::domain::entities::{DocumentChunk, IngestionResult};
use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Porta de saída para repositório de Documentos (interface)
/// Define operações de armazenamento e recuperação de documentos vetorizados
#[async_trait]
pub trait DocumentRepository: Send + Sync {
    /// Ingere um lote de chunks de documentos com seus embeddings
    async fn ingest_documents(
        &self,
        chunks: Vec<DocumentChunk>,
        embeddings: Vec<Vec<f32>>,
    ) -> Result<IngestionResult, DomainError>;

    /// Busca documentos similar baseado em embedding e categoria
    async fn search(
        &self,
        embedding: Vec<f32>,
        limit: usize,
        category: Option<String>,
    ) -> Result<Vec<DocumentChunk>, DomainError>;
}
