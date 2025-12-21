use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use ulid::Ulid;

use crate::{
    AppState,
    error::AppError,
    models::{Conversation, ConverstaionKind},
};

#[derive(Debug, Deserialize)]
pub struct CreateConversation {
    kind: ConverstaionKind,
    title: Option<String>,
    participants: Vec<Ulid>,
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateConversation>,
) -> Result<impl IntoResponse, AppError> {
    let conversation = Conversation {
        id: Ulid::new(),
        kind: input.kind,
        title: input.title,
        participants: input.participants,
        created_at: Utc::now().to_string(),
        last_massage_id: None,
        updated_at: Utc::now().to_string(),
    };

    state
        .conversations
        .write()?
        .insert(conversation.id, conversation.clone());

    // I think this goes someware else
    // for participant in conversation.participants.iter() {
    //     let user_conversation = UserConversation {
    //         conversation_id: conversation.id,
    //         userid: participant.clone(),
    //         last_read_message_id: None,
    //         unread_count: 0,
    //     };

    //     state
    //         .user_conversations
    //         .write()?
    //         .insert(user_conversation.userid, user_conversation);
    // }

    Ok((StatusCode::CREATED, Json(conversation)))
}

pub async fn get_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
) -> Result<impl IntoResponse, AppError> {
    let conversation = state
        .conversations
        .read()?
        .get(&id)
        .cloned()
        .ok_or(AppError::NotFound)?;

    Ok(Json(conversation))
}

#[derive(Debug, Deserialize)]
pub struct QueryConversation {
    pub conversation_id: Ulid,
    pub title: Option<String>,
    pub participants: Vec<Ulid>,
    pub last_massage: LastMessage,
    pub unread_count: u32,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct LastMessage {
    pub message_id: Ulid,
    pub sender_id: Ulid,
    pub content: String,
    pub created_at: String,
}

pub async fn query_users_conversations(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Ulid>,
) -> Result<impl IntoResponse, AppError> {
    //TODO: paginate this

    let conversations = state
        .conversations
        .read()?
        .values()
        .filter(|conversation| conversation.participants.contains(&user_id))
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(conversations))
}

#[derive(Debug, Deserialize)]
pub struct UpdateChat {
    title: Option<String>,
    participants: Option<Vec<Ulid>>,
    last_massage_id: Option<Ulid>,
}

pub async fn update_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
    Json(input): Json<UpdateChat>,
) -> Result<impl IntoResponse, AppError> {
    let mut conversation = state
        .conversations
        .read()?
        .get(&id)
        .cloned()
        .ok_or(AppError::NotFound)?;

    // TODO: add a validation to this

    let mut updated = false;

    if conversation.kind == ConverstaionKind::Group {
        if let Some(title) = input.title {
            conversation.title = Some(title);
            updated = true;
        }

        if let Some(participants) = input.participants {
            conversation.participants = participants;
            updated = true;
        }
    }

    if let Some(last_message_id) = input.last_massage_id {
        conversation.last_massage_id = Some(last_message_id);
        updated = true;
    }

    if updated {
        conversation.updated_at = Utc::now().to_string();
    }

    state
        .conversations
        .write()?
        .insert(conversation.id, conversation.clone());

    Ok(Json(conversation))
}

pub async fn delete_conversation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Ulid>,
) -> Result<impl IntoResponse, AppError> {
    if state.conversations.write()?.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
