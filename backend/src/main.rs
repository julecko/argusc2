use axum::{
    routing::get,
    Router,
    http::StatusCode,
};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

// API endpoint
async fn hello() -> &'static str {
    "Hello from Rust"
}

// SPA fallback (return index.html)
async fn spa_fallback() -> Result<axum::response::Html<String>, StatusCode> {
    let index = tokio::fs::read_to_string("../frontend/dist/index.html")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::response::Html(index))
}

#[tokio::main]
async fn main() {
    let api_routes = Router::new()
        .route("/hello", get(hello));

    let static_files = ServeDir::new("../frontend/dist")
        .fallback(ServeFile::new("../frontend/dist/index.html"));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(static_files);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
