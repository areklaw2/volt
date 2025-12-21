pub mod error;
pub mod handlers;

use std::{
    collections::{HashMap, HashSet},
    sync::{Mutex, RwLock},
};
use tokio::sync::broadcast;
use ulid::Ulid;

use crate::handlers::chat::Chat;

pub type ChatDb = RwLock<HashMap<Ulid, Chat>>;

pub struct AppState {
    pub user_set: Mutex<HashSet<String>>,
    pub chats: ChatDb,
    pub tx: broadcast::Sender<String>,
}
