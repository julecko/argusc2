// src/routes/devices.rs

use crate::{auth::Claims, state::AppState};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Serialize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/all", get(list_devices))
        .route("/{device_id}", get(get_device))
}

// ── Shared error helpers ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn err(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

// ── /all ──────────────────────────────────────────────────────────────────────

/// Lightweight summary row returned by GET /all
#[derive(Serialize)]
pub struct DeviceSummary {
    pub device_id: String,
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub ip_external: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub is_online: bool,
    pub last_seen: i64,
}

async fn list_devices(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<Vec<DeviceSummary>>, (StatusCode, Json<ErrorResponse>)> {
    // Fetch base device data
    let rows = sqlx::query!(
        r#"
        SELECT
            d.device_id,
            d.hostname,
            d.username,
            d.ip_external,
            d.os           AS "os: String",
            d.os_version,
            UNIX_TIMESTAMP(d.last_seen) as last_seen
        FROM devices d
        GROUP BY
            d.device_id, d.hostname, d.username, d.ip_external,
            d.os, d.os_version, d.last_seen
        ORDER BY d.last_seen DESC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            err(format!("DB error: {e}")),
        )
    })?;

    // Check which device_ids are currently live in memory
    let online_ids = {
        let devices = state.devices.read().await;
        devices
            .iter()
            .filter(|(_, d)| d.active)
            .map(|(id, _)| id.clone())
            .collect::<std::collections::HashSet<_>>()
    };

    let summaries = rows
        .into_iter()
        .map(|r| DeviceSummary {
            is_online: online_ids.contains(&r.device_id),
            device_id: r.device_id,
            hostname: r.hostname,
            username: r.username,
            ip_external: r.ip_external,
            os: r.os,
            os_version: r.os_version,
            last_seen: r.last_seen,
        })
        .collect();

    Ok(Json(summaries))
}

// ── /[device_id] ──────────────────────────────────────────────────────────────

/// Full device detail row returned by GET /:device_id
#[derive(Serialize)]
pub struct DeviceDetail {
    // Identity
    pub device_id: String,
    pub program_id: i64,
    pub hostname: Option<String>,
    pub username: Option<String>,

    // Network
    pub ip_internal: Option<String>,
    pub ip_external: Option<String>,
    pub mac_address: Option<String>,
    pub country_code: Option<String>,
    pub city: Option<String>,

    // OS / hardware
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub arch: Option<String>,
    pub cpu: Option<String>,
    pub cpu_cores: Option<u8>,
    pub ram_bytes: Option<u64>,
    pub disk_bytes: Option<u64>,
    pub is_elevated: bool,
    pub timezone: Option<String>,

    // Timestamps
    pub first_seen: String,
    pub last_seen: String,

    // Runtime state (from memory)
    pub is_online: bool,
    pub caps: Vec<String>,

    // Aggregates
    pub upload_count: i64,
    pub event_count: i64,
    pub recent_events: Vec<RecentEvent>,
}

#[derive(Serialize)]
pub struct RecentEvent {
    pub id: i64,
    pub level: String,
    pub message: String,
    pub created_at: String,
}

async fn get_device(
    State(state): State<AppState>,
    _claims: Claims,
    Path(device_id): Path<String>,
) -> Result<Json<DeviceDetail>, (StatusCode, Json<ErrorResponse>)> {
    // ── Core device row ───────────────────────────────────────────────────
    let row = sqlx::query!(
        r#"
        SELECT
            d.device_id,
            d.program_id,
            d.hostname,
            d.username,
            d.ip_internal,
            d.ip_external,
            d.mac_address,
            d.os           AS "os: String",
            d.os_version,
            d.arch         AS "arch: String",
            d.cpu,
            d.cpu_cores,
            d.ram_bytes,
            d.disk_bytes,
            d.is_elevated,
            d.country_code,
            d.city,
            d.timezone,
            d.first_seen,
            d.last_seen
        FROM devices d
        WHERE d.device_id = ?
        "#,
        device_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            err(format!("DB error: {e}")),
        )
    })?
    .ok_or_else(|| (StatusCode::NOT_FOUND, err("Device not found")))?;

    // ── Upload count ──────────────────────────────────────────────────────
    let upload_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM uploads WHERE device_id = ?",
        device_id
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    // ── Event count ───────────────────────────────────────────────────────
    let event_count =
        sqlx::query_scalar!("SELECT COUNT(*) FROM events WHERE device_id = ?", device_id)
            .fetch_one(&state.db)
            .await
            .unwrap_or(0);

    // ── 10 most recent events ─────────────────────────────────────────────
    let raw_events = sqlx::query!(
        r#"
        SELECT id, level AS "level: String", message, created_at
        FROM events
        WHERE device_id = ?
        ORDER BY created_at DESC
        LIMIT 10
        "#,
        device_id
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let recent_events = raw_events
        .into_iter()
        .map(|e| RecentEvent {
            id: e.id as i64,
            level: e.level,
            message: e.message,
            created_at: e.created_at.to_string(),
        })
        .collect();

    // ── Runtime state from memory ─────────────────────────────────────────
    let (is_online, caps) = {
        let devices = state.devices.read().await;
        match devices.get(&device_id) {
            Some(d) => (d.active, d.caps.clone()),
            None => (false, vec![]),
        }
    };

    Ok(Json(DeviceDetail {
        device_id: row.device_id,
        program_id: row.program_id as i64,
        hostname: row.hostname,
        username: row.username,
        ip_internal: row.ip_internal,
        ip_external: row.ip_external,
        mac_address: row.mac_address,
        country_code: row.country_code,
        city: row.city,
        os: row.os,
        os_version: row.os_version,
        arch: row.arch,
        cpu: row.cpu,
        cpu_cores: row.cpu_cores.map(|v| v as u8),
        ram_bytes: row.ram_bytes.map(|v| v as u64),
        disk_bytes: row.disk_bytes.map(|v| v as u64),
        is_elevated: row.is_elevated != 0,
        timezone: row.timezone,
        first_seen: row.first_seen.to_string(),
        last_seen: row.last_seen.to_string(),
        is_online,
        caps,
        upload_count,
        event_count,
        recent_events,
    }))
}
