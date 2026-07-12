use std::sync::Arc;

use axum::{
    extract::{Path, State, ws::WebSocketUpgrade},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{AppState, domain::ids::UserId, infrastructure::websocket::hub};

pub async fn chat(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let user_id = UserId::from_persistence(user_id);
    let rx = state.event_bus.subscribe();
    let pool = state.pool.clone();
    let send_message = state.send_message.clone();
    let views = state.views.clone();

    ws.on_upgrade(move |socket| hub::handle_socket(socket, user_id, pool, send_message, views, rx))
}
