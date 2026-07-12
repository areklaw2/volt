use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::ids::ConversationId;

#[derive(Serialize)]
pub struct MessageView {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
    pub kind: String,
    pub edited: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

pub struct MessageHistoryQuery {
    pub conversation_id: ConversationId,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[async_trait]
pub trait MessageHistoryQueries: Send + Sync {
    async fn for_conversation(&self, query: MessageHistoryQuery) -> Result<Vec<MessageView>, QueryError>;
}
