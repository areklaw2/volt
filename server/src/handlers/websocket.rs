use axum::{
    extract::{
        Path, State,
        ws::{self, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{AppState, models::Message};

pub async fn websocket(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>, user_id: Uuid) {
    let (mut ws_sender, mut ws_receiver) = stream.split();

    let (tx, mut rx) = mpsc::channel::<Message>(100);

    let mut connections = state.active_connections.write().await;
    connections.entry(user_id).or_insert_with(Vec::new).push(tx);

    let mut send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let json = serde_json::to_string(&message)?;
            if ws_sender.send(ws::Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    let msg = format!("{user_id} joined.");
    tracing::debug!("{msg}");

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(ws::Message::Text(text))) = ws_receiver.next().await {
            let incoming: CreateMessage = serde_json::from_str(&text)?;
            let message = state.messages.
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    state.active_connections.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, username: &str) {
    let mut active_users = state.active_connections.lock().unwrap();

    if !active_users.contains(username) {
        active_users.insert(username.to_owned());
        string.push_str(username);
    }
}

#[derive(Deserialize)]
struct CreateMessage {
    conversation_id: Uuid,
    content: String,
}
