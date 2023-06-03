use super::AppState;
use axum::{
    extract::State,
    extract::Query,
    response::IntoResponse,
    Json
};
use sysinfo::{SystemExt, ProcessExt};
use serde::{Serialize, Deserialize};
use super::utils::empty_string_as_none;


#[derive(Serialize)]
struct ProcessJSON {
    pid: String,
    process_name: String
}

#[derive(Deserialize)]
pub struct QueryLimit {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    limit: Option<usize>
}

#[axum::debug_handler]
pub async fn get_processes_list(State(system_state): State<AppState>, query: Query<QueryLimit>) -> impl IntoResponse {
    let mut sys = system_state.sys.lock().unwrap();
    sys.refresh_processes();
    let _processes = sys.processes();

    let limit = query.limit.unwrap_or(20);

    let v: Vec<_> = sys.processes().iter().take(limit)
        .map(|p|
            ProcessJSON {
                pid: p.0.to_string(),
                process_name: p.1.name().to_string()
            }
        ).collect();

    Json(v)
}