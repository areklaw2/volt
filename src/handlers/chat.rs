use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::AppState;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ChatKind {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "group")]
    Group,
}

#[derive(Debug, Deserialize)]
pub struct CreateChat {
    kind: ChatKind,
    name: Option<String>,
    participants: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chat {
    id: Ulid,
    kind: ChatKind,
    name: Option<String>,
    participants: Vec<String>,
    created_at: String,
    last_massage_id: Option<Ulid>,
}

pub async fn create_chat_handler(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateChat>,
) -> impl IntoResponse {
    let chat = Chat {
        id: Ulid::new(),
        kind: input.kind,
        name: input.name,
        participants: input.participants,
        created_at: Utc::now().to_string(),
        last_massage_id: None,
    };

    state.chats.write().unwrap().insert(chat.id, chat.clone());

    (StatusCode::CREATED, Json(chat))
}

pub async fn get_chat_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
) -> Result<impl IntoResponse, StatusCode> {
    let chat = state
        .chats
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(chat))
}
