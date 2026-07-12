use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    application::commands::{create_conversation::CreateConversationCommand, mark_message_read::MarkMessageReadCommand},
    application::queries::conversation_list::ConversationViewQueries,
    domain::conversation::ConversationKind,
    domain::ids::{ConversationId, MessageId, UserId},
    errors::{AppError, OptionExt},
};

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub conversation_type: String,
    pub sender_id: String,
    pub participants: Vec<String>,
    pub name: Option<String>,
}

fn parse_uuid(value: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(value).map_err(|_| AppError::not_found(format!("invalid id: {value}")))
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let kind = match request.conversation_type.as_str() {
        "direct" => ConversationKind::Direct,
        _ => ConversationKind::Group,
    };
    let creator_id = UserId::from_persistence(parse_uuid(&request.sender_id)?);
    let participants = request
        .participants
        .iter()
        .map(|p| parse_uuid(p).map(UserId::from_persistence))
        .collect::<Result<Vec<_>, _>>()?;

    let id = state
        .create_conversation
        .handle(CreateConversationCommand {
            kind,
            creator_id,
            participants,
            title: request.name,
        })
        .await?;

    let view = state.views.by_id(&id).await?.ok_or_not_found("conversation not found")?;

    Ok((StatusCode::CREATED, Json(view)))
}

pub async fn query_conversations_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = UserId::from_persistence(parse_uuid(&user_id)?);
    let conversations = state.views.for_user(&user_id).await?;
    Ok(Json(conversations))
}

pub async fn mark_as_read(
    State(state): State<Arc<AppState>>,
    Path((id, user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let conversation_id = ConversationId::from_persistence(parse_uuid(&id)?);
    let user_id = UserId::from_persistence(parse_uuid(&user_id)?);

    let last_message_id = sqlx::query!(
        "SELECT last_message_id FROM conversations WHERE id = $1",
        Uuid::from(conversation_id.clone())
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| crate::domain::errors::DomainError::Internal(e.to_string()))?
    .and_then(|r| r.last_message_id);

    if let Some(message_id) = last_message_id {
        state
            .mark_read
            .handle(MarkMessageReadCommand {
                conversation_id,
                user_id,
                message_id: MessageId::from_persistence(message_id),
            })
            .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}
