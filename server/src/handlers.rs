use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::{self, HeaderValue, Method, StatusCode},
    routing::{get, patch, post},
};
use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
};
use secrecy::ExposeSecret;
use tower::{BoxError, ServiceBuilder};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    AppState,
    config::AppConfig,
    handlers::{
        chat::chat,
        conversation::{
            create_conversation, get_conversation, join_conversation, leave_conversation, query_conversations_by_user,
            update_conversation,
        },
        messages::query_messages,
    },
};

pub mod chat;
pub mod conversation;
pub mod messages;

fn conversation_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/conversation", post(create_conversation))
        .route("/conversation/{id}", get(get_conversation).patch(update_conversation))
        .route("/conversation/{id}/join/{user_id}", post(join_conversation))
        .route("/conversation/{id}/leave/{user_id}", post(leave_conversation))
        .route("/conversations/{user_id}", get(query_conversations_by_user))
}

fn message_routes() -> Router<Arc<AppState>> {
    Router::new().route("/messages/{conversation_id}", get(query_messages))
}

fn chat_routes() -> Router<Arc<AppState>> {
    Router::new().route("/chat/{user_id}", get(chat))
}

pub fn routes(config: &AppConfig) -> Router<Arc<AppState>> {
    let clerk_config = ClerkConfiguration::new(None, None, Some(config.clerk_secret_key.expose_secret().to_string()), None);
    let clerk = Clerk::new(clerk_config);

    let http_routes = Router::new().merge(conversation_routes()).merge(message_routes()).layer(
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
        .layer(
            CorsLayer::new()
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE]),
        )
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true))
        .layer(TraceLayer::new_for_http())
}
