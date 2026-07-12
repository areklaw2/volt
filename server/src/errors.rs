use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::application::queries::{conversation_list, message_history};
use crate::domain::errors::DomainError;

pub struct AppError {
    message: String,
    status: StatusCode,
}

impl AppError {
    pub fn not_found(msg: impl std::fmt::Display) -> Self {
        Self {
            message: msg.to_string(),
            status: StatusCode::NOT_FOUND,
        }
    }
}

pub trait OptionExt<T> {
    fn ok_or_not_found(self, msg: impl std::fmt::Display) -> Result<T, AppError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_not_found(self, msg: impl std::fmt::Display) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::not_found(msg))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if self.status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!("internal error: {}", self.message);
        }

        let message = match self.status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal server error".to_string(),
            _ => self.message,
        };

        (self.status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        let status = match err {
            DomainError::ConversationNotFound => StatusCode::NOT_FOUND,
            DomainError::NotAParticipant | DomainError::NotYourMessage => StatusCode::FORBIDDEN,
            DomainError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        Self {
            message: err.to_string(),
            status,
        }
    }
}

impl From<conversation_list::QueryError> for AppError {
    fn from(err: conversation_list::QueryError) -> Self {
        tracing::error!("query error: {err}");
        Self {
            message: "internal server error".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<message_history::QueryError> for AppError {
    fn from(err: message_history::QueryError) -> Self {
        tracing::error!("query error: {err}");
        Self {
            message: "internal server error".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
