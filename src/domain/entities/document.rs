use std::fmt;
use std::str::FromStr;

/// Categoria do processo para roteamento e filtragem de contexto
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessCategory {
    Commercial,
    Support,
    Renewal,
    Financial,
}

impl ProcessCategory {
    pub fn from_label(value: &str) -> Self {
        match value {
            "Comercial" => Self::Commercial,
            "Suporte" => Self::Support,
            "Renovacao" => Self::Renewal,
            "Financeiro" => Self::Financial,
            _ => Self::Commercial,
        }
    }
}

impl fmt::Display for ProcessCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Commercial => "Comercial",
            Self::Support => "Suporte",
            Self::Renewal => "Renovacao",
            Self::Financial => "Financeiro",
        };
        f.write_str(label)
    }
}

impl FromStr for ProcessCategory {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_label(value))
    }
}

/// DocumentChunk representa um pedaço de um documento processado
#[derive(Debug, Clone)]
pub struct DocumentChunk {
    pub content: String,
    pub category: ProcessCategory,
    pub source_filename: String,
}

impl DocumentChunk {
    pub fn new(content: String, category: ProcessCategory, source_filename: String) -> Self {
        Self {
            content,
            category,
            source_filename,
        }
    }
}

/// IngestionResult representa o resultado de uma ingestão bem-sucedida
#[derive(Debug, Clone)]
pub struct IngestionResult {
    pub total_chunks: usize,
    pub message: String,
}

impl IngestionResult {
    pub fn new(total_chunks: usize) -> Self {
        Self {
            total_chunks,
            message: "Documentos indexados com sucesso!".to_string(),
        }
    }
}
