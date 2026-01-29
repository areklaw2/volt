use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::{CreateConversationRequest, UpdateConversationRequest},
    repositories::{DbRepository, InMemoryRepository, participant::Participant, user::User},
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
    pub participants: Vec<Participant>,
    pub users: Vec<User>,
}

#[async_trait]
pub trait ConversationRepository: Send + Sync {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<ConversationAggregate, anyhow::Error>;
    async fn read_conversation(&self, id: Uuid) -> Result<Option<ConversationAggregate>, anyhow::Error>;
    async fn update_conversation(&self, id: Uuid, request: UpdateConversationRequest) -> Result<Option<Conversation>, anyhow::Error>;
    async fn delete_conversation(&self, id: Uuid) -> Result<(), anyhow::Error>;
}

#[async_trait]
impl ConversationRepository for InMemoryRepository {
    async fn create_conversation(&self, request: CreateConversationRequest) -> Result<ConversationAggregate, anyhow::Error> {
        let conversation = Conversation {
            id: Uuid::now_v7(),
            conversation_type: request.conversation_type,
            name: request.name,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.conversations_repo.write().await.insert(conversation.id, conversation.clone());

        let mut participants_repo = self.participants_repo.write().await;
        let mut conversation_index = self.conversation_index.write().await;
        let mut user_index = self.user_index.write().await;

        let mut participants = Vec::new();
        for user_id in request.participants {
            let mut participant = Participant {
                user_id: user_id,
                conversation_id: conversation.id,
                joined_at: None,
                last_read_at: None,
            };

            if request.sender_id == user_id {
                let now = Some(Utc::now());
                participant.joined_at = now;
                participant.last_read_at = now;
            }

            let key = (user_id, conversation.id);
            participants_repo.insert(key, participant.clone());

            user_index.entry(user_id).or_default().push(conversation.id);
            conversation_index.entry(conversation.id).or_default().push(user_id);

            participants.push(participant);
        }

        let users_repo = self.user_repos.read().await;
        let users: Vec<User> = participants.iter().filter_map(|id| users_repo.get(&id.user_id).cloned()).collect();
        let result = ConversationAggregate {
            conversation,
            participants,
            users,
        };

        Ok(result)
    }

    async fn read_conversation(&self, id: Uuid) -> Result<Option<ConversationAggregate>, anyhow::Error> {
        let Some(conversation) = self.conversations_repo.read().await.get(&id).cloned() else {
            return Ok(None);
        };

        let participants_repo = self.participants_repo.read().await;
        let conversation_index = self.conversation_index.read().await;
        let participants = match conversation_index.get(&conversation.id) {
            Some(user_ids) => user_ids
                .iter()
                .filter_map(|user_id| participants_repo.get(&(*user_id, conversation.id)).cloned())
                .collect(),
            None => Vec::new(),
        };

        let users_repo = self.user_repos.read().await;
        let users: Vec<User> = participants.iter().filter_map(|id| users_repo.get(&id.user_id).cloned()).collect();

        let result = ConversationAggregate {
            conversation,
            participants,
            users,
        };

        Ok(Some(result))
    }

    async fn update_conversation(&self, id: Uuid, request: UpdateConversationRequest) -> Result<Option<Conversation>, anyhow::Error> {
        let mut conversations = self.conversations_repo.write().await;
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
        let Some(conversation) = self.conversations_repo.read().await.get(&id).cloned() else {
            return Ok(());
        };

        let mut conversation_index = self.conversation_index.write().await;
        let user_ids = conversation_index.remove(&conversation.id).unwrap_or_default();

        let mut participants_repo = self.participants_repo.write().await;
        let mut user_index = self.user_index.write().await;

        for user_id in user_ids {
            participants_repo.remove(&(user_id, conversation.id));
            if let Some(conversation_ids) = user_index.get_mut(&user_id) {
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
    use crate::dto::CreateUserRequest;
    use crate::repositories::user::UserRepository;

    async fn setup_users(repo: &InMemoryRepository, count: usize) -> Vec<User> {
        let mut users = Vec::new();
        for i in 0..count {
            let user = repo
                .create_user(CreateUserRequest {
                    username: format!("user{i}"),
                    display_name: format!("User {i}"),
                    avatar_url: format!("https://example.com/{i}.png"),
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
        sender_id: Uuid,
        participants: Vec<Uuid>,
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
        let sender = users[0].id;
        let participant_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();
        let request = create_request(ConversationType::Group, Some("Test Group"), sender, participant_ids);

        let agg = repo.create_conversation(request).await.unwrap();

        assert_eq!(agg.conversation.conversation_type, ConversationType::Group);
        assert_eq!(agg.conversation.name, Some("Test Group".to_string()));
        assert_eq!(agg.participants.len(), 2);
        assert_eq!(agg.users.len(), 2);
    }

    #[tokio::test]
    async fn create_conversation_sets_sender_timestamps() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 2).await;
        let sender = users[0].id;
        let participant_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();
        let request = create_request(ConversationType::Direct, None, sender, participant_ids);

        let agg = repo.create_conversation(request).await.unwrap();

        let sender_participant = agg.participants.iter().find(|p| p.user_id == sender).unwrap();
        assert!(sender_participant.joined_at.is_some());
        assert!(sender_participant.last_read_at.is_some());

        let other_participant = agg.participants.iter().find(|p| p.user_id != sender).unwrap();
        assert!(other_participant.joined_at.is_none());
        assert!(other_participant.last_read_at.is_none());
    }

    #[tokio::test]
    async fn create_conversation_generates_unique_id() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id;
        let request1 = create_request(ConversationType::Direct, None, uid, vec![uid]);
        let request2 = create_request(ConversationType::Group, Some("Group"), uid, vec![uid]);

        let conv1 = repo.create_conversation(request1).await.unwrap();
        let conv2 = repo.create_conversation(request2).await.unwrap();

        assert_ne!(conv1.conversation.id, conv2.conversation.id);
    }

    #[tokio::test]
    async fn create_conversation_sets_created_at_timestamp() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id;
        let before = Utc::now();
        let request = create_request(ConversationType::Direct, None, uid, vec![uid]);

        let agg = repo.create_conversation(request).await.unwrap();

        let after = Utc::now();
        assert!(agg.conversation.created_at >= before && agg.conversation.created_at <= after);
    }

    #[tokio::test]
    async fn create_direct_conversation_has_no_name() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id;
        let request = create_request(ConversationType::Direct, None, uid, vec![uid]);

        let agg = repo.create_conversation(request).await.unwrap();

        assert_eq!(agg.conversation.conversation_type, ConversationType::Direct);
        assert!(agg.conversation.name.is_none());
    }

    #[tokio::test]
    async fn create_group_conversation_has_name() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 1).await;
        let uid = users[0].id;
        let request = create_request(ConversationType::Group, Some("My Group Chat"), uid, vec![uid]);

        let agg = repo.create_conversation(request).await.unwrap();

        assert_eq!(agg.conversation.conversation_type, ConversationType::Group);
        assert_eq!(agg.conversation.name, Some("My Group Chat".to_string()));
    }

    #[tokio::test]
    async fn read_conversation_returns_existing_with_participants_and_users() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 3).await;
        let sender = users[0].id;
        let participant_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();
        let request = create_request(ConversationType::Group, Some("Test"), sender, participant_ids.clone());
        let created = repo.create_conversation(request).await.unwrap();

        let read = repo.read_conversation(created.conversation.id).await.unwrap().unwrap();

        assert_eq!(read.conversation.id, created.conversation.id);
        assert_eq!(read.conversation.name, Some("Test".to_string()));
        assert_eq!(read.participants.len(), 3);
        assert_eq!(read.users.len(), 3);
        for uid in &participant_ids {
            assert!(read.participants.iter().any(|p| p.user_id == *uid));
            assert!(read.users.iter().any(|u| u.id == *uid));
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
        let uid = users[0].id;
        let request = create_request(ConversationType::Group, Some("Original"), uid, vec![uid]);
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
        let uid = users[0].id;
        let request = create_request(ConversationType::Group, Some("Test"), uid, vec![uid]);
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
    async fn delete_conversation_removes_conversation_and_participants() {
        let repo = InMemoryRepository::new();
        let users = setup_users(&repo, 2).await;
        let sender = users[0].id;
        let participant_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();
        let request = create_request(ConversationType::Direct, None, sender, participant_ids.clone());
        let created = repo.create_conversation(request).await.unwrap();
        let conv_id = created.conversation.id;

        repo.delete_conversation(conv_id).await.unwrap();

        let read = repo.read_conversation(conv_id).await.unwrap();
        assert!(read.is_none());

        // Verify participants were cleaned up
        let participants_repo = repo.participants_repo.read().await;
        for uid in &participant_ids {
            assert!(participants_repo.get(&(*uid, conv_id)).is_none());
        }
        drop(participants_repo);

        // Verify conversation_index was cleaned up
        let conversation_index = repo.conversation_index.read().await;
        assert!(conversation_index.get(&conv_id).is_none());
        drop(conversation_index);

        // Verify user_index entries were cleaned up
        let user_index = repo.user_index.read().await;
        for uid in &participant_ids {
            if let Some(conv_ids) = user_index.get(uid) {
                assert!(!conv_ids.contains(&conv_id));
            }
        }
    }

    #[tokio::test]
    async fn delete_conversation_succeeds_for_nonexistent() {
        let repo = InMemoryRepository::new();
        let random_id = Uuid::now_v7();

        let result = repo.delete_conversation(random_id).await;

        assert!(result.is_ok());
    }
}
