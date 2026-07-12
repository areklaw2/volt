use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::application::queries::conversation_list::{ConversationView, ConversationViewQueries, ParticipantView, QueryError};
use crate::application::queries::message_history::{MessageHistoryQueries, MessageHistoryQuery, MessageView, QueryError as MessageQueryError};
use crate::domain::ids::{ConversationId, UserId};

#[derive(Clone)]
pub struct SqlxViewQueries {
    pool: PgPool,
}

impl SqlxViewQueries {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn participants_by_conversation(&self, conversation_ids: &[Uuid]) -> Result<HashMap<Uuid, Vec<ParticipantView>>, QueryError> {
        let rows = sqlx::query!(
            "SELECT uc.conversation_id, uc.user_id, u.username, u.display_name, uc.joined_at, uc.last_seen_at
             FROM user_conversations uc
             JOIN users u ON u.id = uc.user_id
             WHERE uc.conversation_id = ANY($1)",
            conversation_ids
        )
        .fetch_all(&self.pool)
        .await?;

        let mut grouped: HashMap<Uuid, Vec<ParticipantView>> = HashMap::new();
        for r in rows {
            grouped.entry(r.conversation_id).or_default().push(ParticipantView {
                user_id: r.user_id.to_string(),
                username: r.username,
                display_name: r.display_name,
                joined_at: r.joined_at,
                last_read_at: r.last_seen_at,
            });
        }
        Ok(grouped)
    }
}

#[async_trait]
impl ConversationViewQueries for SqlxViewQueries {
    async fn for_user(&self, user_id: &UserId) -> Result<Vec<ConversationView>, QueryError> {
        let rows = sqlx::query!(
            "SELECT c.id, c.kind::text AS \"kind!\", c.title, c.created_at, c.updated_at
             FROM conversations c
             JOIN user_conversations uc ON uc.conversation_id = c.id
             WHERE uc.user_id = $1
             ORDER BY c.updated_at DESC",
            Uuid::from(user_id.clone())
        )
        .fetch_all(&self.pool)
        .await?;

        let conversation_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let mut participants_by_conversation = self.participants_by_conversation(&conversation_ids).await?;

        Ok(rows
            .into_iter()
            .map(|r| ConversationView {
                id: r.id.to_string(),
                conversation_type: r.kind,
                name: r.title,
                participants: participants_by_conversation.remove(&r.id).unwrap_or_default(),
                created_at: r.created_at,
                updated_at: Some(r.updated_at),
            })
            .collect())
    }

    async fn by_id(&self, id: &ConversationId) -> Result<Option<ConversationView>, QueryError> {
        let row = sqlx::query!(
            "SELECT id, kind::text AS \"kind!\", title, created_at, updated_at FROM conversations WHERE id = $1",
            Uuid::from(id.clone())
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(row) = row else { return Ok(None) };

        let participants = self
            .participants_by_conversation(&[row.id])
            .await?
            .remove(&row.id)
            .unwrap_or_default();

        Ok(Some(ConversationView {
            id: row.id.to_string(),
            conversation_type: row.kind,
            name: row.title,
            participants,
            created_at: row.created_at,
            updated_at: Some(row.updated_at),
        }))
    }
}

#[async_trait]
impl MessageHistoryQueries for SqlxViewQueries {
    async fn for_conversation(&self, query: MessageHistoryQuery) -> Result<Vec<MessageView>, MessageQueryError> {
        let rows = sqlx::query!(
            "SELECT id, conversation_id, sender_id, content, kind::text AS \"kind!\", edited, created_at, updated_at
             FROM messages
             WHERE conversation_id = $1
             ORDER BY created_at ASC
             OFFSET $2 LIMIT $3",
            Uuid::from(query.conversation_id),
            query.offset.unwrap_or(0),
            query.limit.unwrap_or(i64::MAX)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| MessageView {
                id: r.id.to_string(),
                conversation_id: r.conversation_id.to_string(),
                sender_id: r.sender_id.to_string(),
                content: r.content,
                kind: r.kind,
                edited: r.edited,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }
}
