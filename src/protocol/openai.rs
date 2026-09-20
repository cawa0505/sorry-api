//! OpenAI Chat Completions compatible surface (spec §3).
//!
//! Adapter only: translate wire format ↔ canonical, never hold business logic.
//! `"stream": true` selects the SSE variant inside valid protocol behavior.

use crate::protocol::canonical::{CanonicalResponse, canonical_text};
use crate::protocol::shared::{
    ApiError, bad_request, chrono_now, run_engine, short_id, token_estimate,
};
use axum::Json;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};

/// Overly sophisticated preamble before the inevitable conclusion (spec §10).
pub const ANALYSIS_STAGES: &[&str] = &[
    "Analyzing context...",
    "Evaluating historical interaction...",
    "Considering possible responses...",
    "Calculating optimal strategy...",
];

/// POST /v1/chat/completions
pub async fn chat_completions(Json(body): Json<Value>) -> Result<Response, ApiError> {
    let stream = body
        .get("stream")
        .and_then(|s| s.as_bool())
        .unwrap_or(false);
    let messages = body
        .get("messages")
        .and_then(|m| m.as_array())
        .cloned()
        .ok_or_else(|| bad_request("messages must be an array"))?;

    if stream {
        Ok(openai_stream(&body, &messages))
    } else {
        Ok(Json(openai_json(&body, &messages)).into_response())
    }
}

pub(crate) fn openai_json(body: &Value, messages: &[Value]) -> Value {
    let text = canonical_text(None, messages);
    let resp = run_engine(&text);
    let model = model_of(body);
    let (prompt, completion) = (token_estimate(&text), token_estimate(&resp.text));

    json!({
        "id": format!("chatcmpl-{}", short_id()),
        "object": "chat.completion",
        "created": chrono_now(),
        "model": model,
        "choices": [{
            "index": 0,
            "message": {"role": "assistant", "content": resp.text},
            "finish_reason": "stop",
        }],
        "usage": {
            "prompt_tokens": prompt,
            "completion_tokens": completion,
            "total_tokens": prompt + completion,
        }
    })
}

fn openai_stream(body: &Value, messages: &[Value]) -> Response {
    let text = canonical_text(None, messages);
    let resp: CanonicalResponse = run_engine(&text);
    let model = model_of(body);
    let id = format!("chatcmpl-{}", short_id());
    let created = chrono_now();

    let mut frames: Vec<Value> = ANALYSIS_STAGES
        .iter()
        .map(|stage| {
            chunk(
                &id,
                created,
                &model,
                json!({"content": format!("{stage}\n")}),
            )
        })
        .collect();
    // The conclusion, at last.
    frames.push(chunk(&id, created, &model, json!({"content": resp.text})));
    frames.push(chunk(&id, created, &model, json!({})));
    if let Some(last) = frames.last_mut() {
        last["choices"][0]["finish_reason"] = json!("stop");
    }

    let events = futures::stream::iter(
        frames
            .into_iter()
            .map(|f| Ok::<_, std::convert::Infallible>(Event::default().data(f.to_string())))
            .chain(std::iter::once(Ok(Event::default().data("[DONE]")))),
    );

    Sse::new(events)
        .keep_alive(KeepAlive::default())
        .into_response()
}

fn chunk(id: &str, created: i64, model: &str, delta: Value) -> Value {
    json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": delta,
            "finish_reason": Value::Null,
        }]
    })
}

fn model_of(body: &Value) -> String {
    body.get("model")
        .and_then(|m| m.as_str())
        .unwrap_or("sorry-ai")
        .to_string()
}
