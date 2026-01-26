use std::collections::HashMap;

use anyhow::Ok;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::dto::{CreateParticipantsRequest, UpdateParticipantRequest};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Participant {
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub joined_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait ParticipantRepository: Send + Sync {
    async fn create_conversation_participants(&self, request: CreateParticipantsRequest) -> Result<Vec<Participant>, anyhow::Error>;
    async fn read_conversation_participants(&self, conversation_id: Uuid) -> Result<Vec<Participant>, anyhow::Error>;
    async fn read_participant_conversations(&self, user_id: Uuid) -> Result<Vec<Participant>, anyhow::Error>;
    async fn read_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<Option<Participant>, anyhow::Error>;
    async fn update_participant(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        request: UpdateParticipantRequest,
    ) -> Result<Option<Participant>, anyhow::Error>;
    async fn delete_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error>;
}

#[derive(Debug, Default)]
pub struct InMemoryParticipantRepository {
    participants: RwLock<HashMap<(Uuid, Uuid), Participant>>,
    user_index: RwLock<HashMap<Uuid, Vec<Uuid>>>,
    conversation_index: RwLock<HashMap<Uuid, Vec<Uuid>>>,
}

impl InMemoryParticipantRepository {
    pub fn new() -> Self {
        Self {
            participants: RwLock::default(),
            user_index: RwLock::default(),
            conversation_index: RwLock::default(),
        }
    }
}

#[async_trait]
impl ParticipantRepository for InMemoryParticipantRepository {
    async fn create_conversation_participants(&self, request: CreateParticipantsRequest) -> Result<Vec<Participant>, anyhow::Error> {
        let mut participants = self.participants.write().await;
        let mut conversation_index = self.conversation_index.write().await;
        let mut user_index = self.user_index.write().await;

        let mut result = Vec::new();
        for user_id in request.users {
            let mut participant = Participant {
                user_id,
                conversation_id: request.conversation_id,
                joined_at: None,
                last_read_at: None,
            };

            if request.sender_id == user_id {
                let now = Some(Utc::now());
                participant.joined_at = now;
                participant.last_read_at = now;
            }

            let key = (user_id, request.conversation_id);
            participants.insert(key, participant.clone());

            user_index.entry(user_id).or_default().push(request.conversation_id);
            conversation_index.entry(request.conversation_id).or_default().push(user_id);

            result.push(participant);
        }

        Ok(result)
    }

    async fn read_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<Option<Participant>, anyhow::Error> {
        let key = (user_id, conversation_id);
        Ok(self.participants.read().await.get(&key).cloned())
    }

    async fn read_conversation_participants(&self, conversation_id: Uuid) -> Result<Vec<Participant>, anyhow::Error> {
        let participants = self.participants.read().await;
        let conversation_index = self.conversation_index.read().await;

        let result = match conversation_index.get(&conversation_id) {
            Some(user_ids) => user_ids
                .iter()
                .filter_map(|user_id| participants.get(&(*user_id, conversation_id)).cloned())
                .collect(),
            None => Vec::new(),
        };

        Ok(result)
    }

    async fn read_participant_conversations(&self, user_id: Uuid) -> Result<Vec<Participant>, anyhow::Error> {
        let participants = self.participants.read().await;
        let user_index = self.user_index.read().await;

        let result = match user_index.get(&user_id) {
            Some(conversation_ids) => conversation_ids
                .iter()
                .filter_map(|conversation_id| participants.get(&(user_id, *conversation_id)).cloned())
                .collect(),
            None => Vec::new(),
        };

        Ok(result)
    }

    async fn update_participant(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        request: UpdateParticipantRequest,
    ) -> Result<Option<Participant>, anyhow::Error> {
        let key = (user_id, conversation_id);
        let mut participants = self.participants.write().await;
        let Some(participant) = participants.get_mut(&key) else {
            return Ok(None);
        };

        if let Some(joined_at) = request.joined_at
            && participant.joined_at.is_none()
        {
            participant.joined_at = Some(joined_at);
        }

        if let Some(last_read_at) = request.last_read_at {
            participant.last_read_at = Some(last_read_at);
        }

        Ok(Some(participant.clone()))
    }

    async fn delete_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error> {
        let key = (user_id, conversation_id);
        self.participants.write().await.remove(&key);

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

#[derive(Debug)]
#[allow(unused)]
pub struct DbParticipantRepository {
    pool: Pool<Postgres>,
}

impl DbParticipantRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
#[allow(unused)]
impl ParticipantRepository for DbParticipantRepository {
    async fn create_conversation_participants(&self, request: CreateParticipantsRequest) -> Result<Vec<Participant>, anyhow::Error> {
        todo!()
    }

    async fn read_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<Option<Participant>, anyhow::Error> {
        todo!()
    }

    async fn read_conversation_participants(&self, conversation_id: Uuid) -> Result<Vec<Participant>, anyhow::Error> {
        todo!()
    }

    async fn read_participant_conversations(&self, user_id: Uuid) -> Result<Vec<Participant>, anyhow::Error> {
        todo!()
    }

    async fn update_participant(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        request: UpdateParticipantRequest,
    ) -> Result<Option<Participant>, anyhow::Error> {
        todo!()
    }

    async fn delete_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_request(sender_id: Uuid, conversation_id: Uuid, users: Vec<Uuid>) -> CreateParticipantsRequest {
        CreateParticipantsRequest {
            sender_id,
            conversation_id,
            users,
        }
    }

    #[tokio::test]
    async fn create_conversation_participants_creates_participants_for_multiple_users() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let user1 = Uuid::now_v7();
        let user2 = Uuid::now_v7();
        let user3 = Uuid::now_v7();
        let request = create_request(user1, conversation_id, vec![user1, user2, user3]);

        let participants = repo.create_conversation_participants(request).await.unwrap();
        assert_eq!(participants.len(), 3);
    }

    #[tokio::test]
    async fn create_conversation_participants_sets_timestamps_for_sender() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id]);

        repo.create_conversation_participants(request).await.unwrap();

        let participant = repo.read_participant(sender_id, conversation_id).await.unwrap().unwrap();
        assert!(participant.joined_at.is_some());
        assert!(participant.last_read_at.is_some());
    }

    #[tokio::test]
    async fn create_conversation_participants_non_sender_has_none_timestamps() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let other_user = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id, other_user]);

        repo.create_conversation_participants(request).await.unwrap();

        let participant = repo.read_participant(other_user, conversation_id).await.unwrap().unwrap();
        assert!(participant.joined_at.is_none());
        assert!(participant.last_read_at.is_none());
    }

    #[tokio::test]
    async fn create_conversation_participants_are_persisted_and_readable() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);

        repo.create_conversation_participants(request).await.unwrap();

        let participant = repo.read_participant(user_id, conversation_id).await.unwrap();
        assert!(participant.is_some());
        assert_eq!(participant.unwrap().user_id, user_id);
    }

    #[tokio::test]
    async fn read_participant_returns_existing_participant() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);
        repo.create_conversation_participants(request).await.unwrap();

        let participant = repo.read_participant(user_id, conversation_id).await.unwrap().unwrap();

        assert_eq!(participant.user_id, user_id);
        assert_eq!(participant.conversation_id, conversation_id);
    }

    #[tokio::test]
    async fn read_participant_returns_none_for_nonexistent_participant() {
        let repo = InMemoryParticipantRepository::new();
        let random_user = Uuid::now_v7();
        let random_conversation = Uuid::now_v7();

        let result = repo.read_participant(random_user, random_conversation).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn read_conversation_participants_returns_all_participants() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let user1 = Uuid::now_v7();
        let user2 = Uuid::now_v7();
        let request = create_request(user1, conversation_id, vec![user1, user2]);
        repo.create_conversation_participants(request).await.unwrap();

        let participants = repo.read_conversation_participants(conversation_id).await.unwrap();

        assert_eq!(participants.len(), 2);
        let user_ids: Vec<Uuid> = participants.iter().map(|p| p.user_id).collect();
        assert!(user_ids.contains(&user1));
        assert!(user_ids.contains(&user2));
    }

    #[tokio::test]
    async fn read_conversation_participants_returns_empty_for_nonexistent_conversation() {
        let repo = InMemoryParticipantRepository::new();
        let random_conversation = Uuid::now_v7();

        let participants = repo.read_conversation_participants(random_conversation).await.unwrap();

        assert!(participants.is_empty());
    }

    #[tokio::test]
    async fn read_participant_conversations_returns_all_conversations() {
        let repo = InMemoryParticipantRepository::new();
        let user_id = Uuid::now_v7();
        let conv1 = Uuid::now_v7();
        let conv2 = Uuid::now_v7();
        let request1 = create_request(user_id, conv1, vec![user_id]);
        let request2 = create_request(user_id, conv2, vec![user_id]);
        repo.create_conversation_participants(request1).await.unwrap();
        repo.create_conversation_participants(request2).await.unwrap();

        let conversations = repo.read_participant_conversations(user_id).await.unwrap();

        assert_eq!(conversations.len(), 2);
        let conv_ids: Vec<Uuid> = conversations.iter().map(|p| p.conversation_id).collect();
        assert!(conv_ids.contains(&conv1));
        assert!(conv_ids.contains(&conv2));
    }

    #[tokio::test]
    async fn read_participant_conversations_returns_empty_for_user_not_in_any() {
        let repo = InMemoryParticipantRepository::new();
        let random_user = Uuid::now_v7();

        let conversations = repo.read_participant_conversations(random_user).await.unwrap();

        assert!(conversations.is_empty());
    }

    #[tokio::test]
    async fn update_participant_updates_joined_at_when_none() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let other_user = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id, other_user]);
        repo.create_conversation_participants(request).await.unwrap();

        let now = Utc::now();
        let update = UpdateParticipantRequest {
            joined_at: Some(now),
            last_read_at: None,
        };
        let updated = repo.update_participant(other_user, conversation_id, update).await.unwrap().unwrap();

        assert_eq!(updated.joined_at, Some(now));
    }

    #[tokio::test]
    async fn update_participant_does_not_overwrite_existing_joined_at() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let sender_id = Uuid::now_v7();
        let request = create_request(sender_id, conversation_id, vec![sender_id]);
        repo.create_conversation_participants(request).await.unwrap();

        let original = repo.read_participant(sender_id, conversation_id).await.unwrap().unwrap();
        let original_joined_at = original.joined_at;

        let new_time = Utc::now();
        let update = UpdateParticipantRequest {
            joined_at: Some(new_time),
            last_read_at: None,
        };
        let updated = repo.update_participant(sender_id, conversation_id, update).await.unwrap().unwrap();

        assert_eq!(updated.joined_at, original_joined_at);
    }

    #[tokio::test]
    async fn update_participant_updates_last_read_at() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);
        repo.create_conversation_participants(request).await.unwrap();

        let new_time = Utc::now();
        let update = UpdateParticipantRequest {
            joined_at: None,
            last_read_at: Some(new_time),
        };
        let updated = repo.update_participant(user_id, conversation_id, update).await.unwrap().unwrap();

        assert_eq!(updated.last_read_at, Some(new_time));
    }

    #[tokio::test]
    async fn update_participant_returns_none_for_nonexistent() {
        let repo = InMemoryParticipantRepository::new();
        let random_user = Uuid::now_v7();
        let random_conversation = Uuid::now_v7();

        let update = UpdateParticipantRequest {
            joined_at: Some(Utc::now()),
            last_read_at: None,
        };
        let result = repo.update_participant(random_user, random_conversation, update).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_participant_removes_from_main_storage() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let request = create_request(user_id, conversation_id, vec![user_id]);
        repo.create_conversation_participants(request).await.unwrap();

        repo.delete_participant(user_id, conversation_id).await.unwrap();

        let result = repo.read_participant(user_id, conversation_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_participant_removes_from_conversation_index() {
        let repo = InMemoryParticipantRepository::new();
        let conversation_id = Uuid::now_v7();
        let user1 = Uuid::now_v7();
        let user2 = Uuid::now_v7();
        let request = create_request(user1, conversation_id, vec![user1, user2]);
        repo.create_conversation_participants(request).await.unwrap();

        repo.delete_participant(user1, conversation_id).await.unwrap();

        let participants = repo.read_conversation_participants(conversation_id).await.unwrap();
        assert_eq!(participants.len(), 1);
        assert_eq!(participants[0].user_id, user2);
    }

    #[tokio::test]
    async fn delete_participant_removes_from_user_index() {
        let repo = InMemoryParticipantRepository::new();
        let user_id = Uuid::now_v7();
        let conv1 = Uuid::now_v7();
        let conv2 = Uuid::now_v7();
        let request1 = create_request(user_id, conv1, vec![user_id]);
        let request2 = create_request(user_id, conv2, vec![user_id]);
        repo.create_conversation_participants(request1).await.unwrap();
        repo.create_conversation_participants(request2).await.unwrap();

        repo.delete_participant(user_id, conv1).await.unwrap();

        let conversations = repo.read_participant_conversations(user_id).await.unwrap();
        assert_eq!(conversations.len(), 1);
        assert_eq!(conversations[0].conversation_id, conv2);
    }

    #[tokio::test]
    async fn delete_participant_succeeds_for_nonexistent() {
        let repo = InMemoryParticipantRepository::new();
        let random_user = Uuid::now_v7();
        let random_conversation = Uuid::now_v7();

        let result = repo.delete_participant(random_user, random_conversation).await;

        assert!(result.is_ok());
    }
}
