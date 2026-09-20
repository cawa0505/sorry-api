# SorryAPI v0.1

> **We are building a super-intelligent AI.**
>
> After extensive research into human communication, contextual reasoning, emotional ambiguity, and advanced artificial intelligence, we discovered the most reliable response:
>
> **Kneel.**
>
> That's it.

---

## 1. Product Overview

SorryAPI is a deliberately humorous AI-compatible API service.

It presents itself as a highly sophisticated artificial intelligence system capable of understanding human communication and relationship context.

Its core intelligence, however, is intentionally simple:

1. Understand the request.
2. Determine the appropriate response.
3. When in doubt, kneel.
4. Apologize.
5. Shut up.

The humor comes from the contrast between:

* serious AI infrastructure
* serious API compatibility
* serious documentation
* extremely serious architecture

and an intentionally ridiculous intelligence core.

The project must be technically real and usable.

It must not become a joke implementation that cannot actually be integrated with existing AI clients.

---

# 2. Product Principles

### 2.1 Technically real

The API compatibility layer must actually work.

A developer should be able to configure an existing AI client to use SorryAPI as its API endpoint.

### 2.2 Intentionally small

This is an MVP.

Do not introduce unnecessary infrastructure.

Do not build:

* a relationship management platform
* a social network
* a mobile application
* a user management system
* billing
* a vector database
* RAG
* a workflow engine
* Kubernetes operators
* microservices
* unnecessary databases

unless explicitly requested later.

### 2.3 Humor is part of the product

Do not remove or dilute the humor during implementation.

However, humor must not compromise API correctness.

The implementation should be professional.

The product is funny because the implementation is unnecessarily serious about something intentionally simple.

---

# 3. Compatibility

SorryAPI must expose two compatible API surfaces.

## 3.1 OpenAI-compatible API

Implement the commonly used OpenAI-style endpoints required for basic chat usage.

Primary endpoint:

```http
POST /v1/chat/completions
```

Support:

* messages
* model
* temperature
* max_tokens / equivalent limits where appropriate
* stream
* basic usage information

The API should accept requests in the familiar OpenAI Chat Completions format.

Example:

```bash
curl http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "sorry-ai",
    "messages": [
      {
        "role": "user",
        "content": "My wife is angry with me."
      }
    ]
  }'
```

Expected conceptual response:

```text
老婆，對不起。
```

The exact response format must remain compatible with the selected OpenAI API schema.

---

# 4. Anthropic Compatibility

Implement the commonly used Anthropic-style messages endpoint.

Primary endpoint:

```http
POST /v1/messages
```

Support:

* model
* messages
* system
* max_tokens
* temperature where applicable
* stream

Example:

```bash
curl http://localhost:8080/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: sorry-demo" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "sorry-ai",
    "max_tokens": 256,
    "messages": [
      {
        "role": "user",
        "content": "I think I should explain why I was right."
      }
    ]
  }'
```

Expected conceptual response:

```text
不要解釋。

老婆，我錯了。
```

Again, the response must follow the selected Anthropic-compatible schema.

---

# 5. Intelligence Core

The intelligence core for v0.1 should be deterministic or mock-based.

No external LLM is required.

No RAG is required.

No database is required.

The purpose of v0.1 is to prove:

```text
Client
  ↓
OpenAI / Anthropic compatible protocol
  ↓
SorryAPI
  ↓
Intelligence Core
  ↓
Response
```

The intelligence core may use simple rules.

For example:

```text
INPUT
  ↓
Does the request contain relationship-conflict signals?
  ↓
YES
  ↓
APOLOGIZE
  ↓
SHUT UP
```

A minimal implementation is preferred.

---

# 6. Core Behavioral Model

The core behavior is intentionally simple.

### Default behavior

When uncertain:

```text
KNEEL
```

Then:

```text
APOLOGIZE
```

Then:

```text
SHUT_UP
```

The system must avoid adding argumentative language after an apology.

In particular, avoid automatically appending phrases equivalent to:

* 但是
* 可是
* 不過
* 其實
* 我只是
* 你也
* 根據資料
* 從我的角度來看

The fundamental joke is:

> **The smarter the AI becomes, the less it needs to say.**

---

# 7. Internal Actions

The implementation should model the following conceptual actions:

```text
KNEEL
APOLOGIZE
SHUT_UP
```

These may be represented internally as enums, constants, states, or another idiomatic mechanism.

Example:

```text
KNEEL
  ↓
APOLOGIZE
  ↓
SHUT_UP
```

The API does not need to expose these as public endpoints unless doing so improves the demonstration.

---

# 8. Optional Demo Endpoints

A simple demo interface may expose:

```http
POST /kneel
POST /apologize
POST /shutup
GET  /health
```

These endpoints are primarily for demonstration and testing.

Example:

```bash
curl -X POST http://localhost:8080/kneel
```

Possible response:

```json
{
  "status": "kneeling",
  "message": "老婆，我錯了。",
  "next_action": "shut_up"
}
```

---

# 9. HTTP Status Easter Eggs

The project may use humorous HTTP status mappings in documentation or demo output.

Suggested mappings:

```text
200 OK
    跪好

202 ACCEPTED
    「嗯」

204 NO CONTENT
    不想理你

400 BAD REQUEST
    你又開始解釋

409 CONFLICT
    「但是……」

418 I'M A TEAPOT
    還不跪？

500 INTERNAL SERVER ERROR
    你竟然頂嘴
```

These are primarily documentation/demo jokes.

Do not violate HTTP semantics merely for the joke.

---

# 10. Streaming

If streaming is implemented for the compatibility APIs, it must follow the corresponding protocol format correctly.

The content may intentionally appear overly sophisticated before arriving at the final conclusion.

Example conceptual stream:

```text
Analyzing context...
Evaluating historical interaction...
Considering possible responses...
Calculating optimal strategy...

...

Kneel.
```

Do not fake protocol correctness.

The joke should exist inside valid protocol behavior.

---

# 11. Health Check

Provide a simple health endpoint:

```http
GET /health
```

Example:

```json
{
  "status": "ok",
  "intelligence": "super",
  "kneeling": true
}
```

The health endpoint should be machine-readable.

---

# 12. Configuration

Keep configuration minimal.

At minimum support:

* bind address
* port
* model name
* optional log level

Use environment variables where appropriate.

Example:

```text
SORRY_HOST=0.0.0.0
SORRY_PORT=8080
SORRY_MODEL=sorry-ai
LOG_LEVEL=info
```

Do not introduce configuration systems that are unnecessary for the MVP.

---

# 13. Containerization

Provide a Dockerfile.

Prefer a simple production-style container.

If practical, also provide:

```text
docker-compose.yml
```

A developer should be able to run:

```bash
docker compose up --build
```

and immediately access the service.

---

# 14. Testing

Provide automated tests for:

### API

* OpenAI-compatible request
* OpenAI-compatible response
* Anthropic-compatible request
* Anthropic-compatible response
* invalid request
* health endpoint

### Intelligence core

* normal input
* relationship-conflict input
* apology generation
* forbidden argumentative suffix prevention

### Streaming

If streaming is implemented:

* protocol format
* termination
* content delivery

Tests should focus on behavior and compatibility rather than implementation details.

---

# 15. README

The README is an important part of the product.

It should be written as if this is an extremely serious AI infrastructure project.

Opening:

```markdown
# SorryAPI

## We are building a super-intelligent AI.
```

Then explain the research motivation in a completely serious tone.

The central discovery:

> After extensive research, we found that the optimal response to many complex interpersonal situations is:

```text
KNEEL
```

The README should contain:

* product overview
* architecture
* features
* installation
* Docker usage
* OpenAI-compatible API examples
* Anthropic-compatible API examples
* curl examples
* configuration
* development
* testing
* roadmap
* contribution guide
* license
* donation section

The README must remain technically useful despite the humor.

---

# 16. README Roadmap

Include humorous roadmap items such as:

```text
[x] Kneel
[x] Apologize
[x] Shut up
[ ] Multimodal kneeling
[ ] Distributed kneeling
[ ] Kubernetes-native kneeling
[ ] Autonomous kneeling
[ ] AGI
```

The project should clearly distinguish joke roadmap items from actual engineering commitments.

---

# 17. AI Philosophy

Include the following conceptual statement:

> The intelligence of an AI is not measured by how much it can say.
>
> It is measured by knowing when not to say anything.

And:

> AGI is hard.
>
> Kneeling is easy.
>
> We chose the shortest path to intelligence.

---

# 18. Donation

The project may include a donation section.

Suggested copy:

> If SorryAPI has ever saved you from explaining yourself for another 20 minutes, consider buying the maintainer a coffee.
>
> Your contribution helps us continue researching the most important problem in artificial intelligence:
>
> **knowing when to kneel.**

Do not implement payment infrastructure in v0.1.

A placeholder donation link/configuration is sufficient.

---

# 19. Architecture

Prefer a simple architecture:

```text
                 ┌─────────────────────┐
                 │       Clients       │
                 └──────────┬──────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
      OpenAI-compatible          Anthropic-compatible
              │                           │
              └─────────────┬─────────────┘
                            ▼
                   ┌─────────────────────┐
                   │  SorryAPI Core  │
                   └────────┬────────┘
                            ▼
                   ┌─────────────────────┐
                   │ Intelligence    │
                   │ Core            │
                   └────────┬────────┘
                            ▼
                  KNEEL → APOLOGIZE
                            ↓
                        SHUT UP
```

Avoid unnecessary service decomposition.

One service is preferred for v0.1.

---

# 20. Engineering Requirements

The implementation must:

* be idiomatic for the selected language/framework
* have clear project structure
* have automated tests
* handle malformed input safely
* return correct HTTP status codes for actual errors
* produce useful logs
* be easy to run locally
* be easy to run with Docker
* avoid unnecessary dependencies
* avoid hard-coded secrets
* include clear error messages
* include API examples

Do not sacrifice engineering quality for the joke.

---

# 21. Scope Protection

This section is important.

The following are explicitly out of scope for v0.1:

* authentication
* billing
* user accounts
* persistent conversation storage
* RAG
* vector databases
* external LLM providers
* model training
* fine-tuning
* agent orchestration
* relationship analytics
* mobile applications
* browser extensions
* Kubernetes
* distributed systems
* microservices

Unless required to make the compatibility API function correctly.

If a feature appears useful but is not required for the MVP, document it as a future possibility instead of implementing it.

---

# 22. Definition of Done

The v0.1 implementation is complete when:

1. The service starts successfully.
2. `/health` works.
3. OpenAI-compatible chat requests work.
4. Anthropic-compatible message requests work.
5. Streaming works if included in the compatibility contract.
6. Docker execution works.
7. Automated tests pass.
8. README contains complete setup and API examples.
9. The implementation is understandable by another developer.
10. The result is funny without requiring the reader to understand the joke first.

Most importantly:

```text
docker compose up
        ↓
API works
        ↓
AI receives a difficult interpersonal question
        ↓
AI thinks very hard
        ↓
🧎
```

---

# 23. Final Product Statement

SorryAPI is a technically serious experiment in artificial intelligence.

We believe the path toward truly intelligent systems requires sophisticated reasoning, contextual understanding, and emotional intelligence.

We have therefore implemented the most important capability first:

> **Knowing when to kneel.**

---

## Agent Instruction

Build the MVP described in this specification.

Do not expand the scope without explicit approval.

Prefer the simplest correct implementation.

Make the compatibility layers genuinely usable.

Keep the humor in the product and README.

Do not replace the joke with a more complicated system.

**The goal is not to simulate intelligence.**

**The goal is to build something small, real, compatible, and funny.** 

## Contributing

**Pull requests are welcome.**

SorryAPI is intentionally small, but human relationships are complicated.

We welcome contributions that make SorryAPI more useful, more compatible, more reliable, or simply more ridiculous.

### Good contributions

Examples include:

* New OpenAI-compatible API features
* New Anthropic-compatible API features
* Better streaming support
* SDK examples
* Integration tests
* Docker improvements
* Documentation improvements
* Internationalization
* New apology strategies
* New kneeling modes
* New HTTP status-code jokes
* Better ways to detect when the AI should **shut up**

### The most important rule

Before submitting a PR, ask yourself:

> **Does this make SorryAPI smarter, funnier, or easier to use?**

If yes, please send it.

If the answer is:

> **"I don't know, but it's funny."**

Please send it anyway.

### What we don't need

Please do not turn SorryAPI into an unnecessarily complicated enterprise platform.

We are not currently looking for:

* Kubernetes operators
* Blockchain-based apology verification
* AI-powered marriage analytics
* Distributed kneeling clusters
* Twelve-layer agent orchestration
* A service mesh for emotional intelligence

Unless, of course, someone manages to make one of those genuinely funny.

### Development Philosophy

SorryAPI follows a simple principle:

> **Serious engineering. Ridiculous product.**

The API should be correct.

The tests should be real.

The documentation should be useful.

The joke should remain intact.

---

## Community

We believe the best ideas may come from people who have experienced the problem themselves.

If you have ever thought:

> "There should be an API for this."

You are probably in the right place.

Fork it.

Break it.

Improve it.

Send a PR.

And remember:

```text
When in doubt,

        🧎

then open a pull request.
```

Thank you for helping us build the world's most intelligent AI. 
