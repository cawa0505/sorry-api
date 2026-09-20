# AGENTS.md — SorryAPI

A serious AI infrastructure project whose core insight is: **kneel, apologize,
shut up.** Deterministic OpenAI/Anthropic-compatible API + MCP server.

## Commands

```bash
cargo check        # fast type check
cargo test         # unit + integration (tests/http.rs)
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo build --release
docker compose up --build   # full product at :8080
```

## Architecture (design.md)

```
HTTP → protocol adapter (OpenAI/Anthropic) → canonical request
     → IntelligenceEngine → Vec<Action> → canonical response → renderer
```

- Protocol adapters (`src/protocol/`) hold **zero business rules**. They only
  translate wire ↔ canonical and call `shared::run_engine` — the single seam.
- Intelligence (`src/intelligence/`) is deterministic: `RuleEngine` always
  returns `[Kneel, Apologize, ShutUp]`. Renders `老婆，我錯了。`, terminates
  at ShutUp, strips forbidden qualifiers (但是/可是/其實/according to…).
- MCP (`src/mcp/`) exposes `kneel`/`apologize`/`shut_up`/`sorry` + two
  resources. No orchestration, no host tools, no shell. Agent decides, MCP
  provides capability.
- Never add qualifiers after the apology. Never fake protocol errors with
  joke HTTP statuses (418 is a doc joke only).

## Constraints (spec v0.1)

- No auth/billing/persistence/RAG/vector DB/external LLM unless required for
  compatibility. MVP scope protection is explicit in the spec.
- Config via env: `SORRY_HOST`, `SORRY_PORT`, `SORRY_MODEL`,
  `SORRY_MCP_TRANSPORT` (none|stdio|http|both).

## Verification

DoD for any change: `cargo test` + `cargo clippy` green, and if HTTP-touching
code changed, boot the server and curl `/health` + one completion.

Specs: `openspec/changes/init-sorry-api/` (proposal, 5 capability specs, design,
tasks). Reference docs: `docs/reference/`.