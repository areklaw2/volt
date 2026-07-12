use std::sync::Arc;

use axum::{Json, extract::Multipart, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use uuid::Uuid;

use crate::{AppState, domain::errors::DomainError, errors::AppError};

const MAX_UPLOAD_BYTES: usize = 10 * 1024 * 1024;

#[derive(Serialize)]
pub struct UploadResponse {
    pub url: String,
}

pub async fn upload_image(State(state): State<Arc<AppState>>, mut multipart: Multipart) -> Result<impl IntoResponse, AppError> {
    let Some(field) = multipart.next_field().await? else {
        return Err(AppError::bad_request("no file provided"));
    };

    let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
    let extension = match content_type.as_str() {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        other => return Err(AppError::bad_request(format!("unsupported content type: {other}"))),
    };

    let bytes = field.bytes().await?;
    if bytes.len() > MAX_UPLOAD_BYTES {
        return Err(AppError::bad_request("file too large"));
    }

    let filename = format!("{}.{extension}", Uuid::now_v7());
    let path = std::path::Path::new(&state.upload_dir).join(&filename);

    tokio::fs::write(&path, &bytes)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

    let url = format!("{}/media/{filename}", state.public_url);

    Ok((StatusCode::CREATED, Json(UploadResponse { url })))
}
