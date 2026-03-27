use crate::state::AppState;
use axum::Router;

mod accounts;
mod auth;
mod capabilities;
mod download;
mod program_types;
mod programs;
mod devices;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/programs", programs::router())
        .nest("/capabilities", capabilities::router())
        .nest("/program-types", program_types::router())
        .nest("/download", download::router())
        .nest("/accounts", accounts::router())
        .nest("/devices", devices::router())
}