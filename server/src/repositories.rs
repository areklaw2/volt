use std::collections::HashMap;

use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::repositories::{
    conversation::{Conversation, ConversationRepository},
    message::{Message, MessageRepository},
    participant::{UserConversation, UserConversationRepository},
    user::{User, UserRepository},
};

pub mod conversation;
pub mod message;
pub mod participant;
pub mod user;

pub trait Repository: Send + Sync + ConversationRepository + MessageRepository + UserConversationRepository + UserRepository {}

pub struct DbRepository {
    pub pool: Pool<Postgres>,
}

impl Repository for DbRepository {}

impl DbRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

pub struct InMemoryRepository {
    conversations_repo: RwLock<HashMap<Uuid, Conversation>>,
    messages_repo: RwLock<HashMap<Uuid, Message>>,
    user_repos: RwLock<HashMap<Uuid, User>>,
    username_to_user_index: RwLock<HashMap<String, User>>,
    user_conversations_repo: RwLock<HashMap<(Uuid, Uuid), UserConversation>>,
    user_to_conversations_index: RwLock<HashMap<Uuid, Vec<Uuid>>>,
    conversation_to_users_index: RwLock<HashMap<Uuid, Vec<Uuid>>>,
}

impl Repository for InMemoryRepository {}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            conversations_repo: RwLock::default(),
            messages_repo: RwLock::default(),
            user_repos: RwLock::default(),
            username_to_user_index: RwLock::default(),
            user_conversations_repo: RwLock::default(),
            user_to_conversations_index: RwLock::default(),
            conversation_to_users_index: RwLock::default(),
        }
    }
}
