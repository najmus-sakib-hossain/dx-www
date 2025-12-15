//! # dx-error — Binary Error Boundaries
//!
//! Isolate component failures without crashing the entire app.
//!
//! ## Features
//! - WASM panic hooks
//! - Component-level isolation
//! - Automatic retry logic
//! - Binary error reporting

use bincode;
use console_error_panic_hook;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Binary protocol opcodes for error handling
pub mod opcodes {
    pub const ERROR_BOUNDARY: u8 = 0xB0;
    pub const ERROR_RECOVER: u8 = 0xB1;
    pub const ERROR_REPORT: u8 = 0xB2;
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
}

/// Component error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentError {
    pub component_id: u16,
    pub error_code: u16,
    pub severity: ErrorSeverity,
    pub message: String,
    pub timestamp: i64,
    pub retry_count: u8,
}

/// Error boundary state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundaryState {
    Normal,
    Failed,
    Recovering,
}

/// Error boundary (tracks component failures)
pub struct ErrorBoundary {
    component_id: u16,
    state: Arc<Mutex<BoundaryState>>,
    error: Arc<Mutex<Option<ComponentError>>>,
    max_retries: u8,
    retry_count: Arc<Mutex<u8>>,
}

impl ErrorBoundary {
    /// Create new error boundary
    pub fn new(component_id: u16, max_retries: u8) -> Self {
        Self {
            component_id,
            state: Arc::new(Mutex::new(BoundaryState::Normal)),
            error: Arc::new(Mutex::new(None)),
            max_retries,
            retry_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Catch error and update state
    pub fn catch_error(&self, error: ComponentError) {
        let mut state = self.state.lock().unwrap();
        *state = BoundaryState::Failed;
        
        let mut error_slot = self.error.lock().unwrap();
        *error_slot = Some(error);
    }

    /// Get current state
    pub fn get_state(&self) -> BoundaryState {
        self.state.lock().unwrap().clone()
    }

    /// Get current error
    pub fn get_error(&self) -> Option<ComponentError> {
        self.error.lock().unwrap().clone()
    }

    /// Attempt recovery
    pub fn recover(&self) -> bool {
        let mut retry_count = self.retry_count.lock().unwrap();
        
        if *retry_count >= self.max_retries {
            return false;
        }
        
        *retry_count += 1;
        
        let mut state = self.state.lock().unwrap();
        *state = BoundaryState::Recovering;
        
        true
    }

    /// Reset boundary on successful recovery
    pub fn reset(&self) {
        let mut state = self.state.lock().unwrap();
        *state = BoundaryState::Normal;
        
        let mut error = self.error.lock().unwrap();
        *error = None;
        
        let mut retry_count = self.retry_count.lock().unwrap();
        *retry_count = 0;
    }

    /// Check if boundary has failed
    pub fn has_failed(&self) -> bool {
        *self.state.lock().unwrap() == BoundaryState::Failed
    }
}

/// Global error boundary registry
pub struct ErrorBoundaryRegistry {
    boundaries: Arc<Mutex<HashMap<u16, ErrorBoundary>>>,
}

impl ErrorBoundaryRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            boundaries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register error boundary
    pub fn register(&self, component_id: u16, max_retries: u8) {
        let mut boundaries = self.boundaries.lock().unwrap();
        boundaries.insert(component_id, ErrorBoundary::new(component_id, max_retries));
    }

    /// Get error boundary
    pub fn get(&self, component_id: u16) -> Option<ErrorBoundary> {
        let boundaries = self.boundaries.lock().unwrap();
        boundaries.get(&component_id).cloned()
    }

    /// Report error for component
    pub fn report_error(&self, error: ComponentError) {
        let boundaries = self.boundaries.lock().unwrap();
        if let Some(boundary) = boundaries.get(&error.component_id) {
            boundary.catch_error(error);
        }
    }

    /// Recover component
    pub fn recover(&self, component_id: u16) -> bool {
        let boundaries = self.boundaries.lock().unwrap();
        if let Some(boundary) = boundaries.get(&component_id) {
            boundary.recover()
        } else {
            false
        }
    }
}

impl Default for ErrorBoundaryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Install panic hook for WASM
#[cfg(target_arch = "wasm32")]
pub fn install_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Binary error encoding
pub mod binary {
    use super::*;

    /// Encode error boundary message
    pub fn encode_error_boundary(component_id: u16, error_code: u16) -> Vec<u8> {
        let mut buf = Vec::with_capacity(5);
        buf.push(opcodes::ERROR_BOUNDARY);
        buf.extend_from_slice(&component_id.to_le_bytes());
        buf.extend_from_slice(&error_code.to_le_bytes());
        buf
    }

    /// Encode recovery message
    pub fn encode_recover(component_id: u16) -> Vec<u8> {
        let mut buf = Vec::with_capacity(3);
        buf.push(opcodes::ERROR_RECOVER);
        buf.extend_from_slice(&component_id.to_le_bytes());
        buf
    }

    /// Encode error report
    pub fn encode_report(error: &ComponentError) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(opcodes::ERROR_REPORT);
        
        // Serialize error to binary
        let error_bytes = bincode::serialize(error).unwrap_or_default();
        buf.extend_from_slice(&(error_bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(&error_bytes);
        
        buf
    }

    /// Decode error report
    pub fn decode_report(data: &[u8]) -> Option<ComponentError> {
        if data.len() < 5 {
            return None;
        }
        
        let len = u32::from_le_bytes(data[1..5].try_into().ok()?) as usize;
        if data.len() < 5 + len {
            return None;
        }
        
        bincode::deserialize(&data[5..5 + len]).ok()
    }
}

/// Fallback UI configuration
#[derive(Debug, Clone)]
pub struct FallbackConfig {
    pub show_error_details: bool,
    pub show_retry_button: bool,
    pub custom_message: Option<String>,
}

impl Default for FallbackConfig {
    fn default() -> Self {
        Self {
            show_error_details: cfg!(debug_assertions),
            show_retry_button: true,
            custom_message: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_boundary() {
        let boundary = ErrorBoundary::new(1, 3);
        
        assert_eq!(boundary.get_state(), BoundaryState::Normal);
        assert!(!boundary.has_failed());
        
        let error = ComponentError {
            component_id: 1,
            error_code: 500,
            severity: ErrorSeverity::Error,
            message: "Test error".to_string(),
            timestamp: 12345,
            retry_count: 0,
        };
        
        boundary.catch_error(error.clone());
        
        assert_eq!(boundary.get_state(), BoundaryState::Failed);
        assert!(boundary.has_failed());
        
        let caught_error = boundary.get_error().unwrap();
        assert_eq!(caught_error.error_code, 500);
    }

    #[test]
    fn test_error_recovery() {
        let boundary = ErrorBoundary::new(1, 3);
        
        let error = ComponentError {
            component_id: 1,
            error_code: 500,
            severity: ErrorSeverity::Error,
            message: "Test error".to_string(),
            timestamp: 12345,
            retry_count: 0,
        };
        
        boundary.catch_error(error);
        
        assert!(boundary.recover());
        assert_eq!(boundary.get_state(), BoundaryState::Recovering);
        
        boundary.reset();
        assert_eq!(boundary.get_state(), BoundaryState::Normal);
    }

    #[test]
    fn test_max_retries() {
        let boundary = ErrorBoundary::new(1, 2);
        
        let error = ComponentError {
            component_id: 1,
            error_code: 500,
            severity: ErrorSeverity::Error,
            message: "Test error".to_string(),
            timestamp: 12345,
            retry_count: 0,
        };
        
        boundary.catch_error(error.clone());
        
        assert!(boundary.recover()); // Retry 1
        boundary.catch_error(error.clone());
        assert!(boundary.recover()); // Retry 2
        boundary.catch_error(error);
        assert!(!boundary.recover()); // Max retries exceeded
    }

    #[test]
    fn test_registry() {
        let registry = ErrorBoundaryRegistry::new();
        
        registry.register(1, 3);
        registry.register(2, 5);
        
        let error = ComponentError {
            component_id: 1,
            error_code: 404,
            severity: ErrorSeverity::Warning,
            message: "Not found".to_string(),
            timestamp: 99999,
            retry_count: 0,
        };
        
        registry.report_error(error);
        
        let boundary = registry.get(1).unwrap();
        assert!(boundary.has_failed());
    }

    #[test]
    fn test_binary_encoding() {
        let error = ComponentError {
            component_id: 42,
            error_code: 500,
            severity: ErrorSeverity::Critical,
            message: "Fatal error".to_string(),
            timestamp: 123456789,
            retry_count: 2,
        };
        
        let encoded = binary::encode_report(&error);
        assert_eq!(encoded[0], opcodes::ERROR_REPORT);
        
        let decoded = binary::decode_report(&encoded).unwrap();
        assert_eq!(decoded.component_id, 42);
        assert_eq!(decoded.error_code, 500);
        assert_eq!(decoded.message, "Fatal error");
    }
}

