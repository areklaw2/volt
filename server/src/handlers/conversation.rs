use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    errors::{AppError, OptionExt},
    models::{Conversation, ConverstaionType, Message},
    repositories::participant::Participant,
};

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    conversation_type: ConverstaionType,
    first_message: String,
    sender_id: Uuid,
    participants: Vec<Uuid>,
    title: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateConversationResponse {
    id: Uuid,
    kind: ConverstaionType,
    title: Option<String>,
    first_message: Message,
    created_at: chrono::DateTime<Utc>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let conversation_id = Uuid::now_v7();
    let message_id = Uuid::now_v7();
    let now = Utc::now();

    let mut user_conversations = state.user_conversations.write()?;
    for participant in input.participants.iter() {
        let user_conversation = Participant {
            user_id: *participant,
            conversation_id,
            joined_at: Some(now),
            last_read_at: if *participant == input.sender_id { Some(now) } else { None },
        };
        user_conversations.insert(user_conversation);
    }

    let conversation = Conversation {
        id: conversation_id,
        converstion_type: input.conversation_type,
        name: input.title,
        created_at: now,
        updated_at: None,
    };

    state.conversations.write()?.insert(conversation_id, conversation.clone());

    let message = Message {
        id: message_id,
        conversation_id,
        sender_id: input.sender_id,
        content: input.first_message,
        created_at: now,
        updated_at: None,
    };

    state.messages.write()?.insert(message_id, message.clone());

    let response = CreateConversationResponse {
        id: conversation.id,
        kind: conversation.converstion_type.clone(),
        title: conversation.name.clone(),
        first_message: message,
        created_at: conversation.created_at,
        updated_at: conversation.updated_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryConversationResponse {
    items: ConversationItems,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConversationItems {
    conversation_id: Uuid,
    title: Option<String>,
    participants: Vec<Uuid>,
    last_message: LastMessage,
    unread_count: u32,
    updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastMessage {
    message_id: Uuid,
    sender_id: Uuid,
    content: String,
    created_at: String,
}

pub async fn query_users_conversations(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    //TODO: paginate this

    // Get conversation IDs where user is a participant
    let user_conversation_ids: Vec<Uuid> = state
        .user_conversations
        .read()?
        .iter()
        .filter(|p| p.user_id == user_id)
        .map(|p| p.conversation_id)
        .collect();

    let conversations = state
        .conversations
        .read()?
        .values()
        .filter(|conversation| user_conversation_ids.contains(&conversation.id))
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(conversations))
}

pub async fn get_conversation(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    let conversation = state
        .conversations
        .read()?
        .get(&id)
        .cloned()
        .ok_or_not_found("Conversation not found")?;

    Ok(Json(conversation))
}

#[derive(Debug, Deserialize)]
pub struct UpdateConversation {
    title: Option<String>,
}

pub async fn update_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateConversation>,
) -> Result<impl IntoResponse, AppError> {
    let mut conversation = state
        .conversations
        .read()?
        .get(&id)
        .cloned()
        .ok_or_not_found("Conversation not found")?;

    // TODO: add a validation to this

    let mut updated = false;

    if conversation.converstion_type == ConverstaionType::Group {
        if let Some(title) = input.title {
            conversation.name = Some(title);
            updated = true;
        }
    }

    if updated {
        conversation.updated_at = Some(Utc::now());
    }

    state.conversations.write()?.insert(conversation.id, conversation.clone());

    Ok(Json(conversation))
}

pub async fn delete_conversation(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    if state.conversations.write()?.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
