pub mod utils;
mod asset_handler;

use std::sync::{Arc, Mutex};

use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server
};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;

type Snapshot = Vec<f32>;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<Snapshot>(1);

    let app_state = AppState { tx: tx.clone() };

    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpu", get(get_cpus_usage))
        .route("/ws/cpu", get(get_realtime_cpus_usage))
        .route("/asset/*path", get(asset_handler::get_asset))
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();            
            let _ = tx.send(v);

            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<Snapshot>
}

async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

#[axum::debug_handler]
async fn get_cpus_usage(State(app_state): State<AppState>) -> impl IntoResponse {
    let mut rx = app_state.tx.subscribe();
    let v = rx.recv().await.unwrap();

    Json(v)
}

#[axum::debug_handler]
async fn get_realtime_cpus_usage(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
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