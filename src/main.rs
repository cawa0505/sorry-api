//! Entry point. One process serves HTTP (+MCP over Streamable HTTP by
//! default); `--mcp-stdio` runs the MCP server over STDIO instead
//! (for `command:`-style agent configs where stdout is the protocol channel).

use sorry_api::config::{Config, McpTransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // STDIO MCP mode: stdout is the protocol channel, so logging goes to stderr.
    let stdio_mode = std::env::args().any(|a| a == "--mcp-stdio");
    let filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    if stdio_mode {
        tracing::info!("SorryAPI MCP (stdio transport) starting");
        return sorry_api::mcp::run_stdio().await;
    }

    let cfg = Config::from_env();
    tracing::info!(
        "SorryAPI starting: model={}, mcp_transport={:?}",
        cfg.model,
        cfg.mcp_transport
    );

    match cfg.mcp_transport {
        McpTransport::None => sorry_api::server::serve(cfg.addr(), false).await?,
        McpTransport::Stdio => {
            // HTTP API runs on the port; MCP is served to an attached agent
            // process via STDIO in a detached task is unreliable (stdin is the
            // HTTP process's), so STDIO mode here means: no HTTP at all.
            sorry_api::mcp::run_stdio().await?
        }
        McpTransport::Http | McpTransport::Both => {
            sorry_api::server::serve(cfg.addr(), true).await?
        }
    }
    Ok(())
}
