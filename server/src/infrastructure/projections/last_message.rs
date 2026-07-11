use crate::domain::events::DomainEvent;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn project_last_message(pool: &PgPool, event: &DomainEvent) -> Result<(), sqlx::Error> {
    if let DomainEvent::MessageSent {
        message_id,
        conversation_id,
        created_at,
        ..
    } = event
    {
        sqlx::query!(
            "UPDATE conversations SET last_message_id = $1, updated_at = $2 WHERE id = $3",
            Uuid::from(message_id.clone()),
            created_at,
            Uuid::from(conversation_id.clone())
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}
