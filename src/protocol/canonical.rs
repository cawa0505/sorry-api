//! Canonical input/output types between protocol adapters and the core.

use crate::intelligence::Action;
use serde_json::Value;

/// Normalized user-facing text.
pub struct CanonicalRequest {
    pub text: String,
}

/// Normalized output: ordered actions plus rendered text.
pub struct CanonicalResponse {
    pub actions: Vec<Action>,
    pub text: String,
}

/// Flatten OpenAI- or Anthropic-shaped messages into the text the engine sees.
pub fn canonical_text(system: Option<String>, messages: &[Value]) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(s) = system {
        parts.push(s);
    }
    for m in messages {
        let role = m.get("role").and_then(|r| r.as_str()).unwrap_or("");
        let content = match m.get("content") {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Array(blocks)) => blocks
                .iter()
                .filter_map(|b| {
                    b.get("text")
                        .and_then(|t| t.as_str())
                        .or_else(|| b.get("content").and_then(|c| c.as_str()))
                })
                .collect::<Vec<_>>()
                .join(" "),
            _ => String::new(),
        };
        if matches!(role, "user" | "assistant" | "system") {
            parts.push(content);
        }
    }
    parts.join("\n")
}
