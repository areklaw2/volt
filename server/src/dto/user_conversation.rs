use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateUserConversationRequest {
    pub joined_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ParticipantResponse {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
}
