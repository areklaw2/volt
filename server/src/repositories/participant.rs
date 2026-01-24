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
    async fn create_conversation_participants(&self, request: CreateParticipantsRequest) -> Result<(), anyhow::Error>;
    async fn read_conversation_participants(&self, conversation_id: Uuid) -> Result<Vec<Participant>, anyhow::Error>;
    async fn read_participant_conversations(&self, user_id: Uuid) -> Result<Vec<Participant>, anyhow::Error>;
    async fn read_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<Option<Participant>, anyhow::Error>;
    async fn update_participant(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        request: UpdateParticipantRequest,
    ) -> Result<Participant, anyhow::Error>;
    async fn delete_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error>;
}

#[derive(Debug)]
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
    async fn create_conversation_participants(&self, request: CreateParticipantsRequest) -> Result<(), anyhow::Error> {
        let mut participants = self.participants.write().await;
        let mut conversation_index = self.conversation_index.write().await;
        let mut user_index = self.user_index.write().await;

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
            participants.insert(key, participant);

            user_index.entry(user_id).or_default().push(request.conversation_id);
            conversation_index.entry(request.conversation_id).or_default().push(user_id);
        }

        Ok(())
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
    ) -> Result<Participant, anyhow::Error> {
        let key = (user_id, conversation_id);
        let mut participants = self.participants.write().await;
        let participant = participants.get_mut(&key).ok_or_else(|| anyhow::anyhow!("Participant not found"))?;

        if let Some(joined_at) = request.joined_at
            && participant.joined_at.is_none()
        {
            participant.joined_at = Some(joined_at);
        }

        if let Some(last_read_at) = request.last_read_at {
            participant.last_read_at = Some(last_read_at);
        }

        Ok(participant.clone())
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
    async fn create_conversation_participants(&self, request: CreateParticipantsRequest) -> Result<(), anyhow::Error> {
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
    ) -> Result<Participant, anyhow::Error> {
        todo!()
    }

    async fn delete_participant(&self, user_id: Uuid, conversation_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}
