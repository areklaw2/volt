use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::{mesagge::CreateMessageRequest, pagination::Pagination},
    repositories::{DbRepository, InMemoryRepository},
};

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
    async fn read_messages(&self, conversation_id: Uuid, pagination: Pagination) -> Result<Vec<Message>, anyhow::Error>;
}

#[async_trait]
impl MessageRepository for InMemoryRepository {
    async fn create_message(&self, request: CreateMessageRequest) -> Result<Message, anyhow::Error> {
        let message = Message {
            id: Uuid::now_v7(),
            conversation_id: request.conversation_id,
            sender_id: request.sender_id,
            content: request.content,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.messages_repo.write().await.insert(message.id, message.clone());

        Ok(message)
    }

    async fn read_messages(&self, conversation_id: Uuid, pagination: Pagination) -> Result<Vec<Message>, anyhow::Error> {
        let messages = self.messages_repo.read().await;

        let messages = messages
            .values()
            .filter(|m| m.conversation_id == conversation_id)
            .skip(pagination.offset.unwrap_or(0))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect();

        Ok(messages)
    }
}

#[async_trait]
#[allow(unused)]
impl MessageRepository for DbRepository {
    async fn create_message(&self, request: CreateMessageRequest) -> Result<Message, anyhow::Error> {
        todo!()
    }

    async fn read_messages(&self, conversation_id: Uuid, pagination: Pagination) -> Result<Vec<Message>, anyhow::Error> {
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
        let repo = InMemoryRepository::new();
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
        let repo = InMemoryRepository::new();
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
        let repo = InMemoryRepository::new();
        let before = Utc::now();
        let request = create_request(Uuid::now_v7(), Uuid::now_v7(), "Test");

        let message = repo.create_message(request).await.unwrap();

        let after = Utc::now();
        assert!(message.created_at >= before && message.created_at <= after);
    }

    #[tokio::test]
    async fn list_messages_returns_messages_for_conversation() {
        let repo = InMemoryRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv_id, sender_id, "First")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Second")).await.unwrap();

        let messages = repo.read_messages(conv_id, Pagination::default()).await.unwrap();

        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn list_messages_filters_by_conversation_id() {
        let repo = InMemoryRepository::new();
        let conv1 = Uuid::now_v7();
        let conv2 = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv1, sender_id, "Conv1 msg")).await.unwrap();
        repo.create_message(create_request(conv2, sender_id, "Conv2 msg")).await.unwrap();

        let messages = repo.read_messages(conv1, Pagination::default()).await.unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].conversation_id, conv1);
    }

    #[tokio::test]
    async fn list_messages_applies_pagination_offset() {
        let repo = InMemoryRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv_id, sender_id, "First")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Second")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Third")).await.unwrap();

        let pagination = Pagination {
            offset: Some(1),
            limit: None,
        };
        let messages = repo.read_messages(conv_id, pagination).await.unwrap();

        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn list_messages_applies_pagination_limit() {
        let repo = InMemoryRepository::new();
        let conv_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        repo.create_message(create_request(conv_id, sender_id, "First")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Second")).await.unwrap();
        repo.create_message(create_request(conv_id, sender_id, "Third")).await.unwrap();

        let pagination = Pagination {
            offset: None,
            limit: Some(2),
        };
        let messages = repo.read_messages(conv_id, pagination).await.unwrap();

        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn list_messages_returns_empty_for_no_messages() {
        let repo = InMemoryRepository::new();
        let conv_id = Uuid::now_v7();

        let messages = repo.read_messages(conv_id, Pagination::default()).await.unwrap();

        assert!(messages.is_empty());
    }
}
