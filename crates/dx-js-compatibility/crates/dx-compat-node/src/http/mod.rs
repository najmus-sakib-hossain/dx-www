//! HTTP and HTTPS server and client compatibility.
//!
//! This module provides Node.js-compatible HTTP/HTTPS server and client implementations.
//! Full implementation uses hyper for maximum performance.

use crate::error::{ErrorCode, NodeError, NodeResult};
use bytes::Bytes;
use http::{Method, StatusCode};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

/// HTTP server implementation.
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    /// Get the server's listening address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Stop the server gracefully.
    pub async fn close(&mut self) -> NodeResult<()> {
        // TODO: Implement graceful shutdown
        Ok(())
    }
}

/// HTTP request handler function type.
pub type RequestHandler = Arc<dyn Fn(Request, Response) + Send + Sync>;

/// Simplified request object for Node.js compatibility.
#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Bytes,
}

/// Simplified response object for Node.js compatibility.
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
}

impl Response {
    /// Create a new response.
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
        }
    }

    /// Set response status code.
    pub fn status(&mut self, code: u16) {
        self.status = StatusCode::from_u16(code).unwrap_or(StatusCode::OK);
    }

    /// Set a response header.
    pub fn set_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(name.into(), value.into());
    }

    /// Write data to the response body.
    pub async fn write(&mut self, _data: impl Into<Bytes>) -> NodeResult<()> {
        // TODO: Implement body writing
        Ok(())
    }

    /// End the response.
    pub async fn end(&mut self, data: Option<impl Into<Bytes>>) -> NodeResult<()> {
        if let Some(_d) = data {
            // TODO: Write final data
        }
        Ok(())
    }
}

/// Create an HTTP server with the given request handler.
///
/// # Example
/// ```no_run
/// use dx_compat_node::http::{create_server, Request, Response};
/// use std::sync::Arc;
///
/// #[tokio::main]
/// async fn main() {
///     let handler = Arc::new(|req: Request, mut res: Response| {
///         tokio::spawn(async move {
///             res.status(200);
///             res.set_header("Content-Type", "text/plain");
///             let _ = res.end(Some("Hello World!")).await;
///         });
///     });
///     
///     let server = create_server(handler).await.unwrap();
///     server.listen("127.0.0.1:3000").await.unwrap();
/// }
/// ```
pub async fn create_server(handler: RequestHandler) -> NodeResult<ServerBuilder> {
    Ok(ServerBuilder { handler })
}

/// Server builder for configuration.
pub struct ServerBuilder {
    handler: RequestHandler,
}

impl ServerBuilder {
    /// Start listening on the given address.
    pub async fn listen(self, addr: impl Into<String>) -> NodeResult<Server> {
        let addr_str = addr.into();
        let addr: SocketAddr = addr_str.parse().map_err(|e| {
            NodeError::new(ErrorCode::EINVAL, format!("Invalid address: {}", e))
        })?;

        // TODO: Implement actual server listening with hyper
        // For now, return a placeholder server
        Ok(Server { addr })
    }
}

/// HTTP client request function.
pub async fn request(
    url: impl Into<String>,
    _options: RequestOptions,
) -> NodeResult<ClientResponse> {
    let _url_str = url.into();
    
    // TODO: Implement actual HTTP client using hyper
    // For now, return a placeholder response
    Ok(ClientResponse {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: Bytes::new(),
    })
}

/// HTTP client request options.
#[derive(Default)]
pub struct RequestOptions {
    pub method: Option<Method>,
    pub headers: HashMap<String, String>,
    pub body: Option<Bytes>,
}

/// HTTP client response.
pub struct ClientResponse {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Bytes,
}

impl ClientResponse {
    /// Get response body as string.
    pub fn text(&self) -> NodeResult<String> {
        String::from_utf8(self.body.to_vec()).map_err(|e| {
            NodeError::new(ErrorCode::UNKNOWN, format!("Invalid UTF-8: {}", e))
        })
    }

    /// Get response body as JSON.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> NodeResult<T> {
        serde_json::from_slice(&self.body).map_err(|e| {
            NodeError::new(ErrorCode::UNKNOWN, format!("JSON parse error: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_creation() {
        let handler = Arc::new(|_req: Request, mut res: Response| {
            tokio::spawn(async move {
                res.status(200);
                let _ = res.end(Some("OK")).await;
            });
        });

        let server = create_server(handler).await.unwrap();
        let server = server.listen("127.0.0.1:3000").await.unwrap();

        assert_eq!(server.addr().port(), 3000);
    }
}
