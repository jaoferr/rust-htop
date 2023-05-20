pub mod utils;
mod asset_handler;

use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server
};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let app_state = AppState::default();

    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpu", get(get_cpus_usage))
        .route("/asset/*path", get(asset_handler::get_asset))
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

            {
                let mut cpus = app_state.cpus.lock().unwrap();
                *cpus = v;
            }
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

#[derive(Default, Clone)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>
}

async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

#[axum::debug_handler]
async fn get_cpus_usage(State(state): State<AppState>) -> impl IntoResponse {
    let v = state.cpus.lock().unwrap().clone();

    Json(v)
}
