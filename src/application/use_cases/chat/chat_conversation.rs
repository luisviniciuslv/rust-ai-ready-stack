use crate::domain::entities::{Conversation, ConversationMessage};
use crate::domain::ports::ChatProvider;
use crate::error::{AppError, AppResult};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct ChatConversationInput {
    pub id: Option<String>,
    pub content: String,
}

pub struct ChatConversationResult {
    pub id: String,
    pub content: String,
    pub category: String,
    pub time: String,
}

/// Caso de uso: Processar mensagem de conversa com histórico e RAG
pub struct ChatConversationUseCase {
    chat_provider: Arc<dyn ChatProvider>,
}

impl ChatConversationUseCase {
    pub fn new(chat_provider: Arc<dyn ChatProvider>) -> Self {
        Self { chat_provider }
    }

    pub async fn execute(
        &self,
        conversations: &RwLock<HashMap<String, Conversation>>,
        req: ChatConversationInput,
    ) -> AppResult<ChatConversationResult> {
        println!(">>> Iniciando execução de ChatConversationUseCase");
        println!(
            "Tipo de requisição: {}",
            if req.id.is_some() {
                "continuação"
            } else {
                "nova conversa"
            }
        );

        let (conversation_id, time_user_message, conversation_history) = {
            let mut store = conversations.write().await;

            let conversation_id = match &req.id {
                Some(id) => {
                    println!("Continuando conversa com ID: {}", id);
                    if let Some(conversation) = store.get_mut(id) {
                        conversation.last_interaction = Utc::now();
                        id.clone()
                    } else {
                        return Err(AppError::NotFound(format!(
                            "Conversa com ID {} não encontrada",
                            id
                        )));
                    }
                }
                None => {
                    let new_id = Uuid::new_v4().to_string();
                    println!("Criando nova conversa com ID: {}", new_id);
                    let new_conv = Conversation {
                        id: new_id.clone(),
                        messages: Vec::new(),
                        last_interaction: Utc::now(),
                    };
                    store.insert(new_id.clone(), new_conv);
                    new_id
                }
            };

            let time_user_message = Utc::now();
            let conversation = store.get_mut(&conversation_id).ok_or_else(|| {
                AppError::InternalError(
                    "Falha ao recuperar conversa após criação/localização".to_string(),
                )
            })?;

            println!(
                "Mensagem do usuário adicionada ao histórico: {}",
                req.content
            );
            conversation.messages.push(ConversationMessage {
                content: req.content.clone(),
                time: time_user_message,
                sender: "User".to_string(),
            });

            let conversation_history = build_conversation_history(&conversation.messages);
            println!(
                "[DEBUG] Histórico atualizado com {} mensagens",
                conversation.messages.len()
            );

            (conversation_id, time_user_message, conversation_history)
        };

        let chat_response = self
            .chat_provider
            .process_message(&req.content, &conversation_history)
            .await?;

        println!("Resposta do processador recebida, adicionando ao histórico");

        {
            let mut store = conversations.write().await;
            let conversation = store.get_mut(&conversation_id).ok_or_else(|| {
                AppError::InternalError(
                    "Falha ao recuperar conversa para persistir resposta".to_string(),
                )
            })?;

            conversation.messages.push(ConversationMessage {
                content: chat_response.content.clone(),
                time: Utc::now(),
                sender: "Assistant".to_string(),
            });

            println!(
                "Resposta persistida. Total de mensagens na conversa: {}",
                conversation.messages.len()
            );
        }

        println!("<<< Execução concluída com sucesso");
        println!(
            "ID da conversa: {}, Categoria: {}, Tempo da resposta: {}",
            conversation_id,
            chat_response.category,
            time_user_message.to_rfc3339()
        );

        Ok(ChatConversationResult {
            id: conversation_id,
            content: chat_response.content,
            category: chat_response.category,
            time: time_user_message.to_rfc3339(),
        })
    }
}

fn build_conversation_history(messages: &[ConversationMessage]) -> String {
    let max_messages = 10;

    messages
        .iter()
        .rev()
        .take(max_messages)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|m| {
            format!(
                "-----\nTime: {}\nRole: {}\nMessage: {}",
                m.time.to_rfc3339(),
                m.sender,
                m.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
