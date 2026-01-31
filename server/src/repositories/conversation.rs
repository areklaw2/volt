use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::conversation::{CreateConversationRequest, UpdateConversationRequest},
    repositories::{DbRepository, InMemoryRepository, participant::UserConversation, user::User},
};

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

pub struct ConversationAggregate {
    pub conversation: Conversation,
    pub user_conversations: Vec<UserConversation>,
    pub users: Vec<User>,
}

#[async_trait]
pub trait ConversationRepository: Send + Sync {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<ConversationAggregate, anyhow::Error>;
    async fn read_conversation(&self, id: Uuid) -> Result<Option<ConversationAggregate>, anyhow::Error>;
    async fn read_conversations_by_user(&self, user_id: String) -> Result<Vec<ConversationAggregate>, anyhow::Error>;
    async fn update_conversation(&self, id: Uuid, request: UpdateConversationRequest) -> Result<Option<Conversation>, anyhow::Error>;
    async fn delete_conversation(&self, id: Uuid) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl ConversationRepository for InMemoryRepository {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<ConversationAggregate, anyhow::Error> {
        let name = match request.conversation_type {
            ConversationType::Direct => None,
            ConversationType::Group => request.name,
        };
        let conversation = Conversation {
            id: Uuid::now_v7(),
            conversation_type: request.conversation_type,
            name,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.conversations_repo.write().await.insert(conversation.id, conversation.clone());

        let mut user_conversations_repo = self.user_conversations_repo.write().await;
        let mut conversation_to_users_index = self.conversation_to_users_index.write().await;
        let mut user_to_conversations_index = self.user_to_conversations_index.write().await;

        let users_repo = self.user_repos.read().await;
        let mut user_conversations = Vec::new();
        for user_id in request.participants {
            let Some(user) = users_repo.get(&user_id).cloned() else {
                continue;
            };

            let mut participant = UserConversation {
                user_id: user.id.clone(),
                conversation_id: conversation.id,
                joined_at: None,
                last_read_at: None,
            };

            if request.sender_id == user.id {
                let now = Some(Utc::now());
                participant.joined_at = now;
                participant.last_read_at = now;
            }

            let key = (user.id.clone(), conversation.id);
            user_conversations_repo.insert(key, participant.clone());

            user_to_conversations_index
                .entry(user.id.clone())
                .or_default()
                .push(conversation.id);
            conversation_to_users_index
                .entry(conversation.id)
                .or_default()
                .push(user.id.clone());

            user_conversations.push(participant);
        }

        let users: Vec<User> = user_conversations
            .iter()
            .filter_map(|uc| users_repo.get(&uc.user_id).cloned())
            .collect();
        let result = ConversationAggregate {
            conversation,
            user_conversations,
            users,
        };

        Ok(result)
    }

    async fn read_conversation(&self, id: Uuid) -> Result<Option<ConversationAggregate>, anyhow::Error> {
        let Some(conversation) = self.conversations_repo.read().await.get(&id).cloned() else {
            return Ok(None);
        };

        let user_conversations_repo = self.user_conversations_repo.read().await;
        let conversation_to_users_index = self.conversation_to_users_index.read().await;
        let user_conversations = match conversation_to_users_index.get(&conversation.id) {
            Some(user_ids) => user_ids
                .iter()
                .filter_map(|user_id| user_conversations_repo.get(&(user_id.clone(), conversation.id)).cloned())
                .collect(),
            None => Vec::new(),
        };

        let users_repo = self.user_repos.read().await;
        let users: Vec<User> = user_conversations
            .iter()
            .filter_map(|uc| users_repo.get(&uc.user_id).cloned())
            .collect();

        let result = ConversationAggregate {
            conversation,
            user_conversations,
            users,
        };

        Ok(Some(result))
    }

    async fn read_conversations_by_user(&self, user_id: String) -> Result<Vec<ConversationAggregate>, anyhow::Error> {
        let user_to_conversations_index = self.user_to_conversations_index.read().await;

        let Some(conversation_ids) = user_to_conversations_index.get(&user_id) else {
            return Ok(Vec::new());
        };

        let users_repo = self.user_repos.read().await;
        let conversations_repo = self.conversations_repo.read().await;
        let conversation_to_users_index = self.conversation_to_users_index.read().await;
        let user_conversations_repo = self.user_conversations_repo.read().await;

        let mut result: Vec<ConversationAggregate> = Vec::new();
        for conversation_id in conversation_ids {
            let Some(conversation) = conversations_repo.get(conversation_id) else {
                continue;
            };

            let user_conversations: Vec<UserConversation> = match conversation_to_users_index.get(conversation_id) {
                Some(user_ids) => user_ids
                    .iter()
                    .filter_map(|uid| user_conversations_repo.get(&(uid.clone(), *conversation_id)).cloned())
                    .collect(),
                None => Vec::new(),
            };

            let users: Vec<User> = user_conversations
                .iter()
                .filter_map(|uc| users_repo.get(&uc.user_id).cloned())
                .collect();

            result.push(ConversationAggregate {
                conversation: conversation.clone(),
                user_conversations,
                users,
            });
        }

        Ok(result)
    }

    async fn update_conversation(&self, id: Uuid, request: UpdateConversationRequest) -> Result<Option<Conversation>, anyhow::Error> {
        let mut conversations_repo = self.conversations_repo.write().await;
        let Some(conversation) = conversations_repo.get_mut(&id) else {
            return Ok(None);
        };

        if let Some(name) = request.name {
            conversation.name = Some(name);
            conversation.updated_at = Some(Utc::now());
        }

        Ok(Some(conversation.clone()))
    }

    async fn delete_conversation(&self, id: Uuid) -> Result<(), anyhow::Error> {
        let Some(conversation) = self.conversations_repo.read().await.get(&id).cloned() else {
            return Ok(());
        };

        let mut conversation_to_users_index = self.conversation_to_users_index.write().await;
        let user_ids = conversation_to_users_index.remove(&conversation.id).unwrap_or_default();

        let mut user_conversations_repo = self.user_conversations_repo.write().await;
        let mut user_to_conversations_index = self.user_to_conversations_index.write().await;

        for user_id in user_ids {
            user_conversations_repo.remove(&(user_id.clone(), conversation.id));
            if let Some(conversation_ids) = user_to_conversations_index.get_mut(&user_id) {
                conversation_ids.retain(|cid| *cid != conversation.id);
            }
        }

        self.conversations_repo.write().await.remove(&id);
        Ok(())
    }
}

#[async_trait]
#[allow(unused)]
impl ConversationRepository for DbRepository {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<ConversationAggregate, anyhow::Error> {
        todo!()
    }

    async fn read_conversation(&self, id: Uuid) -> Result<Option<ConversationAggregate>, anyhow::Error> {
        todo!()
    }

    async fn read_conversations_by_user(&self, user_id: String) -> Result<Vec<ConversationAggregate>, anyhow::Error> {
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
    use crate::dto::user::CreateUserRequest;
    use crate::repositories::user::UserRepository;

    async fn setup_users(repo: &InMemoryRepository, count: usize) -> Vec<User> {
        let mut users = Vec::new();
        for i in 0..count {
            let user = repo
                .create_or_read_user(CreateUserRequest {
                    id: format!("user_23{i}"),
                    username: format!("user{i}"),
                    display_name: format!("User {i}"),
                })
                .await
                .unwrap();
            users.push(user);
        }
        users
    }

    fn create_request(
        conversation_type: ConversationType,
        name: Option<&str>,
        sender_id: String,
        participants: Vec<String>,
    ) -> CreateConversationRequest {
        CreateConversationRequest {
            conversation_type,
            name: name.map(String::from),
            sender_id,
            participants,
        }
    }

    #[tokio::test]
    async fn create_conversation_returns_conversation_with_correct_fields() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 2).await;
        let sender = users[0].id.clone();
        let usernames: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        let request = create_request(ConversationType::Group, Some("Test Group"), sender, usernames);

        let agg = repo.create_conversation(request).await.unwrap();

        assert_eq!(agg.conversation.conversation_type, ConversationType::Group);
        assert_eq!(agg.conversation.name, Some("Test Group".to_string()));
        assert_eq!(agg.user_conversations.len(), 2);
        assert_eq!(agg.users.len(), 2);
    }

    #[tokio::test]
    async fn create_conversation_sets_sender_timestamps() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 2).await;
        let sender = users[0].id.clone();
        let usernames: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        let request = create_request(ConversationType::Direct, None, sender.clone(), usernames);

        let agg = repo.create_conversation(request).await.unwrap();

        let sender_participant = agg.user_conversations.iter().find(|p| p.user_id == sender).unwrap();
        assert!(sender_participant.joined_at.is_some());
        assert!(sender_participant.last_read_at.is_some());

        let other_participant = agg.user_conversations.iter().find(|p| p.user_id != sender).unwrap();
        assert!(other_participant.joined_at.is_none());
        assert!(other_participant.last_read_at.is_none());
    }

    #[tokio::test]
    async fn create_conversation_generates_unique_id() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id.clone();
        let request1 = create_request(ConversationType::Direct, None, uid.clone(), vec![users[0].id.clone()]);
        let request2 = create_request(ConversationType::Group, Some("Group"), uid, vec![users[0].id.clone()]);

        let conv1 = repo.create_conversation(request1).await.unwrap();
        let conv2 = repo.create_conversation(request2).await.unwrap();

        assert_ne!(conv1.conversation.id, conv2.conversation.id);
    }

    #[tokio::test]
    async fn create_conversation_sets_created_at_timestamp() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id.clone();
        let before = Utc::now();
        let request = create_request(ConversationType::Direct, None, uid, vec![users[0].id.clone()]);

        let agg = repo.create_conversation(request).await.unwrap();

        let after = Utc::now();
        assert!(agg.conversation.created_at >= before && agg.conversation.created_at <= after);
    }

    #[tokio::test]
    async fn create_direct_conversation_has_no_name() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id.clone();
        let request = create_request(ConversationType::Direct, None, uid, vec![users[0].id.clone()]);

        let agg = repo.create_conversation(request).await.unwrap();

        assert_eq!(agg.conversation.conversation_type, ConversationType::Direct);
        assert!(agg.conversation.name.is_none());
    }

    #[tokio::test]
    async fn create_group_conversation_has_name() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id.clone();
        let request = create_request(ConversationType::Group, Some("My Group Chat"), uid, vec![users[0].id.clone()]);

        let agg = repo.create_conversation(request).await.unwrap();

        assert_eq!(agg.conversation.conversation_type, ConversationType::Group);
        assert_eq!(agg.conversation.name, Some("My Group Chat".to_string()));
    }

    #[tokio::test]
    async fn read_conversation_returns_existing_with_user_conversations_and_users() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 3).await;
        let sender = users[0].id.clone();
        let usernames: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        let request = create_request(ConversationType::Group, Some("Test"), sender, usernames.clone());
        let created = repo.create_conversation(request).await.unwrap();

        let read = repo.read_conversation(created.conversation.id).await.unwrap().unwrap();

        assert_eq!(read.conversation.id, created.conversation.id);
        assert_eq!(read.conversation.name, Some("Test".to_string()));
        assert_eq!(read.user_conversations.len(), 3);
        assert_eq!(read.users.len(), 3);
        for user in users {
            assert!(read.user_conversations.iter().any(|p| p.user_id == user.id));
            assert!(read.users.iter().any(|u| u.id == user.id));
        }
    }

    #[tokio::test]
    async fn read_conversation_returns_none_for_nonexistent() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.read_conversation(random_id).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn update_conversation_updates_name() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id.clone();
        let request = create_request(ConversationType::Group, Some("Original"), uid, vec![users[0].id.clone()]);
        let created = repo.create_conversation(request).await.unwrap();

        let update = UpdateConversationRequest {
            name: Some("Updated Name".to_string()),
        };
        let updated = repo.update_conversation(created.conversation.id, update).await.unwrap().unwrap();

        assert_eq!(updated.name, Some("Updated Name".to_string()));
    }

    #[tokio::test]
    async fn update_conversation_sets_updated_at() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id.clone();
        let request = create_request(ConversationType::Group, Some("Test"), uid, vec![users[0].id.clone()]);
        let created = repo.create_conversation(request).await.unwrap();
        assert!(created.conversation.updated_at.is_none());

        let before = Utc::now();
        let update = UpdateConversationRequest {
            name: Some("New Name".to_string()),
        };
        let updated = repo.update_conversation(created.conversation.id, update).await.unwrap().unwrap();
        let after = Utc::now();

        assert!(updated.updated_at.is_some());
        let updated_at = updated.updated_at.unwrap();
        assert!(updated_at >= before && updated_at <= after);
    }

    #[tokio::test]
    async fn update_conversation_returns_none_for_nonexistent() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let update = UpdateConversationRequest {
            name: Some("Name".to_string()),
        };
        let result = repo.update_conversation(random_id, update).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_conversation_removes_conversation_and_user_conversations() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 2).await;
        let sender = users[0].id.clone();
        let usernames: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        let participant_ids: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        let request = create_request(ConversationType::Direct, None, sender, usernames.clone());
        let created = repo.create_conversation(request).await.unwrap();
        let conv_id = created.conversation.id;

        repo.delete_conversation(conv_id).await.unwrap();

        let read = repo.read_conversation(conv_id).await.unwrap();
        assert!(read.is_none());

        let user_conversations_repo = repo.user_conversations_repo.read().await;
        for uid in &participant_ids {
            assert!(user_conversations_repo.get(&(uid.clone(), conv_id)).is_none());
        }
        drop(user_conversations_repo);

        let conversation_to_users_index = repo.conversation_to_users_index.read().await;
        assert!(conversation_to_users_index.get(&conv_id).is_none());
        drop(conversation_to_users_index);

        let user_to_conversations_index = repo.user_to_conversations_index.read().await;
        for uid in &participant_ids {
            if let Some(conv_ids) = user_to_conversations_index.get(uid) {
                assert!(!conv_ids.contains(&conv_id));
            }
        }
    }

    #[tokio::test]
    async fn read_conversations_by_user_returns_conversations_for_user() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 2).await;
        let sender = users[0].id.clone();
        let usernames: Vec<String> = users.iter().map(|u| u.id.clone()).collect();

        let conv1 = repo
            .create_conversation(create_request(
                ConversationType::Group,
                Some("Group 1"),
                sender.clone(),
                usernames.clone(),
            ))
            .await
            .unwrap();
        let conv2 = repo
            .create_conversation(create_request(
                ConversationType::Group,
                Some("Group 2"),
                sender.clone(),
                usernames.clone(),
            ))
            .await
            .unwrap();

        let result = repo.read_conversations_by_user(sender).await.unwrap();

        assert_eq!(result.len(), 2);
        let ids: Vec<Uuid> = result.iter().map(|a| a.conversation.id).collect();
        assert!(ids.contains(&conv1.conversation.id));
        assert!(ids.contains(&conv2.conversation.id));
    }

    #[tokio::test]
    async fn read_conversations_by_user_returns_empty_for_user_with_no_conversations() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;

        let result = repo.read_conversations_by_user(users[0].id.clone()).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn read_conversations_by_user_returns_empty_for_nonexistent_user() {
        let repo = InMemoryRepository::new();

        let result = repo.read_conversations_by_user("nonexistent".to_string()).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn read_conversations_by_user_does_not_return_other_users_conversations() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 2).await;
        let user1 = users[0].id.clone();
        let user2 = users[1].id.clone();

        repo.create_conversation(create_request(ConversationType::Direct, None, user1, vec![users[0].id.clone()]))
            .await
            .unwrap();

        let result = repo.read_conversations_by_user(user2).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn delete_conversation_succeeds_for_nonexistent() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.delete_conversation(random_id).await;

        assert!(result.is_ok());
    }
}
