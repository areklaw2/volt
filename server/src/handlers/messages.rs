use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    application::queries::message_history::{MessageHistoryQueries, MessageHistoryQuery},
    domain::ids::ConversationId,
    errors::AppError,
};

#[derive(Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn query_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let messages = state
        .views
        .for_conversation(MessageHistoryQuery {
            conversation_id: ConversationId::from_persistence(conversation_id),
            offset: pagination.offset,
            limit: pagination.limit,
        })
        .await?;

    Ok(Json(messages))
}
