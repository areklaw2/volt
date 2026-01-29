pub mod config;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod repositories;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    repositories::{DbRepository, InMemoryRepository, Repository, message::Message},
};

pub struct AppState {
    pub repository: Arc<dyn Repository>,
    pub active_connections: Arc<RwLock<HashMap<Uuid, Vec<mpsc::Sender<Message>>>>>,
}

pub async fn configure_state(config: &AppConfig) -> Result<Arc<AppState>, anyhow::Error> {
    let active_connections = Arc::default();
    let repository: Arc<dyn Repository> = if config.data_in_memory {
        Arc::new(InMemoryRepository::new())
    } else {
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.database_url.expose_secret())
            .await?;
        Arc::new(DbRepository::new(pool))
    };

    let state = Arc::new(AppState {
        repository,
        active_connections,
    });

    Ok(state)
}
