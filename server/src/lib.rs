pub mod config;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;

use std::{
    collections::{HashMap, HashSet},
    sync::{Mutex, RwLock},
};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{
    models::{Conversation, Message},
    repositories::{participant::Participant, user::User},
};

pub type ConversationDb = RwLock<HashMap<Uuid, Conversation>>;
pub type UserDb = RwLock<HashMap<Uuid, User>>;
pub type MessageDb = RwLock<HashMap<Uuid, Message>>;
pub type UserConverstationsDb = RwLock<HashSet<Participant>>;

pub struct AppState {
    pub active_users: Mutex<HashSet<String>>,
    pub users: UserDb,
    pub conversations: ConversationDb,
    pub messages: MessageDb,
    pub user_conversations: UserConverstationsDb,
    pub tx: broadcast::Sender<String>,
}
