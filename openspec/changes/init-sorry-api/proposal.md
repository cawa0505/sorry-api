# Proposal

## Why

SorryAPI 是一個刻意搞笑但技術完全真實的 AI 相容 API 服務。在複雜的人際溝通（特別是伴侶/家庭衝突）情境中，過多的解釋與辯解往往只會升級矛盾；最可靠且最高智慧的應對方式即是直接「下跪（Kneel）」、「認錯道歉（Apologize）」並「閉嘴（Shut Up）」。

本專案旨在透過嚴謹的 Rust 現代化架構（Axum + Tokio + Serde）實作完整相容 OpenAI Chat Completions 與 Anthropic Messages 協議的 API 伺服器，讓任何標準 AI 用戶端（如 OpenCode、Chatbox、NextChat 等）皆能無縫接入，並以最高規格的工程品質呈現極具幽默感的智慧核心。

## What Changes

- 初始化基於 Rust 2024 / Axum 0.8 的 SorryAPI 服務專案結構。
- 實作 Protocol 與 Intelligence 分離架構：Canonical Request/Response 中介層。
- 實作 OpenAI-compatible API：`POST /v1/chat/completions`（支援 JSON 與 SSE streaming）。
- 實作 Anthropic-compatible API：`POST /v1/messages`（支援 JSON 與 SSE streaming）。
- 實作獨立 Demo 端點：`GET /health`、`POST /kneel`、`POST /apologize`、`POST /shutup`。
- 實作確定性 Intelligence Core：識別衝突信號、觸發 Actions（Kneel, Apologize, ShutUp），嚴格過濾辯解贅詞（如「但是」、「可是」、「其實」）。
- 實作 MCP Server（與 HTTP 同進程）：暴露 `kneel`、`apologize`、`shut_up`、`sorry` 四個確定性 Tools 與 `sorry://capabilities`、`sorry://philosophy` Resources。
- 容器化支援：提供高效能 multi-stage `Dockerfile` 與 `docker-compose.yml`。

## Capabilities

### New Capabilities
- `protocol/openai`: OpenAI Chat Completions API 相容介面，包含 non-streaming 與 streaming SSE 格式。
- `protocol/anthropic`: Anthropic Messages API 相容介面，包含 non-streaming 與 streaming SSE 格式。
- `intelligence/core`: 核心認知與動作引擎（Actions: Kneel, Apologize, ShutUp）與辯解文字過濾機制。
- `server/demo-endpoints`: 基礎健康檢查 `/health` 與內部動作直接演示端點。
- `protocol/mcp`: MCP Server 工具介面（kneel / apologize / shut_up / sorry）與能力探索 Resources，遵循標準 MCP 協議與錯誤語義。

### Modified Capabilities
<!-- 本專案為初始化變更，無既有 capability 需修改 -->

## Impact

- 專案根目錄將建立 Rust workspace / package（`Cargo.toml`、`src/`、`tests/`）。
- 建立對外 HTTP 服務預設監聽 `0.0.0.0:8080`，支援環境變數設定。
- MCP Server 內嵌於同一進程（不拆獨立服務），供 Agent 透過標準 MCP 協議呼叫。
- 提供可由 Docker / Docker Compose 一鍵啟動的完整執行環境。
