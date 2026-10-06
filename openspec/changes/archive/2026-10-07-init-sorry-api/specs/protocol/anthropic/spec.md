# Spec Delta: protocol/anthropic

## Purpose

提供與 Anthropic Messages API 規範高度相容的 HTTP API 介面，支援 header 驗證（可選 demo 金鑰）、system prompt、non-streaming 與 SSE streaming 模式。

## ADDED Requirements

### Requirement: Messages endpoint accepts Anthropic format
系統 MUST 提供 `POST /v1/messages` 端點，接受合法的 Anthropic Messages 請求（包含 `model`, `messages`, `system`, `max_tokens`, `stream`, `temperature` 等欄位）。

#### Scenario: Non-streaming message request
- **WHEN** 用戶端發送 `POST /v1/messages` 且 `stream: false` 或未提供 `stream`
- **THEN** 系統 MUST 回傳 HTTP 200 與標準 Anthropic 格式之 JSON 物件，包含 `id`, `type: "message"`, `role: "assistant"`, `content: [{ type: "text", text: ... }]`, `model`, `stop_reason: "end_turn"`, 與 `usage`。

### Requirement: Messages supports Anthropic SSE streaming
系統 MUST 支援 `stream: true` 參數，以 Anthropic 定義的 event-stream 格式逐步推送事件。

#### Scenario: Streaming message events sequence
- **WHEN** 用戶端發送 `POST /v1/messages` 且 `stream: true`
- **THEN** 系統 MUST 回傳 HTTP 200 與 `Content-Type: text/event-stream`，依序發送 `message_start`, `content_block_start`, `content_block_delta`（包含 text delta）, `content_block_stop`, `message_delta`, 及 `message_stop` 事件。

### Requirement: Anthropic headers compatibility
系統 SHALL 寬鬆處理 `x-api-key` 與 `anthropic-version` 等 header，不因 demo 環境缺乏認證服務而阻擋正常請求。

#### Scenario: Request with custom anthropic headers
- **WHEN** 用戶端帶入 `x-api-key: sorry-demo` 與 `anthropic-version: 2023-06-01`
- **THEN** 系統 MUST 正常解析並處理請求，回傳 HTTP 200。
