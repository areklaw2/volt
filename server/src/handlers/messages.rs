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
    dto::{Pagination, UpdateMessageRequest},
    errors::AppError,
};

pub async fn query_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let messages = state.repository.list_messages(conversation_id, pagination).await?;
    Ok(Json(messages))
}

pub async fn update_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateMessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    match state.repository.update_message(id, input).await? {
        Some(message) => Ok(Json(message)),
        None => Err(AppError::not_found("Message not found")),
    }
}

pub async fn delete_message(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    state.repository.delete_message(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
