use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ConversationMessage {
    pub sender: String,
    pub content: String,
    #[allow(dead_code)]
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Conversation {
    #[allow(dead_code)]
    pub id: String,
    pub messages: Vec<ConversationMessage>,
    pub last_interaction: DateTime<Utc>,
}
