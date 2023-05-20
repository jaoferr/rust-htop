use super::AppState;

use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::IntoResponse,
    Json
};

#[axum::debug_handler]
pub async fn get_cpus_usage(State(app_state): State<AppState>) -> impl IntoResponse {
    let mut rx = app_state.tx.subscribe();
    let v = rx.recv().await.unwrap();

    Json(v)
}

#[axum::debug_handler]
pub async fn get_realtime_cpus_usage(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |ws: WebSocket| async {
        realtime_cpus_usage_stream(state, ws).await;
    })
}

async fn realtime_cpus_usage_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let payload = serde_json::to_string(&msg).unwrap();

        ws.send(Message::Text(payload)).await.unwrap();
    }
}
