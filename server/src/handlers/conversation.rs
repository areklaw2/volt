use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    dto::{
        ConversationParticipantRequest, ConversationParticipantResponse, CreateConversationRequest, CreateParticipantsRequest,
        ParticipantResponse, UpdateConversationRequest,
    },
    errors::AppError,
    repositories::conversation::{Conversation, ConversationType},
};

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ConversationParticipantRequest>,
) -> Result<impl IntoResponse, AppError> {
    let conversation_request = CreateConversationRequest {
        conversation_type: input.conversation_type.clone(),
        name: input.name.clone(),
    };
    let conversation = state.conversations.create_conversation(conversation_request).await?;

    let participants_request = CreateParticipantsRequest {
        sender_id: input.sender_id,
        conversation_id: conversation.id,
        users: input.participants.clone(),
    };

    let participants = state.participants.create_conversation_participants(participants_request).await?;
    let users = state.users.read_users(input.participants).await?;

    if participants.len() != users.len() {
        return Err(AppError::bad_request("A requested participant may not exist"));
    }

    let users_map: HashMap<Uuid, _> = users.into_iter().map(|u| (u.id, u)).collect();
    let participant_responses: Vec<ParticipantResponse> = participants
        .into_iter()
        .filter_map(|p| {
            users_map.get(&p.user_id).map(|user| ParticipantResponse {
                id: user.id,
                username: user.username.clone(),
                display_name: user.display_name.clone(),
                avatar_url: user.avatar_url.clone(),
                joined_at: p.joined_at,
                last_read_at: p.last_read_at,
            })
        })
        .collect();

    let response = ConversationParticipantResponse {
        id: conversation.id,
        conversation_type: conversation.conversation_type,
        name: conversation.name,
        participants: participant_responses,
        created_at: conversation.created_at,
        updated_at: conversation.updated_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_conversation(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    let Some(conversation) = state.conversations.read_conversation(id).await? else {
        return Err(AppError::not_found("Conversation not found"));
    };

    let participants = state.participants.read_conversation_participants(conversation.id).await?;
    let user_ids: Vec<Uuid> = participants.iter().map(|p| p.user_id).collect();
    let users = state.users.read_users(user_ids).await?;
    if participants.len() != users.len() {
        return Err(AppError::bad_request("A requested participant may not exist"));
    }

    let users_map: HashMap<Uuid, _> = users.into_iter().map(|u| (u.id, u)).collect();
    let participant_responses: Vec<ParticipantResponse> = participants
        .into_iter()
        .filter_map(|p| {
            users_map.get(&p.user_id).map(|user| ParticipantResponse {
                id: user.id,
                username: user.username.clone(),
                display_name: user.display_name.clone(),
                avatar_url: user.avatar_url.clone(),
                joined_at: p.joined_at,
                last_read_at: p.last_read_at,
            })
        })
        .collect();

    let response = ConversationParticipantResponse {
        id: conversation.id,
        conversation_type: conversation.conversation_type,
        name: conversation.name,
        participants: participant_responses,
        created_at: conversation.created_at,
        updated_at: conversation.updated_at,
    };

    Ok((StatusCode::OK, Json(response)))
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
