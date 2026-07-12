use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    application::commands::edit_message::EditMessageCommand,
    application::queries::message_history::{MessageHistoryQueries, MessageHistoryQuery},
    domain::ids::{ConversationId, MessageId, UserId},
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

#[derive(Deserialize)]
pub struct EditMessageRequest {
    pub editor_id: String,
    pub content: String,
}

pub async fn edit_message(
    State(state): State<Arc<AppState>>,
    Path(message_id): Path<Uuid>,
    Json(request): Json<EditMessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let editor_id = Uuid::parse_str(&request.editor_id).map_err(|_| AppError::bad_request("invalid editor_id"))?;

    state
        .edit_message
        .handle(EditMessageCommand {
            message_id: MessageId::from_persistence(message_id),
            editor_id: UserId::from_persistence(editor_id),
            content: request.content,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
