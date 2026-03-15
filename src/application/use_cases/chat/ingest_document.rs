use crate::domain::entities::{DocumentChunk, IngestionResult, ProcessCategory};
use crate::domain::errors::DomainError;
use crate::domain::ports::{DocumentRepository, EmbeddingGenerator};
use crate::utils::chunk_generator;
use std::path::Path;
use std::sync::Arc;

/// Caso de uso genérico para ingestão de arquivos em pipelines RAG.
///
/// Fluxo:
/// 1. Valida o arquivo de entrada (suporta `.txt` e `.md`)
/// 2. Extrai o conteúdo textual
/// 3. Segmenta o texto em chunks com overlap
/// 4. Gera embeddings em lotes
/// 5. Persiste chunks + embeddings em um repositório vetorial
pub struct IngestDocumentUseCase {
    document_repository: Arc<dyn DocumentRepository>,
    embedding_generator: Arc<dyn EmbeddingGenerator>,
}

impl IngestDocumentUseCase {
    pub fn new(
        document_repository: Arc<dyn DocumentRepository>,
        embedding_generator: Arc<dyn EmbeddingGenerator>,
    ) -> Self {
        Self {
            document_repository,
            embedding_generator,
        }
    }

    /// Executa o fluxo completo de ingestão de um arquivo para RAG.
    ///
    /// Etapas:
    /// 1. Validar e extrair texto do arquivo
    /// 2. Criar chunks do texto
    /// 3. Gerar embeddings para cada chunk
    /// 4. Salvar no repositório de documentos
    pub async fn execute(
        &self,
        file_path: &str,
        processing_label: &str,
    ) -> Result<IngestionResult, DomainError> {
        // Etapa 1: Validar e extrair texto
        let raw_text = self.extract_text_from_file(file_path).await?;

        // Converter string de contexto/processo para enum
        let process_category = ProcessCategory::from_label(processing_label);

        // Etapa 2: Criar chunks
        let chunks = chunk_generator::create_chunks(
            &raw_text,
            process_category,
            file_path,
            1000, // Tamanho do chunk
            200,  // Overlap
        );

        let total_chunks = chunks.len();

        // Etapa 3: Gerar embeddings em lotes
        let embeddings = self.generate_embeddings_for_chunks(&chunks).await?;

        // Converter DocumentChunk do chunk_generator para o domínio
        let domain_chunks: Vec<DocumentChunk> = chunks
            .into_iter()
            .map(|c| DocumentChunk::new(c.content, c.category, c.source_filename))
            .collect();

        // Etapa 4: Salvar no repositório
        self.document_repository
            .ingest_documents(domain_chunks, embeddings)
            .await?;

        Ok(IngestionResult::new(total_chunks))
    }

    /// Extrai texto de arquivos `.txt` e `.md` dentro da pasta `files_rag`.
    async fn extract_text_from_file(&self, file_path: &str) -> Result<String, DomainError> {
        let path = Path::new(file_path);

        // Extrair apenas o nome do arquivo
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| DomainError::InvalidData("Nome de arquivo inválido".to_string()))?;

        // Construir caminho seguro
        let safe_path = Path::new("files_rag").join(file_name);

        // Validar que o arquivo existe e possui formato aceito
        let extension = safe_path.extension().and_then(|s| s.to_str());
        let is_supported = matches!(extension, Some("txt") | Some("md"));

        if !safe_path.exists() || !is_supported {
            return Err(DomainError::InvalidData(
                "Arquivo não encontrado ou formato inválido (suportado: .txt, .md)".to_string(),
            ));
        }

        std::fs::read_to_string(&safe_path)
            .map_err(|e| DomainError::VectorDbError(format!("Erro ao ler arquivo: {}", e)))
            .map(|text| text.trim().to_string())
    }

    /// Gera embeddings para chunks em lotes
    async fn generate_embeddings_for_chunks(
        &self,
        chunks: &[DocumentChunk],
    ) -> Result<Vec<Vec<f32>>, DomainError> {
        let mut embeddings: Vec<Vec<f32>> = Vec::new();

        // Processar em lotes de 50 chunks
        for (i, sub_chunks) in chunks.chunks(50).enumerate() {
            let contents: Vec<String> = sub_chunks.iter().map(|c| c.content.clone()).collect();

            let sub_embeddings = self
                .embedding_generator
                .generate_embeddings_batch(contents)
                .await
                .map_err(|e| {
                    DomainError::VectorDbError(format!(
                        "Erro ao gerar embeddings no lote {}: {}",
                        i + 1,
                        e
                    ))
                })?;

            embeddings.extend(sub_embeddings);
        }

        Ok(embeddings)
    }
}
