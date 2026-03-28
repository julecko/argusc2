// src/ws/shared.rs

use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket};
use serde::Serialize;

// ── Socket helpers ────────────────────────────────────────────────────────────

pub async fn recv_text(socket: &mut WebSocket) -> Option<String> {
    match socket.recv().await {
        Some(Ok(Message::Text(t))) => Some(t.trim().to_string()),
        _ => None,
    }
}

pub async fn send_text(socket: &mut WebSocket, msg: &str) -> bool {
    socket
        .send(Message::Text(msg.to_string().into()))
        .await
        .is_ok()
}

// ── Push event bus ────────────────────────────────────────────────────────────

/// Deliver `payload` to every frontend browser socket watching `device_id`.
/// Called by the implant handler on EVENT and on connect/disconnect.
pub async fn push_to_frontends(device_id: &str, payload: String, state: &AppState) {
    let map = state.frontend_senders.read().await;
    if let Some(senders) = map.get(device_id) {
        for tx in senders {
            let _ = tx.send(payload.clone()).await;
        }
    }
}

// ── Outbound JSON envelope sent to the browser ────────────────────────────────

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FrontendMsg {
    /// Response to a command the browser issued
    Response {
        id: String,
        ok: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
    /// Push event forwarded from the implant
    Event {
        event: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        level: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
    /// Device online/offline status
    Status { connected: bool, device_id: String },
}

pub fn to_json(msg: &FrontendMsg) -> String {
    serde_json::to_string(msg).unwrap_or_else(|_| r#"{"type":"error"}"#.into())
}
