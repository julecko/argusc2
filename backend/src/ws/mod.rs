use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::info;
use uuid::Uuid;
use crate::state::{AppState, Device};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr, state: AppState) {
    let device_id = match socket.recv().await {
        Some(Ok(Message::Text(text))) => {
            let text = text.trim().to_string();
            if text == "new" {
                None
            } else {
                Uuid::parse_str(&text).ok()
            }
        }
        _ => {
            info!("Client disconnected before handshake");
            return;
        }
    };

    let device_id = {
        let mut devices = state.devices.write().await;

        match device_id.and_then(|id| devices.get_mut(&id)) {
            Some(existing) => {
                existing.active = true;
                existing.ip = addr.ip().to_string();
                info!("Device reconnected: {} (id: {})", addr.ip(), existing.id);
                existing.id
            }
            None => {
                let device = Device::new(addr.ip().to_string());
                let id = device.id;
                devices.insert(id, device);
                info!("Device connected (new): {} (id: {})", addr.ip(), id);
                id
            }
        }
    };

    if socket
        .send(Message::Text(device_id.to_string().into()))
        .await
        .is_err()
    {
        return;
    }

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => info!("Received from {device_id}: {text}"),
            Message::Ping(_) | Message::Pong(_) => {}
            Message::Close(_) => break,
            _ => {}
        }
    }

    if let Some(device) = state.devices.write().await.get_mut(&device_id) {
        device.active = false;
    }
    info!("Device disconnected: {} (id: {})", addr.ip(), device_id);
}
