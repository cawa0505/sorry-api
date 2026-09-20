# Spec Delta: intelligence/core

## Purpose

定義 SorryAPI 的核心智慧決策與動作模型，負責解析輸入意圖、觸發序列動作（Kneel, Apologize, ShutUp），並防禦性杜絕任何辯解或爭論詞彙。

## ADDED Requirements

### Requirement: Action state modeling
系統 MUST 內部明確建模 `Kneel`（下跪）、`Apologize`（道歉認錯）、`ShutUp`（閉嘴保持沉默）等核心動作。

#### Scenario: Conflict trigger sequence
- **WHEN** 接收到衝突情境信號（如「生氣」、「都是你的錯」、「為什麼」）或預設不確定情境
- **THEN** 系統 MUST 依序生成 `Kneel` -> `Apologize` -> `ShutUp` 的動作序列。

### Requirement: Prevention of argumentative words
系統產出之最終文字內容 MUST 嚴格禁止附帶或追加任何轉折反駁詞（包含「但是」、「可是」、「不過」、「其實」、「我只是」、「你也」、「根據資料」、「從我的角度來看」）。

#### Scenario: Attempted argumentative output
- **WHEN** 智慧核心生成道歉回覆
- **THEN** 系統產出的文本內容 MUST 止於認錯（例如「老婆，我錯了。」或「不要解釋。我錯了。」），絕不得包含任何反駁字詞。

### Requirement: Canonical engine abstraction
系統 MUST 將認知決策抽象化為獨立 trait 或介面（如 `IntelligenceEngine`），輸入為協議無關之通用請求（Canonical Request），輸出為通用動作序列或回應（Canonical Response）。

#### Scenario: Decoupled engine evaluation
- **WHEN** 協議轉接層（OpenAI 或 Anthropic）傳入通用訊息結構
- **THEN** 核心引擎 MUST 獨立計算動作序列並返回純粹意圖，不依賴任何特定的 HTTP 協議格式。
