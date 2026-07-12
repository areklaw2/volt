use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    application::commands::create_user::CreateUserCommand,
    domain::{ids::UserId, repository::UserRepository},
    errors::AppError,
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub id: Option<String>,
    pub username: String,
    pub display_name: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

pub async fn create_or_read_user(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let id = request
        .id
        .and_then(|id| Uuid::parse_str(&id).ok())
        .map(UserId::from_persistence);

    let user = state
        .create_user
        .handle(CreateUserCommand {
            id,
            username: request.username,
            display_name: request.display_name,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            id: user.id().to_string(),
            username: user.username().as_str().to_string(),
            display_name: user.display_name().as_str().to_string(),
            created_at: *user.created_at(),
        }),
    ))
}

pub async fn get_users(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let users = state.users.find_all().await.map_err(|e| crate::domain::errors::DomainError::Internal(e.to_string()))?;

    let response: Vec<UserResponse> = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id().to_string(),
            username: u.username().as_str().to_string(),
            display_name: u.display_name().as_str().to_string(),
            created_at: *u.created_at(),
        })
        .collect();

    Ok(Json(response))
}
