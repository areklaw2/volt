use axum::{
    extract::{
        State,
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;

use crate::AppState;

pub async fn websocket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut username = String::new();
    while let Some(Ok(message)) = receiver.next().await {
        let Message::Text(user_id) = message else {
            continue;
        };

        check_username(&state, &mut username, user_id.as_str());
        if username.is_empty() {
            if let Err(e) = sender
                .send(Message::Text(Utf8Bytes::from_static(
                    "Username already taken.",
                )))
                .await
            {
                tracing::warn!("Failed to send error message: {:?}", e);
            }

            return;
        }

        break;
    }

    let mut rx = state.tx.subscribe();

    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let name = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    state.active_users.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, username: &str) {
    let mut active_users = state.active_users.lock().unwrap();

    if !active_users.contains(username) {
        active_users.insert(username.to_owned());
        string.push_str(username);
    }
}
