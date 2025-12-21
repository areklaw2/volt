use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::broadcast;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use volt::{
    AppState, ConversationDb, MessageDb, UserConverstationsDb, UserDb,
    handlers::{
        conversation::{
            create_conversation, delete_conversation, get_conversation, query_users_conversations,
            update_conversation,
        },
        messages::{create_message, delete_message, get_message, query_messages, update_message},
        websocket::websocket,
    },
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, _rx) = broadcast::channel(100);
    let users = UserDb::default();
    let conversations = ConversationDb::default();
    let user_conversations = UserConverstationsDb::default();
    let messages = MessageDb::default();
    let active_users = Mutex::new(HashSet::new());
    let app_state = Arc::new(AppState {
        tx,
        users,
        conversations,
        user_conversations,
        messages,
        active_users,
    });

    let http_routes = Router::new()
        .route("/api/v1/conversation", post(create_conversation))
        .route(
            "/api/v1/conversation/{id}",
            get(get_conversation)
                .patch(update_conversation)
                .delete(delete_conversation),
        )
        .route(
            "/api/v1/conversations/{user_id}",
            get(query_users_conversations),
        )
        .route("/api/v1/message", post(create_message))
        .route(
            "/api/v1/message/{id}",
            get(get_message)
                .patch(update_message)
                .delete(delete_message),
        )
        .route("/api/v1/messages/{conversation_id}", get(query_messages))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let ws_routes = Router::new().route("/ws", get(websocket));

    let app = Router::new()
        .merge(http_routes)
        .merge(ws_routes)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
