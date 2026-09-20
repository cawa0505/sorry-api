# Spec Delta: protocol/openai

## Purpose

提供與 OpenAI Chat Completions 規範高度相容的 HTTP API 介面，支援 non-streaming 與 Server-Sent Events (SSE) streaming 模式，使現有 AI 用戶端可無縫接入。

## ADDED Requirements

### Requirement: Chat completions endpoint accepts OpenAI format
系統 MUST 提供 `POST /v1/chat/completions` 端點，接受合法的 OpenAI Chat Completion 請求（包含 `model`, `messages`, `stream`, `temperature`, `max_tokens` 等欄位）。

#### Scenario: Non-streaming chat request
- **WHEN** 用戶端發送 `POST /v1/chat/completions` 且 `stream: false` 或未提供 `stream`
- **THEN** 系統 MUST 回傳 HTTP 200 與標準 OpenAI 格式之 JSON 物件，包含 `id`, `object: "chat.completion"`, `created`, `model`, `choices`（內含 `message: { role: "assistant", content: ... }` 與 `finish_reason: "stop"`），以及 `usage`。

### Requirement: Chat completions supports SSE streaming
系統 MUST 支援 `stream: true` 參數，以 `text/event-stream` 格式逐步推送輸出。

#### Scenario: Streaming chat request
- **WHEN** 用戶端發送 `POST /v1/chat/completions` 且 `stream: true`
- **THEN** 系統 MUST 回傳 HTTP 200 與 `Content-Type: text/event-stream`，每個 chunk 遵循 `data: {...}\n\n` 格式（`object: "chat.completion.chunk"`），並以 `data: [DONE]\n\n` 結束串流。

### Requirement: Error handling conforms to OpenAI format
當請求格式錯誤或缺漏必要欄位時，系統 MUST 回傳 OpenAI 標準錯誤 JSON 結構。

#### Scenario: Missing messages field
- **WHEN** 用戶端發送未包含 `messages` 或 JSON 解析失敗的請求
- **THEN** 系統 MUST 回傳 HTTP 400 與包含 `error: { message: ..., type: "invalid_request_error" }` 的 JSON。
