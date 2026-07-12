use crate::domain::conversation::{Conversation, ConversationKind};
use crate::domain::errors::DomainError;
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
                    let event = conversation.add_participant(participant)?;
                    self.events.publish(event).await.ok();
                }

                conversation
            }
        };

        self.conversations
            .save(&conversation)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(conversation.id().clone())
    }
}
