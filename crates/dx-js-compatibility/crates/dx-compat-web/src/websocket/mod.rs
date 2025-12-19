//! WebSocket API.

use crate::error::{WebError, WebResult};

/// WebSocket ready state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReadyState {
    /// Connecting
    Connecting = 0,
    /// Open
    Open = 1,
    /// Closing
    Closing = 2,
    /// Closed
    Closed = 3,
}

/// WebSocket message type.
#[derive(Debug, Clone)]
pub enum Message {
    /// Text message
    Text(String),
    /// Binary message
    Binary(Vec<u8>),
}

/// WebSocket client.
pub struct WebSocket {
    _url: String,
    ready_state: ReadyState,
}

impl WebSocket {
    /// Create a new WebSocket connection.
    pub async fn new(url: &str) -> WebResult<Self> {
        // TODO: Implement actual WebSocket connection
        Ok(Self {
            _url: url.to_string(),
            ready_state: ReadyState::Open,
        })
    }

    /// Send a message.
    pub async fn send(&mut self, _message: Message) -> WebResult<()> {
        if self.ready_state != ReadyState::Open {
            return Err(WebError::WebSocket("WebSocket is not open".to_string()));
        }
        // TODO: Implement
        Ok(())
    }

    /// Close the connection.
    pub async fn close(&mut self, _code: Option<u16>, _reason: Option<&str>) {
        self.ready_state = ReadyState::Closed;
    }

    /// Get ready state.
    pub fn ready_state(&self) -> ReadyState {
        self.ready_state
    }
}
