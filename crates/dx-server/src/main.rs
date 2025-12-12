//! # dx-server CLI
//!
//! Entry point for the Holographic Server

use dx_server::{serve, ServerState};
use std::net::SocketAddr;
use tracing_subscriber;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("dx_server=debug,tower_http=debug")
        .init();

    // Create server state
    let state = ServerState::new();

    // Parse address
    let addr: SocketAddr = "127.0.0.1:3000".parse()?;

    // Load artifacts from dist-macro (for Day 15 demo)
    if let Err(e) = state.load_artifacts(std::path::Path::new("dist-macro")) {
        tracing::warn!("Failed to load initial artifacts: {}", e);
    }

    // Start server
    serve(addr, state).await?;

    Ok(())
}
