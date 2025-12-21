use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Ulid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ConverstaionKind {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "group")]
    Group,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Conversation {
    pub id: Ulid,
    #[serde(rename = "type")]
    pub kind: ConverstaionKind,
    pub title: Option<String>,
    pub participants: Vec<Ulid>,
    pub created_at: String,
    pub last_message_id: Ulid,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserConversation {
    pub userid: Ulid,
    pub conversation_id: Ulid,
    pub last_read_message_id: Option<Ulid>,
    pub unread_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum MessageKind {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: Ulid,
    pub conversation_id: Ulid,
    pub sender_id: Ulid,
    pub content: String,
    #[serde(rename = "type")]
    pub kind: MessageKind,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReadReciepts {
    pub message_id: Ulid,
    pub user_id: Ulid,
    pub read_at: String,
}
