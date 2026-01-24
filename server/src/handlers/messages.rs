use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    dto::{CreateMessageRequest, Pagination, UpdateMessageRequest},
    errors::AppError,
};

pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateMessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let message = state.messages.create_message(input).await?;
    Ok((StatusCode::CREATED, Json(message)))
}

pub async fn get_message(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    match state.messages.read_message(id).await? {
        Some(message) => Ok(Json(message)),
        None => Err(AppError::not_found("Message not found")),
    }
}

pub async fn query_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
    pagination: Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let messages = state.messages.list_messages(conversation_id, pagination.0).await?;
    Ok(Json(messages))
}

pub async fn update_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateMessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    match state.messages.update_message(id, input).await? {
        Some(message) => Ok(Json(message)),
        None => Err(AppError::not_found("Message not found")),
    }
}

pub async fn delete_message(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    state.messages.delete_message(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
