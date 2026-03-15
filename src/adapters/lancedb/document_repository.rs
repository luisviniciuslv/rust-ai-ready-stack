use crate::adapters::lancedb::{to_domain_error, LanceDbRepo};
use crate::domain::entities::{DocumentChunk, IngestionResult};
use crate::domain::errors::DomainError;
use crate::domain::ports::DocumentRepository;
use async_trait::async_trait;

/// Adaptador LanceDB que implementa DocumentRepository
#[async_trait]
impl DocumentRepository for LanceDbRepo {
    async fn ingest_documents(
        &self,
        chunks: Vec<DocumentChunk>,
        embeddings: Vec<Vec<f32>>,
    ) -> Result<IngestionResult, DomainError> {
        let total_chunks = chunks.len();

        self.add_documents(chunks, embeddings)
            .await
            .map_err(to_domain_error)?;

        Ok(IngestionResult::new(total_chunks))
    }

    async fn search(
        &self,
        embedding: Vec<f32>,
        limit: usize,
        category: Option<String>,
    ) -> Result<Vec<DocumentChunk>, DomainError> {
        self.search(embedding, limit, category)
            .await
            .map_err(to_domain_error)
            .map(|results| {
                results
                    .into_iter()
                    .map(|chunk| {
                        DocumentChunk::new(chunk.content, chunk.category, chunk.source_filename)
                    })
                    .collect()
            })
    }
}
