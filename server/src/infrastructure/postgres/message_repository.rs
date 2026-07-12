use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::ids::{ConversationId, MessageId, UserId};
use crate::domain::message::{Message, MessageKind};
use crate::domain::repository::{MessageRepository, RepoError};

#[derive(Clone)]
pub struct SqlxMessageRepository {
    pool: PgPool,
}

impl SqlxMessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepository for SqlxMessageRepository {
    async fn find_by_id(&self, id: &MessageId) -> Result<Option<Message>, RepoError> {
        let row = sqlx::query!(
            "SELECT id, conversation_id, sender_id, content, kind AS \"kind: MessageKind\", edited, created_at, updated_at
             FROM messages WHERE id = $1",
            Uuid::from(id.clone())
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            Message::from_persistence(
                MessageId::from_persistence(r.id),
                ConversationId::from_persistence(r.conversation_id),
                UserId::from_persistence(r.sender_id),
                r.content,
                r.kind,
                r.edited,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    async fn save(&self, message: &Message) -> Result<(), RepoError> {
        sqlx::query!(
            "INSERT INTO messages (id, conversation_id, sender_id, content, kind, edited, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5::message_kind, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET content = $4, edited = $6, updated_at = $8",
            Uuid::from(message.id().clone()),
            Uuid::from(message.conversation_id().clone()),
            Uuid::from(message.sender_id().clone()),
            message.content().as_str(),
            message.kind().clone() as _,
            *message.edited(),
            *message.created_at(),
            *message.updated_at()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
