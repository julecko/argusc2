// Routes:
//   GET    /api/programs          → list all programs (with type join)
//   GET    /api/programs/:id      → single program by id
//   POST   /api/programs          → upload new program (multipart)
//   PATCH  /api/programs/:id      → update editable fields
//   DELETE /api/programs/:id      → delete program + file

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

use crate::{
    auth::Claims,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_programs).post(upload_program))
        .route("/{id}", get(get_program).patch(update_program).delete(delete_program))
}

// ── Shared helpers ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn err(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

fn rand_hex64() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    // Simple unique key: sha256(timestamp + random bytes)
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let input = format!("{}-{}", nanos, uuid::Uuid::new_v4());
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
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
    pub allowed_downloads: i64,
    pub created_at:       String,
    pub updated_at:       String,
}

// ── GET /api/programs ─────────────────────────────────────────────────────────

async fn list_programs(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<Vec<ProgramRow>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = sqlx::query_as::<_, ProgramRow>(
        r#"
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
        ORDER BY p.created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok(Json(rows))
}

// ── GET /api/programs/:id ─────────────────────────────────────────────────────

async fn get_program(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    _claims: Claims,
) -> Result<Json<ProgramRow>, (StatusCode, Json<ErrorResponse>)> {
    let row = sqlx::query_as::<_, ProgramRow>(
        r#"
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
        WHERE p.id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, err("Program not found")))?;

    Ok(Json(row))
}

// ── POST /api/programs ────────────────────────────────────────────────────────
// Accepts multipart/form-data:
//   file             — binary file
//   name             — display name
//   version          — e.g. "v1.0"
//   type_id          — FK to program_types
//   os               — windows | linux | android | mac
//   allowed_downloads — number, 0 = unlimited
//   description      — optional text

async fn upload_program(
    State(state): State<AppState>,
    claims: Claims,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ProgramRow>), (StatusCode, Json<ErrorResponse>)> {
    // ── Parse multipart fields ────────────────────────────────
    let mut file_bytes:        Option<Vec<u8>> = None;
    let mut original_name:     String          = String::new();
    let mut name:              String          = String::new();
    let mut version:           String          = String::new();
    let mut type_id:           i64             = 0;
    let mut os:                String          = String::new();
    let mut allowed_downloads: i64             = 0;
    let mut description:       Option<String>  = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, err(format!("Multipart error: {e}"))))?
    {
        match field.name().unwrap_or("") {
            "file" => {
                original_name = field
                    .file_name()
                    .unwrap_or("unknown")
                    .to_string();
                file_bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| (StatusCode::BAD_REQUEST, err(format!("Read error: {e}"))))?
                        .to_vec(),
                );
            }
            "name"              => name              = field.text().await.unwrap_or_default(),
            "version"           => version           = field.text().await.unwrap_or_default(),
            "type_id"           => type_id           = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "os"                => os                = field.text().await.unwrap_or_default(),
            "allowed_downloads" => allowed_downloads = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "description"       => {
                let t = field.text().await.unwrap_or_default();
                description = if t.is_empty() { None } else { Some(t) };
            }
            _ => {}
        }
    }

    // ── Validate ──────────────────────────────────────────────
    let bytes = file_bytes.ok_or_else(|| (StatusCode::BAD_REQUEST, err("File is required")))?;
    if name.trim().is_empty()    { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("Name is required"))); }
    if version.trim().is_empty() { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("Version is required"))); }
    if type_id == 0              { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("type_id is required"))); }
    if os.trim().is_empty()      { return Err((StatusCode::UNPROCESSABLE_ENTITY, err("OS is required"))); }

    // ── Hash + store file ─────────────────────────────────────
    let file_hash    = hash_bytes(&bytes);
    let ws_key       = rand_hex64();
    let storage_dir  = PathBuf::from("files/programs");
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

    let filesize = bytes.len() as i64;

    // ── Look up uploader id ───────────────────────────────────
    let uploader_id: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM admins WHERE username = ?",
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    // ── Insert ────────────────────────────────────────────────
    let insert_id = sqlx::query(
        r#"
        INSERT INTO programs
            (type_id, uploaded_by, name, original_name, version, os,
             storage_path, filesize, file_hash, ws_key, description, allowed_downloads)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
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
            (StatusCode::CONFLICT, err("A program with this file hash already exists"))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}")))
        }
    })?
    .last_insert_id() as i64;

    // ── Return full row ───────────────────────────────────────
    let row = sqlx::query_as::<_, ProgramRow>(
        r#"
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
        WHERE p.id = ?
        "#,
    )
    .bind(insert_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    Ok((StatusCode::CREATED, Json(row)))
}

// ── PATCH /api/programs/:id ───────────────────────────────────────────────────
// Only allows updating: name, version, os, description, allowed_downloads

#[derive(Deserialize)]
struct UpdateRequest {
    name:              Option<String>,
    version:           Option<String>,
    os:                Option<String>,
    description:       Option<String>,
    allowed_downloads: Option<i64>,
    ws_key:            Option<String>,
}

async fn update_program(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    _claims: Claims,
    Json(body): Json<UpdateRequest>,
) -> Result<Json<ProgramRow>, (StatusCode, Json<ErrorResponse>)> {
    // Only update fields that were provided
    if let Some(ref name) = body.name {
        if name.trim().is_empty() {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, err("Name cannot be empty")));
        }
    }

    sqlx::query(
        r#"
        UPDATE programs SET
            name              = COALESCE(?, name),
            version           = COALESCE(?, version),
            os                = COALESCE(?, os),
            description       = COALESCE(?, description),
            allowed_downloads = COALESCE(?, allowed_downloads),
            ws_key            = COALESCE(?, ws_key)
        WHERE id = ?
        "#,
    )
    .bind(body.name.as_deref())
    .bind(body.version.as_deref())
    .bind(body.os.as_deref())
    .bind(body.description.as_deref())
    .bind(body.allowed_downloads)
    .bind(body.ws_key.as_deref())
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?;

    // Return updated row
    let row = sqlx::query_as::<_, ProgramRow>(
        r#"
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
        WHERE p.id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, err(format!("DB error: {e}"))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, err("Program not found")))?;

    Ok(Json(row))
}

// ── DELETE /api/programs/:id ──────────────────────────────────────────────────

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