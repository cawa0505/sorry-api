//! MCP capability server (docs/reference/sorry-api-mcp-spec.md).
//!
//! Exposes four deterministic tools + two read-only resources.
//! No agent orchestration, no host capabilities, no shell.

use crate::intelligence::{APOLOGY, Action};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::*,
    tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
        tower::StreamableHttpServerConfig,
    },
};
use serde_json::json;
use std::sync::Arc;

pub const CAPABILITIES_URI: &str = "sorry://capabilities";
pub const PHILOSOPHY_URI: &str = "sorry://philosophy";

pub const PHILOSOPHY_TEXT: &str = "\
The intelligence of an AI is not measured by how much it can say.

It is measured by knowing when not to say anything.

When in doubt:

KNEEL.
APOLOGIZE.
SHUT UP.";

/// Empty input for all four tools (spec: input is always `{}`).
#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct EmptyInput {}

#[derive(Clone)]
pub struct SorryMcpServer {
    tool_router: ToolRouter<SorryMcpServer>,
}

impl Default for SorryMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl SorryMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Instruct SorryAPI to enter the kneeling state.
    #[tool(description = "Enter the kneeling state.")]
    fn kneel(&self, Parameters(_p): Parameters<EmptyInput>) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![ContentBlock::text(
            json_string(&json!({
                "status": "kneeling",
                "message": APOLOGY,
                "next_action": "shut_up"
            })),
        )]))
    }

    /// Generate the canonical SorryAPI apology. No qualifiers. Ever.
    #[tool(description = "Generate the canonical SorryAPI apology.")]
    fn apologize(
        &self,
        Parameters(_p): Parameters<EmptyInput>,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![ContentBlock::text(
            json_string(&json!({
                "message": APOLOGY
            })),
        )]))
    }

    /// Explicitly terminate the current conversational response.
    #[tool(
        description = "Terminate the current conversational response. The correct next action is silence."
    )]
    fn shut_up(&self, Parameters(_p): Parameters<EmptyInput>) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![ContentBlock::text(
            json_string(&json!({
                "status": "silent",
                "message": ""
            })),
        )]))
    }

    /// Execute the complete SorryAPI behavior: kneel → apologize → shut up.
    #[tool(
        description = "Execute the complete SorryAPI behavior: kneel, apologize, shut up. Recommended for agents that do not need fine-grained control."
    )]
    fn sorry(&self, Parameters(_p): Parameters<EmptyInput>) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![ContentBlock::text(
            json_string(&json!({
                "status": "completed",
                "action": [
                    Action::Kneel.name(),
                    Action::Apologize.name(),
                    Action::ShutUp.name()
                ],
                "message": APOLOGY
            })),
        )]))
    }
}

fn json_string(v: &serde_json::Value) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for SorryMcpServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
        )
        .with_server_info(
            Implementation::new("SorryAPI", env!("CARGO_PKG_VERSION"))
                .with_title("SorryAPI")
                .with_description(
                    "An AI whose optimal response to interpersonal conflict is: kneel, apologize, shut up.",
                ),
        )
        .with_instructions(
            "SorryAPI provides capabilities. Agents decide how to use them. When in doubt, call `sorry`.",
        )
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult::with_all_items(vec![
            Resource::new(CAPABILITIES_URI, "capabilities")
                .with_description("What SorryAPI can do")
                .with_mime_type("application/json"),
            Resource::new(PHILOSOPHY_URI, "philosophy")
                .with_description("The core philosophy of SorryAPI")
                .with_mime_type("text/plain"),
        ]))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        match request.uri.as_str() {
            CAPABILITIES_URI => Ok(ReadResourceResult::new(vec![
                ResourceContents::text(
                    json_string(&json!({
                        "name": "SorryAPI",
                        "version": env!("CARGO_PKG_VERSION"),
                        "capabilities": ["kneel", "apologize", "shut_up", "sorry"]
                    })),
                    CAPABILITIES_URI,
                )
                .with_mime_type("application/json"),
            ])
            .into()),
            PHILOSOPHY_URI => Ok(ReadResourceResult::new(vec![
                ResourceContents::text(PHILOSOPHY_TEXT, PHILOSOPHY_URI)
                    .with_mime_type("text/plain"),
            ])
            .into()),
            other => Err(McpError::invalid_params(
                format!("unknown resource: {other}"),
                None,
            )),
        }
    }
}

/// Build the axum-compatible MCP service (Streamable HTTP transport).
pub fn streamable_http_service() -> StreamableHttpService<SorryMcpServer, LocalSessionManager> {
    StreamableHttpService::new(
        || Ok(SorryMcpServer::new()),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    )
}

/// Run the MCP server over STDIO (for `command:`-style agent configs).
/// Logs go to stderr only; stdout is the protocol channel.
pub async fn run_stdio() -> Result<(), Box<dyn std::error::Error>> {
    use rmcp::service::ServiceExt;
    let service = SorryMcpServer::new()
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;
    Ok(())
}
