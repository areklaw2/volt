use async_trait::async_trait;
use chrono::{DateTime, Utc};

pub struct ConversationListItem {
    pub conversation_id: String,
    pub title: Option<String>,
    pub last_message_preview: Option<String>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub unread_count: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

#[async_trait]
pub trait ConversationListQueries: Send + Sync {
    async fn for_user(&self, user_id: &str) -> Result<Vec<ConversationListItem>, QueryError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockConversationListQueries {
        items: Vec<ConversationListItem>,
    }

    #[async_trait]
    impl ConversationListQueries for MockConversationListQueries {
        async fn for_user(&self, user_id: &str) -> Result<Vec<ConversationListItem>, QueryError> {
            if user_id.is_empty() {
                return Err(QueryError::Db(sqlx::Error::RowNotFound));
            }
            Ok(self
                .items
                .iter()
                .map(|item| ConversationListItem {
                    conversation_id: item.conversation_id.clone(),
                    title: item.title.clone(),
                    last_message_preview: item.last_message_preview.clone(),
                    last_message_at: item.last_message_at,
                    unread_count: item.unread_count,
                })
                .collect())
        }
    }

    #[tokio::test]
    async fn for_user_returns_items() {
        let queries = MockConversationListQueries {
            items: vec![ConversationListItem {
                conversation_id: "conv-1".to_string(),
                title: Some("Group".to_string()),
                last_message_preview: Some("hey".to_string()),
                last_message_at: None,
                unread_count: 3,
            }],
        };

        let result = queries.for_user("user-1").await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].conversation_id, "conv-1");
        assert_eq!(result[0].unread_count, 3);
    }

    #[tokio::test]
    async fn for_user_propagates_query_error() {
        let queries = MockConversationListQueries { items: vec![] };

        let result = queries.for_user("").await;

        assert!(matches!(result, Err(QueryError::Db(_))));
    }
}
