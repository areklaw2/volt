use chrono::{DateTime, Utc};
use getset::Getters;

use crate::domain::{
    errors::DomainError,
    events::DomainEvent,
    ids::{ConversationId, MessageId, UserId},
};

#[derive(Debug, PartialEq, Clone, sqlx::Type)]
#[sqlx(type_name = "message_kind", rename_all = "lowercase")]
pub enum MessageKind {
    Text,
    Image,
}

#[derive(Debug, Getters, PartialEq)]
pub struct Message {
    #[getset(get = "pub")]
    id: MessageId,
    #[getset(get = "pub")]
    conversation_id: ConversationId,
    #[getset(get = "pub")]
    sender_id: UserId,
    #[getset(get = "pub")]
    content: String,
    #[getset(get = "pub")]
    kind: MessageKind,
    #[getset(get = "pub")]
    edited: bool,
    #[getset(get = "pub")]
    created_at: DateTime<Utc>,
    #[getset(get = "pub")]
    updated_at: Option<DateTime<Utc>>,
}

impl Message {
    pub fn new(
        id: MessageId,
        conversation_id: ConversationId,
        sender_id: UserId,
        content: String,
        kind: MessageKind,
    ) -> Result<(Self, DomainEvent), DomainError> {
        if content.trim().is_empty() {
            return Err(DomainError::EmptyMessage);
        }
        if matches!(kind, MessageKind::Image) && !looks_like_url(&content) {
            return Err(DomainError::ImageNeedsUrl);
        }

        let message = Self {
            id: id.clone(),
            conversation_id: conversation_id.clone(),
            sender_id: sender_id.clone(),
            content,
            kind,
            edited: false,
            created_at: Utc::now(),
            updated_at: None,
        };
        let event = DomainEvent::MessageSent {
            message_id: id,
            conversation_id,
            sender_id,
            content: message.content.clone(),
            kind: message.kind.clone(),
            created_at: message.created_at,
        };
        Ok((message, event))
    }

    pub fn edit(&mut self, editor: &UserId, new_content: String) -> Result<DomainEvent, DomainError> {
        if &self.sender_id != editor {
            return Err(DomainError::NotYourMessage);
        }
        if new_content.trim().is_empty() {
            return Err(DomainError::EmptyMessage);
        }
        self.content = new_content;
        self.edited = true;
        self.updated_at = Some(Utc::now());
        Ok(DomainEvent::MessageEdited {
            message_id: self.id.clone(),
            conversation_id: self.conversation_id.clone(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_persistence(
        id: MessageId,
        conversation_id: ConversationId,
        sender_id: UserId,
        content: String,
        kind: MessageKind,
        edited: bool,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            conversation_id,
            sender_id,
            content,
            kind,
            edited,
            created_at,
            updated_at,
        }
    }
}

fn looks_like_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        errors::DomainError,
        events::DomainEvent,
        ids::{ConversationId, MessageId, UserId},
        message::{Message, MessageKind},
    };

    #[test]
    fn new_rejects_empty_content() {
        let result = Message::new(
            MessageId::new(),
            ConversationId::new(),
            UserId::new(),
            "   ".to_string(),
            MessageKind::Text,
        );

        assert_eq!(result.err(), Some(DomainError::EmptyMessage));
    }

    #[test]
    fn new_rejects_image_without_url() {
        let result = Message::new(
            MessageId::new(),
            ConversationId::new(),
            UserId::new(),
            "not a url".to_string(),
            MessageKind::Image,
        );

        assert_eq!(result.err(), Some(DomainError::ImageNeedsUrl));
    }

    #[test]
    fn new_builds_text_message_and_emits_event() {
        let id = MessageId::new();
        let conversation_id = ConversationId::new();
        let sender_id = UserId::new();

        let (message, event) = Message::new(
            id.clone(),
            conversation_id.clone(),
            sender_id.clone(),
            "hello".to_string(),
            MessageKind::Text,
        )
        .unwrap();

        assert_eq!(message.id(), &id);
        assert_eq!(message.conversation_id(), &conversation_id);
        assert_eq!(message.sender_id(), &sender_id);
        assert_eq!(message.content, "hello");
        assert_eq!(message.kind, MessageKind::Text);
        assert!(!message.edited);
        assert_eq!(message.updated_at, None);

        match event {
            DomainEvent::MessageSent {
                message_id,
                conversation_id: event_conversation_id,
                sender_id: event_sender_id,
                content,
                kind,
                created_at,
            } => {
                assert_eq!(message_id, id);
                assert_eq!(event_conversation_id, conversation_id);
                assert_eq!(event_sender_id, sender_id);
                assert_eq!(content, "hello");
                assert_eq!(kind, MessageKind::Text);
                assert_eq!(created_at, message.created_at);
            }
            _ => panic!("expected MessageSent event"),
        }
    }

    #[test]
    fn new_builds_image_message_with_url() {
        let (message, _event) = Message::new(
            MessageId::new(),
            ConversationId::new(),
            UserId::new(),
            "https://example.com/img.png".to_string(),
            MessageKind::Image,
        )
        .unwrap();

        assert_eq!(message.kind, MessageKind::Image);
        assert_eq!(message.content, "https://example.com/img.png");
    }

    #[test]
    fn edit_rejects_editor_who_is_not_sender() {
        let (mut message, _event) = Message::new(
            MessageId::new(),
            ConversationId::new(),
            UserId::new(),
            "hello".to_string(),
            MessageKind::Text,
        )
        .unwrap();

        let result = message.edit(&UserId::new(), "changed".to_string());

        assert_eq!(result.err(), Some(DomainError::NotYourMessage));
    }

    #[test]
    fn edit_rejects_empty_content() {
        let sender_id = UserId::new();
        let (mut message, _event) = Message::new(
            MessageId::new(),
            ConversationId::new(),
            sender_id.clone(),
            "hello".to_string(),
            MessageKind::Text,
        )
        .unwrap();

        let result = message.edit(&sender_id, "   ".to_string());

        assert_eq!(result.err(), Some(DomainError::EmptyMessage));
    }

    #[test]
    fn edit_updates_content_and_emits_event() {
        let id = MessageId::new();
        let conversation_id = ConversationId::new();
        let sender_id = UserId::new();
        let (mut message, _event) = Message::new(
            id.clone(),
            conversation_id.clone(),
            sender_id.clone(),
            "hello".to_string(),
            MessageKind::Text,
        )
        .unwrap();

        let event = message.edit(&sender_id, "updated".to_string()).unwrap();

        assert_eq!(message.content, "updated");
        assert!(message.edited);
        assert!(message.updated_at.is_some());

        match event {
            DomainEvent::MessageEdited {
                message_id,
                conversation_id: event_conversation_id,
            } => {
                assert_eq!(message_id, id);
                assert_eq!(event_conversation_id, conversation_id);
            }
            _ => panic!("expected MessageEdited event"),
        }
    }

    #[test]
    fn from_persistence_reconstructs_message_fields() {
        let id = MessageId::new();
        let conversation_id = ConversationId::new();
        let sender_id = UserId::new();
        let created_at = chrono::Utc::now();

        let message = Message::from_persistence(
            id.clone(),
            conversation_id.clone(),
            sender_id.clone(),
            "hello".to_string(),
            MessageKind::Text,
            true,
            created_at,
            Some(created_at),
        );

        assert_eq!(message.id(), &id);
        assert_eq!(message.conversation_id(), &conversation_id);
        assert_eq!(message.sender_id(), &sender_id);
        assert_eq!(message.content, "hello");
        assert_eq!(message.kind, MessageKind::Text);
        assert!(message.edited);
        assert_eq!(message.created_at, created_at);
        assert_eq!(message.updated_at, Some(created_at));
    }
}
