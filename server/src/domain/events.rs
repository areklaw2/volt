use chrono::{DateTime, Utc};

use crate::domain::ids::{ConversationId, MessageId, UserId};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    MessageSent {
        message_id: MessageId,
        conversation_id: ConversationId,
        sender_id: UserId,
        content: String,
        created_at: DateTime<Utc>,
    },
    MessageEdited {
        message_id: MessageId,
        conversation_id: ConversationId,
    },
    ParticipantAdded {
        conversation_id: ConversationId,
        user_id: UserId,
    },
    ConversationRead {
        conversation_id: ConversationId,
        user_id: UserId,
        up_to: MessageId,
    },
}
