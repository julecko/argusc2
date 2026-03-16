use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;
use crate::state::AppState;
 
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/devices", get(get_devices))
}
 
#[derive(Serialize)]
struct DeviceResponse {
    id: Uuid,
    ip: String,
    active: bool,
}

async fn get_devices(State(state): State<AppState>) -> Json<Vec<DeviceResponse>> {
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
