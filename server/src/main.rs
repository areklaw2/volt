use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use volt::{
    configure_state,
    handlers::{configure_http_routes, configure_ws_routes},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = configure_state().await.unwrap();

    // let clerk_config = ClerkConfiguration::new(None, None, Some(clerk_secret_key), None);
    // let clerk = Clerk::new(clerk_config);

    let app = Router::new()
        .merge(configure_http_routes())
        .merge(configure_ws_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
