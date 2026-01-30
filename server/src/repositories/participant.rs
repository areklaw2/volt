use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::user_conversation::{CreateUserConversationsRequest, UpdateUserConversationRequest},
    repositories::{DbRepository, InMemoryRepository},
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct UserConversation {
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub joined_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait UserConversationRepository: Send + Sync {
    async fn create_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<UserConversation, anyhow::Error>;
    async fn create_user_conversations(&self, request: CreateUserConversationsRequest)
    -> Result<Vec<UserConversation>, anyhow::Error>;
    async fn read_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<Option<UserConversation>, anyhow::Error>;
    async fn update_user_conversation(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        request: UpdateUserConversationRequest,
    ) -> Result<Option<UserConversation>, anyhow::Error>;
    async fn delete_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl UserConversationRepository for InMemoryRepository {
    async fn create_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<UserConversation, anyhow::Error> {
        let mut user_conversations = self.user_conversations_repo.write().await;
        let mut conversation_index = self.conversation_index.write().await;
        let mut user_index = self.user_index.write().await;

        let now = Some(Utc::now());
        let user_conversation = UserConversation {
            user_id,
            conversation_id: conversation_id,
            joined_at: now,
            last_read_at: now,
        };

        let key = (user_id, conversation_id);
        user_conversations.insert(key, user_conversation.clone());

        user_index.entry(user_id).or_default().push(conversation_id);
        conversation_index.entry(conversation_id).or_default().push(user_id);

        Ok(user_conversation)
    }

    async fn create_user_conversations(
        &self,
        request: CreateUserConversationsRequest,
    ) -> Result<Vec<UserConversation>, anyhow::Error> {
        let mut user_conversations = self.user_conversations_repo.write().await;
        let mut conversation_index = self.conversation_index.write().await;
        let mut user_index = self.user_index.write().await;

        let mut result = Vec::new();
        for user_id in request.users {
            let mut user_conversation = UserConversation {
                user_id,
                conversation_id: request.conversation_id,
                joined_at: None,
                last_read_at: None,
            };

            if request.sender_id == user_id {
                let now = Some(Utc::now());
                user_conversation.joined_at = now;
                user_conversation.last_read_at = now;
            }

            let key = (user_id, request.conversation_id);
            user_conversations.insert(key, user_conversation.clone());

            user_index.entry(user_id).or_default().push(request.conversation_id);
            conversation_index.entry(request.conversation_id).or_default().push(user_id);

            result.push(user_conversation);
        }

        Ok(result)
    }

    async fn read_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<Option<UserConversation>, anyhow::Error> {
        let key = (user_id, conversation_id);
        Ok(self.user_conversations_repo.read().await.get(&key).cloned())
    }

    async fn update_user_conversation(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        request: UpdateUserConversationRequest,
    ) -> Result<Option<UserConversation>, anyhow::Error> {
        let key = (user_id, conversation_id);
        let mut user_conversations = self.user_conversations_repo.write().await;
        let Some(user_conversation) = user_conversations.get_mut(&key) else {
            return Ok(None);
        };

        if let Some(joined_at) = request.joined_at
            && user_conversation.joined_at.is_none()
        {
            user_conversation.joined_at = Some(joined_at);
        }

        if let Some(last_read_at) = request.last_read_at {
            user_conversation.last_read_at = Some(last_read_at);
        }

        Ok(Some(user_conversation.clone()))
    }

    async fn delete_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error> {
        let key = (user_id, conversation_id);
        self.user_conversations_repo.write().await.remove(&key);

        let mut user_index = self.user_index.write().await;
        if let Some(conversation_ids) = user_index.get_mut(&user_id) {
            conversation_ids.retain(|id| *id != conversation_id);
        }

        let mut conversation_index = self.conversation_index.write().await;
        if let Some(user_ids) = conversation_index.get_mut(&conversation_id) {
            user_ids.retain(|id| *id != user_id);
        }

        Ok(())
    }
}

#[async_trait]
#[allow(unused)]
impl UserConversationRepository for DbRepository {
    async fn create_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<UserConversation, anyhow::Error> {
        todo!()
    }

    async fn create_user_conversations(
        &self,
        request: CreateUserConversationsRequest,
    ) -> Result<Vec<UserConversation>, anyhow::Error> {
        todo!()
    }

    async fn read_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<Option<UserConversation>, anyhow::Error> {
        todo!()
    }

    async fn update_user_conversation(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        request: UpdateUserConversationRequest,
    ) -> Result<Option<UserConversation>, anyhow::Error> {
        todo!()
    }

    async fn delete_user_conversation(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_user_conversation_creates_user_conversation() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let user1 = Uuid::now_v7();

        let user_conversation = repo.create_user_conversation(user1, conversation_id).await.unwrap();
        assert!(user_conversation.joined_at.is_some());
        assert!(user_conversation.last_read_at.is_some());
    }

    fn create_request(sender_id: Uuid, conversation_id: Uuid, users: Vec<Uuid>) -> CreateUserConversationsRequest {
        CreateUserConversationsRequest {
            sender_id,
            conversation_id,
            users,
        }
    }

    #[tokio::test]
    async fn create_conversation_user_conversations_creates_user_conversations_for_multiple_users() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let user1 = Uuid::now_v7();
        let user2 = Uuid::now_v7();
        let user3 = Uuid::now_v7();
        let request = create_request(user1, conversation_id, vec![user1, user2, user3]);

        let user_conversations = repo.create_user_conversations(request).await.unwrap();
        assert_eq!(user_conversations.len(), 3);
    }

    #[tokio::test]
    async fn create_conversation_user_conversations_sets_timestamps_for_sender() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id]);

        repo.create_user_conversations(request).await.unwrap();

        let user_conversation = repo.read_user_conversation(sender_id, conversation_id).await.unwrap().unwrap();
        assert!(user_conversation.joined_at.is_some());
        assert!(user_conversation.last_read_at.is_some());
    }

    #[tokio::test]
    async fn create_conversation_user_conversations_non_sender_has_none_timestamps() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let other_user = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id, other_user]);

        repo.create_user_conversations(request).await.unwrap();

        let user_conversation = repo.read_user_conversation(other_user, conversation_id).await.unwrap().unwrap();
        assert!(user_conversation.joined_at.is_none());
        assert!(user_conversation.last_read_at.is_none());
    }

    #[tokio::test]
    async fn create_conversation_user_conversations_are_persisted_and_readable() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);

        repo.create_user_conversations(request).await.unwrap();

        let user_conversation = repo.read_user_conversation(user_id, conversation_id).await.unwrap();
        assert!(user_conversation.is_some());
        assert_eq!(user_conversation.unwrap().user_id, user_id);
    }

    #[tokio::test]
    async fn read_user_conversation_returns_existing_user_conversation() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);
        repo.create_user_conversations(request).await.unwrap();

        let user_conversation = repo.read_user_conversation(user_id, conversation_id).await.unwrap().unwrap();

        assert_eq!(user_conversation.user_id, user_id);
        assert_eq!(user_conversation.conversation_id, conversation_id);
    }

    #[tokio::test]
    async fn read_user_conversation_returns_none_for_nonexistent_user_conversation() {
        let repo = InMemoryRepository::new();
        let random_user = Uuid::now_v7();
        let random_conversation = Uuid::now_v7();

        let result = repo.read_user_conversation(random_user, random_conversation).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn update_user_conversation_updates_joined_at_when_none() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let other_user = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id, other_user]);
        repo.create_user_conversations(request).await.unwrap();

        let now = Utc::now();
        let update = UpdateUserConversationRequest {
            joined_at: Some(now),
            last_read_at: None,
        };
        let updated = repo
            .update_user_conversation(other_user, conversation_id, update)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.joined_at, Some(now));
    }

    #[tokio::test]
    async fn update_user_conversation_does_not_overwrite_existing_joined_at() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id]);
        repo.create_user_conversations(request).await.unwrap();

        let original = repo.read_user_conversation(sender_id, conversation_id).await.unwrap().unwrap();
        let original_joined_at = original.joined_at;

        let new_time = Utc::now();
        let update = UpdateUserConversationRequest {
            joined_at: Some(new_time),
            last_read_at: None,
        };
        let updated = repo
            .update_user_conversation(sender_id, conversation_id, update)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.joined_at, original_joined_at);
    }

    #[tokio::test]
    async fn update_user_conversation_updates_last_read_at() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);
        repo.create_user_conversations(request).await.unwrap();

        let new_time = Utc::now();
        let update = UpdateUserConversationRequest {
            joined_at: None,
            last_read_at: Some(new_time),
        };
        let updated = repo
            .update_user_conversation(user_id, conversation_id, update)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.last_read_at, Some(new_time));
    }

    #[tokio::test]
    async fn update_user_conversation_returns_none_for_nonexistent() {
        let repo = InMemoryRepository::new();
        let random_user = Uuid::now_v7();
        let random_conversation = Uuid::now_v7();

        let update = UpdateUserConversationRequest {
            joined_at: Some(Utc::now()),
            last_read_at: None,
        };
        let result = repo
            .update_user_conversation(random_user, random_conversation, update)
            .await
            .unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_user_conversation_removes_from_main_storage() {
        let repo = InMemoryRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);
        repo.create_user_conversations(request).await.unwrap();

        repo.delete_user_conversation(user_id, conversation_id).await.unwrap();

        let result = repo.read_user_conversation(user_id, conversation_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_user_conversation_succeeds_for_nonexistent() {
        let repo = InMemoryRepository::new();
        let random_user = Uuid::now_v7();
        let random_conversation = Uuid::now_v7();

        let result = repo.delete_user_conversation(random_user, random_conversation).await;

        assert!(result.is_ok());
    }
}
