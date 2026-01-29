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
    dto::{ConversationResponse, CreateConversationRequest, ParticipantResponse, UpdateConversationRequest},
    errors::AppError,
    repositories::conversation::{Conversation, ConversationAggregate, ConversationType},
};

//TODO: move this to seperate dto file
fn conversation_response_from(agg: ConversationAggregate) -> ConversationResponse {
    let users_map: HashMap<Uuid, _> = agg.users.into_iter().map(|u| (u.id, u)).collect();
    let participant_responses: Vec<ParticipantResponse> = agg
        .participants
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

    ConversationResponse {
        id: agg.conversation.id,
        conversation_type: agg.conversation.conversation_type,
        name: agg.conversation.name,
        participants: participant_responses,
        created_at: agg.conversation.created_at,
        updated_at: agg.conversation.updated_at,
    }
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let agg = state.repository.create_conversation(request).await?;

    if agg.participants.len() != agg.users.len() {
        return Err(AppError::bad_request("A requested participant may not exist"));
    }

    let response = conversation_response_from(agg);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_conversation(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    let agg = state
        .repository
        .read_conversation(id)
        .await?
        .ok_or_else(|| AppError::not_found("Conversation not found"))?;

    if agg.participants.len() != agg.users.len() {
        return Err(AppError::bad_request("A requested participant may not exist"));
    }

    let response = conversation_response_from(agg);
    Ok((StatusCode::OK, Json(response)))
}

pub async fn update_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let agg = state
        .repository
        .read_conversation(id)
        .await?
        .ok_or_else(|| AppError::not_found("Conversation not found"))?;

    if agg.conversation.conversation_type != ConversationType::Group {
        return Ok(Json(agg.conversation));
    }

    let update_request = UpdateConversationRequest { name: input.name };
    match state.repository.update_conversation(id, update_request).await? {
        Some(updated) => Ok(Json(updated)),
        None => Err(AppError::not_found("Conversation not found")),
    }
}

pub async fn join_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.repository.create_participant(user_id, id).await?;
    Ok(StatusCode::CREATED)
}

pub async fn leave_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.repository.delete_participant(user_id, id).await?;
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
    let user_participants = state.repository.read_participant_conversations(user_id).await?;
    let conversation_ids: Vec<Uuid> = user_participants.iter().map(|p| p.conversation_id).collect();

    let mut conversations: Vec<Conversation> = Vec::new();
    for conversation_id in conversation_ids {
        if let Some(agg) = state.repository.read_conversation(conversation_id).await? {
            conversations.push(agg.conversation);
        }
    }

    Ok(Json(conversations))
}
