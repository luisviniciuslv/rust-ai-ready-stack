use crate::adapters::openai::to_domain_error;
use crate::domain::errors::DomainError;
use crate::domain::ports::EmbeddingGenerator;
use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::{config::OpenAIConfig, Client};
use async_trait::async_trait;

pub struct OpenAiEmbeddingGenerator {
    client: Client<OpenAIConfig>,
}

impl OpenAiEmbeddingGenerator {
    pub fn new(client: Client<OpenAIConfig>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl EmbeddingGenerator for OpenAiEmbeddingGenerator {
    async fn generate_embeddings_batch(
        &self,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>, DomainError> {
        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-3-small")
            .input(texts)
            .build()
            .map_err(to_domain_error)?;

        let response = self
            .client
            .embeddings()
            .create(request)
            .await
            .map_err(to_domain_error)?;

        Ok(response.data.into_iter().map(|d| d.embedding).collect())
    }
}
