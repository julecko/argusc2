use crate::state::AppState;
use axum::Router;
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
}

#[derive(Serialize)]
struct DeviceResponse {
    id: Uuid,
    ip: String,
    active: bool,
}
