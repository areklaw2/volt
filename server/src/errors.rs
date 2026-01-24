use std::sync::PoisonError;

use axum::{
    Json,
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use validator::ValidationErrors;
pub struct AppError {
    error: anyhow::Error,
    status: StatusCode,
}

impl AppError {
    pub fn new(error: anyhow::Error, status: StatusCode) -> Self {
        Self { error, status }
    }

    pub fn bad_request(msg: impl std::fmt::Display) -> Self {
        Self {
            error: anyhow::anyhow!(msg.to_string()),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn not_found(msg: impl std::fmt::Display) -> Self {
        Self {
            error: anyhow::anyhow!(msg.to_string()),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn validation_error(msg: impl std::fmt::Display) -> Self {
        Self {
            error: anyhow::anyhow!(msg.to_string()),
            status: StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    pub fn internal_error(error: anyhow::Error) -> Self {
        Self {
            error,
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub trait OptionExt<T> {
    fn ok_or_bad_request(self, msg: impl std::fmt::Display) -> Result<T, AppError>;
    fn ok_or_not_found(self, msg: impl std::fmt::Display) -> Result<T, AppError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_bad_request(self, msg: impl std::fmt::Display) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::bad_request(msg))
    }

    fn ok_or_not_found(self, msg: impl std::fmt::Display) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::not_found(msg))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:?}", self.error);

        let error_message = match self.status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal server error".to_string(),
            _ => self.error.to_string(),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (self.status, body).into_response()
    }
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(err: serde_json::error::Error) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self {
            error: err,
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        Self {
            error: anyhow::anyhow!("Lock poisoned: {}", err),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
