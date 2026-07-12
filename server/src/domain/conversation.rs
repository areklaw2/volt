use chrono::{DateTime, Utc};
use getset::Getters;

use crate::domain::{
    errors::DomainError,
    events::DomainEvent,
    ids::{ConversationId, UserId},
};

#[derive(Debug, PartialEq, Clone, sqlx::Type)]
#[sqlx(type_name = "conversation_kind", rename_all = "lowercase")]
pub enum ConversationKind {
    Direct,
    Group,
}

#[derive(Debug, PartialEq)]
pub struct Participant {
    pub user_id: UserId,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Getters, PartialEq)]
pub struct Conversation {
    #[getset(get = "pub")]
    id: ConversationId,
    #[getset(get = "pub")]
    kind: ConversationKind,
    #[getset(get = "pub")]
    title: Option<String>,
    #[getset(get = "pub")]
    participants: Vec<Participant>,
    #[getset(get = "pub")]
    created_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new_direct(id: ConversationId, a: UserId, b: UserId) -> Result<Self, DomainError> {
        if a == b {
            return Err(DomainError::DirectWithSelf);
        }

        let now = Utc::now();
        Ok(Self {
            id,
            kind: ConversationKind::Direct,
            title: None,
            participants: vec![
                Participant {
                    user_id: a,
                    joined_at: now,
                },
                Participant {
                    user_id: b,
                    joined_at: now,
                },
            ],
            created_at: now,
        })
    }

    pub fn new_group(id: ConversationId, title: String, creator: UserId) -> Result<Self, DomainError> {
        if title.trim().is_empty() {
            return Err(DomainError::GroupNeedsTitle);
        }

        let now = Utc::now();
        Ok(Self {
            id,
            kind: ConversationKind::Group,
            title: Some(title),
            participants: vec![Participant {
                user_id: creator,
                joined_at: now,
            }],
            created_at: now,
        })
    }

    pub fn is_participant(&self, user_id: &UserId) -> bool {
        self.participants.iter().any(|p| &p.user_id == user_id)
    }

    pub fn add_participant(&mut self, user_id: UserId) -> Result<DomainEvent, DomainError> {
        if matches!(self.kind, ConversationKind::Direct) {
            return Err(DomainError::CannotAddToDirect);
        }

        if self.is_participant(&user_id) {
            return Err(DomainError::AlreadyParticipant);
        }

        self.participants.push(Participant {
            user_id: user_id.clone(),
            joined_at: Utc::now(),
        });
        Ok(DomainEvent::ParticipantAdded {
            conversation_id: self.id.clone(),
            user_id,
        })
    }

    pub(crate) fn from_persistence(
        id: ConversationId,
        kind: ConversationKind,
        title: Option<String>,
        participants: Vec<Participant>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            kind,
            title,
            participants,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::{
        conversation::{Conversation, ConversationKind, Participant},
        errors::DomainError,
        events::DomainEvent,
        ids::{ConversationId, UserId},
    };

    #[test]
    fn new_direct_rejects_same_user_on_both_sides() {
        let id = ConversationId::new();
        let a = UserId::new();
        let b = a.clone();

        let result = Conversation::new_direct(id, a, b);

        assert_eq!(result, Err(DomainError::DirectWithSelf));
    }

    #[test]
    fn new_direct_builds_conversation_with_both_participants() {
        let id = ConversationId::new();
        let a = UserId::new();
        let b = UserId::new();

        let convo = Conversation::new_direct(id.clone(), a.clone(), b.clone()).unwrap();

        assert_eq!(convo.id(), &id);
        assert_eq!(convo.kind(), &ConversationKind::Direct);
        assert_eq!(convo.title(), &None);
        assert_eq!(convo.participants.len(), 2);
        assert!(convo.participants.iter().any(|p| p.user_id == a));
        assert!(convo.participants.iter().any(|p| p.user_id == b));
    }

    #[test]
    fn new_group_rejects_empty_title() {
        let result = Conversation::new_group(ConversationId::new(), "".into(), UserId::new());

        assert_eq!(result, Err(DomainError::GroupNeedsTitle));
    }

    #[test]
    fn new_group_rejects_whitespace_only_title() {
        let result = Conversation::new_group(ConversationId::new(), "   ".into(), UserId::new());

        assert_eq!(result, Err(DomainError::GroupNeedsTitle));
    }

    #[test]
    fn new_group_builds_conversation_with_creator_as_sole_participant() {
        let id = ConversationId::new();
        let creator = UserId::new();

        let convo = Conversation::new_group(id.clone(), "The Group Chat".into(), creator.clone()).unwrap();

        assert_eq!(convo.id(), &id);
        assert_eq!(convo.kind(), &ConversationKind::Group);
        assert_eq!(convo.title(), &Some("The Group Chat".to_string()));
        assert_eq!(convo.participants.len(), 1);
        assert_eq!(convo.participants[0].user_id, creator);
    }

    #[test]
    fn is_participant_true_for_member_false_for_stranger() {
        let creator = UserId::new();
        let stranger = UserId::new();
        let convo = Conversation::new_group(ConversationId::new(), "The Group Chat".into(), creator.clone()).unwrap();

        assert!(convo.is_participant(&creator));
        assert!(!convo.is_participant(&stranger));
    }

    #[test]
    fn add_participant_adds_to_group_and_emits_event() {
        let creator = UserId::new();
        let new_member = UserId::new();
        let conversation_id = ConversationId::new();
        let mut convo = Conversation::new_group(conversation_id.clone(), "The Group Chat".into(), creator).unwrap();

        let event = convo.add_participant(new_member.clone()).unwrap();

        assert!(convo.is_participant(&new_member));
        assert_eq!(convo.participants.len(), 2);
        match event {
            DomainEvent::ParticipantAdded {
                conversation_id: event_conversation_id,
                user_id,
            } => {
                assert_eq!(event_conversation_id, conversation_id);
                assert_eq!(user_id, new_member);
            }
            _ => panic!("expected ParticipantAdded event"),
        }
    }

    #[test]
    fn add_participant_rejects_direct_conversation() {
        let mut convo = Conversation::new_direct(ConversationId::new(), UserId::new(), UserId::new()).unwrap();

        let result = convo.add_participant(UserId::new());

        assert_eq!(result.err(), Some(DomainError::CannotAddToDirect));
    }

    #[test]
    fn add_participant_rejects_existing_participant() {
        let creator = UserId::new();
        let mut convo = Conversation::new_group(ConversationId::new(), "The Group Chat".into(), creator.clone()).unwrap();

        let result = convo.add_participant(creator);

        assert_eq!(result.err(), Some(DomainError::AlreadyParticipant));
    }

    #[test]
    fn from_persistence_reconstructs_conversation_fields() {
        let id = ConversationId::new();
        let title = Some("Rebuilt".to_string());
        let created_at = Utc::now();
        let participants = vec![Participant {
            user_id: UserId::new(),
            joined_at: created_at,
        }];

        let convo = Conversation::from_persistence(id.clone(), ConversationKind::Group, title.clone(), participants, created_at);

        assert_eq!(convo.id(), &id);
        assert_eq!(convo.kind(), &ConversationKind::Group);
        assert_eq!(convo.title(), &title);
        assert_eq!(convo.created_at(), &created_at);
        assert_eq!(convo.participants.len(), 1);
    }
}
