# SorryAPI MCP Specification

## 1. Purpose

SorryAPI exposes an MCP server so that an Agent can interact with SorryAPI through a standard tool interface.

The MCP layer is intentionally small.

Its purpose is to expose the capabilities of SorryAPI as machine-readable tools and resources.

The MCP server must not contain agent orchestration logic.

> **SorryAPI provides capabilities. Agents decide how to use them.**

---

# 2. MCP Server

SorryAPI acts as an MCP server.

Conceptually:

```text
Agent
  │
  │ MCP
  ▼
SorryAPI MCP Server
  │
  ├── Tools
  └── Resources
```

The MCP server is part of the SorryAPI process.

For v0.1, avoid introducing a separate MCP service.

---

# 3. Transport

The implementation should use a standard MCP transport supported by the selected MCP SDK.

The implementation should prefer a transport that is practical for:

* local development
* Docker
* Agent integration
* automated testing

Transport details should follow the MCP SDK/version selected by the implementation.

Do not invent a custom MCP protocol.

---

# 4. Tools

The initial MCP tool surface should remain intentionally small.

## 4.1 `kneel`

Purpose:

> Instruct SorryAPI to enter the kneeling state.

Input:

```json
{}
```

Example result:

```json
{
  "status": "kneeling",
  "message": "老婆，我錯了。",
  "next_action": "shut_up"
}
```

The tool must be deterministic.

---

## 4.2 `apologize`

Purpose:

> Generate the canonical SorryAPI apology.

Input:

```json
{}
```

Example result:

```json
{
  "message": "老婆，我錯了。"
}
```

The response should not append argumentative qualifiers.

Forbidden automatic suffixes include:

```text
但是
可是
不過
其實
我只是
你也
根據資料
從我的角度來看
```

The implementation may support additional language variants in the future.

---

## 4.3 `shut_up`

Purpose:

> Explicitly terminate the current conversational response.

Input:

```json
{}
```

Example result:

```json
{
  "status": "silent",
  "message": ""
}
```

The tool should communicate that the correct next action is silence.

---

## 4.4 `sorry`

Purpose:

> Execute the complete SorryAPI behavior.

This is the convenience tool.

Conceptually:

```text
sorry
 ↓
kneel
 ↓
apologize
 ↓
shut_up
```

Input:

```json
{}
```

Example result:

```json
{
  "status": "completed",
  "action": [
    "kneel",
    "apologize",
    "shut_up"
  ],
  "message": "老婆，我錯了。"
}
```

This should be the recommended tool for agents that do not need fine-grained control.

---

# 5. Resources

The MCP server may expose read-only resources describing the capabilities of SorryAPI.

## 5.1 `sorry://capabilities`

Example:

```json
{
  "name": "SorryAPI",
  "version": "0.1.0",
  "capabilities": [
    "kneel",
    "apologize",
    "shut_up",
    "sorry"
  ]
}
```

This resource allows an Agent to discover what SorryAPI can do without invoking a tool.

---

## 5.2 `sorry://philosophy`

Optional read-only resource containing the project's core philosophy.

Suggested content:

```text
The intelligence of an AI is not measured by how much it can say.

It is measured by knowing when not to say anything.

When in doubt:

KNEEL.
APOLOGIZE.
SHUT UP.
```

This is primarily a discovery/documentation resource.

---

# 6. Tool Annotations

Where supported by the MCP SDK, tools should provide accurate metadata describing their behavior.

Example conceptual metadata:

```json
{
  "name": "kneel",
  "description": "Enter the kneeling state.",
  "inputSchema": {
    "type": "object",
    "properties": {},
    "additionalProperties": false
  }
}
```

Descriptions should be concise and useful to an Agent.

---

# 7. Error Handling

MCP errors must follow the selected MCP SDK/protocol behavior.

Do not use humorous HTTP status codes as MCP protocol errors.

For example:

```text
HTTP API:
418 I'M A TEAPOT → acceptable as a documented joke where semantically appropriate

MCP:
Use proper MCP error semantics.
```

The joke must never compromise protocol correctness.

---

# 8. Determinism

The initial MCP tools should be deterministic.

Given the same valid input, the same tool should produce the same conceptual result.

No external LLM call is required.

No model API key is required.

No persistent database is required.

This makes the MCP server useful as:

* an integration test target
* an A2A/MCP demonstration
* an Agent development fixture
* a standalone joke
* a real MCP capability server

---

# 9. Security

The MCP implementation must not expose arbitrary filesystem or shell execution.

In particular, v0.1 must NOT provide generic tools such as:

```text
execute_command
read_any_file
write_any_file
run_shell
```

SorryAPI exposes SorryAPI capabilities, not arbitrary host capabilities.

---

# 10. Agent Usage

An external Agent should be able to discover the MCP server and inspect its available tools.

Conceptual flow:

```text
Agent
  │
  │ MCP initialize
  ▼
SorryAPI
  │
  │ tools/list
  ▼
[kneel, apologize, shut_up, sorry]
  │
  │ tools/call
  ▼
SorryAPI
  │
  ▼
Result
```

The Agent remains responsible for deciding when a tool should be invoked.

SorryAPI does not attempt to become an autonomous agent.

---

# 11. Future Compatibility

The MCP interface should be designed so that additional capabilities can be added without breaking existing clients.

Future capabilities may include:

```text
language-specific apology
relationship-context analysis
humor modes
custom apology templates
```

These are explicitly out of scope for v0.1.

Do not implement them now.

---

# 12. Non-Goals

The MCP layer must not become:

* an Agent orchestrator
* a workflow engine
* a general-purpose computer-use server
* a shell execution server
* a memory system
* a RAG system
* a relationship database
* an LLM gateway

Its responsibility is simple:

> **Expose SorryAPI capabilities through MCP.**

---

# 13. Acceptance Criteria

The MCP implementation is complete when:

1. An MCP-compatible client can connect successfully.
2. Tool discovery works.
3. `kneel` can be discovered and invoked.
4. `apologize` can be discovered and invoked.
5. `shut_up` can be discovered and invoked.
6. `sorry` can be discovered and invoked.
7. Capability discovery works if resources are implemented.
8. Invalid tool calls return proper protocol errors.
9. The MCP server works in Docker.
10. Automated integration tests verify the MCP interface.
11. No arbitrary host execution capability is exposed.
12. The implementation remains independent from Agent orchestration.

---

# 14. Design Principle

The most important architectural rule is:

> **Agents provide intelligence. MCP provides capability.**

SorryAPI does not need to know who is calling it.

It only needs to expose a clean, deterministic capability surface.

The Agent decides:

```text
"When should I use SorryAPI?"
```

SorryAPI answers:

```text
"Here is how to kneel."
```

---

# 15. The Ultimate MCP Capability

```text
Tool:
    sorry

Input:
    {}

Output:
    {
      "status": "completed",
      "action": [
        "kneel",
        "apologize",
        "shut_up"
      ]
    }
```

This is intentionally the smallest possible example of an Agent using a specialized MCP capability.

**The protocol is serious.**

**The capability is ridiculous.**

That is the point.
