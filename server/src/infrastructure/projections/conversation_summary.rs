use crate::domain::events::DomainEvent;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn project_conversation_summary(pool: &PgPool, event: &DomainEvent) -> Result<(), sqlx::Error> {
    if let DomainEvent::ConversationRead {
        conversation_id,
        user_id,
        up_to,
    } = event
    {
        sqlx::query!(
            "UPDATE user_conversations
               SET last_read_message_id = $1, last_seen_at = NOW()
               WHERE conversation_id = $2 AND user_id = $3",
            Uuid::from(up_to.clone()),
            Uuid::from(conversation_id.clone()),
            user_id.to_string()
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}
