//! Minimal operational endpoints. No trading or account mutation endpoints exist yet.

use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct StatusResponse {
    status: &'static str,
    mode: &'static str,
    live_trading_enabled: bool,
}

async fn health() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "ok",
        mode: "paper",
        live_trading_enabled: false,
    })
}

async fn ready() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "ready",
        mode: "paper",
        live_trading_enabled: false,
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
}
