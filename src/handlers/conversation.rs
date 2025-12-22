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
    models::{Conversation, ConverstaionKind, Message, MessageKind, UserConversation},
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
    id: Ulid,
    #[serde(rename = "type")]
    kind: ConverstaionKind,
    title: Option<String>,
    participants: Vec<Ulid>,
    first_message: Message,
    created_at: String,
    updated_at: String,
    unread_message_count: u32,
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let conversation_id = Ulid::new();
    let message_id = Ulid::new();

    let mut user_conversations = state.user_conversations.write()?;
    for participant in input.participants.iter() {
        let unread_count = match *participant == input.sender_id {
            true => 0,
            false => 1,
        };

        let user_conversation = UserConversation {
            userid: input.sender_id,
            conversation_id,
            last_read_message_id: message_id,
            unread_count: unread_count,
        };
        user_conversations.insert(user_conversation);
    }

    let conversation = Conversation {
        id: conversation_id,
        kind: input.conversation_type,
        title: input.title,
        participants: input.participants,
        last_message_id: message_id,
        created_at: Utc::now().to_string(),
        updated_at: Utc::now().to_string(),
    };

    state
        .conversations
        .write()?
        .insert(conversation_id, conversation.clone());

    let message = Message {
        id: message_id,
        conversation_id: conversation_id,
        sender_id: input.sender_id,
        content: input.first_message,
        kind: input.message_type,
        created_at: Utc::now().to_string(),
        updated_at: None,
    };

    state.messages.write()?.insert(message_id, message.clone());

    let response = CreateConversationResponse {
        id: conversation.id,
        kind: conversation.kind.clone(),
        title: conversation.title.clone(),
        participants: conversation.participants.clone(),
        first_message: message.clone(),
        unread_message_count: 0,
        created_at: conversation.created_at.clone(),
        updated_at: conversation.updated_at.clone(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryConversationResponse {
    items: ConversationItems,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConversationItems {
    conversation_id: Ulid,
    title: Option<String>,
    participants: Vec<Ulid>,
    last_message: LastMessage,
    unread_count: u32,
    updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastMessage {
    message_id: Ulid,
    sender_id: Ulid,
    content: String,
    created_at: String,
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

#[derive(Debug, Deserialize)]
pub struct UpdateConversation {
    title: Option<String>,
    participants: Option<Vec<Ulid>>,
    last_message_id: Option<Ulid>,
}

pub async fn update_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
    Json(input): Json<UpdateConversation>,
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
