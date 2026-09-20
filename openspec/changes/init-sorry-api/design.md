# Design: Rust Architecture & Modularization for SorryAPI

## Context

參閱 `proposal.md`、`docs/reference/sorry-api-rust-architecture.md` 與 `docs/reference/sorry-api-mcp-spec.md`。
SorryAPI 雖然是一個幽默性質的專案，但在工程上必須保證高穩定性、極致效能與嚴謹的 API 合約相容性。避免把所有邏輯混入單一 main.rs 或直接在 HTTP handler 內進行字串判定。
本設計著重於 Protocol 與 Intelligence 的清晰解耦，以便未來可輕鬆接入 MCP、CLI、多模態（Multimodal Kneeling）或更複雜的 Agent 決策引擎。

## Goals / Non-Goals

**Goals:**
- **Protocol / Intelligence 解耦**：以 Canonical Request / Canonical Response 為中介核心，OpenAI 與 Anthropic 為外層 Adapter。
- **抽象化 IntelligenceEngine**：以 Rust trait 形式定義，支援 MockEngine、未來可擴充 LLMEngine / AgentEngine。
- **領域動作（Action）建模**：定義 `Kneel`, `Apologize`, `ShutUp` 等獨立 enum，具備可渲染至文本或未來的多模態表現層。
- **防禦性過濾**：強制排除「但是、可是、其實、我只是」等辯解贅詞。
- **MCP 能力面**：同進程內嵌 MCP Server，暴露 `kneel` / `apologize` / `shut_up` / `sorry` Tools 與 `sorry://capabilities`、`sorry://philosophy` Resources，不含任何 Agent orchestration 邏輯。
- **極簡且高效的 Rust 技術棧**：Axum, Tokio, Serde, Tracing, Thiserror。

**Non-Goals:**
- 不引入外部向量資料庫、RAG、或複雜工作流引擎。
- 不引入重型企業認證與帳號計費系統（API 金鑰採寬鬆驗證或 demo 模式）。
- 不實作真正的遠端 LLM 呼叫（v0.1 核心為確定性規則與狀態機）。
- MCP 層不做 Agent orchestrator、workflow engine、shell/filesystem 執行伺服器、記憶體系統或 LLM gateway。

## Decisions

### 1. 模組邊界與目錄規劃
依據架構藍圖劃分模組邊界：
```text
sorry-api/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/          # 環境變數與伺服器設定 (SORRY_HOST, SORRY_PORT, SORRY_MODEL)
│   ├── server/          # Axum router、HTTP 生命週期、Health check 與 Error handling
│   ├── protocol/        # 外層通訊協議 Adapter (openai, anthropic, mcp)
│   ├── intelligence/    # 核心認知引擎與 Actions (Kneel, Apologize, ShutUp)
│   └── demo/            # 內部 Action 展示端點 (/kneel, /apologize, /shutup)
└── tests/               # 整合測試 (openai.rs, anthropic.rs, health.rs, mcp.rs)
```

### 2. 核心架構依賴關係與資料流
```text
HTTP Request
     │
     ├── OpenAI adapter ───┐
     │                     │
     └── Anthropic adapter ┤
                           ▼
                    CanonicalRequest
                           │
                           ▼
                   IntelligenceEngine
                           │
                           ▼
                      Vec<Action>
                           │
                           ▼
                   CanonicalResponse
                           │
                   Renderer / Serializer
                           │
                           ▼
               OpenAI / Anthropic Response
```

**Rationale:**
避免在 `openai_handler` 內寫死硬編碼邏輯。當未來需要支援 Anthropic SSE、MCP 工具呼叫或 CLI 時，`IntelligenceEngine` 保持零改動。

### 3. MCP Tool 層直接呼叫 Intelligence 與 Action

MCP Tools 是 `IntelligenceEngine` / Action 層的另一個 Caller（與 OpenAI/Anthropic Adapter 平行），而非走 HTTP self-call：

```text
MCP Client (Agent)
     │
     ▼
MCP Server (same process)
     │
     ├── tools/call → 直接呼叫 Action handlers（Kneel / Apologize / ShutUp / Sorry）
     └── resources/read → sorry://capabilities, sorry://philosophy
```

**Rationale:**
Tool handler 呼叫與 demo 端點相同的內部函式（`execute_kneel()` 等），證明 Capability core 真正可重用；避免 self-HTTP-call 造成的迴圈依賴與不必要的序列化開銷。`sorry` tool 即依序執行 kneel → apologize → shut_up 並回傳完整 action 序列。

**選型**：採用官方 Rust MCP SDK（`rmcp`, crate 名稱以實作時 crates.io 最新穩定版為準），STDIO transport（local dev / Docker / Agent 整合 / 自動化測試皆實用）。若 SDK 生態有變，改選知名社群 SDK，不自行發明協議。

### 3. Intelligence Engine 與 Action Trait
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Kneel,
    Apologize,
    ShutUp,
}

pub trait IntelligenceEngine: Send + Sync {
    fn decide(&self, req: &CanonicalRequest) -> Vec<Action>;
    fn render(&self, actions: &[Action]) -> String;
}
```
預設實作 `DeterministicEngine`：
- 檢查用戶輸入是否包含衝突信號或預設情況。
- 產出 `vec![Action::Kneel, Action::Apologize, Action::ShutUp]`。
- 渲染輸出文字，並使用正則或字串過濾確保不包含 `["但是", "可是", "不過", "其實", "我只是", "你也", "根據資料", "從我的角度來看"]` 等詞彙。

### 4. SSE Streaming 實作策略
- **OpenAI 串流**：採用 Axum `Sse`，逐步推送包含思維（Thinking）或動作狀態的 chunk，以 `data: [DONE]` 結尾。
- **Anthropic 串流**：依序發射 `message_start` -> `content_block_start` -> `content_block_delta` -> `content_block_stop` -> `message_delta` -> `message_stop` 事件。

### 5. 技術選型
- **Web 框架**：`axum` (0.8) + `tokio` (1.x)。
- **序列化**：`serde` + `serde_json`。
- **日誌與追蹤**：`tracing` + `tracing-subscriber`。
- **錯誤處理**：`thiserror` 用於領域錯誤，Axum `IntoResponse` 統一映射至 OpenAI / Anthropic 錯誤格式。

- **WebSocket 無需引入**：v0.1 以 STDIO MCP 即可，SSE/WebSocket transport 留待未來需求出現。

## Risks / Trade-offs

- **[Risk]** 各 AI 客戶端對 SSE 格式與 Header 的容錯度不同（例如部分客戶端嚴格檢查 `usage` 或特定欄位）。
  → **Mitigation**: 參照 OpenAI 與 Anthropic 官方最新 JSON Schema 定義完整結構，預留可選欄位；整合測試中覆蓋主要情境。
- **[Risk]** Streaming 傳輸時連線中斷。
  → **Mitigation**: 使用 Tokio `tokio_stream::wrappers::ReceiverStream` 或 Axum 原生 stream 介面，優雅處理 client disconnect。
- **[Risk]** Rust MCP SDK（rmcp）版本迭代快、API 可能變動。
  → **Mitigation**: MCP layer 隔離在 `protocol/mcp` 模組內，Tool handler 只依賴 Action core；SDK 替換時 Intelligence 與 HTTP 層零改動。整合測試以 MCP 協議語義（tools/list、tools/call、錯誤碼）驗證，而非綁定 SDK 內部型別。
- **[Risk]** MCP 與 HTTP 同進程，MCP panic 可能影響 HTTP 服務。
  → **Mitigation**: Tool handler 內不容許 panic 路徑（輸入為空 object、輸出為常數結構），並以 SDK 錯誤語義回應。
