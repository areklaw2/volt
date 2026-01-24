pub mod config;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod repositories;

use sqlx::postgres::PgPoolOptions;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    repositories::{
        conversation::{ConversationRepository, DbConversationRepository, InMemoryConversationRepository},
        message::{DbMessageRepository, InMemoryMessageRepository, Message, MessageRepository},
        participant::{DbParticipantRepository, InMemoryParticipantRepository, Participant, ParticipantRepository},
        user::{DbUserRepository, InMemoryUserRepository, UserRepository},
    },
};

pub type UserConverstationsDb = RwLock<HashSet<Participant>>;

pub struct AppState {
    pub users: Arc<dyn UserRepository>,
    pub participants: Arc<dyn ParticipantRepository>,
    pub conversations: Arc<dyn ConversationRepository>,
    pub messages: Arc<dyn MessageRepository>,
    pub active_connections: Arc<RwLock<HashMap<Uuid, Vec<mpsc::Sender<Message>>>>>,
}

pub async fn configure_state() -> Result<Arc<AppState>, anyhow::Error> {
    let config = AppConfig::from_env()?;
    let active_connections = Arc::default();

    let state = match config.data_in_memory {
        true => {
            let users = Arc::new(InMemoryUserRepository::default());
            let messages = Arc::new(InMemoryMessageRepository::default());
            let conversations = Arc::new(InMemoryConversationRepository::default());
            let participants = Arc::new(InMemoryParticipantRepository::default());

            Arc::new(AppState {
                users,
                participants,
                conversations,
                messages,
                active_connections,
            })
        }
        false => {
            let pool = PgPoolOptions::new()
                .max_connections(100)
                .acquire_timeout(Duration::from_secs(3))
                .connect(&config.database_url)
                .await?;

            let users = Arc::new(DbUserRepository::new(pool.clone()));
            let messages = Arc::new(DbMessageRepository::new(pool.clone()));
            let conversations = Arc::new(DbConversationRepository::new(pool.clone()));
            let participants = Arc::new(DbParticipantRepository::new(pool.clone()));

            Arc::new(AppState {
                users,
                participants,
                conversations,
                messages,
                active_connections,
            })
        }
    };

    Ok(state)
}
