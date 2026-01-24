use std::collections::HashMap;

use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::dto::{CreateUserRequest, UpdateUserRequest};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, anyhow::Error>;
    async fn read_user(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error>;
    async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<User, anyhow::Error>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), anyhow::Error>;
}

#[derive(Debug)]
pub struct InMemoryUserRepository {
    users: tokio::sync::RwLock<HashMap<Uuid, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self { users: RwLock::default() }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, anyhow::Error> {
        let user = User {
            id: Uuid::now_v7(),
            username: request.username,
            display_name: request.display_name,
            avatar_url: request.avatar_url,
            created_at: Utc::now(),
        };

        self.users.write().await.insert(user.id, user.clone());

        Ok(user)
    }

    async fn read_user(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error> {
        Ok(self.users.read().await.get(&user_id).cloned())
    }

    async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<User, anyhow::Error> {
        let mut users = self.users.write().await;
        let user = users.get_mut(&user_id).ok_or_else(|| anyhow::anyhow!("User not found"))?;

        if let Some(display_name) = request.display_name {
            user.display_name = display_name;
        }
        if let Some(avatar_url) = request.avatar_url {
            user.avatar_url = avatar_url;
        }

        Ok(user.clone())
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        self.users.write().await.remove(&user_id);
        Ok(())
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct DbUserRepository {
    pool: Pool<Postgres>,
}

impl DbUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
#[allow(unused)]
impl UserRepository for DbUserRepository {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, anyhow::Error> {
        todo!()
    }

    async fn read_user(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error> {
        todo!()
    }

    async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<User, anyhow::Error> {
        todo!()
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}
