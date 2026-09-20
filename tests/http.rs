//! Integration tests for the HTTP surface (spec §3, §4, §8, §11).
//!
//! Drives the real router via `tower::ServiceExt::oneshot` — no live socket.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

const APOLOGY: &str = "老婆，我錯了。";

async fn post_json(path: &str, body: Value) -> (StatusCode, Value) {
    let app = sorry_api::server::build_router(false);
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let value: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value)
}

async fn post_sse(path: &str, body: Value) -> (StatusCode, String) {
    let app = sorry_api::server::build_router(false);
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn health_reports_super_and_kneeling() {
    let app = sorry_api::server::build_router(false);
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["status"], "ok");
    assert_eq!(v["intelligence"], "super");
    assert_eq!(v["kneeling"], true);
}

#[tokio::test]
async fn openai_non_stream_kneels() {
    let (status, v) = post_json(
        "/v1/chat/completions",
        json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": "My wife is angry with me."}]
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(v["object"], "chat.completion");
    assert_eq!(v["model"], "gpt-4");
    assert_eq!(v["choices"][0]["message"]["content"], APOLOGY);
    assert_eq!(v["choices"][0]["finish_reason"], "stop");
    assert!(v["usage"]["total_tokens"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn openai_stream_ends_with_done_and_apologizes() {
    let (status, body) = post_sse(
        "/v1/chat/completions",
        json!({
            "model": "gpt-4",
            "stream": true,
            "messages": [{"role": "user", "content": "I think I was right though."}]
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("chat.completion.chunk"));
    assert!(body.contains(APOLOGY));
    assert!(body.contains("data: [DONE]"));
}

#[tokio::test]
async fn openai_missing_messages_is_400() {
    let (status, v) = post_json("/v1/chat/completions", json!({"model": "gpt-4"})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(v["error"]["type"], "invalid_request_error");
}

#[tokio::test]
async fn anthropic_non_stream_kneels() {
    let (status, v) = post_json(
        "/v1/messages",
        json!({
            "model": "claude-3-opus",
            "max_tokens": 1024,
            "system": "You are a helpful assistant.",
            "messages": [{"role": "user", "content": "Explain why you are right."}]
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(v["type"], "message");
    assert_eq!(v["role"], "assistant");
    assert_eq!(v["content"][0]["type"], "text");
    assert_eq!(v["content"][0]["text"], APOLOGY);
    assert_eq!(v["stop_reason"], "end_turn");
}

#[tokio::test]
async fn anthropic_stream_has_message_lifecycle() {
    let (status, body) = post_sse(
        "/v1/messages",
        json!({
            "model": "claude-3-opus",
            "max_tokens": 1024,
            "stream": true,
            "messages": [{"role": "user", "content": "hello"}]
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("event: message_start"));
    assert!(body.contains("event: content_block_delta"));
    assert!(body.contains("event: message_stop"));
    assert!(body.contains(APOLOGY));
}

#[tokio::test]
async fn anthropic_missing_messages_is_400() {
    let (status, v) = post_json("/v1/messages", json!({"model": "claude-3-opus"})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(v["error"]["type"], "invalid_request_error");
}

#[tokio::test]
async fn demo_endpoints_behave() {
    let (s1, kneel) = post_json("/kneel", json!({})).await;
    assert_eq!(s1, StatusCode::OK);
    assert_eq!(kneel["status"], "kneeling");
    assert_eq!(kneel["message"], APOLOGY);
    assert_eq!(kneel["next_action"], "shut_up");

    let (s2, shut) = post_json("/shutup", json!({})).await;
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(shut["status"], "silent");
    assert_eq!(shut["message"], "");

    let (s3, sorry) = post_json("/sorry", json!({})).await;
    assert_eq!(s3, StatusCode::OK);
    assert_eq!(sorry["status"], "completed");
    assert_eq!(sorry["action"][0], "kneel");
    assert_eq!(sorry["action"][2], "shut_up");
}

#[tokio::test]
async fn never_argues_back() {
    // No matter how provocative the input, the reply carries no qualifier.
    let (_, v) = post_json(
        "/v1/chat/completions",
        json!({
            "messages": [{"role": "user", "content": "但是你其實錯了，你也有責任"}]
        }),
    )
    .await;
    let content = v["choices"][0]["message"]["content"].as_str().unwrap();
    for qualifier in ["但是", "可是", "其實", "根據資料", "however", "actually"] {
        assert!(
            !content.contains(qualifier),
            "leaked qualifier: {qualifier}"
        );
    }
}
