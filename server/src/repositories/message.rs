use std::collections::HashMap;

use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::dto::{CreateMessageRequest, Pagination, UpdateMessageRequest};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn create_message(&self, request: CreateMessageRequest) -> Result<Message, anyhow::Error>;
    async fn read_message(&self, message_id: Uuid) -> Result<Option<Message>, anyhow::Error>;
    async fn list_messages(&self, conversation_id: Uuid, pagination: Pagination) -> Result<Vec<Message>, anyhow::Error>;
    async fn update_message(&self, message_id: Uuid, request: UpdateMessageRequest) -> Result<Option<Message>, anyhow::Error>;
    async fn delete_message(&self, message_id: Uuid) -> Result<(), anyhow::Error>;
}

#[derive(Debug, Default)]
pub struct InMemoryMessageRepository {
    messages: tokio::sync::RwLock<HashMap<Uuid, Message>>,
}

impl InMemoryMessageRepository {
    pub fn new() -> Self {
        Self {
            messages: RwLock::default(),
        }
    }
}

#[async_trait]
impl MessageRepository for InMemoryMessageRepository {
    async fn create_message(&self, request: CreateMessageRequest) -> Result<Message, anyhow::Error> {
        let message = Message {
            id: Uuid::now_v7(),
            conversation_id: request.conversation_id,
            sender_id: request.sender_id,
            content: request.content,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.messages.write().await.insert(message.id, message.clone());

        Ok(message)
    }

    async fn read_message(&self, message_id: Uuid) -> Result<Option<Message>, anyhow::Error> {
        Ok(self.messages.read().await.get(&message_id).cloned())
    }

    async fn list_messages(&self, conversation_id: Uuid, pagination: Pagination) -> Result<Vec<Message>, anyhow::Error> {
        let messages = self.messages.read().await;

        let messages = messages
            .values()
            .filter(|m| m.conversation_id == conversation_id)
            .skip(pagination.offset.unwrap_or(0))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect();

        Ok(messages)
    }

    async fn update_message(&self, message_id: Uuid, request: UpdateMessageRequest) -> Result<Option<Message>, anyhow::Error> {
        let mut messages = self.messages.write().await;
        let Some(message) = messages.get_mut(&message_id) else {
            return Ok(None);
        };

        if let Some(content) = request.content {
            message.content = content;
            message.updated_at = Some(Utc::now())
        }

        Ok(Some(message.clone()))
    }

    async fn delete_message(&self, message_id: Uuid) -> Result<(), anyhow::Error> {
        self.messages.write().await.remove(&message_id);
        Ok(())
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct DbMessageRepository {
    pool: Pool<Postgres>,
}

impl DbMessageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
#[allow(unused)]
impl MessageRepository for DbMessageRepository {
    async fn create_message(&self, request: CreateMessageRequest) -> Result<Message, anyhow::Error> {
        todo!()
    }

    async fn read_message(&self, message_id: Uuid) -> Result<Option<Message>, anyhow::Error> {
        todo!()
    }

    async fn list_messages(&self, conversation_id: Uuid, pagination: Pagination) -> Result<Vec<Message>, anyhow::Error> {
        todo!()
    }

    async fn update_message(&self, message_id: Uuid, request: UpdateMessageRequest) -> Result<Option<Message>, anyhow::Error> {
        todo!()
    }

    async fn delete_message(&self, message_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_request(conversation_id: Uuid, sender_id: Uuid, content: &str) -> CreateMessageRequest {
        CreateMessageRequest {
            conversation_id,
            sender_id,
            content: content.to_string(),
        }
    }

    #[tokio::test]
    async fn create_message_returns_message_with_correct_fields() {
        let repo = InMemoryMessageRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let request = create_request(conv_id, sender_id, "Hello world");

        let message = repo.create_message(request).await.unwrap();

        assert_eq!(message.conversation_id, conv_id);
        assert_eq!(message.sender_id, sender_id);
        assert_eq!(message.content, "Hello world");
    }

    #[tokio::test]
    async fn create_message_generates_unique_id() {
        let repo = InMemoryMessageRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let request1 = create_request(conv_id, sender_id, "First");
        let request2 = create_request(conv_id, sender_id, "Second");

        let msg1 = repo.create_message(request1).await.unwrap();
        let msg2 = repo.create_message(request2).await.unwrap();

        assert_ne!(msg1.id, msg2.id);
    }

    #[tokio::test]
    async fn create_message_sets_created_at_timestamp() {
        let repo = InMemoryMessageRepository::new();
        let before = Utc::now();
        let request = create_request(Uuid::now_v7(), Uuid::now_v7(), "Test");

        let message = repo.create_message(request).await.unwrap();

        let after = Utc::now();
        assert!(message.created_at >= before && message.created_at <= after);
    }

    #[tokio::test]
    async fn read_message_returns_existing_message() {
        let repo = InMemoryMessageRepository::new();
        let request = create_request(Uuid::now_v7(), Uuid::now_v7(), "Test message");
        let created = repo.create_message(request).await.unwrap();

        let message = repo.read_message(created.id).await.unwrap().unwrap();

        assert_eq!(message.id, created.id);
        assert_eq!(message.content, "Test message");
    }

    #[tokio::test]
    async fn read_message_returns_none_for_nonexistent() {
        let repo = InMemoryMessageRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.read_message(random_id).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn list_messages_returns_messages_for_conversation() {
        let repo = InMemoryMessageRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv_id, sender_id, "First")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Second")).await.unwrap();

        let messages = repo.list_messages(conv_id, Pagination::default()).await.unwrap();

        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn list_messages_filters_by_conversation_id() {
        let repo = InMemoryMessageRepository::new();
        let conv1 = Uuid::now_v7();
        let conv2 = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv1, sender_id, "Conv1 msg")).await.unwrap();
        repo.create_message(create_request(conv2, sender_id, "Conv2 msg")).await.unwrap();

        let messages = repo.list_messages(conv1, Pagination::default()).await.unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].conversation_id, conv1);
    }

    #[tokio::test]
    async fn list_messages_applies_pagination_offset() {
        let repo = InMemoryMessageRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv_id, sender_id, "First")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Second")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Third")).await.unwrap();

        let pagination = Pagination {
            offset: Some(1),
            limit: None,
        };
        let messages = repo.list_messages(conv_id, pagination).await.unwrap();

        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn list_messages_applies_pagination_limit() {
        let repo = InMemoryMessageRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv_id, sender_id, "First")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Second")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Third")).await.unwrap();

        let pagination = Pagination {
            offset: None,
            limit: Some(2),
        };
        let messages = repo.list_messages(conv_id, pagination).await.unwrap();

        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn list_messages_returns_empty_for_no_messages() {
        let repo = InMemoryMessageRepository::new();
        let conv_id = Uuid::now_v7();

        let messages = repo.list_messages(conv_id, Pagination::default()).await.unwrap();

        assert!(messages.is_empty());
    }

    #[tokio::test]
    async fn update_message_updates_content() {
        let repo = InMemoryMessageRepository::new();
        let request = create_request(Uuid::now_v7(), Uuid::now_v7(), "Original");
        let created = repo.create_message(request).await.unwrap();

        let update = UpdateMessageRequest {
            content: Some("Updated content".to_string()),
        };
        let updated = repo.update_message(created.id, update).await.unwrap().unwrap();

        assert_eq!(updated.content, "Updated content");
    }

    #[tokio::test]
    async fn update_message_sets_updated_at() {
        let repo = InMemoryMessageRepository::new();
        let request = create_request(Uuid::now_v7(), Uuid::now_v7(), "Original");
        let created = repo.create_message(request).await.unwrap();
        assert!(created.updated_at.is_none());

        let before = Utc::now();
        let update = UpdateMessageRequest {
            content: Some("Updated".to_string()),
        };
        let updated = repo.update_message(created.id, update).await.unwrap().unwrap();
        let after = Utc::now();

        assert!(updated.updated_at.is_some());
        let updated_at = updated.updated_at.unwrap();
        assert!(updated_at >= before && updated_at <= after);
    }

    #[tokio::test]
    async fn update_message_returns_none_for_nonexistent() {
        let repo = InMemoryMessageRepository::new();
        let random_id = Uuid::now_v7();

        let update = UpdateMessageRequest {
            content: Some("Content".to_string()),
        };
        let result = repo.update_message(random_id, update).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_message_removes_message() {
        let repo = InMemoryMessageRepository::new();
        let request = create_request(Uuid::now_v7(), Uuid::now_v7(), "To delete");
        let created = repo.create_message(request).await.unwrap();

        repo.delete_message(created.id).await.unwrap();

        let read = repo.read_message(created.id).await.unwrap();
        assert!(read.is_none());
    }

    #[tokio::test]
    async fn delete_message_succeeds_for_nonexistent() {
        let repo = InMemoryMessageRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.delete_message(random_id).await;

        assert!(result.is_ok());
    }
}
