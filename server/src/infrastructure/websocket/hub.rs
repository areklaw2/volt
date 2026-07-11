use crate::domain::events::DomainEvent;
use crate::domain::ids::UserId;
use axum::extract::ws::{Message, WebSocket};
use tokio::sync::broadcast;

pub async fn handle_socket(mut socket: WebSocket, user_id: UserId, mut rx: broadcast::Receiver<DomainEvent>) {
    while let Ok(event) = rx.recv().await {
        if let DomainEvent::MessageSent { conversation_id, .. } = &event {
            if user_is_in(&user_id, conversation_id).await {
                let payload = serialize_for_client(&event);
                if socket.send(Message::Text(payload.into())).await.is_err() {
                    break; // client disconnected
                }
            }
        }
    }
}

async fn user_is_in(_user_id: &UserId, _conversation_id: &crate::domain::ids::ConversationId) -> bool {
    todo!("cached participant-set lookup")
}

fn serialize_for_client(_event: &DomainEvent) -> String {
    todo!("serde json payload")
}
