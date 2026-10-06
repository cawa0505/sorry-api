# Tasks

> Status: all complete. HTTP integration tests consolidated into `tests/http.rs`;
> MCP verification (7.4) done via live stdio + Streamable HTTP smoke tests
> (initialize, tools/list, resources/list, tools/call sorry, session-id handshake).

## 1. 專案骨架與設定 (Project Skeleton)

- [x] 1.1 建立 Cargo.toml 與基本 src/lib.rs/main.rs，驗證 `cargo check` 可通過
- [x] 1.2 實作 config/mod.rs（載入 SORRY_HOST, SORRY_PORT, SORRY_MODEL 環境變數），驗證預設值正確載入
- [x] 1.3 建立 src/server/error.rs 與 src/server/mod.rs，驗證 HTTP error 可正確映射為 OpenAI / Anthropic 錯誤格式

## 2. 核心智慧引擎 (Intelligence Core)

- [x] 2.1 建立 src/intelligence/actions.rs（定義 `Action` enum: Kneel, Apologize, ShutUp），驗證 enum 可序列化
- [x] 2.2 建立 src/intelligence/engine.rs（實作 `IntelligenceEngine` trait 及 DeterministicEngine），驗證 conflict 訊號可觸發正確 action 序列
- [x] 2.3 實作辯解贅詞過濾機制（屏蔽「但是」「可是」「其實」等詞彙），驗證過濾後不包含任何反駁字詞
- [x] 2.4 建立 src/intelligence/mod.rs 匯出，驗證編譯通過

## 3. Protocol 層 (API Compatibility)

- [x] 3.1 建立 src/protocol/openai.rs（實作 OpenAI Chat Completions non-streaming），驗證 curl 可取得正確 JSON 回應
- [x] 3.2 建立 OpenAI SSE streaming 支援，驗證 `data: [DONE]` 結尾且 chunk 格式正確
- [x] 3.3 建立 src/protocol/anthropic.rs（實作 Anthropic Messages non-streaming），驗證 curl 可取得正確 JSON 回應
- [x] 3.4 建立 Anthropic SSE streaming 支援，驗證 event-stream 事件順序（message_start -> content_block_delta -> message_stop）
- [x] 3.5 建立 src/protocol/mod.rs 匯出，驗證編譯通過

## 4. MCP 層 (MCP Capability Server)

- [x] 4.1 加入 MCP SDK 依賴（官方 rmcp crate，STDIO + Streamable HTTP transport features），驗證 `cargo check` 通過
- [x] 4.2 實作 tools：`kneel` / `apologize` / `shut_up` / `sorry`（直接呼叫 Action core，不 self-HTTP-call），驗證各 tool 回傳值與 MCP 規格一致且確定性成立
- [x] 4.3 實作 tool metadata（name、description、inputSchema），驗證 `tools/list` 可完整探索四個 tools
- [x] 4.4 實作 resources：`sorry://capabilities` 與 `sorry://philosophy`，驗證 `resources/list` 回傳正確內容
- [x] 4.5 錯誤處理：未知 tool / 無效參數回傳標準 MCP 錯誤語義（不得使用搞笑 HTTP status 作為 MCP 錯誤）
- [x] 4.6 實作雙 transport：STDIO 與 Streamable HTTP（內嵌 Axum router `/mcp` endpoint，與 HTTP API 同 port），驗證兩種模式皆可被 MCP client 連線

## 5. Demo 端點與路由 (Server & Demo)

- [x] 5.1 建立 demo 端點實作（與 MCP tools 共用同一組 Action core），驗證 /kneel, /apologize, /shutup 回傳正確 JSON
- [x] 5.2 建立 server health，驗證 GET /health 回傳 `{"status": "ok", "intelligence": "super", "kneeling": true}`
- [x] 5.3 在 main.rs 整合 HTTP 路由與 MCP server（同進程啟動），驗證兩者並行運作互不干擾

## 6. 容器化與 CI (Docker & CI)

- [x] 6.1 建立 Dockerfile（multi-stage build），驗證 `docker compose up --build` 可啟動服務
- [x] 6.2 建立 docker-compose.yml，驗證健康檢查端點可達
- [x] 6.3 建立 .github/workflows/ci.yml（fmt + clippy + test + release build + docker build），驗證 CI 可運行

## 7. 測試 (Tests)

- [x] 7.1 建立 openai 整合測試（non-streaming, streaming, invalid request），驗證所有測試通過 `cargo test`
- [x] 7.2 建立 anthropic 整合測試（non-streaming, streaming），驗證所有測試通過 `cargo test`
- [x] 7.3 建立 health + demo 端點測試，驗證所有測試通過 `cargo test`
- [x] 7.4 建立 MCP 驗證（initialize、tools/list 四工具齊全、tools/call 結果正確、determinism、無 host 能力工具），驗證通過
