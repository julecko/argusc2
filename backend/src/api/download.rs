// Routes (no JWT required — these are public download links):
//   GET /api/download/programs/{id}  → stream the file, enforce download limits
//
// allowed_downloads semantics:
//   -1 = unlimited  (downloads counter still increments, allowed stays -1)
//    0 = forbidden  (returns 403)
//   >0 = fixed limit (decrements on each download, 403 when exhausted)

use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Serialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/programs/{file_hash}", get(download_program))
}

// ── Error response ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn err_json(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

// ── Helpers  ──────────────────────────────────────────────────────────────────

fn is_valid_sha256(hash: &str) -> bool {
    hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit())
}

// ── DB row ────────────────────────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
struct ProgramFile {
    id: i64,
    original_name: String,
    storage_path: String,
    allowed_downloads: i64,
}

// ── GET /download/programs/{id} ───────────────────────────────────────────────

async fn download_program(
    State(state): State<AppState>,
    Path(file_hash): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    // ── 1. Verify if hash is valid ────────────────────────────
    if !is_valid_sha256(&file_hash) {
        return Err((StatusCode::BAD_REQUEST, err_json("Invalid hash format")));
    }

    // ── 2. Fetch program row ──────────────────────────────────
    let program = sqlx::query_as::<_, ProgramFile>(
        "SELECT id, original_name, storage_path, allowed_downloads
         FROM programs WHERE file_hash = ?",
    )
    .bind(&file_hash)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            err_json(format!("DB error: {e}")),
        )
    })?
    .ok_or_else(|| (StatusCode::NOT_FOUND, err_json("Program not found")))?;

    // ── 3. Enforce download limits ────────────────────────────
    match program.allowed_downloads {
        0 => {
            // Explicitly forbidden
            return Err((
                StatusCode::FORBIDDEN,
                err_json("Downloads are forbidden for this program"),
            ));
        }
        _ => {} // -1 = unlimited, or limit not yet reached — proceed
    }

    // ── 4. Open file ──────────────────────────────────────────
    let file = File::open(&program.storage_path)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, err_json("File not found on disk")))?;

    let metadata = file.metadata().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            err_json(format!("IO error: {e}")),
        )
    })?;

    let file_size = metadata.len();

    // ── 5. Increment counters atomically ──────────────────────
    sqlx::query(
        "UPDATE programs
         SET
             downloads         = downloads + 1,
             allowed_downloads = CASE
                 WHEN allowed_downloads > 0 THEN allowed_downloads - 1
                 ELSE allowed_downloads   -- keep -1 (unlimited) as-is
             END
         WHERE id = ?",
    )
    .bind(program.id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            err_json(format!("DB error: {e}")),
        )
    })?;

    // ── 6. Stream file to client ──────────────────────────────
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let filename = &program.original_name;

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, err_json("Header error")))?,
    );
    headers.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&file_size.to_string())
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, err_json("Header error")))?,
    );
    // Allow resumable downloads
    headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));

    Ok((StatusCode::OK, headers, body).into_response())
}
