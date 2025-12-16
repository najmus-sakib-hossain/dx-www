//! Plaintext benchmark server for throughput testing
//! 
//! Target: Beat Actix Web's ~1,200,000 RPS
//! 
//! Run with: cargo run --release --bin plaintext_server

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::signal;

/// Plaintext handler - returns static string (no allocation per request)
async fn plaintext() -> &'static str {
    "Hello, World!"
}

/// JSON handler for comparison
async fn json() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"message": "Hello, World!"}))
}

/// Health check endpoint
async fn health() -> &'static str {
    "OK"
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    // Configure for maximum throughput
    let app = Router::new()
        .route("/plaintext", get(plaintext))
        .route("/json", get(json))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    println!("🚀 dx-server Throughput Benchmark Server");
    println!("   Target: Beat Actix Web's 1,200,000 RPS");
    println!("   Listening on: {}", addr);
    println!("");
    println!("   Endpoints:");
    println!("     GET /plaintext  - Plain text response");
    println!("     GET /json       - JSON response");
    println!("     GET /health     - Health check");
    println!("");
    println!("Press Ctrl+C to stop...");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
