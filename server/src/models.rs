use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Ulid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Conversation {
    pub id: Ulid,
    pub converstion_type: ConverstaionType,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConverstaionType {
    Direct,
    Group,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Participant {
    pub conversation_id: Ulid,
    pub user_id: Ulid,
    pub joined_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: Ulid,
    pub conversation_id: Ulid,
    pub sender_id: Ulid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
