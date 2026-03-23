use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;
use crate::{auth::Claims, state::AppState};
 
mod auth;
mod programs;
mod download;
mod program_types;
mod capabilities;
 
pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/programs", programs::router())
        .nest("/capabilities", capabilities::router())
        .nest("/program-types", program_types::router())
        .nest("/download", download::router())
        .route("/devices", get(get_devices))
}
 
#[derive(Serialize)]
struct DeviceResponse {
    id: Uuid,
    ip: String,
    active: bool,
}

async fn get_devices(State(state): State<AppState>, _claims: Claims) -> Json<Vec<DeviceResponse>> {
    let devices = state.devices.read().await;
    let response = devices.values()
        .map(|d| DeviceResponse {
            id: d.id,
            ip: d.ip.clone(),
            active: d.active,
        })
        .collect();
    Json(response)
}
