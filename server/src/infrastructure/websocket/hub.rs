use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::application::commands::send_message::{SendMessageCommand, SendMessageHandler};
use crate::domain::events::DomainEvent;
use crate::domain::ids::{ConversationId, UserId};
use crate::domain::message::MessageKind;
use crate::domain::repository::{ConversationRepository, EventPublisher, MessageRepository};

#[derive(Deserialize)]
struct IncomingMessage {
    conversation_id: String,
    sender_id: String,
    content: String,
    #[serde(default)]
    kind: Option<String>,
}

#[derive(Serialize)]
struct OutgoingMessage {
    id: String,
    conversation_id: String,
    sender_id: String,
    content: String,
    kind: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn handle_socket<C, M, P>(
    socket: WebSocket,
    user_id: UserId,
    pool: PgPool,
    send_message: std::sync::Arc<SendMessageHandler<C, M, P>>,
    mut rx: broadcast::Receiver<DomainEvent>,
) where
    C: ConversationRepository,
    M: MessageRepository,
    P: EventPublisher,
{
    let (mut sink, mut stream) = socket.split();

    loop {
        tokio::select! {
            incoming = stream.next() => {
                let Some(Ok(WsMessage::Text(text))) = incoming else { break };
                let Ok(payload) = serde_json::from_str::<IncomingMessage>(&text) else { continue };
                let Ok(conversation_id) = Uuid::parse_str(&payload.conversation_id) else { continue };
                let Ok(sender_id) = Uuid::parse_str(&payload.sender_id) else { continue };

                let kind = match payload.kind.as_deref() {
                    Some("image") => MessageKind::Image,
                    _ => MessageKind::Text,
                };

                let command = SendMessageCommand {
                    conversation_id: ConversationId::from_persistence(conversation_id),
                    sender_id: UserId::from_persistence(sender_id),
                    content: payload.content,
                    kind,
                };

                if let Err(err) = send_message.handle(command).await {
                    tracing::warn!("failed to send message: {err}");
                }
            }
            event = rx.recv() => {
                let event = match event {
                    Ok(event) => event,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                };

                let DomainEvent::MessageSent { message_id, conversation_id, sender_id, content, kind, created_at } = &event else {
                    continue;
                };

                match user_is_in(&pool, &user_id, conversation_id).await {
                    Ok(true) => {}
                    _ => continue,
                }

                let kind_str = match kind {
                    MessageKind::Text => "text",
                    MessageKind::Image => "image",
                };

                let payload = OutgoingMessage {
                    id: message_id.to_string(),
                    conversation_id: conversation_id.to_string(),
                    sender_id: sender_id.to_string(),
                    content: content.clone(),
                    kind: kind_str.to_string(),
                    created_at: *created_at,
                    updated_at: None,
                };
                let Ok(json) = serde_json::to_string(&payload) else { continue };

                if sink.send(WsMessage::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    }
}

async fn user_is_in(pool: &PgPool, user_id: &UserId, conversation_id: &ConversationId) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT 1 AS present FROM user_conversations WHERE user_id = $1 AND conversation_id = $2",
        Uuid::from(user_id.clone()),
        Uuid::from(conversation_id.clone())
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}
