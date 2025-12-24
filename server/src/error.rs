use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    LockPoisoned(String),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::LockPoisoned(context) => {
                tracing::error!("Lock poisoned: {}", context);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error: lock poisoned",
                )
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        AppError::LockPoisoned(err.to_string())
    }
}
