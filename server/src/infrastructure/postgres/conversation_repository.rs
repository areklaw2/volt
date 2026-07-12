use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::conversation::{Conversation, ConversationKind, Participant};
use crate::domain::ids::{ConversationId, UserId};
use crate::domain::repository::{ConversationRepository, RepoError};

#[derive(Clone)]
pub struct SqlxConversationRepository {
    pool: PgPool,
}

impl SqlxConversationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConversationRepository for SqlxConversationRepository {
    async fn find_by_id(&self, id: &ConversationId) -> Result<Option<Conversation>, RepoError> {
        let conv = sqlx::query!(
            "SELECT id, kind AS \"kind: ConversationKind\", title, created_at FROM conversations WHERE id = $1",
            Uuid::from(id.clone())
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(conv) = conv else { return Ok(None) };

        let parts = sqlx::query!(
            "SELECT user_id, joined_at FROM user_conversations WHERE conversation_id = $1",
            Uuid::from(id.clone())
        )
        .fetch_all(&self.pool)
        .await?;

        let participants = parts
            .into_iter()
            .map(|p| Participant {
                user_id: UserId::from_persistence(p.user_id),
                joined_at: p.joined_at,
            })
            .collect();

        Ok(Some(Conversation::from_persistence(
            ConversationId::from_persistence(conv.id),
            conv.kind,
            conv.title,
            participants,
            conv.created_at,
        )))
    }

    async fn save(&self, c: &Conversation) -> Result<(), RepoError> {
        let mut tx = self.pool.begin().await?;

        // note: no updated_at in this statement — projector owns that column
        sqlx::query!(
            "INSERT INTO conversations (id, kind, title, created_at)
               VALUES ($1, $2::conversation_kind, $3, $4)
               ON CONFLICT (id) DO UPDATE SET title = $3",
            Uuid::from(c.id().clone()),
            c.kind().clone() as _,
            c.title().as_deref(),
            *c.created_at()
        )
        .execute(&mut *tx)
        .await?;

        for participant in c.participants() {
            sqlx::query!(
                "INSERT INTO user_conversations (user_id, conversation_id, joined_at)
                   VALUES ($1, $2, $3)
                   ON CONFLICT (user_id, conversation_id) DO NOTHING",
                Uuid::from(participant.user_id.clone()),
                Uuid::from(c.id().clone()),
                participant.joined_at
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
