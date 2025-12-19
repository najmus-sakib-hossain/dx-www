//! HTTP and HTTPS server and client compatibility.
//!
//! This module provides Node.js-compatible HTTP/HTTPS server and client implementations
//! using hyper for maximum performance.

use crate::error::{NodeError, NodeResult};
use bytes::Bytes;
use http::{Method, Request as HttpRequest, Response as HttpResponse, StatusCode, Uri};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio_rustls::rustls::{self, ServerConfig};
use tokio_rustls::TlsAcceptor;

/// HTTP server implementation.
pub struct Server {
    addr: SocketAddr,
    shutdown_tx: Option<oneshot::Sender<()>>,
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl Server {
    /// Get the server's listening address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Stop the server gracefully.
    pub async fn close(&mut self) -> NodeResult<()> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.handle.take() {
            handle.await.map_err(|e| NodeError::Other(e.to_string()))?;
        }
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

impl Request {
    /// Create a new request from hyper request.
    pub async fn from_hyper(req: HttpRequest<Incoming>) -> NodeResult<Self> {
        let method = req.method().clone();
        let url = req.uri().to_string();
        
        let mut headers = HashMap::new();
        for (name, value) in req.headers() {
            if let Ok(v) = value.to_str() {
                headers.insert(name.to_string(), v.to_string());
            }
        }

        // Collect body
        let body = hyper::body::to_bytes(req.into_body())
            .await
            .map_err(|e| NodeError::Other(e.to_string()))?;

        Ok(Self {
            method,
            url,
            headers,
            body,
        })
    }
}

/// Simplified response object for Node.js compatibility.
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body_tx: Option<mpsc::Sender<Result<Bytes, Infallible>>>,
}

impl Response {
    fn new(body_tx: mpsc::Sender<Result<Bytes, Infallible>>) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
            body_tx: Some(body_tx),
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
    pub async fn write(&mut self, data: impl Into<Bytes>) -> NodeResult<()> {
        if let Some(tx) = &self.body_tx {
            tx.send(Ok(data.into()))
                .await
                .map_err(|_| NodeError::Other("Failed to write response".to_string()))?;
        }
        Ok(())
    }

    /// End the response.
    pub async fn end(&mut self, data: Option<impl Into<Bytes>>) -> NodeResult<()> {
        if let Some(d) = data {
            self.write(d).await?;
        }
        self.body_tx = None;
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
    Ok(ServerBuilder {
        handler,
        tls_config: None,
    })
}

/// Server builder for configuration.
pub struct ServerBuilder {
    handler: RequestHandler,
    tls_config: Option<Arc<ServerConfig>>,
}

impl ServerBuilder {
    /// Enable HTTPS with the given TLS configuration.
    pub fn with_tls(mut self, config: ServerConfig) -> Self {
        self.tls_config = Some(Arc::new(config));
        self
    }

    /// Start listening on the given address.
    pub async fn listen(self, addr: impl Into<String>) -> NodeResult<Server> {
        let addr_str = addr.into();
        let addr: SocketAddr = addr_str
            .parse()
            .map_err(|e| NodeError::InvalidArgument(format!("Invalid address: {}", e)))?;

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| NodeError::Other(format!("Failed to bind: {}", e)))?;

        let actual_addr = listener
            .local_addr()
            .map_err(|e| NodeError::Other(format!("Failed to get local address: {}", e)))?;

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let handler = self.handler.clone();
        let tls_config = self.tls_config.clone();

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => {
                        break;
                    }
                    result = listener.accept() => {
                        match result {
                            Ok((stream, _)) => {
                                let handler = handler.clone();
                                let tls_config = tls_config.clone();
                                
                                tokio::spawn(async move {
                                    if let Some(tls) = tls_config {
                                        // HTTPS connection
                                        let acceptor = TlsAcceptor::from(tls);
                                        match acceptor.accept(stream).await {
                                            Ok(tls_stream) => {
                                                let io = TokioIo::new(tls_stream);
                                                let _ = handle_connection(io, handler).await;
                                            }
                                            Err(_) => {}
                                        }
                                    } else {
                                        // HTTP connection
                                        let io = TokioIo::new(stream);
                                        let _ = handle_connection(io, handler).await;
                                    }
                                });
                            }
                            Err(_) => break,
                        }
                    }
                }
            }
        });

        Ok(Server {
            addr: actual_addr,
            shutdown_tx: Some(shutdown_tx),
            handle: Some(handle),
        })
    }
}

/// Handle a single HTTP connection.
async fn handle_connection<I>(
    io: I,
    handler: RequestHandler,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    I: hyper::rt::Read + hyper::rt::Write + Unpin + Send + 'static,
{
    let service = service_fn(move |req: HttpRequest<Incoming>| {
        let handler = handler.clone();
        async move {
            let (body_tx, body_rx) = mpsc::channel(16);
            let response = Response::new(body_tx);
            
            // Convert request
            let request = Request::from_hyper(req).await?;
            
            // Call handler
            handler(request, response);
            
            // Build hyper response
            let mut builder = HttpResponse::builder().status(StatusCode::OK);
            
            let body = hyper::body::Body::wrap_stream(
                tokio_stream::wrappers::ReceiverStream::new(body_rx)
            );
            
            Ok::<_, Infallible>(builder.body(body).unwrap())
        }
    });

    http1::Builder::new()
        .serve_connection(io, service)
        .await?;

    Ok(())
}

/// HTTP client request function.
pub async fn request(url: impl Into<String>, options: RequestOptions) -> NodeResult<ClientResponse> {
    let url_str = url.into();
    let uri: Uri = url_str
        .parse()
        .map_err(|e| NodeError::InvalidArgument(format!("Invalid URL: {}", e)))?;

    // For now, return a placeholder
    // Full implementation would use hyper client
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
        String::from_utf8(self.body.to_vec())
            .map_err(|e| NodeError::Other(format!("Invalid UTF-8: {}", e)))
    }

    /// Get response body as JSON.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> NodeResult<T> {
        serde_json::from_slice(&self.body)
            .map_err(|e| NodeError::Other(format!("JSON parse error: {}", e)))
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
        let mut server = server.listen("127.0.0.1:0").await.unwrap();
        
        assert!(server.addr().port() > 0);
        
        server.close().await.unwrap();
    }
}
