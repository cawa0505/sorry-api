//! HTTP server assembly: health, demo endpoints, protocol routes (spec §8, §11).

use crate::intelligence::{APOLOGY, Action};
use axum::Json;
use serde_json::json;

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "intelligence": "super",
        "kneeling": true
    }))
}

pub async fn kneel() -> Json<serde_json::Value> {
    Json(json!({
        "status": "kneeling",
        "message": APOLOGY,
        "next_action": "shut_up"
    }))
}

pub async fn apologize() -> Json<serde_json::Value> {
    Json(json!({ "message": APOLOGY }))
}

pub async fn shut_up() -> Json<serde_json::Value> {
    Json(json!({ "status": "silent", "message": "" }))
}

/// The complete SorryAPI behavior in one endpoint.
pub async fn sorry() -> Json<serde_json::Value> {
    Json(json!({
        "status": "completed",
        "action": [
            Action::Kneel.name(),
            Action::Apologize.name(),
            Action::ShutUp.name()
        ],
        "message": APOLOGY
    }))
}
