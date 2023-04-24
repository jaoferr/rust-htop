use std::sync::Arc;

use axum::{
    extract::{State, Path},
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server, body::Body,
};
use futures::lock::Mutex;
use sysinfo::{CpuExt, System, SystemExt};
use mime::Mime;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpu", get(get_cpus_usage))
        .route("/asset/*path", get(get_asset))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();

    println!("Hello, world!");
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn root_get() -> impl IntoResponse {
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

fn find_mime_type (filename : &String) -> Mime{

    let parts : Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
            Some(v) =>
                match *v {
                    "png" => mime::IMAGE_PNG,
                    "jpg" => mime::IMAGE_JPEG,
                    "json" => mime::APPLICATION_JSON,
                    "js" => mime::APPLICATION_JAVASCRIPT_UTF_8,
                    "mjs" => mime::APPLICATION_JAVASCRIPT_UTF_8,
                    "css" => mime::TEXT_CSS_UTF_8,
                    &_ => mime::TEXT_PLAIN,
                },
            None => mime::TEXT_PLAIN,
        };

    return res;
}

async fn get_asset(Path(path): Path<String>) -> impl IntoResponse {
    let mut asset_path = "assets/".to_owned();
    asset_path.push_str(&path);

    let content_type = find_mime_type(&asset_path);
    let asset = tokio::fs::read_to_string(&asset_path).await;

    if asset.is_ok() {
        Response::builder()
            .header("content-type", content_type.to_string())
            .body(asset.unwrap())
            .unwrap().into_response()
    } else {
        Response::builder()
            .body(Body::empty())
            .unwrap().into_response()
    }

}