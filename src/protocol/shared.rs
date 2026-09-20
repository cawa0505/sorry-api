//! Helpers shared by both protocol adapters.
//!
//! This module owns the ONLY route from wire requests to the intelligence
//! core: `run_engine`. Adapters never embed rules inline (design.md).

use crate::intelligence::{IntelligenceEngine, Request as IntReq, render};
use axum::Json;
use axum::http::StatusCode;
use serde_json::{Value, json};

pub type ApiError = (StatusCode, Json<Value>);

/// The single seam into the intelligence core.
pub(crate) fn run_engine(text: &str) -> crate::protocol::canonical::CanonicalResponse {
    let engine = crate::intelligence::RuleEngine;
    let actions = engine.respond(&IntReq {
        text: text.to_string(),
    });
    let out = render(&actions);
    crate::protocol::canonical::CanonicalResponse { actions, text: out }
}

pub(crate) fn bad_request(msg: &str) -> ApiError {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": {
                "message": msg,
                "type": "invalid_request_error",
                "code": "invalid_request"
            }
        })),
    )
}

pub(crate) fn short_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    format!("{nanos:x}")
}

pub(crate) fn chrono_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

// ponytail: ~4 bytes/token heuristic; swap in a real tokenizer only if a client cares
pub(crate) fn token_estimate(s: &str) -> u64 {
    (s.len() as u64).div_ceil(4)
}
