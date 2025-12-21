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
    AppState, ChatDb,
    handlers::{
        chat::{create_chat_handler, get_chat_handler},
        websocket::websocket_handler,
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

    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);
    let chats = ChatDb::default();
    let app_state = Arc::new(AppState {
        user_set,
        tx,
        chats,
    });

    let http_routes = Router::new()
        .route("/api/v1/chat/{id}", get(get_chat_handler))
        .route("/api/v1/chat", post(create_chat_handler))
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

    let ws_routes = Router::new().route("/ws", get(websocket_handler));

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
