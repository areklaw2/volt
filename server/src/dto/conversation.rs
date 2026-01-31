use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::user_conversation::ParticipantResponse,
    repositories::conversation::{ConversationAggregate, ConversationType},
};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateConversationRequest {
    pub conversation_type: ConversationType,
    pub sender_id: Uuid,
    pub participants: Vec<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateConversationRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ConversationResponse {
    pub id: Uuid,
    pub conversation_type: ConversationType,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub participants: Vec<ParticipantResponse>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ConversationAggregate> for ConversationResponse {
    fn from(agg: ConversationAggregate) -> Self {
        let users_map: HashMap<Uuid, _> = agg.users.into_iter().map(|u| (u.id, u)).collect();
        let participant_responses: Vec<ParticipantResponse> = agg
            .user_conversations
            .into_iter()
            .filter_map(|p| {
                users_map.get(&p.user_id).map(|user| ParticipantResponse {
                    id: user.id,
                    username: user.username.clone(),
                    display_name: user.display_name.clone(),
                    joined_at: p.joined_at,
                    last_read_at: p.last_read_at,
                })
            })
            .collect();

        ConversationResponse {
            id: agg.conversation.id,
            conversation_type: agg.conversation.conversation_type,
            name: agg.conversation.name,
            participants: participant_responses,
            created_at: agg.conversation.created_at,
            updated_at: agg.conversation.updated_at,
        }
    }
}
