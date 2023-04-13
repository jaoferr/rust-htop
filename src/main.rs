use std::sync::Arc;

use axum::{Server, Router, routing::get, extract::State, Json, response::{IntoResponse, Html}, http::Response};
use futures::lock::Mutex;
use sysinfo::{System, SystemExt, CpuExt};

#[tokio::main]
async fn main() {
    let router = Router::new()
    .route("/", get(root_get))
    .route("/api/cpu", get(get_cpus_usage))
    .route("/index.mjs", get(get_index_mjs))
    .with_state(AppState {
        sys: Arc::new(Mutex::new(System::new())),
    });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap())
    .serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();

    println!("Hello, world!");
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn root_get() -> impl IntoResponse{
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

#[axum::debug_handler]
async fn get_cpus_usage(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().await;
    sys.refresh_cpu();
    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(v)
}

async fn get_index_mjs() -> impl IntoResponse{
    let asset = tokio::fs::read_to_string("src/index.mjs").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(asset)
        .unwrap()
}