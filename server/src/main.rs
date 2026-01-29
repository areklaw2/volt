use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use volt::{config::AppConfig, configure_state, handlers::routes};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env().unwrap();
    let state = configure_state(&config).await.unwrap();

    let app = Router::new().merge(routes(&config)).with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port)).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
