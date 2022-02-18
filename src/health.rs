use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct HealthResponse {
    healthy: bool,
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse{healthy: true}))
}
