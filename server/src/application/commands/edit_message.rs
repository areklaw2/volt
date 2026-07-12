use crate::domain::{
    errors::DomainError,
    ids::{MessageId, UserId},
    repository::{EventPublisher, MessageRepository},
};

pub struct EditMessageCommand {
    pub message_id: MessageId,
    pub editor_id: UserId,
    pub content: String,
}

pub struct EditMessageHandler<M: MessageRepository, P: EventPublisher> {
    messages: M,
    events: P,
}

impl<M: MessageRepository, P: EventPublisher> EditMessageHandler<M, P> {
    pub fn new(messages: M, events: P) -> Self {
        Self { messages, events }
    }

    pub async fn handle(&self, command: EditMessageCommand) -> Result<(), DomainError> {
        let mut message = self
            .messages
            .find_by_id(&command.message_id)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or(DomainError::MessageNotFound)?;

        let event = message.edit(&command.editor_id, command.content)?;

        self.messages
            .save(&message)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        self.events.publish(event).await.ok();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;
    use crate::domain::events::DomainEvent;
    use crate::domain::ids::ConversationId;
    use crate::domain::message::{Message, MessageKind};
    use crate::domain::repository::{PublishError, RepoError};

    struct MockMessageRepository {
        message: Mutex<Option<Message>>,
    }

    #[async_trait]
    impl MessageRepository for MockMessageRepository {
        async fn find_by_id(&self, _id: &MessageId) -> Result<Option<Message>, RepoError> {
            Ok(self.message.lock().unwrap().take())
        }

        async fn save(&self, message: &Message) -> Result<(), RepoError> {
            *self.message.lock().unwrap() = Some(clone_message(message));
            Ok(())
        }
    }

    fn clone_message(message: &Message) -> Message {
        Message::from_persistence(
            message.id().clone(),
            message.conversation_id().clone(),
            message.sender_id().clone(),
            message.content().clone(),
            message.kind().clone(),
            *message.edited(),
            *message.created_at(),
            *message.updated_at(),
        )
    }

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

    #[tokio::test]
    async fn handle_returns_message_not_found_when_missing() {
        let handler = EditMessageHandler::new(MockMessageRepository { message: Mutex::new(None) }, MockEventPublisher::default());

        let result = handler
            .handle(EditMessageCommand {
                message_id: MessageId::new(),
                editor_id: UserId::new(),
                content: "hello".into(),
            })
            .await;

        assert_eq!(result.err(), Some(DomainError::MessageNotFound));
    }

    #[tokio::test]
    async fn handle_rejects_editor_who_is_not_sender() {
        let sender = UserId::new();
        let (message, _) = Message::new(MessageId::new(), ConversationId::new(), sender, "hello".into(), MessageKind::Text).unwrap();
        let message_id = message.id().clone();
        let handler = EditMessageHandler::new(
            MockMessageRepository {
                message: Mutex::new(Some(message)),
            },
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(EditMessageCommand {
                message_id,
                editor_id: UserId::new(),
                content: "changed".into(),
            })
            .await;

        assert_eq!(result.err(), Some(DomainError::NotYourMessage));
    }

    #[tokio::test]
    async fn handle_saves_edit_and_publishes_event_on_success() {
        let sender = UserId::new();
        let (message, _) = Message::new(
            MessageId::new(),
            ConversationId::new(),
            sender.clone(),
            "hello".into(),
            MessageKind::Text,
        )
        .unwrap();
        let message_id = message.id().clone();
        let handler = EditMessageHandler::new(
            MockMessageRepository {
                message: Mutex::new(Some(message)),
            },
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(EditMessageCommand {
                message_id: message_id.clone(),
                editor_id: sender,
                content: "updated".into(),
            })
            .await;

        assert!(result.is_ok());

        let saved = handler.messages.message.lock().unwrap().take().unwrap();
        assert_eq!(saved.content(), "updated");
        assert!(*saved.edited());

        match &*handler.events.published.lock().unwrap() {
            Some(DomainEvent::MessageEdited {
                message_id: event_id,
                content,
                ..
            }) => {
                assert_eq!(event_id, &message_id);
                assert_eq!(content, "updated");
            }
            _ => panic!("expected MessageEdited event"),
        }
    }
}
