pub mod utils;
mod asset_handler;
mod processes;
mod cpu_usage;

use std::{sync::{Arc, Mutex}};
use axum::{
    routing::get,
    Router,
    Server,
    response::{IntoResponse, Html}
};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;

type Snapshot = Vec<f32>;

#[derive(Clone)]
pub struct AppState {
    sys: Arc<Mutex<System>>,
    tx: broadcast::Sender<Snapshot>
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<Snapshot>(1);

    let app_state = AppState {
        tx: tx.clone(),
        sys: Arc::new(Mutex::new(System::new()))
    };

    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpu", get(cpu_usage::get_cpus_usage))
        .route("/api/processes", get(processes::get_processes_list))
        .route("/ws/cpu", get(cpu_usage::get_realtime_cpus_usage))
        .route("/vendor/:type/:module/*path", get(asset_handler::get_npm_asset))
        .route("/asset/*path", get(asset_handler::get_asset))
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(v);

            // std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/views/index.html").await.unwrap();
    Html(markup)
}
