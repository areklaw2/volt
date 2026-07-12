use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::ids::{ConversationId, UserId};

#[derive(Serialize)]
pub struct ParticipantView {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub joined_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ConversationView {
    pub id: String,
    pub conversation_type: String,
    pub name: Option<String>,
    pub participants: Vec<ParticipantView>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

#[async_trait]
pub trait ConversationViewQueries: Send + Sync {
    async fn for_user(&self, user_id: &UserId) -> Result<Vec<ConversationView>, QueryError>;
    async fn by_id(&self, id: &ConversationId) -> Result<Option<ConversationView>, QueryError>;
}
