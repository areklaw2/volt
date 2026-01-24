use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use volt::{
    AppState, ConversationDb, MessageDb, UserConverstationsDb, UserDb,
    config::AppConfig,
    handlers::{http_routes, ws_routes},
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

    let config = AppConfig::from_env().expect("Failed to load configuration");
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("can't connect to database");

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

    let app = Router::new()
        .merge(http_routes())
        .merge(ws_routes())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
