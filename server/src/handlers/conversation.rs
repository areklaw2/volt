use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    dto::conversation::{ConversationResponse, CreateConversationRequest, UpdateConversationRequest},
    errors::{AppError, OptionExt},
    repositories::conversation::ConversationType,
};

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let agg = state.repository.create_conversation(request).await?;
    let response = ConversationResponse::from(agg);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_conversation(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    let agg = state
        .repository
        .read_conversation(id)
        .await?
        .ok_or_not_found("Conversation not found")?;

    let response = ConversationResponse::from(agg);
    Ok(Json(response))
}

pub async fn update_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let agg = state
        .repository
        .read_conversation(id)
        .await?
        .ok_or_not_found("Conversation not found")?;

    if agg.conversation.conversation_type != ConversationType::Group {
        return Ok(Json(agg.conversation));
    }

    let updated = state
        .repository
        .update_conversation(id, request)
        .await?
        .ok_or_not_found("Conversation not found")?;

    Ok(Json(updated))
}

pub async fn join_conversation(
    State(state): State<Arc<AppState>>,
    Path((id, user_id)): Path<(Uuid, String)>,
) -> Result<impl IntoResponse, AppError> {
    state.repository.create_user_conversation(user_id, id).await?;
    Ok(StatusCode::CREATED)
}

pub async fn leave_conversation(
    State(state): State<Arc<AppState>>,
    Path((id, user_id)): Path<(Uuid, String)>,
) -> Result<impl IntoResponse, AppError> {
    state.repository.delete_user_conversation(user_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn query_conversations_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let aggs = state.repository.read_conversations_by_user(user_id).await?;
    let response: Vec<ConversationResponse> = aggs.into_iter().map(|agg| ConversationResponse::from(agg)).collect();
    Ok(Json(response))
}
