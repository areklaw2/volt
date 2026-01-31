use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    AppState,
    dto::user::{CreateUserRequest, UpdateUserRequest},
    errors::{AppError, OptionExt},
};

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.repository.create_user(request).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let user = state.repository.read_user(id).await?.ok_or_not_found("User not found")?;
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.repository.update_user(id, request).await?.ok_or_not_found("User not found")?;
    Ok(Json(user))
}

pub async fn get_users(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let users = state.repository.read_users().await?;
    Ok(Json(users))
}

pub async fn delete_user(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    state.repository.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
