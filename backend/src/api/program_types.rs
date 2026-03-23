// Routes (all require valid JWT):
//   GET /api/program-types    → list all program types

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;
use crate::{auth::Claims, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/all",  get(list_program_types))
}

#[derive(Serialize)]
struct ErrorResponse { error: String }

fn err(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ProgramTypeRow {
    pub id:    i64,
    pub name:  String,
    pub color: String,
}

async fn list_program_types(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<Vec<ProgramTypeRow>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = sqlx::query_as::<_, ProgramTypeRow>(
        "SELECT id, name, color FROM program_types ORDER BY name ASC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok(Json(rows))
}
