pub mod config;
pub mod error;
pub mod models;
pub mod routes;

use std::{
    collections::{HashMap, HashSet},
    sync::{Mutex, RwLock},
};
use tokio::sync::broadcast;
use ulid::Ulid;

use crate::models::{Conversation, Message, User, UserConversation};

pub type ConversationDb = RwLock<HashMap<Ulid, Conversation>>;
pub type UserDb = RwLock<HashMap<Ulid, User>>;
pub type MessageDb = RwLock<HashMap<Ulid, Message>>;
pub type UserConverstationsDb = RwLock<HashSet<UserConversation>>;

pub struct AppState {
    pub active_users: Mutex<HashSet<String>>,
    pub users: UserDb,
    pub conversations: ConversationDb,
    pub messages: MessageDb,
    pub user_conversations: UserConverstationsDb,
    pub tx: broadcast::Sender<String>,
}
