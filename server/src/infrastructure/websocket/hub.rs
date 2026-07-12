use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::application::commands::send_message::{SendMessageCommand, SendMessageHandler};
use crate::application::queries::conversation_list::ConversationViewQueries;
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
    edited: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
struct OutgoingEdit {
    id: String,
    conversation_id: String,
    content: String,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn handle_socket<C, M, P, V>(
    socket: WebSocket,
    user_id: UserId,
    pool: PgPool,
    send_message: std::sync::Arc<SendMessageHandler<C, M, P>>,
    views: V,
    mut rx: broadcast::Receiver<DomainEvent>,
) where
    C: ConversationRepository,
    M: MessageRepository,
    P: EventPublisher,
    V: ConversationViewQueries,
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

                let json = match &event {
                    DomainEvent::MessageSent { message_id, conversation_id, sender_id, content, kind, created_at } => {
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
                            edited: false,
                            created_at: *created_at,
                            updated_at: None,
                        };

                        serde_json::to_string(&serde_json::json!({ "type": "message", "message": payload }))
                    }
                    DomainEvent::MessageEdited { message_id, conversation_id, content, updated_at } => {
                        match user_is_in(&pool, &user_id, conversation_id).await {
                            Ok(true) => {}
                            _ => continue,
                        }

                        let payload = OutgoingEdit {
                            id: message_id.to_string(),
                            conversation_id: conversation_id.to_string(),
                            content: content.clone(),
                            updated_at: *updated_at,
                        };

                        serde_json::to_string(&serde_json::json!({ "type": "message_edited", "message_edited": payload }))
                    }
                    // fires for both brand-new conversations and later invites — either way, this
                    // user now belongs to a conversation their client doesn't know about yet, so
                    // push the full view rather than making them wait for a page refresh.
                    DomainEvent::ParticipantAdded { conversation_id, user_id: added_user_id } if added_user_id == &user_id => {
                        let Ok(Some(view)) = views.by_id(conversation_id).await else { continue };
                        serde_json::to_string(&serde_json::json!({ "type": "conversation", "conversation": view }))
                    }
                    _ => continue,
                };

                let Ok(json) = json else { continue };

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
