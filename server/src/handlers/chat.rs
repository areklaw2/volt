use axum::{
    extract::{
        Path, State,
        ws::{self, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{AppState, dto::mesagge::CreateMessageRequest, repositories::message::Message};

pub async fn chat(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>, user_id: Uuid) {
    let (mut ws_sender, mut ws_receiver) = stream.split();

    let (tx, mut rx) = mpsc::channel::<Message>(100);

    {
        let mut connections = state.active_connections.write().await;
        connections.entry(user_id).or_insert_with(Vec::new).push(tx);
    }

    let msg = format!("{user_id} joined.");
    tracing::debug!("{msg}");

    let mut send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let Ok(json) = serde_json::to_string(&message) else {
                tracing::error!("Failed to serialize message");
                continue;
            };
            if ws_sender.send(ws::Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(ws::Message::Text(text))) = ws_receiver.next().await {
            let Ok(request) = serde_json::from_str::<CreateMessageRequest>(&text) else {
                tracing::error!("Failed to parse message request");
                continue;
            };

            let conversation_id = request.conversation_id;

            let Ok(message) = state_clone.repository.create_message(request).await else {
                tracing::error!("Failed to create message");
                continue;
            };

            let Ok(Some(conversation)) = state_clone.repository.read_conversation(conversation_id).await else {
                tracing::error!("Failed to get conversation");
                continue;
            };

            let connections = state_clone.active_connections.read().await;
            for participant in conversation.user_conversations {
                if let Some(senders) = connections.get(&participant.user_id) {
                    for sender in senders {
                        let _ = sender.send(message.clone()).await;
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    let mut connections = state.active_connections.write().await;
    if let Some(senders) = connections.get_mut(&user_id) {
        senders.retain(|s| !s.is_closed());
        if senders.is_empty() {
            connections.remove(&user_id);
        }
    }
}
