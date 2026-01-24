use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::{self, HeaderValue, Method, StatusCode},
    routing::{get, post},
};
use tower::{BoxError, ServiceBuilder};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    AppState,
    handlers::{
        conversation::{create_conversation, delete_conversation, get_conversation, query_users_conversations},
        messages::{create_message, delete_message, get_message, query_messages, update_message},
        websocket::websocket,
    },
};

pub mod conversation;
pub mod messages;
pub mod websocket;

pub fn configure_http_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/conversation", post(create_conversation))
        .route(
            "/api/v1/conversation/{id}",
            get(get_conversation).patch(get_conversation).delete(delete_conversation),
        )
        .route("/api/v1/conversations/{user_id}", get(query_users_conversations))
        .route("/api/v1/message", post(create_message))
        .route(
            "/api/v1/message/{id}",
            get(get_message).patch(update_message).delete(delete_message),
        )
        .route("/api/v1/messages/{conversation_id}", get(query_messages))
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
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE]),
        )
}

pub fn configure_ws_routes() -> Router<Arc<AppState>> {
    Router::new().route("/ws/{user_id}", get(websocket))
}
