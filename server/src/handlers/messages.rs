use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    errors::{AppError, OptionExt},
    models::Message,
};

pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateMessage>,
) -> Result<impl IntoResponse, AppError> {
    let message = Message {
        id: Uuid::now_v7(),
        conversation_id: input.conversation_id,
        sender_id: input.sender_id,
        content: input.content,
        created_at: Utc::now(),
        updated_at: None,
    };

    state.messages.write()?.insert(message.id, message.clone());

    Ok((StatusCode::CREATED, Json(message)))
}

pub async fn get_message(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    let message = state.messages.read()?.get(&id).cloned().ok_or_not_found("Message not found")?;

    Ok(Json(message))
}

pub async fn query_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    //TODO: paginate this

    let messages = state
        .messages
        .read()?
        .values()
        .filter(|message| message.conversation_id == conversation_id)
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(messages))
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessage {
    content: Option<String>,
}

pub async fn update_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateMessage>,
) -> Result<impl IntoResponse, AppError> {
    let mut message = state.messages.read()?.get(&id).cloned().ok_or_not_found("Message not found")?;

    // TODO: add a validation to this

    let mut updated = false;
    if let Some(content) = input.content {
        message.content = content;
        updated = true;
    }

    if updated {
        message.updated_at = Some(Utc::now());
    }

    state.messages.write()?.insert(message.id, message.clone());

    Ok(Json(message))
}

pub async fn delete_message(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
    if state.messages.write()?.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
