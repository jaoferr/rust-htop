use super::AppState;
use axum::{
    extract::State,
    response::IntoResponse,
    Json
};
use sysinfo::{SystemExt, ProcessExt};


#[derive(serde::Serialize)]
struct ProcessJSON {
    pid: String,
    process_name: String
}

#[axum::debug_handler]
pub async fn get_processes_list(State(system_state): State<AppState>) -> impl IntoResponse {
    let mut sys = system_state.sys.lock().unwrap();
    sys.refresh_processes();
    let _processes = sys.processes();

    let v: Vec<_> = sys.processes().iter()
        .map(|p|
            ProcessJSON {
                pid: p.0.to_string(),
                process_name: p.1.name().to_string()
            }
        ).collect();

    Json(v)
}