//! Minimal configuration (spec §12). Env vars only, no config framework.

use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub model: String,
    /// MCP transport to enable on startup: none | stdio | http | both
    pub mcp_transport: McpTransport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpTransport {
    None,
    Stdio,
    Http,
    Both,
}

impl std::str::FromStr for McpTransport {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" | "off" => Ok(Self::None),
            "stdio" => Ok(Self::Stdio),
            "http" => Ok(Self::Http),
            "both" | "all" => Ok(Self::Both),
            other => Err(format!(
                "unknown SORRY_MCP_TRANSPORT '{other}' (use none|stdio|http|both)"
            )),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("SORRY_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("SORRY_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            model: std::env::var("SORRY_MODEL").unwrap_or_else(|_| "sorry-ai".into()),
            mcp_transport: std::env::var("SORRY_MCP_TRANSPORT")
                .unwrap_or_else(|_| "http".into())
                .parse()
                .unwrap_or(McpTransport::Http),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        use std::net::IpAddr;
        let ip: IpAddr = self.host.parse().unwrap_or(IpAddr::from([0, 0, 0, 0]));
        SocketAddr::new(ip, self.port)
    }
}
