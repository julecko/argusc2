use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tracing::info;
 
pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(ws_handler))
}
 
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}
 
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        info!("Received: {:?}", msg);
        // TODO: handle message
    }
}
