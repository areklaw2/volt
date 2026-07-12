pub mod application;
pub mod config;
pub mod domain;
pub mod errors;
pub mod handlers;
pub mod infrastructure;

use std::{sync::Arc, time::Duration};

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

use crate::{
    application::commands::create_conversation::CreateConversationHandler, application::commands::create_user::CreateUserHandler,
    application::commands::edit_message::EditMessageHandler, application::commands::leave_conversation::LeaveConversationHandler,
    application::commands::mark_message_read::MarkReadHandler, application::commands::send_message::SendMessageHandler,
    config::AppConfig, infrastructure::events::bus::EventBus,
    infrastructure::postgres::conversation_repository::SqlxConversationRepository,
    infrastructure::postgres::message_repository::SqlxMessageRepository, infrastructure::postgres::queries::SqlxViewQueries,
    infrastructure::postgres::user_repository::SqlxUserRepository,
};

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub event_bus: EventBus,
    pub users: SqlxUserRepository,
    pub views: SqlxViewQueries,
    pub create_user: CreateUserHandler<SqlxUserRepository>,
    pub create_conversation: CreateConversationHandler<SqlxConversationRepository, EventBus>,
    pub send_message: Arc<SendMessageHandler<SqlxConversationRepository, SqlxMessageRepository, EventBus>>,
    pub edit_message: EditMessageHandler<SqlxMessageRepository, EventBus>,
    pub leave_conversation: LeaveConversationHandler<SqlxConversationRepository, EventBus>,
    pub mark_read: MarkReadHandler<EventBus>,
    pub upload_dir: String,
    pub public_url: String,
}

pub async fn configure_state(config: &AppConfig) -> Result<Arc<AppState>, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(3))
        .connect(config.database_url.expose_secret())
        .await?;

    tokio::fs::create_dir_all(&config.upload_dir).await?;

    let event_bus = EventBus::new(pool.clone());
    let users_repo = SqlxUserRepository::new(pool.clone());
    let conversations_repo = SqlxConversationRepository::new(pool.clone());
    let messages_repo = SqlxMessageRepository::new(pool.clone());
    let views = SqlxViewQueries::new(pool.clone());

    let create_user = CreateUserHandler::new(users_repo.clone());
    let create_conversation = CreateConversationHandler::new(conversations_repo.clone(), event_bus.clone());
    let send_message = Arc::new(SendMessageHandler::new(
        conversations_repo.clone(),
        messages_repo.clone(),
        event_bus.clone(),
    ));
    let edit_message = EditMessageHandler::new(messages_repo.clone(), event_bus.clone());
    let leave_conversation = LeaveConversationHandler::new(conversations_repo.clone(), event_bus.clone());
    let mark_read = MarkReadHandler::new(event_bus.clone());

    let state = Arc::new(AppState {
        pool,
        event_bus,
        users: users_repo,
        views,
        create_user,
        create_conversation,
        send_message,
        edit_message,
        leave_conversation,
        mark_read,
        upload_dir: config.upload_dir.clone(),
        public_url: config.public_url.clone(),
    });

    Ok(state)
}
