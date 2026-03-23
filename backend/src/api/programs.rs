// allowed_downloads semantics:
//   -1 = unlimited
//    0 = forbidden (no downloads allowed)
//   >0 = fixed limit
//
// Routes:
//   GET    /api/programs          → list all programs
//   GET    /api/programs/{id}     → single program
//   POST   /api/programs          → upload (multipart)
//   PATCH  /api/programs/{id}     → update editable fields incl. type_id, capabilities, ws_key
//   DELETE /api/programs/{id}     → delete program + file

use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    http::StatusCode,
    routing::{get},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs;

use crate::{auth::Claims, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",     get(list_programs).post(upload_program))
        .route("/{id}", get(get_program).patch(update_program).delete(delete_program))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ErrorResponse { error: String }

fn err(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

fn hash_bytes(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    format!("{:x}", h.finalize())
}

fn rand_hex64() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let input = format!("{}-{}", nanos, uuid::Uuid::new_v4());
    hash_bytes(input.as_bytes())
}

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Serialize, sqlx::FromRow)]
pub struct ProgramRow {
    pub id:               i64,
    pub type_id:          i64,
    pub type_name:        Option<String>,
    pub type_color:       Option<String>,
    pub uploaded_by:      Option<i64>,
    pub uploader_name:    Option<String>,
    pub name:             String,
    pub original_name:    String,
    pub version:          String,
    pub os:               String,
    pub storage_path:     String,
    pub filesize:         i64,
    pub file_hash:        String,
    pub ws_key:           String,
    pub description:      Option<String>,
    pub downloads:        i64,
    pub allowed_downloads: i64,  // -1 = unlimited, 0 = forbidden, >0 = fixed limit
    pub created_at:       String,
    pub updated_at:       String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CapabilityRef {
    pub id:   i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct ProgramDetail {
    #[serde(flatten)]
    pub program:      ProgramRow,
    pub capabilities: Vec<CapabilityRef>,
}

// ── Shared query for ProgramRow ───────────────────────────────────────────────

const PROGRAM_SELECT: &str = r#"
    SELECT
        p.id, p.type_id,
        pt.name  AS type_name,
        pt.color AS type_color,
        p.uploaded_by,
        a.username AS uploader_name,
        p.name, p.original_name, p.version, p.os,
        p.storage_path, p.filesize, p.file_hash, p.ws_key,
        p.description, p.downloads, p.allowed_downloads,
        CAST(p.created_at AS CHAR) AS created_at,
        CAST(p.updated_at AS CHAR) AS updated_at
    FROM programs p
    LEFT JOIN program_types pt ON pt.id = p.type_id
    LEFT JOIN admins a         ON a.id  = p.uploaded_by
"#;

async fn fetch_capabilities(
    db: &sqlx::MySqlPool,
    program_id: i64,
) -> Result<Vec<CapabilityRef>, sqlx::Error> {
    sqlx::query_as::<_, CapabilityRef>(
        r#"SELECT c.id, c.name
           FROM capabilities c
           JOIN program_capabilities pc ON pc.capability_id = c.id
           WHERE pc.program_id = ?"#,
    )
    .bind(program_id)
    .fetch_all(db)
    .await
}

async fn replace_capabilities(
    db: &sqlx::MySqlPool,
    program_id: i64,
    capability_ids: &[i64],
) -> Result<(), sqlx::Error> {
    // Delete existing
    sqlx::query("DELETE FROM program_capabilities WHERE program_id = ?")
        .bind(program_id)
        .execute(db)
        .await?;

    // Insert new
    for &cap_id in capability_ids {
        sqlx::query(
            "INSERT IGNORE INTO program_capabilities (program_id, capability_id) VALUES (?, ?)",
        )
        .bind(program_id)
        .bind(cap_id)
        .execute(db)
        .await?;
    }

    Ok(())
}

// ── GET /api/programs ─────────────────────────────────────────────────────────

async fn list_programs(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<Vec<ProgramRow>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = sqlx::query_as::<_, ProgramRow>(
        &format!("{PROGRAM_SELECT} ORDER BY p.created_at DESC"),
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok(Json(rows))
}

// ── GET /api/programs/{id} ────────────────────────────────────────────────────

async fn get_program(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<Json<ProgramDetail>, (StatusCode, Json<ErrorResponse>)> {
    let program = sqlx::query_as::<_, ProgramRow>(
        &format!("{PROGRAM_SELECT} WHERE p.id = ?"),
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, err("Program not found")))?;

    let capabilities = fetch_capabilities(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok(Json(ProgramDetail { program, capabilities }))
}

// ── POST /api/programs ────────────────────────────────────────────────────────

async fn upload_program(
    State(state): State<AppState>,
    claims: Claims,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ProgramDetail>), (StatusCode, Json<ErrorResponse>)> {
    let mut file_bytes:        Option<Vec<u8>> = None;
    let mut original_name:     String          = String::new();
    let mut name:              String          = String::new();
    let mut version:           String          = String::new();
    let mut type_id:           i64             = 0;
    let mut os:                String          = String::new();
    let mut allowed_downloads: i64             = -1; // default = unlimited
    let mut description:       Option<String>  = None;
    let mut ws_key_input:      String          = String::new();
    let mut capability_ids:    Vec<i64>        = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, err(format!("Multipart error: {e}"))))?
    {
        match field.name().unwrap_or("") {
            "file" => {
                original_name = field.file_name().unwrap_or("unknown").to_string();
                file_bytes = Some(
                    field.bytes().await
                        .map_err(|e| (StatusCode::BAD_REQUEST, err(format!("Read error: {e}"))))?
                        .to_vec(),
                );
            }
            "name"              => name              = field.text().await.unwrap_or_default(),
            "version"           => version           = field.text().await.unwrap_or_default(),
            "type_id"           => type_id           = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "os"                => os                = field.text().await.unwrap_or_default(),
            "allowed_downloads" => allowed_downloads = field.text().await.unwrap_or_default().parse().unwrap_or(-1),
            "description"       => {
                let t = field.text().await.unwrap_or_default();
                description = if t.is_empty() { None } else { Some(t) };
            }
            "ws_key" => ws_key_input = field.text().await.unwrap_or_default(),
            // capabilities[] repeated field — each value is a capability id
            "capabilities[]" => {
                if let Ok(id) = field.text().await.unwrap_or_default().parse::<i64>() {
                    capability_ids.push(id);
                }
            }
            _ => {}
        }
    }

    let bytes = file_bytes
        .ok_or_else(|| (StatusCode::BAD_REQUEST, err("File is required")))?;
    if name.trim().is_empty()    { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("Name is required"))); }
    if version.trim().is_empty() { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("Version is required"))); }
    if type_id == 0              { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("type_id is required"))); }
    if os.trim().is_empty()      { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("OS is required"))); }

    // Use provided ws_key or generate one
    let ws_key = if ws_key_input.len() == 64 {
        ws_key_input
    } else {
        rand_hex64()
    };

    let file_hash    = hash_bytes(&bytes);
    let storage_dir  = PathBuf::from("uploads");
    fs::create_dir_all(&storage_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("Storage error: {e}"))))?;

    let storage_path = storage_dir
        .join(format!("{}_{}", &file_hash[..16], &original_name))
        .to_string_lossy()
        .to_string();

    fs::write(&storage_path, &bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("Write error: {e}"))))?;

    let filesize: i64 = bytes.len() as i64;

    let uploader_id: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM admins WHERE username = ?",
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    let insert_id = sqlx::query(
        r#"INSERT INTO programs
            (type_id, uploaded_by, name, original_name, version, os,
             storage_path, filesize, file_hash, ws_key, description, allowed_downloads)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(type_id)
    .bind(uploader_id)
    .bind(name.trim())
    .bind(&original_name)
    .bind(version.trim())
    .bind(os.trim())
    .bind(&storage_path)
    .bind(filesize)
    .bind(&file_hash)
    .bind(&ws_key)
    .bind(&description)
    .bind(allowed_downloads)
    .execute(&state.db)
    .await
    .map_err(|e| {
        if e.to_string().to_lowercase().contains("duplicate") {
            (StatusCode::CONFLICT, err("A program with this file hash or ws_key already exists"))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}")))
        }
    })?
    .last_insert_id() as i64;

    if !capability_ids.is_empty() {
        replace_capabilities(&state.db, insert_id, &capability_ids)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;
    }

    let program = sqlx::query_as::<_, ProgramRow>(
        &format!("{PROGRAM_SELECT} WHERE p.id = ?"),
    )
    .bind(insert_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    let capabilities = fetch_capabilities(&state.db, insert_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok((StatusCode::CREATED, Json(ProgramDetail { program, capabilities })))
}

// ── PATCH /api/programs/{id} ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct UpdateRequest {
    name:              Option<String>,
    version:           Option<String>,
    os:                Option<String>,
    description:       Option<String>,
    allowed_downloads: Option<i64>,   // -1 = unlimited, 0 = forbidden
    ws_key:            Option<String>,
    type_id:           Option<i64>,
    capability_ids:    Option<Vec<i64>>,
}

async fn update_program(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    _claims: Claims,
    Json(body): Json<UpdateRequest>,
) -> Result<Json<ProgramDetail>, (StatusCode, Json<ErrorResponse>)> {
    if let Some(ref name) = body.name {
        if name.trim().is_empty() {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, err("Name cannot be empty")));
        }
    }
    if let Some(ref ws) = body.ws_key {
        if ws.len() != 64 {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, err("ws_key must be exactly 64 characters")));
        }
    }

    // Check program exists
    let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM programs WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    if exists.is_none() {
        return Err((StatusCode::NOT_FOUND, err("Program not found")));
    }

    sqlx::query(
        r#"UPDATE programs SET
            name              = COALESCE(?, name),
            version           = COALESCE(?, version),
            os                = COALESCE(?, os),
            description       = COALESCE(?, description),
            allowed_downloads = COALESCE(?, allowed_downloads),
            ws_key            = COALESCE(?, ws_key),
            type_id           = COALESCE(?, type_id)
           WHERE id = ?"#,
    )
    .bind(body.name.as_deref())
    .bind(body.version.as_deref())
    .bind(body.os.as_deref())
    .bind(body.description.as_deref())
    .bind(body.allowed_downloads)
    .bind(body.ws_key.as_deref())
    .bind(body.type_id)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    // Replace capabilities if provided
    if let Some(ref cap_ids) = body.capability_ids {
        replace_capabilities(&state.db, id, cap_ids)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;
    }

    let program = sqlx::query_as::<_, ProgramRow>(
        &format!("{PROGRAM_SELECT} WHERE p.id = ?"),
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    let capabilities = fetch_capabilities(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok(Json(ProgramDetail { program, capabilities }))
}

// ── DELETE /api/programs/{id} ──────────────────────────────────────────────────

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

async fn delete_program(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<Json<DeleteResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get storage path before deleting the row
    let path: Option<String> =
        sqlx::query_scalar("SELECT storage_path FROM programs WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    let storage_path = path.ok_or_else(|| (StatusCode::NOT_FOUND, err("Program not found")))?;

    // Delete DB row first
    sqlx::query("DELETE FROM programs WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    // Remove file from disk — non-fatal if already gone
    let _ = fs::remove_file(&storage_path).await;

    Ok(Json(DeleteResponse {
        message: "Program deleted successfully".to_string(),
    }))
}
