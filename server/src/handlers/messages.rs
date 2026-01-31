use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{AppState, dto::pagination::Pagination, errors::AppError};

pub async fn query_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let messages = state.repository.read_messages(conversation_id, pagination).await?;
    Ok(Json(messages))
}
