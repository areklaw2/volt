use std::collections::HashMap;

use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::dto::{CreateConversationRequest, UpdateConversationRequest};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationType {
    Direct,
    Group,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub conversation_type: ConversationType,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait ConversationRepository: Send + Sync {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<Conversation, anyhow::Error>;
    async fn read_conversation(&self, id: Uuid) -> Result<Option<Conversation>, anyhow::Error>;
    async fn update_conversation(&self, id: Uuid, request: UpdateConversationRequest) -> Result<Option<Conversation>, anyhow::Error>;
    async fn delete_conversation(&self, id: Uuid) -> Result<(), anyhow::Error>;
}

#[derive(Debug, Default)]
pub struct InMemoryConversationRepository {
    conversations: RwLock<HashMap<Uuid, Conversation>>,
}

impl InMemoryConversationRepository {
    pub fn new() -> Self {
        Self {
            conversations: RwLock::default(),
        }
    }
}

#[async_trait]
impl ConversationRepository for InMemoryConversationRepository {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<Conversation, anyhow::Error> {
        let conversation = Conversation {
            id: Uuid::now_v7(),
            conversation_type: request.conversation_type,
            name: request.name,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.conversations.write().await.insert(conversation.id, conversation.clone());

        Ok(conversation)
    }

    async fn read_conversation(&self, id: Uuid) -> Result<Option<Conversation>, anyhow::Error> {
        Ok(self.conversations.read().await.get(&id).cloned())
    }

    async fn update_conversation(&self, id: Uuid, request: UpdateConversationRequest) -> Result<Option<Conversation>, anyhow::Error> {
        let mut conversations = self.conversations.write().await;
        let Some(conversation) = conversations.get_mut(&id) else {
            return Ok(None);
        };

        if let Some(name) = request.name {
            conversation.name = Some(name);
            conversation.updated_at = Some(Utc::now());
        }

        Ok(Some(conversation.clone()))
    }

    async fn delete_conversation(&self, id: Uuid) -> Result<(), anyhow::Error> {
        self.conversations.write().await.remove(&id);
        Ok(())
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct DbConversationRepository {
    pool: Pool<Postgres>,
}

impl DbConversationRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
#[allow(unused)]
impl ConversationRepository for DbConversationRepository {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<Conversation, anyhow::Error> {
        todo!()
    }

    async fn read_conversation(&self, id: Uuid) -> Result<Option<Conversation>, anyhow::Error> {
        todo!()
    }

    async fn update_conversation(&self, id: Uuid, request: UpdateConversationRequest) -> Result<Option<Conversation>, anyhow::Error> {
        todo!()
    }

    async fn delete_conversation(&self, id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_request(conversation_type: ConversationType, name: Option<&str>) -> CreateConversationRequest {
        CreateConversationRequest {
            conversation_type,
            name: name.map(String::from),
        }
    }

    #[tokio::test]
    async fn create_conversation_returns_conversation_with_correct_fields() {
        let repo = InMemoryConversationRepository::new();
        let request = create_request(ConversationType::Group, Some("Test Group"));

        let conversation = repo.create_conversation(request).await.unwrap();

        assert_eq!(conversation.conversation_type, ConversationType::Group);
        assert_eq!(conversation.name, Some("Test Group".to_string()));
    }

    #[tokio::test]
    async fn create_conversation_generates_unique_id() {
        let repo = InMemoryConversationRepository::new();
        let request1 = create_request(ConversationType::Direct, None);
        let request2 = create_request(ConversationType::Group, Some("Group"));

        let conv1 = repo.create_conversation(request1).await.unwrap();
        let conv2 = repo.create_conversation(request2).await.unwrap();

        assert_ne!(conv1.id, conv2.id);
    }

    #[tokio::test]
    async fn create_conversation_sets_created_at_timestamp() {
        let repo = InMemoryConversationRepository::new();
        let before = Utc::now();
        let request = create_request(ConversationType::Direct, None);

        let conversation = repo.create_conversation(request).await.unwrap();

        let after = Utc::now();
        assert!(conversation.created_at >= before && conversation.created_at <= after);
    }

    #[tokio::test]
    async fn create_direct_conversation_has_no_name() {
        let repo = InMemoryConversationRepository::new();
        let request = create_request(ConversationType::Direct, None);

        let conversation = repo.create_conversation(request).await.unwrap();

        assert_eq!(conversation.conversation_type, ConversationType::Direct);
        assert!(conversation.name.is_none());
    }

    #[tokio::test]
    async fn create_group_conversation_has_name() {
        let repo = InMemoryConversationRepository::new();
        let request = create_request(ConversationType::Group, Some("My Group Chat"));

        let conversation = repo.create_conversation(request).await.unwrap();

        assert_eq!(conversation.conversation_type, ConversationType::Group);
        assert_eq!(conversation.name, Some("My Group Chat".to_string()));
    }

    #[tokio::test]
    async fn read_conversation_returns_existing() {
        let repo = InMemoryConversationRepository::new();
        let request = create_request(ConversationType::Group, Some("Test"));
        let created = repo.create_conversation(request).await.unwrap();

        let conversation = repo.read_conversation(created.id).await.unwrap().unwrap();

        assert_eq!(conversation.id, created.id);
        assert_eq!(conversation.name, Some("Test".to_string()));
    }

    #[tokio::test]
    async fn read_conversation_returns_none_for_nonexistent() {
        let repo = InMemoryConversationRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.read_conversation(random_id).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn update_conversation_updates_name() {
        let repo = InMemoryConversationRepository::new();
        let request = create_request(ConversationType::Group, Some("Original"));
        let created = repo.create_conversation(request).await.unwrap();

        let update = UpdateConversationRequest {
            name: Some("Updated Name".to_string()),
        };
        let updated = repo.update_conversation(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.name, Some("Updated Name".to_string()));
    }

    #[tokio::test]
    async fn update_conversation_sets_updated_at() {
        let repo = InMemoryConversationRepository::new();
        let request = create_request(ConversationType::Group, Some("Test"));
        let created = repo.create_conversation(request).await.unwrap();
        assert!(created.updated_at.is_none());

        let before = Utc::now();
        let update = UpdateConversationRequest {
            name: Some("New Name".to_string()),
        };
        let updated = repo.update_conversation(created.id, update).await.unwrap().unwrap();
        let after = Utc::now();

        assert!(updated.updated_at.is_some());
        let updated_at = updated.updated_at.unwrap();
        assert!(updated_at >= before && updated_at <= after);
    }

    #[tokio::test]
    async fn update_conversation_returns_none_for_nonexistent() {
        let repo = InMemoryConversationRepository::new();
        let random_id = Uuid::now_v7();

        let update = UpdateConversationRequest {
            name: Some("Name".to_string()),
        };
        let result = repo.update_conversation(random_id, update).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_conversation_removes_conversation() {
        let repo = InMemoryConversationRepository::new();
        let request = create_request(ConversationType::Direct, None);
        let created = repo.create_conversation(request).await.unwrap();

        repo.delete_conversation(created.id).await.unwrap();

        let read = repo.read_conversation(created.id).await.unwrap();
        assert!(read.is_none());
    }

    #[tokio::test]
    async fn delete_conversation_succeeds_for_nonexistent() {
        let repo = InMemoryConversationRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.delete_conversation(random_id).await;

        assert!(result.is_ok());
    }
}
