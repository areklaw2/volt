use async_trait::async_trait;
use thiserror::Error;

use crate::domain::conversation::Conversation;
use crate::domain::events::DomainEvent;
use crate::domain::ids::{ConversationId, MessageId, UserId};
use crate::domain::message::Message;
use crate::domain::user::User;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum PublishError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

#[async_trait]
pub trait ConversationRepository: Send + Sync {
    async fn find_by_id(&self, id: &ConversationId) -> Result<Option<Conversation>, RepoError>;
    async fn save(&self, conversation: &Conversation) -> Result<(), RepoError>;
}

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn find_by_id(&self, id: &MessageId) -> Result<Option<Message>, RepoError>;
    async fn save(&self, message: &Message) -> Result<(), RepoError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepoError>;
    async fn find_all(&self) -> Result<Vec<User>, RepoError>;
    async fn save(&self, user: &User) -> Result<(), RepoError>;
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<(), PublishError>;
}
