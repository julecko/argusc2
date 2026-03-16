mod logger;
mod ws;
mod api;
mod state;
mod db;
mod setup;

use tracing::{info};
use axum::{
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use state::AppState;
use std::env;

#[tokio::main]
async fn main() {
    let _guard: logger::LoggerGuard = logger::init();

    let args: Vec<String> = env::args().collect();
    
    let db = db::connect().await;
    
    if args.len() > 1 && args[1] == "create-admin" {
        setup::create_admin(&db).await;
        return;
    }

    let app_state = AppState { 
        devices: Default::default(),
        db 
    };

    let static_files = ServeDir::new("../frontend/build")
        .fallback(ServeFile::new("../frontend/build/index.html"));

    let app: Router = Router::new()
        .nest("/api", api::router())
        .nest("/ws", ws::router())
        .with_state(app_state)
        .fallback_service(static_files);

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Server running at http://127.0.0.1:3000");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
