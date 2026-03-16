mod logger;

use tracing::{info, warn, error};
use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

// API endpoint
async fn hello() -> &'static str {
    "Hello from Rust"
}

#[tokio::main]
async fn main() {
    let _guard: logger::LoggerGuard = logger::init();
    let api_routes: Router = Router::new()
        .route("/hello", get(hello));

    let static_files = ServeDir::new("../frontend/build")
        .fallback(ServeFile::new("../frontend/build/index.html"));

    let app: Router = Router::new()
        .nest("/api", api_routes)
        .fallback_service(static_files);

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Server running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
