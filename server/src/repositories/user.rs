use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::user::{CreateUserRequest, UpdateUserRequest},
    repositories::{DbRepository, InMemoryRepository},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, anyhow::Error>;
    async fn read_user(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error>;
    async fn read_users(&self, user_ids: Vec<Uuid>) -> Result<Vec<User>, anyhow::Error>;
    async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<Option<User>, anyhow::Error>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl UserRepository for InMemoryRepository {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, anyhow::Error> {
        let user = User {
            id: Uuid::now_v7(),
            username: request.username,
            display_name: request.display_name,
            created_at: Utc::now(),
        };

        self.user_repos.write().await.insert(user.id, user.clone());
        self.username_to_user_index
            .write()
            .await
            .insert(user.username.clone(), user.clone());

        Ok(user)
    }

    async fn read_user(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error> {
        Ok(self.user_repos.read().await.get(&user_id).cloned())
    }

    async fn read_users(&self, user_ids: Vec<Uuid>) -> Result<Vec<User>, anyhow::Error> {
        let users = self.user_repos.read().await;
        Ok(user_ids.iter().filter_map(|id| users.get(id).cloned()).collect())
    }

    async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<Option<User>, anyhow::Error> {
        let mut users = self.user_repos.write().await;
        let Some(user) = users.get_mut(&user_id) else {
            return Ok(None);
        };

        if let Some(display_name) = request.display_name {
            user.display_name = display_name;
        }

        Ok(Some(user.clone()))
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        self.user_repos.write().await.remove(&user_id);
        Ok(())
    }
}

#[async_trait]
#[allow(unused)]
impl UserRepository for DbRepository {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, anyhow::Error> {
        todo!()
    }

    async fn read_user(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error> {
        todo!()
    }

    async fn read_users(&self, user_ids: Vec<Uuid>) -> Result<Vec<User>, anyhow::Error> {
        todo!()
    }

    async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<Option<User>, anyhow::Error> {
        todo!()
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_request(username: &str, display_name: &str) -> CreateUserRequest {
        CreateUserRequest {
            username: username.to_string(),
            display_name: display_name.to_string(),
        }
    }

    #[tokio::test]
    async fn create_user_returns_user_with_correct_fields() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice Smith");

        let user = repo.create_user(request).await.unwrap();

        assert_eq!(user.username, "alice");
        assert_eq!(user.display_name, "Alice Smith");
    }

    #[tokio::test]
    async fn create_user_generates_unique_id() {
        let repo = InMemoryRepository::new();
        let request1 = create_request("alice", "Alice");
        let request2 = create_request("bob", "Bob");

        let user1 = repo.create_user(request1).await.unwrap();
        let user2 = repo.create_user(request2).await.unwrap();

        assert_ne!(user1.id, user2.id);
    }

    #[tokio::test]
    async fn create_user_sets_created_at_timestamp() {
        let repo = InMemoryRepository::new();
        let before = Utc::now();
        let request = create_request("alice", "Alice");

        let user = repo.create_user(request).await.unwrap();

        let after = Utc::now();
        assert!(user.created_at >= before && user.created_at <= after);
    }

    #[tokio::test]
    async fn create_user_persists_user() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice");

        let created = repo.create_user(request).await.unwrap();
        let read = repo.read_user(created.id).await.unwrap();

        assert!(read.is_some());
        assert_eq!(read.unwrap().id, created.id);
    }

    #[tokio::test]
    async fn read_user_returns_existing_user() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice Smith");
        let created = repo.create_user(request).await.unwrap();

        let user = repo.read_user(created.id).await.unwrap().unwrap();

        assert_eq!(user.id, created.id);
        assert_eq!(user.username, "alice");
        assert_eq!(user.display_name, "Alice Smith");
    }

    #[tokio::test]
    async fn read_user_returns_none_for_nonexistent_user() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.read_user(random_id).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn update_user_updates_display_name_only() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest {
            display_name: Some("Alice Updated".to_string()),
        };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice Updated");
    }

    #[tokio::test]
    async fn update_user_updates_avatar_url_only() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest { display_name: None };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice");
    }

    #[tokio::test]
    async fn update_user_updates_both_fields() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest {
            display_name: Some("Alice New".to_string()),
        };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice New");
    }

    #[tokio::test]
    async fn update_user_with_all_none_leaves_user_unchanged() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest { display_name: None };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice");
    }

    #[tokio::test]
    async fn update_user_returns_none_for_nonexistent_user() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let update = UpdateUserRequest {
            display_name: Some("New Name".to_string()),
        };
        let result = repo.update_user(random_id, update).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_user_removes_existing_user() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice");
        let created = repo.create_user(request).await.unwrap();

        repo.delete_user(created.id).await.unwrap();

        let read = repo.read_user(created.id).await.unwrap();
        assert!(read.is_none());
    }

    #[tokio::test]
    async fn delete_user_succeeds_for_nonexistent_user() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.delete_user(random_id).await;

        assert!(result.is_ok());
    }
}
