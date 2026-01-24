use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateParticipantsRequest {
    pub sender_id: Uuid,
    pub conversation_id: Uuid,
    pub users: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateParticipantRequest {
    pub joined_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
}
