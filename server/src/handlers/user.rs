use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    AppState,
    dto::user::CreateUserRequest,
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
