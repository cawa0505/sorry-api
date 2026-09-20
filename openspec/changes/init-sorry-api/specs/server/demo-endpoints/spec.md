# Spec Delta: server/demo-endpoints

## Purpose

提供系統執行狀態檢測（Health Check）以及直接觸發內部動作（Kneel, Apologize, ShutUp）的展示型 HTTP 端點。

## ADDED Requirements

### Requirement: Health check endpoint
系統 MUST 提供 `GET /health` 端點，回傳服務運行狀態及幽默度指標的機器可讀 JSON。

#### Scenario: Query health status
- **WHEN** 發送 `GET /health`
- **THEN** 系統 MUST 回傳 HTTP 200 與 JSON，包含 `status: "ok"`, `intelligence: "super"`, `kneeling: true`。

### Requirement: Standalone action demo endpoints
系統 MUST 提供 `/kneel`, `/apologize`, `/shutup` 等展示端點，供測試與展示內部動作狀態。

#### Scenario: Direct kneel invocation
- **WHEN** 發送 `POST /kneel`
- **THEN** 系統 MUST 回傳 HTTP 200 與 JSON，包含 `status: "kneeling"`, `message: "老婆，我錯了。"`, `next_action: "shut_up"`。

#### Scenario: Direct apologize invocation
- **WHEN** 發送 `POST /apologize`
- **THEN** 系統 MUST 回傳 HTTP 200 與 JSON，包含 `status: "apologizing"`, `message: "對不起，都是我的錯。"`, `next_action: "shut_up"`。

#### Scenario: Direct shutup invocation
- **WHEN** 發送 `POST /shutup`
- **THEN** 系統 MUST 回傳 HTTP 200 與 JSON，包含 `status: "silent"`, `message: "……"`, `next_action: null`。
