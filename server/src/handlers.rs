use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::{self, HeaderValue, Method, StatusCode},
    routing::{get, post},
};
use tower::{BoxError, ServiceBuilder};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use crate::{
    AppState,
    config::AppConfig,
    handlers::{
        chat::chat,
        conversation::{create_conversation, mark_as_read, query_conversations_by_user},
        messages::query_messages,
        upload::upload_image,
        user::{create_or_read_user, get_users},
    },
};

pub mod chat;
pub mod conversation;
pub mod messages;
pub mod upload;
pub mod user;

fn conversation_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/conversation", post(create_conversation))
        .route("/conversation/{id}/read/{user_id}", post(mark_as_read))
        .route("/conversations/{user_id}", get(query_conversations_by_user))
}

fn message_routes() -> Router<Arc<AppState>> {
    Router::new().route("/messages/{conversation_id}", get(query_messages))
}

fn chat_routes() -> Router<Arc<AppState>> {
    Router::new().route("/chat/{user_id}", get(chat))
}

fn user_routes() -> Router<Arc<AppState>> {
    Router::new().route("/user", post(create_or_read_user)).route("/users", get(get_users))
}

fn upload_routes() -> Router<Arc<AppState>> {
    Router::new().route("/upload", post(upload_image))
}

pub fn routes(config: &AppConfig) -> Router<Arc<AppState>> {
    let http_routes = Router::new()
        .merge(conversation_routes())
        .merge(message_routes())
        .merge(user_routes())
        .merge(upload_routes())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}")))
                    }
                }))
                .timeout(Duration::from_secs(100))
                .into_inner(),
        );

    let api_routes = Router::new().merge(http_routes).merge(chat_routes());

    Router::new()
        .nest("/api/v1", api_routes)
        .nest_service("/media", ServeDir::new(&config.upload_dir))
        .layer(
            CorsLayer::new()
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE]),
        )
        .layer(TraceLayer::new_for_http())
}
