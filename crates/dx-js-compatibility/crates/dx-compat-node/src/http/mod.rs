//! HTTP server and client compatibility.

use crate::error::NodeResult;

/// HTTP server placeholder.
pub struct Server {
    _port: u16,
}

impl Server {
    /// Create a new HTTP server.
    pub fn new(port: u16) -> Self {
        Self { _port: port }
    }
}

/// Create an HTTP server.
pub fn create_server(_handler: impl Fn()) -> NodeResult<Server> {
    // TODO: Implement using hyper
    Ok(Server::new(3000))
}
