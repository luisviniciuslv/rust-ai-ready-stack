use crate::domain::entities::{DocumentChunk, ProcessCategory};

/// Estratégia de Chunking corrigida para suportar UTF-8
pub fn create_chunks(
    text: &str,
    category: ProcessCategory,
    filename: &str,
    chunk_size: usize,
    overlap: usize,
) -> Vec<DocumentChunk> {
    // Convertemos a string para um vetor de caracteres para evitar erro de byte boundary
    let chars: Vec<char> = text.chars().collect();
    let mut chunks = Vec::new();
    let mut start = 0;
    let char_count = chars.len();

    if chunk_size == 0 || overlap >= chunk_size {
        return chunks; // Evita loop infinito
    }

    while start < char_count {
        let end = std::cmp::min(start + chunk_size, char_count);

        // Criamos a string a partir do slice de caracteres com segurança
        let chunk_slice: String = chars[start..end].iter().collect();

        chunks.push(DocumentChunk {
            content: chunk_slice,
            category,
            source_filename: filename.to_string(),
        });

        // Condição de saída para evitar loop no último chunk
        if start + chunk_size >= char_count {
            break;
        }

        start += chunk_size - overlap;
    }

    chunks
}
