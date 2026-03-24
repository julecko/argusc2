use crate::{auth::Claims, state::AppState};
use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use uuid::Uuid;

mod accounts;
mod auth;
mod capabilities;
mod download;
mod program_types;
mod programs;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/programs", programs::router())
        .nest("/capabilities", capabilities::router())
        .nest("/program-types", program_types::router())
        .nest("/download", download::router())
        .nest("/accounts", accounts::router())
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
    let response = devices
        .values()
        .map(|d| DeviceResponse {
            id: d.id,
            ip: d.ip.clone(),
            active: d.active,
        })
        .collect();
    Json(response)
}
