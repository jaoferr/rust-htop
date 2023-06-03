use super::AppState;

use axum::{
    extract::State,
    response::IntoResponse,
    Json
};
use serde::Serialize;
use sysinfo::SystemExt;

#[derive(Serialize)]
pub struct SystemInfo {
    os_name: String,
    total_memory: u64,
    available_memory: u64,
    uptime: u64
}

#[axum::debug_handler]
pub async fn get_sys_info(State(system_state): State<AppState>) -> impl IntoResponse {
    let mut sys = system_state.sys.lock().unwrap();
    sys.refresh_memory();

    let v = SystemInfo {
        os_name: sys.long_os_version().unwrap(),
        total_memory: sys.total_memory() / 1000000,
        available_memory: sys.available_memory() / 1000000,
        uptime: sys.uptime()
    };

    Json(v)
}
