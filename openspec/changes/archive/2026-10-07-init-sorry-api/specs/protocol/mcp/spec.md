# MCP Protocol Specification

## ADDED Requirements

### Requirement: MCP Server In-Process

SorryAPI SHALL embed the MCP server within the same process as the HTTP service. v0.1 SHALL NOT deploy MCP as a separate service.

#### Scenario: MCP server starts with HTTP service

- **WHEN** the SorryAPI service starts
- **THEN** the MCP server is available in the same process
- **AND** no additional service or container is required for MCP functionality

### Requirement: Transport

The MCP server SHALL support both STDIO and Streamable HTTP transports (user decision: option C), switchable via CLI flag or environment variable. STDIO mode SHALL be activated with `--mcp stdio` (exclusive process I/O); Streamable HTTP mode SHALL embed the MCP endpoint into the Axum router at `/mcp` sharing the same port as the HTTP API. Default behavior SHALL start the HTTP API with the Streamable HTTP MCP endpoint enabled. The implementation SHALL NOT invent a custom MCP protocol.

#### Scenario: STDIO transport

- **WHEN** the binary is launched with `--mcp stdio`
- **THEN** the MCP server communicates over standard input/output
- **AND** an MCP client configured with `command: sorry-api` can connect

#### Scenario: Streamable HTTP transport

- **WHEN** the service starts with default configuration (or `--mcp http`)
- **THEN** the MCP endpoint is available at `/mcp` on the same port as the HTTP API
- **AND** an MCP client can connect via Streamable HTTP

#### Scenario: HTTP API and MCP coexist

- **WHEN** both HTTP API and Streamable HTTP MCP are active
- **THEN** they share the same process and port without interference

### Requirement: Tool: kneel

The MCP server SHALL expose a `kneel` tool that instructs SorryAPI to enter the kneeling state. Input SHALL be an empty object `{}`. Output SHALL be `{"status": "kneeling", "message": "老婆，我錯了。", "next_action": "shut_up"}`. The tool SHALL be deterministic.

#### Scenario: kneel invocation

- **WHEN** an Agent calls the `kneel` tool with input `{}`
- **THEN** the result is `{"status": "kneeling", "message": "老婆，我錯了。", "next_action": "shut_up"}`
- **AND** repeated invocations with the same input produce the same result

### Requirement: Tool: apologize

The MCP server SHALL expose an `apologize` tool that generates the canonical SorryAPI apology. Input SHALL be an empty object `{}`. Output SHALL be `{"message": "老婆，我錯了。"}`. The response SHALL NOT append argumentative qualifiers, including（但不限於）：但是、可是、不過、其實、我只是、你也、根據資料、從我的角度來看.

#### Scenario: apologize invocation

- **WHEN** an Agent calls the `apologize` tool with input `{}`
- **THEN** the result is `{"message": "老婆，我錯了。"}`
- **AND** the message contains none of the forbidden argumentative suffixes

### Requirement: Tool: shut_up

The MCP server SHALL expose a `shut_up` tool that explicitly terminates the current conversational response. Input SHALL be an empty object `{}`. Output SHALL be `{"status": "silent", "message": ""}`. The tool SHALL communicate that the correct next action is silence.

#### Scenario: shut_up invocation

- **WHEN** an Agent calls the `shut_up` tool with input `{}`
- **THEN** the result is `{"status": "silent", "message": ""}`

### Requirement: Tool: sorry

The MCP server SHALL expose a `sorry` convenience tool that executes the complete SorryAPI behavior: kneel → apologize → shut_up. Input SHALL be an empty object `{}`. Output SHALL be `{"status": "completed", "action": ["kneel", "apologize", "shut_up"], "message": "老婆，我錯了。"}`. This SHALL be the recommended tool for Agents that do not need fine-grained control.

#### Scenario: sorry invocation

- **WHEN** an Agent calls the `sorry` tool with input `{}`
- **THEN** the result is `{"status": "completed", "action": ["kneel", "apologize", "shut_up"], "message": "老婆，我錯了。"}`
- **AND** the action sequence is exactly `["kneel", "apologize", "shut_up"]` in order

### Requirement: Tool Discovery

The MCP server SHALL support tool discovery (`tools/list`). An external Agent SHALL be able to discover all four tools: `kneel`, `apologize`, `shut_up`, `sorry`.

#### Scenario: tools/list returns all tools

- **WHEN** an MCP client performs initialize and requests `tools/list`
- **THEN** the response contains exactly the tools `kneel`, `apologize`, `shut_up`, `sorry`
- **AND** each tool provides accurate metadata (name, description, inputSchema)

### Requirement: Tool Annotations

Where supported by the MCP SDK, each tool SHALL provide accurate metadata: name, concise description useful to an Agent, and an inputSchema of `{"type": "object", "properties": {}, "additionalProperties": false}`.

#### Scenario: tool metadata present

- **WHEN** tool metadata is inspected via `tools/list`
- **THEN** each tool declares its name, description, and empty-object inputSchema

### Requirement: Resources: capabilities and philosophy

The MCP server SHALL expose the read-only resource `sorry://capabilities` returning `{"name": "SorryAPI", "version": "0.1.0", "capabilities": ["kneel", "apologize", "shut_up", "sorry"]}`. The MCP server MAY expose the read-only resource `sorry://philosophy` containing the project core philosophy text.

#### Scenario: capabilities resource read

- **WHEN** an MCP client reads `sorry://capabilities`
- **THEN** the response contains the SorryAPI name, version, and the full capability list

#### Scenario: philosophy resource read

- **WHEN** the optional `sorry://philosophy` resource is implemented and an MCP client reads it
- **THEN** the response contains the philosophy text (intelligence measured by knowing when not to say anything; KNEEL / APOLOGIZE / SHUT UP)

### Requirement: MCP Error Handling

MCP errors SHALL follow the selected MCP SDK/protocol error semantics. Humorous HTTP status codes (e.g. 418) SHALL NOT be used as MCP protocol errors. The joke SHALL NOT compromise protocol correctness.

#### Scenario: invalid tool call

- **WHEN** an MCP client invokes an unknown tool or supplies invalid arguments
- **THEN** the server returns a proper MCP protocol error (per the SDK/protocol semantics)
- **AND** the error is not a humorous HTTP status code

### Requirement: Determinism

All initial MCP tools SHALL be deterministic: the same valid input produces the same conceptual result. No external LLM call, no model API key, and no persistent database SHALL be required.

#### Scenario: repeated calls produce identical results

- **WHEN** any tool is invoked multiple times with the same valid input
- **THEN** every invocation returns the same conceptual result

### Requirement: Security — No Host Capabilities

The MCP implementation SHALL NOT expose arbitrary filesystem or shell execution. v0.1 SHALL NOT provide generic tools such as `execute_command`, `read_any_file`, `write_any_file`, `run_shell`. The MCP server SHALL only expose SorryAPI capabilities.

#### Scenario: no host execution tool exists

- **WHEN** an MCP client requests `tools/list`
- **THEN** no tool provides filesystem, shell, or arbitrary host execution capability

### Requirement: Agent Decides, MCP Provides

The MCP layer SHALL NOT contain agent orchestration logic. The Agent remains responsible for deciding when a tool is invoked; SorryAPI only exposes a clean, deterministic capability surface. Future capability additions SHALL be possible without breaking existing clients.

#### Scenario: capability surface is non-orchestrating

- **WHEN** the MCP server is inspected
- **THEN** it contains only capability tools/resources and no workflow or orchestration logic
