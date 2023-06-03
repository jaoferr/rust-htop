use super::AppState;
use axum::{
    extract::State,
    response::IntoResponse,
    Json
};
use sysinfo::{SystemExt, DiskExt};
use serde::{Serialize};


#[derive(Serialize)]
struct DiskInfoJSON {
    name: String,
    available_space: u64,
    total_space: u64,
    used_space: u64
}

#[axum::debug_handler]
pub async fn get_disk_info(State(system_state): State<AppState>) -> impl IntoResponse {
    let mut sys = system_state.sys.lock().unwrap();
    sys.refresh_disks_list();

    let v: Vec<_> = sys.disks().iter()
        .map(|disk| {
            return DiskInfoJSON {
                name: disk.name().to_os_string().into_string().unwrap(),
                available_space: disk.available_space() / 1000000,
                total_space: disk.total_space() / 1000000,
                used_space: (disk.total_space() - disk.available_space()) / 1000000
            }
        }
        ).collect();

    Json(v)
}