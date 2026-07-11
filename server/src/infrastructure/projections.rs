use crate::domain::events::DomainEvent;
use sqlx::PgPool;

mod conversation_summary;
mod last_message;

pub use conversation_summary::project_conversation_summary;
pub use last_message::project_last_message;

pub async fn run_projections(pool: &PgPool, event: &DomainEvent) -> Result<(), sqlx::Error> {
    project_last_message(pool, event).await?;
    project_conversation_summary(pool, event).await?;
    Ok(())
}
