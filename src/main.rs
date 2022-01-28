use axum::{
    Router,
    routing::get,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Debug, Serialize, Clone)]
struct HealthResponse {
    healthy: bool,
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse{healthy: true}))
}
