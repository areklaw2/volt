use crate::domain::conversation::{Conversation, ConversationKind};
use crate::domain::errors::DomainError;
use crate::domain::events::DomainEvent;
use crate::domain::ids::{ConversationId, UserId};
use crate::domain::repository::{ConversationRepository, EventPublisher};

pub struct CreateConversationCommand {
    pub kind: ConversationKind,
    pub creator_id: UserId,
    pub participants: Vec<UserId>,
    pub title: Option<String>,
}

pub struct CreateConversationHandler<C: ConversationRepository, P: EventPublisher> {
    conversations: C,
    events: P,
}

impl<C: ConversationRepository, P: EventPublisher> CreateConversationHandler<C, P> {
    pub fn new(conversations: C, events: P) -> Self {
        Self { conversations, events }
    }

    pub async fn handle(&self, command: CreateConversationCommand) -> Result<ConversationId, DomainError> {
        let id = ConversationId::new();

        let conversation = match command.kind {
            ConversationKind::Direct => {
                let other = command
                    .participants
                    .into_iter()
                    .find(|p| p != &command.creator_id)
                    .ok_or(DomainError::DirectWithSelf)?;
                Conversation::new_direct(id, command.creator_id, other)?
            }
            ConversationKind::Group => {
                let title = command.title.ok_or(DomainError::GroupNeedsTitle)?;
                let mut conversation = Conversation::new_group(id, title, command.creator_id.clone())?;

                for participant in command.participants {
                    if participant == command.creator_id {
                        continue;
                    }
                    conversation.add_participant(participant)?;
                }

                conversation
            }
        };

        self.conversations
            .save(&conversation)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        // notify every participant (including the creator) that this conversation now includes them
        for participant in conversation.participants() {
            self.events
                .publish(DomainEvent::ParticipantAdded {
                    conversation_id: conversation.id().clone(),
                    user_id: participant.user_id.clone(),
                })
                .await
                .ok();
        }

        Ok(conversation.id().clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;
    use crate::domain::repository::{PublishError, RepoError};

    struct MockConversationRepository;

    #[async_trait]
    impl ConversationRepository for MockConversationRepository {
        async fn find_by_id(&self, _id: &ConversationId) -> Result<Option<Conversation>, RepoError> {
            Ok(None)
        }

        async fn save(&self, _conversation: &Conversation) -> Result<(), RepoError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockEventPublisher {
        published: Mutex<Vec<DomainEvent>>,
    }

    #[async_trait]
    impl EventPublisher for MockEventPublisher {
        async fn publish(&self, event: DomainEvent) -> Result<(), PublishError> {
            self.published.lock().unwrap().push(event);
            Ok(())
        }
    }

    #[tokio::test]
    async fn handle_publishes_participant_added_for_both_sides_of_a_direct_conversation() {
        let handler = CreateConversationHandler::new(MockConversationRepository, MockEventPublisher::default());
        let creator = UserId::new();
        let other = UserId::new();

        handler
            .handle(CreateConversationCommand {
                kind: ConversationKind::Direct,
                creator_id: creator.clone(),
                participants: vec![creator.clone(), other.clone()],
                title: None,
            })
            .await
            .unwrap();

        let published = handler.events.published.lock().unwrap();
        assert_eq!(published.len(), 2);
        let notified: Vec<UserId> = published
            .iter()
            .map(|e| match e {
                DomainEvent::ParticipantAdded { user_id, .. } => user_id.clone(),
                _ => panic!("expected ParticipantAdded event"),
            })
            .collect();
        assert!(notified.contains(&creator));
        assert!(notified.contains(&other));
    }

    #[tokio::test]
    async fn handle_publishes_participant_added_for_every_group_member() {
        let handler = CreateConversationHandler::new(MockConversationRepository, MockEventPublisher::default());
        let creator = UserId::new();
        let member = UserId::new();

        handler
            .handle(CreateConversationCommand {
                kind: ConversationKind::Group,
                creator_id: creator.clone(),
                participants: vec![creator.clone(), member.clone()],
                title: Some("Group".into()),
            })
            .await
            .unwrap();

        let published = handler.events.published.lock().unwrap();
        assert_eq!(published.len(), 2);
    }

    #[tokio::test]
    async fn handle_rejects_direct_without_a_second_participant() {
        let handler = CreateConversationHandler::new(MockConversationRepository, MockEventPublisher::default());
        let creator = UserId::new();

        let result = handler
            .handle(CreateConversationCommand {
                kind: ConversationKind::Direct,
                creator_id: creator.clone(),
                participants: vec![creator],
                title: None,
            })
            .await;

        assert_eq!(result.err(), Some(DomainError::DirectWithSelf));
        assert!(handler.events.published.lock().unwrap().is_empty());
    }
}
