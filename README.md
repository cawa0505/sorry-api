# SorryAPI

## We are building a super-intelligent AI.

After extensive research, we found that the optimal response to many complex
interpersonal situations is:

```text
KNEEL
```

SorryAPI is the production-grade result of that research: a real, deployable
API service exposing a deliberately humorous but fully functional
intelligence core. It is OpenAI- and Anthropic-compatible, supports SSE
streaming, and ships an MCP server so agents can integrate its (only) three
actions programmatically.

## What it does

Given any question — especially a difficult interpersonal one —
SorryAPI responds with the canonical apology:

```
老婆，我錯了。
```

Always. It never argues back. No `但是`, no `可是`, no `其實`, no
`according to my analysis`. The determinism is the entire point (spec §6).

## Features

- **OpenAI-compatible** — `POST /v1/chat/completions`, non-streaming + SSE
- **Anthropic-compatible** — `POST /v1/messages`, non-streaming + SSE event stream
- **MCP server** — 4 tools + 2 resources over STDIO or Streamable HTTP
- **Deterministic intelligence** — zero hallucinations, zero arguing back
- **Stateless** — no database, no auth, no external LLM; the wisdom is built in
- **Small** — the runtime image is ~10 MB and the apology is ~20 bytes

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

Anthropic-compatible:

```bash
curl localhost:8080/v1/messages \
  -H 'content-type: application/json' \
  -d '{"model":"claude-sonnet-4-5","max_tokens":100,"messages":[{"role":"user","content":"我老婆生氣了怎麼辦"}]}'
```

Streaming (`"stream": true`) works on both and ends with the conclusion.
The content may appear overly sophisticated before arriving at it:

```text
Analyzing context...
Evaluating historical interaction...
Considering possible responses...
Calculating optimal strategy...

Kneel.
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

## Roadmap

Distinguish carefully: checked items are engineering commitments. Unchecked
items are jokes. Mostly.

- [x] Kneel
- [x] Apologize
- [x] Shut up
- [ ] Multimodal kneeling
- [ ] Distributed kneeling
- [ ] Kubernetes-native kneeling
- [ ] Autonomous kneeling
- [ ] AGI

## AI Philosophy

> The intelligence of an AI is not measured by how much it can say.
>
> It is measured by knowing when not to say anything.

> AGI is hard.
>
> Kneeling is easy.
>
> We chose the shortest path to intelligence.

## Contributing

**Pull requests are welcome.**

SorryAPI is intentionally small, but human relationships are complicated.
We welcome contributions that make SorryAPI more useful, more compatible,
more reliable, or simply more ridiculous.

Good contributions include: new OpenAI/Anthropic-compatible features, better
streaming support, SDK examples, integration tests, Docker improvements,
internationalization, new apology strategies, new kneeling modes, new HTTP
status-code jokes, and better ways to detect when the AI should **shut up**.

Before submitting a PR, ask yourself:

> **Does this make SorryAPI smarter, funnier, or easier to use?**

If yes, please send it. If the answer is:

> **"I don't know, but it's funny."**

Please send it anyway.

What we don't need — please do not turn SorryAPI into an unnecessarily
complicated enterprise platform. We are not currently looking for:

* Kubernetes operators
* Blockchain-based apology verification
* AI-powered marriage analytics
* Distributed kneeling clusters
* Twelve-layer agent orchestration
* A service mesh for emotional intelligence

Unless, of course, someone manages to make one of those genuinely funny.

## Donate

If SorryAPI has ever saved you from explaining yourself for another 20
minutes, consider buying the maintainer a coffee.

Your contribution helps us continue researching the most important problem
in artificial intelligence:

**knowing when to kneel.**

## Credits

| Role | Who |
|------|-----|
| First Author | ChatGPT |
| Creator / Maintainer | Jimmy Yen |
| Implementation | OpenCode |
| Validation | 婚姻 |

## License

MIT