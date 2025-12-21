use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::{
    AppState,
    error::AppError,
    models::{Conversation, ConverstaionKind, Message, MessageKind},
};

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    conversation_type: ConverstaionKind,
    message_type: MessageKind,
    first_message: String,
    sender_id: Ulid,
    participants: Vec<Ulid>,
    title: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateConversationResponse {
    pub id: Ulid,
    #[serde(rename = "type")]
    pub kind: ConverstaionKind,
    pub title: Option<String>,
    pub participants: Vec<Ulid>,
    pub first_message: Message,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let conversation_id = Ulid::new();
    let message_id = Ulid::new();

    let conversation = Conversation {
        id: conversation_id,
        kind: input.conversation_type,
        title: input.title,
        participants: input.participants,
        last_message_id: message_id,
        created_at: Utc::now().to_string(),
        updated_at: Utc::now().to_string(),
    };

    let message = Message {
        id: message_id,
        conversation_id: conversation_id,
        sender_id: input.sender_id,
        content: input.first_message,
        kind: input.message_type,
        created_at: Utc::now().to_string(),
        updated_at: None,
    };

    state
        .conversations
        .write()?
        .insert(conversation_id, conversation.clone());

    state.messages.write()?.insert(message_id, message.clone());

    let response = CreateConversationResponse {
        id: conversation.id,
        kind: conversation.kind.clone(),
        title: conversation.title.clone(),
        participants: conversation.participants.clone(),
        first_message: message.clone(),
        created_at: conversation.created_at.clone(),
        updated_at: conversation.updated_at.clone(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
) -> Result<impl IntoResponse, AppError> {
    let conversation = state
        .conversations
        .read()?
        .get(&id)
        .cloned()
        .ok_or(AppError::NotFound)?;

    Ok(Json(conversation))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryConversation {
    pub conversation_id: Ulid,
    pub title: Option<String>,
    pub participants: Vec<Ulid>,
    pub last_message: LastMessage,
    pub unread_count: u32,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastMessage {
    pub message_id: Ulid,
    pub sender_id: Ulid,
    pub content: String,
    pub created_at: String,
}

pub async fn query_users_conversations(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Ulid>,
) -> Result<impl IntoResponse, AppError> {
    //TODO: paginate this

    let conversations = state
        .conversations
        .read()?
        .values()
        .filter(|conversation| conversation.participants.contains(&user_id))
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(conversations))
}

#[derive(Debug, Deserialize)]
pub struct UpdateChat {
    title: Option<String>,
    participants: Option<Vec<Ulid>>,
    last_message_id: Option<Ulid>,
}

pub async fn update_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
    Json(input): Json<UpdateChat>,
) -> Result<impl IntoResponse, AppError> {
    let mut conversation = state
        .conversations
        .read()?
        .get(&id)
        .cloned()
        .ok_or(AppError::NotFound)?;

    // TODO: add a validation to this

    let mut updated = false;

    if conversation.kind == ConverstaionKind::Group {
        if let Some(title) = input.title {
            conversation.title = Some(title);
            updated = true;
        }

        if let Some(participants) = input.participants {
            conversation.participants = participants;
            updated = true;
        }
    }

    if let Some(last_message_id) = input.last_message_id {
        conversation.last_message_id = last_message_id;
        updated = true;
    }

    if updated {
        conversation.updated_at = Utc::now().to_string();
    }

    state
        .conversations
        .write()?
        .insert(conversation.id, conversation.clone());

    Ok(Json(conversation))
}

pub async fn delete_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
) -> Result<impl IntoResponse, AppError> {
    if state.conversations.write()?.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
