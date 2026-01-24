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
    dto::{CreateConversationRequest, CreateMessageRequest, CreateParticipantsRequest, UpdateConversationRequest},
    errors::AppError,
    repositories::{
        conversation::{Conversation, ConversationType},
        message::Message,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateConversationHandlerRequest {
    conversation_type: ConversationType,
    first_message: String,
    sender_id: Uuid,
    participants: Vec<Uuid>,
    title: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateConversationResponse {
    id: Uuid,
    kind: ConversationType,
    title: Option<String>,
    first_message: Message,
    created_at: chrono::DateTime<Utc>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateConversationHandlerRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Create the conversation
    let conversation_request = CreateConversationRequest {
        conversation_type: input.conversation_type.clone(),
        name: input.title.clone(),
    };
    let conversation = state.conversations.create_conversation(conversation_request).await?;

    // Create participants
    let participants_request = CreateParticipantsRequest {
        sender_id: input.sender_id,
        conversation_id: conversation.id,
        users: input.participants,
    };
    state.participants.create_conversation_participants(participants_request).await?;

    // Create the first message
    let message_request = CreateMessageRequest {
        conversation_id: conversation.id,
        sender_id: input.sender_id,
        content: input.first_message,
    };
    let message = state.messages.create_message(message_request).await?;

    let response = CreateConversationResponse {
        id: conversation.id,
        kind: conversation.conversation_type.clone(),
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

    // Get participant entries for this user
    let user_participants = state.participants.read_participant_conversations(user_id).await?;

    // Get the conversation IDs
    let conversation_ids: Vec<Uuid> = user_participants.iter().map(|p| p.conversation_id).collect();

    // Fetch all conversations
    let mut conversations: Vec<Conversation> = Vec::new();
    for conv_id in conversation_ids {
        if let Some(conv) = state.conversations.read_conversation(conv_id).await? {
            conversations.push(conv);
        }
    }

    Ok(Json(conversations))
}

pub async fn get_conversation(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    match state.conversations.read_conversation(id).await? {
        Some(conversation) => Ok(Json(conversation)),
        None => Err(AppError::not_found("Conversation not found")),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateConversationHandlerRequest {
    title: Option<String>,
}

pub async fn update_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateConversationHandlerRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Get the current conversation to check if it's a group
    let conversation = state
        .conversations
        .read_conversation(id)
        .await?
        .ok_or_else(|| AppError::not_found("Conversation not found"))?;

    // Only allow updating name for group conversations
    if conversation.conversation_type != ConversationType::Group {
        return Ok(Json(conversation));
    }

    let update_request = UpdateConversationRequest { name: input.title };

    match state.conversations.update_conversation(id, update_request).await? {
        Some(updated) => Ok(Json(updated)),
        None => Err(AppError::not_found("Conversation not found")),
    }
}

pub async fn delete_conversation(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    state.conversations.delete_conversation(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
