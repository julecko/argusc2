// Routes (all require valid JWT):
//   GET /api/accounts/all     → list all accounts

use crate::{auth::Claims, state::AppState};
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/all", get(list_accounts))
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn err(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AccountsRow {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub last_login: String,
    pub created_at: String,
}

async fn list_accounts(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<Vec<AccountsRow>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = sqlx::query_as::<_, AccountsRow>(
        "SELECT id, username, email, CAST(last_login AS CHAR) as last_login, CAST(created_at AS CHAR) as created_at FROM admins ORDER BY username ASC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            err(format!("DB error: {e}")),
        )
    })?;

    Ok(Json(rows))
}
