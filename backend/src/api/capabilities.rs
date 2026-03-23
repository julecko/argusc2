// Routes (all require valid JWT):
//   GET /api/capabilities     → list all capabilities

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;
use crate::{auth::Claims, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/all",   get(list_capabilities))
}

#[derive(Serialize)]
struct ErrorResponse { error: String }

fn err(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CapabilityRow {
    pub id:          i64,
    pub name:        String,
    pub description: Option<String>,
}

async fn list_capabilities(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<Vec<CapabilityRow>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = sqlx::query_as::<_, CapabilityRow>(
        "SELECT id, name, description FROM capabilities ORDER BY name ASC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok(Json(rows))
}