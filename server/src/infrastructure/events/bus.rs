use async_trait::async_trait;
use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::{
    domain::{
        events::DomainEvent,
        repository::{EventPublisher, PublishError},
    },
    infrastructure::projections::run_projections,
};

#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<DomainEvent>,
    pool: PgPool,
}

impl EventBus {
    pub fn new(pool: PgPool) -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { tx, pool }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.tx.subscribe()
    }
}

#[async_trait]
impl EventPublisher for EventBus {
    async fn publish(&self, event: DomainEvent) -> Result<(), PublishError> {
        run_projections(&self.pool, &event).await?;

        let _ = self.tx.send(event);
        Ok(())
    }
}
