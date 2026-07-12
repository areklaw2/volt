use crate::domain::{
    errors::DomainError,
    ids::{ConversationId, UserId},
    repository::{ConversationRepository, EventPublisher},
};

pub struct LeaveConversationCommand {
    pub conversation_id: ConversationId,
    pub user_id: UserId,
}

pub struct LeaveConversationHandler<C: ConversationRepository, P: EventPublisher> {
    conversations: C,
    events: P,
}

impl<C: ConversationRepository, P: EventPublisher> LeaveConversationHandler<C, P> {
    pub fn new(conversations: C, events: P) -> Self {
        Self { conversations, events }
    }

    pub async fn handle(&self, command: LeaveConversationCommand) -> Result<(), DomainError> {
        let mut conversation = self
            .conversations
            .find_by_id(&command.conversation_id)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or(DomainError::ConversationNotFound)?;

        let event = conversation.remove_participant(&command.user_id)?;

        self.conversations.save(&conversation).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        self.events.publish(event).await.ok();

        Ok(())
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
    }

    #[async_trait]
    impl ConversationRepository for MockConversationRepository {
        async fn find_by_id(&self, _id: &ConversationId) -> Result<Option<Conversation>, RepoError> {
            Ok(self.conversation.lock().unwrap().take())
        }

        async fn save(&self, _conversation: &Conversation) -> Result<(), RepoError> {
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

    #[tokio::test]
    async fn handle_returns_conversation_not_found_when_missing() {
        let handler = LeaveConversationHandler::new(
            MockConversationRepository { conversation: Mutex::new(None) },
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(LeaveConversationCommand {
                conversation_id: ConversationId::new(),
                user_id: UserId::new(),
            })
            .await;

        assert_eq!(result.err(), Some(DomainError::ConversationNotFound));
    }

    #[tokio::test]
    async fn handle_rejects_direct_conversation() {
        let a = UserId::new();
        let conversation = Conversation::new_direct(ConversationId::new(), a.clone(), UserId::new()).unwrap();
        let conversation_id = conversation.id().clone();
        let handler = LeaveConversationHandler::new(
            MockConversationRepository { conversation: Mutex::new(Some(conversation)) },
            MockEventPublisher::default(),
        );

        let result = handler.handle(LeaveConversationCommand { conversation_id, user_id: a }).await;

        assert_eq!(result.err(), Some(DomainError::CannotLeaveDirect));
    }

    #[tokio::test]
    async fn handle_removes_participant_and_publishes_event_on_success() {
        let creator = UserId::new();
        let member = UserId::new();
        let mut conversation = Conversation::new_group(ConversationId::new(), "Group".into(), creator).unwrap();
        conversation.add_participant(member.clone()).unwrap();
        let conversation_id = conversation.id().clone();
        let handler = LeaveConversationHandler::new(
            MockConversationRepository { conversation: Mutex::new(Some(conversation)) },
            MockEventPublisher::default(),
        );

        let result = handler
            .handle(LeaveConversationCommand {
                conversation_id: conversation_id.clone(),
                user_id: member.clone(),
            })
            .await;

        assert!(result.is_ok());

        match &*handler.events.published.lock().unwrap() {
            Some(DomainEvent::ParticipantRemoved { conversation_id: event_id, user_id }) => {
                assert_eq!(event_id, &conversation_id);
                assert_eq!(user_id, &member);
            }
            _ => panic!("expected ParticipantRemoved event"),
        }
    }
}
