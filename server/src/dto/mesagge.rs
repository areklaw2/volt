use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateMessageRequest {
    pub conversation_id: Uuid,
    pub sender_id: String,
    pub content: String,
}
