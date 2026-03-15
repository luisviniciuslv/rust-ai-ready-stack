use crate::application::ChatConversationInput;
use crate::error::AppResult;
use crate::state::AppState;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ChatConversationRequest {
    pub id: Option<String>,
    pub content: String,
}

/// Endpoint que recebe mensagens do usuário e retorna respostas do assistente
/// Delega a lógica de processamento para o ChatConversationUseCase
pub async fn chat_conversation(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatConversationRequest>,
) -> AppResult<impl IntoResponse> {
    let response = state
        .use_cases
        .chat_conversation
        .execute(
            &state.conversations,
            ChatConversationInput {
                id: req.id,
                content: req.content,
            },
        )
        .await?;

    Ok(Json(serde_json::json!({
        "id": response.id,
        "content": response.content,
        "category": response.category,
        "time": response.time
    })))
}
