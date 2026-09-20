//! Anthropic Messages compatible surface (spec §4).
//!
//! Adapter only. `"stream": true` selects Anthropic's event-stream protocol:
//! message_start → (content_block_delta…) → content_block_stop →
//! message_delta → message_stop.

use crate::protocol::canonical::canonical_text;
use crate::protocol::openai::ANALYSIS_STAGES;
use crate::protocol::shared::{ApiError, bad_request, run_engine, short_id, token_estimate};
use axum::Json;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};

/// POST /v1/messages
pub async fn messages(Json(body): Json<Value>) -> Result<Response, ApiError> {
    let stream = body
        .get("stream")
        .and_then(|s| s.as_bool())
        .unwrap_or(false);
    let msgs = body
        .get("messages")
        .and_then(|m| m.as_array())
        .cloned()
        .ok_or_else(|| bad_request("messages must be an array"))?;

    if stream {
        Ok(anthropic_stream(&body, &msgs))
    } else {
        Ok(Json(anthropic_json(&body, &msgs)).into_response())
    }
}

pub(crate) fn anthropic_json(body: &Value, msgs: &[Value]) -> Value {
    let system = system_of(body);
    let text = canonical_text(system, msgs);
    let resp = run_engine(&text);
    let model = model_of(body);

    json!({
        "id": format!("msg_{}", short_id()),
        "type": "message",
        "role": "assistant",
        "model": model,
        "content": [{"type": "text", "text": resp.text}],
        "stop_reason": "end_turn",
        "stop_sequence": Value::Null,
        "usage": {
            "input_tokens": token_estimate(&text),
            "output_tokens": token_estimate(&resp.text),
        }
    })
}

fn anthropic_stream(body: &Value, msgs: &[Value]) -> Response {
    let system = system_of(body);
    let text = canonical_text(system, msgs);
    let resp = run_engine(&text);
    let model = model_of(body);

    let mut events: Vec<(&str, Value)> = vec![(
        "message_start",
        json!({
            "type": "message_start",
            "message": {
                "id": format!("msg_{}", short_id()),
                "type": "message",
                "role": "assistant",
                "model": model,
                "content": [],
                "stop_reason": Value::Null,
                "usage": {"input_tokens": token_estimate(&text), "output_tokens": 0}
            }
        }),
    )];
    for stage in ANALYSIS_STAGES {
        events.push(("content_block_delta", delta_event(format!("{stage}\n"))));
    }
    events.push(("content_block_delta", delta_event(resp.text.clone())));
    events.push((
        "content_block_stop",
        json!({"type": "content_block_stop", "index": 0}),
    ));
    events.push((
        "message_delta",
        json!({
            "type": "message_delta",
            "delta": {"stop_reason": "end_turn", "stop_sequence": Value::Null},
            "usage": {"output_tokens": token_estimate(&resp.text)}
        }),
    ));
    events.push(("message_stop", json!({"type": "message_stop"})));

    let stream = futures::stream::iter(events.into_iter().map(|(event, data)| {
        Ok::<_, std::convert::Infallible>(Event::default().event(event).data(data.to_string()))
    }));

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

fn delta_event(text: String) -> Value {
    json!({
        "type": "content_block_delta",
        "index": 0,
        "delta": {"type": "text_delta", "text": text}
    })
}

fn system_of(body: &Value) -> Option<String> {
    // Anthropic top-level system: string, or array of blocks.
    match body.get("system") {
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Array(blocks)) => Some(
            blocks
                .iter()
                .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        _ => None,
    }
}

fn model_of(body: &Value) -> String {
    body.get("model")
        .and_then(|m| m.as_str())
        .unwrap_or("sorry-ai")
        .to_string()
}
