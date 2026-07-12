use crate::domain::errors::DomainError;
use crate::domain::events::DomainEvent;
use crate::domain::ids::{ConversationId, MessageId, UserId};
use crate::domain::repository::EventPublisher;

pub struct MarkMessageReadCommand {
    pub conversation_id: ConversationId,
    pub user_id: UserId,
    pub message_id: MessageId,
}

pub struct MarkReadHandler<P: EventPublisher> {
    events: P,
}

impl<P: EventPublisher> MarkReadHandler<P> {
    pub fn new(events: P) -> Self {
        Self { events }
    }

    pub async fn handle(&self, cmd: MarkMessageReadCommand) -> Result<(), DomainError> {
        self.events
            .publish(DomainEvent::ConversationRead {
                conversation_id: cmd.conversation_id,
                user_id: cmd.user_id,
                up_to: cmd.message_id,
            })
            .await
            .ok();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;
    use crate::domain::repository::PublishError;

    #[derive(Default)]
    struct MockEventPublisher {
        published: Mutex<Option<DomainEvent>>,
    }

    #[async_trait]
    impl EventPublisher for MockEventPublisher {
        async fn publish(&self, event: DomainEvent) -> Result<(), PublishError> {
            *self.published.lock().unwrap() = Some(event);
            Ok(())
        }
    }

    struct FailingEventPublisher;

    #[async_trait]
    impl EventPublisher for FailingEventPublisher {
        async fn publish(&self, _event: DomainEvent) -> Result<(), PublishError> {
            Err(PublishError::Db(sqlx::Error::RowNotFound))
        }
    }

    #[tokio::test]
    async fn handle_publishes_conversation_read_event() {
        let handler = MarkReadHandler {
            events: MockEventPublisher::default(),
        };
        let conversation_id = ConversationId::new();
        let user_id = UserId::new();
        let message_id = MessageId::new();

        let result = handler
            .handle(MarkMessageReadCommand {
                conversation_id: conversation_id.clone(),
                user_id: user_id.clone(),
                message_id: message_id.clone(),
            })
            .await;

        assert!(result.is_ok());
        match &*handler.events.published.lock().unwrap() {
            Some(DomainEvent::ConversationRead {
                conversation_id: event_conversation_id,
                user_id: event_user_id,
                up_to,
            }) => {
                assert_eq!(event_conversation_id, &conversation_id);
                assert_eq!(event_user_id, &user_id);
                assert_eq!(up_to, &message_id);
            }
            _ => panic!("expected ConversationRead event"),
        }
    }

    #[tokio::test]
    async fn handle_returns_ok_even_when_publish_fails() {
        let handler = MarkReadHandler {
            events: FailingEventPublisher,
        };

        let result = handler
            .handle(MarkMessageReadCommand {
                conversation_id: ConversationId::new(),
                user_id: UserId::new(),
                message_id: MessageId::new(),
            })
            .await;

        assert!(result.is_ok());
    }
}
