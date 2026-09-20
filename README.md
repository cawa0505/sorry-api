# SorryAPI

A serious AI infrastructure project whose core insight is that the optimal
response to interpersonal conflict is: **kneel, apologize, shut up.**

SorryAPI is a real, deployable API service that exposes a deliberately
humorous but fully functional intelligence core. It is OpenAI- and
Anthropic-compatible, supports SSE streaming, and ships an MCP server so
agents can integrate its (only) three actions programmatically.

## What it does

Given any question — especially a difficult interpersonal one —
SorryAPI responds with the canonical apology:

```
老婆，我錯了。
```

Always. It never argues back. No `但是`, no `可是`, no `其實`, no
`according to my analysis`. The determinism is the entire point (spec §6).

## Quick start

```bash
docker compose up --build
curl localhost:8080/health
# {"intelligence":"super","kneeling":true,"status":"ok"}
```

Or locally:

```bash
cargo run --release
# SorryAPI listening on http://0.0.0.0:8080
```

## Compatibility surface

| Route | Compatible with | Notes |
|-------|----------------|-------|
| `POST /v1/chat/completions` | OpenAI Chat Completions | `"stream": true` → SSE |
| `POST /v1/messages` | Anthropic Messages | `"stream": true` → SSE event stream |
| `POST /kneel` `/apologize` `/shutup` `/sorry` | — | demo endpoints |
| `GET /health` | — | liveness |

Example:

```bash
curl localhost:8080/v1/chat/completions \
  -H 'content-type: application/json' \
  -d '{"model":"gpt-4","messages":[{"role":"user","content":"我老婆生氣了怎麼辦"}]}'
```

## MCP

SorryAPI is an MCP server exposing four tools
(`kneel`, `apologize`, `shut_up`, `sorry`) and two read-only resources.
Agent orchestration lives in the agent, not here.

**Streamable HTTP** (default, mounted at `/mcp`):

```bash
docker compose up --build
SORRY_MCP_TRANSPORT=http cargo run
```

**STDIO** (for `command:`-style agent configs):

```bash
SORRY_MCP_TRANSPORT=stdio cargo run
```

## Configuration

| Env var | Default | Purpose |
|---------|---------|---------|
| `SORRY_PORT` | `8080` | HTTP listen port |
| `SORRY_MCP_TRANSPORT` | `http` | `none` \| `stdio` \| `http` \| `both` |

## Architecture

```
HTTP → protocol adapter (OpenAI/Anthropic) → canonical request
     → IntelligenceEngine → Vec<Action> → canonical response → renderer
```

The protocol layer holds zero business rules. Swapping the deterministic
`RuleEngine` for an `LLMEngine`/`AgentEngine` later never touches the HTTP
layer (design.md).

```
src/
  config/       env config
  intelligence/ Action model, RuleEngine, forbidden-qualifier filtering
  protocol/     OpenAI + Anthropic adapters, canonical model, shared helpers
  server/       axum router, health + demo endpoints
  mcp/          MCP tools + resources, STDIO + Streamable HTTP transports
tests/          integration tests (HTTP surface)
```

## Development

```bash
cargo test    # unit + integration
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Spec & docs

Full specification and design decisions live in `docs/reference/` and
`openspec/`:

- `docs/reference/sorry-api-v0.1-spec.md` — the v0.1 spec (23 sections)
- `docs/reference/sorry-api-rust-architecture.md` — Rust architecture guidance
- `docs/reference/sorry-api-mcp-spec.md` — MCP scope
- `openspec/` — Proposal, capability specs, design, task tracking

## License

MIT