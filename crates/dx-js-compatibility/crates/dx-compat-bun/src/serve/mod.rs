//! Bun.serve() HTTP server.

use crate::error::{BunError, BunResult};
use std::path::PathBuf;

/// Server configuration.
#[derive(Debug)]
pub struct ServeOptions {
    /// Port to listen on
    pub port: u16,
    /// Hostname to bind to
    pub hostname: Option<String>,
    /// Unix socket path
    pub unix: Option<PathBuf>,
}

impl Default for ServeOptions {
    fn default() -> Self {
        Self {
            port: 3000,
            hostname: None,
            unix: None,
        }
    }
}

/// Server handle.
pub struct Server {
    /// Port the server is listening on
    pub port: u16,
    /// Hostname the server is bound to
    pub hostname: String,
}

impl Server {
    /// Stop the server.
    pub async fn stop(&self) {
        // TODO: Implement
    }
}

/// Create an HTTP server.
pub async fn serve(options: ServeOptions) -> BunResult<Server> {
    // TODO: Implement using hyper
    Ok(Server {
        port: options.port,
        hostname: options.hostname.unwrap_or_else(|| "0.0.0.0".to_string()),
    })
}
