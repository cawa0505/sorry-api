//! Router assembly: health + demo endpoints + protocol surface (+ MCP mount).
//!
//! One binary, one router. No service decomposition (spec §19).

pub mod demo;

use crate::protocol::{anthropic, openai};
use axum::Router;
use axum::routing::{get, post};

pub fn build_router(mount_mcp: bool) -> Router {
    let mut app = Router::new()
        .route("/health", get(crate::server::demo::health))
        .route("/v1/chat/completions", post(openai::chat_completions))
        .route("/v1/messages", post(anthropic::messages))
        .route("/kneel", post(crate::server::demo::kneel))
        .route("/apologize", post(crate::server::demo::apologize))
        .route("/shutup", post(crate::server::demo::shut_up))
        .route("/sorry", post(crate::server::demo::sorry));

    if mount_mcp {
        app = app.route_service("/mcp", crate::mcp::streamable_http_service());
    }
    app
}

/// Serve the HTTP surface. Streaming vs non-streaming is dispatched inside
/// each protocol handler from the request body's `"stream": true`.
pub async fn serve(addr: std::net::SocketAddr, mount_mcp: bool) -> Result<(), std::io::Error> {
    let app = build_router(mount_mcp);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("SorryAPI listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("Shutting down. The AI is already silent.");
}
