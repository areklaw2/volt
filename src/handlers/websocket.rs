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

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut username = String::new();
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            check_username(&state, &mut username, name.as_str());

            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(Utf8Bytes::from_static(
                        "Username already taken.",
                    )))
                    .await;

                return;
            }
        }
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

    state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}
