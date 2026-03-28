// src/ws/mod.rs

pub mod frontend;
pub mod implant;
pub mod shared;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/implant",  implant::router())
        .nest("/frontend", frontend::router())
}
