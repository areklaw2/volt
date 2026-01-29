use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::{CreateUserRequest, UpdateUserRequest},
    repositories::{DbRepository, InMemoryRepository},
};

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
            avatar_url: request.avatar_url,
            created_at: Utc::now(),
        };

        self.user_repos.write().await.insert(user.id, user.clone());

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
        if let Some(avatar_url) = request.avatar_url {
            user.avatar_url = avatar_url;
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

    fn create_request(username: &str, display_name: &str, avatar_url: &str) -> CreateUserRequest {
        CreateUserRequest {
            username: username.to_string(),
            display_name: display_name.to_string(),
            avatar_url: avatar_url.to_string(),
        }
    }

    #[tokio::test]
    async fn create_user_returns_user_with_correct_fields() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice Smith", "https://example.com/alice.png");

        let user = repo.create_user(request).await.unwrap();

        assert_eq!(user.username, "alice");
        assert_eq!(user.display_name, "Alice Smith");
        assert_eq!(user.avatar_url, "https://example.com/alice.png");
    }

    #[tokio::test]
    async fn create_user_generates_unique_id() {
        let repo = InMemoryRepository::new();
        let request1 = create_request("alice", "Alice", "https://example.com/a.png");
        let request2 = create_request("bob", "Bob", "https://example.com/b.png");

        let user1 = repo.create_user(request1).await.unwrap();
        let user2 = repo.create_user(request2).await.unwrap();

        assert_ne!(user1.id, user2.id);
    }

    #[tokio::test]
    async fn create_user_sets_created_at_timestamp() {
        let repo = InMemoryRepository::new();
        let before = Utc::now();
        let request = create_request("alice", "Alice", "https://example.com/a.png");

        let user = repo.create_user(request).await.unwrap();

        let after = Utc::now();
        assert!(user.created_at >= before && user.created_at <= after);
    }

    #[tokio::test]
    async fn create_user_persists_user() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice", "https://example.com/a.png");

        let created = repo.create_user(request).await.unwrap();
        let read = repo.read_user(created.id).await.unwrap();

        assert!(read.is_some());
        assert_eq!(read.unwrap().id, created.id);
    }

    #[tokio::test]
    async fn read_user_returns_existing_user() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice Smith", "https://example.com/a.png");
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
        let request = create_request("alice", "Alice", "https://example.com/a.png");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest {
            display_name: Some("Alice Updated".to_string()),
            avatar_url: None,
        };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice Updated");
        assert_eq!(updated.avatar_url, "https://example.com/a.png");
    }

    #[tokio::test]
    async fn update_user_updates_avatar_url_only() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice", "https://example.com/a.png");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest {
            display_name: None,
            avatar_url: Some("https://example.com/new.png".to_string()),
        };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice");
        assert_eq!(updated.avatar_url, "https://example.com/new.png");
    }

    #[tokio::test]
    async fn update_user_updates_both_fields() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice", "https://example.com/a.png");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest {
            display_name: Some("Alice New".to_string()),
            avatar_url: Some("https://example.com/new.png".to_string()),
        };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice New");
        assert_eq!(updated.avatar_url, "https://example.com/new.png");
    }

    #[tokio::test]
    async fn update_user_with_all_none_leaves_user_unchanged() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice", "https://example.com/a.png");
        let created = repo.create_user(request).await.unwrap();

        let update = UpdateUserRequest {
            display_name: None,
            avatar_url: None,
        };
        let updated = repo.update_user(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.display_name, "Alice");
        assert_eq!(updated.avatar_url, "https://example.com/a.png");
    }

    #[tokio::test]
    async fn update_user_returns_none_for_nonexistent_user() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let update = UpdateUserRequest {
            display_name: Some("New Name".to_string()),
            avatar_url: None,
        };
        let result = repo.update_user(random_id, update).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_user_removes_existing_user() {
        let repo = InMemoryRepository::new();
        let request = create_request("alice", "Alice", "https://example.com/a.png");
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
