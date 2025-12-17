//! Raw Hyper server for maximum throughput
//! 
//! This bypasses Axum's routing overhead for pure throughput measurement.
//! Use this for the final "beat Actix" benchmark.
//!
//! Run with: cargo run --release --bin hyper_raw_server

use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// Pre-computed plaintext response bytes
const PLAINTEXT_BODY: &[u8] = b"Hello, World!";
const PLAINTEXT_CONTENT_LENGTH: &str = "13";

/// Handle incoming request - zero allocation path
async fn handle_request(
    _req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    // Zero-copy response construction
    let response = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .header("content-length", PLAINTEXT_CONTENT_LENGTH)
        .header("server", "dx-server/0.1")
        .body(Full::new(Bytes::from_static(PLAINTEXT_BODY)))
        .unwrap();
    
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Raw Hyper Throughput Server");
    println!("   Target: Maximum RPS (beat Actix ~1.2M RPS)");
    println!("   Listening on: {}", addr);
    println!("");
    println!("   Responding with plain text on all paths");
    println!("");
    println!("Press Ctrl+C to stop...");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("Connection error: {:?}", e);
            }
        });
    }
}
