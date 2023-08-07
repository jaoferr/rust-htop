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
    process_name: String,
    memory_usage: u64,
    command: String,
    cpu_usage: f32
}

#[derive(Deserialize)]
pub struct QueryArgs {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    limit: Option<usize>,
    order: Option<String>
}

#[axum::debug_handler]
pub async fn get_processes_list(State(system_state): State<AppState>, query: Query<QueryArgs>) -> impl IntoResponse {
    let mut sys = system_state.sys.lock().unwrap();
    sys.refresh_processes();

    let limit = query.limit.unwrap_or(20);

    let mut process_list: Vec<_> = sys.processes().iter().collect();
    process_list.sort_by(|c, n| {
        let current_process = sys.process(c.0.to_owned()).unwrap();
        let next_process = sys.process(n.0.to_owned()).unwrap();
        current_process.memory().cmp(&next_process.memory())
    });

    match query.order.as_deref() {
        Some("asc") => (),
        Some("desc") => process_list.reverse(),
        _ => process_list.reverse()
    }

    let v: Vec<_> = process_list.iter().take(limit)
        .map(|p| {
            let this_process = sys.process(p.0.to_owned()).unwrap();
            return ProcessJSON {
                pid: p.0.to_string(),
                process_name: p.1.name().to_string(),
                memory_usage: this_process.memory() / 1000,
                command: this_process.cmd().concat().to_string(),
                cpu_usage: this_process.cpu_usage()
            }
        }
    ).collect();   

    Json(v)
}