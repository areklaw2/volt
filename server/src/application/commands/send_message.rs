use crate::domain::{
    errors::DomainError,
    ids::{ConversationId, MessageId, UserId},
    message::{Message, MessageKind},
    repository::{ConversationRepository, EventPublisher, MessageRepository},
};

pub struct SendMessageCommnad {
    pub conversation_id: ConversationId,
    pub sender_id: UserId,
    pub content: String,
    pub kind: MessageKind,
}

pub struct SendMessageHandler<C: ConversationRepository, M: MessageRepository, P: EventPublisher> {
    conversations: C,
    messages: M,
    events: P,
}

impl<C: ConversationRepository, M: MessageRepository, P: EventPublisher> SendMessageHandler<C, M, P> {
    pub fn new(conversations: C, messages: M, events: P) -> Self {
        Self {
            conversations,
            messages,
            events,
        }
    }

    pub async fn handle(&self, command: SendMessageCommnad) -> Result<MessageId, DomainError> {
        let conversation = self
            .conversations
            .find_by_id(&command.conversation_id)
            .await
            .map_err(|_| DomainError::ConversationNotFound)?
            .ok_or(DomainError::ConversationNotFound)?;

        if !conversation.is_participant(&command.sender_id) {
            return Err(DomainError::NotAParticipant);
        }

        let (message, event) = Message::new(
            MessageId::new(),
            command.conversation_id,
            command.sender_id,
            command.content,
            command.kind,
        )?;

        self.messages.save(&message).await.map_err(|_| DomainError::ConversationNotFound)?;
        self.events.publish(event).await.ok();
        Ok(message.id().clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;
    use crate::domain::conversation::Conversation;
    use crate::domain::events::DomainEvent;
    use crate::domain::repository::{PublishError, RepoError};

    struct MockConversationRepository {
        conversation: Mutex<Option<Conversation>>,
        err: bool,
    }

    #[async_trait]
    impl ConversationRepository for MockConversationRepository {
        async fn find_by_id(&self, _id: &ConversationId) -> Result<Option<Conversation>, RepoError> {
            if self.err {
                return Err(RepoError::Db(sqlx::Error::RowNotFound));
            }
            Ok(self.conversation.lock().unwrap().take())
        }

        async fn save(&self, _conversation: &Conversation) -> Result<(), RepoError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockMessageRepository {
        saved_id: Mutex<Option<MessageId>>,
    }

    #[async_trait]
    impl MessageRepository for MockMessageRepository {
        async fn find_by_id(&self, _id: &MessageId) -> Result<Option<Message>, RepoError> {
            Ok(None)
        }

        async fn save(&self, message: &Message) -> Result<(), RepoError> {
            *self.saved_id.lock().unwrap() = Some(message.id().clone());
            Ok(())
        }
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

    fn command(conversation_id: ConversationId, sender_id: UserId, content: &str) -> SendMessageCommnad {
        SendMessageCommnad {
            conversation_id,
            sender_id,
            content: content.to_string(),
            kind: MessageKind::Text,
        }
    }

    #[tokio::test]
    async fn handle_returns_conversation_not_found_when_missing() {
        let handler = SendMessageHandler::new(
            MockConversationRepository {
                conversation: Mutex::new(None),
                err: false,
            },
            MockMessageRepository::default(),
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(command(ConversationId::new(), UserId::new(), "hello"))
            .await;

        assert_eq!(result.err(), Some(DomainError::ConversationNotFound));
    }

    #[tokio::test]
    async fn handle_returns_conversation_not_found_when_repo_errors() {
        let handler = SendMessageHandler::new(
            MockConversationRepository {
                conversation: Mutex::new(None),
                err: true,
            },
            MockMessageRepository::default(),
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(command(ConversationId::new(), UserId::new(), "hello"))
            .await;

        assert_eq!(result.err(), Some(DomainError::ConversationNotFound));
    }

    #[tokio::test]
    async fn handle_returns_not_a_participant_when_sender_is_not_in_conversation() {
        let conversation =
            Conversation::new_group(ConversationId::new(), "Group".into(), UserId::new()).unwrap();
        let handler = SendMessageHandler::new(
            MockConversationRepository {
                conversation: Mutex::new(Some(conversation)),
                err: false,
            },
            MockMessageRepository::default(),
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(command(ConversationId::new(), UserId::new(), "hello"))
            .await;

        assert_eq!(result.err(), Some(DomainError::NotAParticipant));
    }

    #[tokio::test]
    async fn handle_propagates_domain_error_from_message_validation() {
        let sender = UserId::new();
        let conversation =
            Conversation::new_group(ConversationId::new(), "Group".into(), sender.clone()).unwrap();
        let handler = SendMessageHandler::new(
            MockConversationRepository {
                conversation: Mutex::new(Some(conversation)),
                err: false,
            },
            MockMessageRepository::default(),
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(command(ConversationId::new(), sender, "   "))
            .await;

        assert_eq!(result.err(), Some(DomainError::EmptyMessage));
    }

    #[tokio::test]
    async fn handle_saves_message_and_publishes_event_on_success() {
        let sender = UserId::new();
        let conversation_id = ConversationId::new();
        let conversation =
            Conversation::new_group(conversation_id.clone(), "Group".into(), sender.clone()).unwrap();
        let handler = SendMessageHandler::new(
            MockConversationRepository {
                conversation: Mutex::new(Some(conversation)),
                err: false,
            },
            MockMessageRepository::default(),
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(command(conversation_id.clone(), sender.clone(), "hello"))
            .await;

        let message_id = result.unwrap();
        assert_eq!(
            handler.messages.saved_id.lock().unwrap().as_ref(),
            Some(&message_id)
        );

        match &*handler.events.published.lock().unwrap() {
            Some(DomainEvent::MessageSent {
                message_id: event_id,
                conversation_id: event_conversation_id,
                sender_id: event_sender_id,
                ..
            }) => {
                assert_eq!(event_id, &message_id);
                assert_eq!(event_conversation_id, &conversation_id);
                assert_eq!(event_sender_id, &sender);
            }
            _ => panic!("expected MessageSent event"),
        }
    }
}
