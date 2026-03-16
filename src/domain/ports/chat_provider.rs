use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Resposta normalizada retornada por qualquer provedor de chat LLM.
pub struct ChatProviderResponse {
    pub content: String,
    pub category: String,
}

/// Porta de saída genérica para provedores de LLM (chat).
///
/// Implementações disponíveis no boilerplate:
/// - [`OpenAiRagChatProvider`](crate::adapters::openai::OpenAiRagChatProvider) — OpenAI GPT com RAG
/// - `AnthropicChatProvider` — Claude (Anthropic) · **a implementar**
/// - `OllamaChatProvider` — modelos locais via Ollama · **a implementar**
///
/// Para trocar de provedor basta passar uma implementação diferente ao construir
/// [`ChatConversationUseCase`](crate::application::use_cases::chat::ChatConversationUseCase).
#[async_trait]
pub trait ChatProvider: Send + Sync {
    /// Processa a mensagem do usuário dentro do contexto do histórico e retorna a resposta
    /// juntamente com a categoria inferida pelo provedor.
    async fn process_message(
        &self,
        user_message: &str,
        conversation_history: &str,
    ) -> Result<ChatProviderResponse, DomainError>;
}
