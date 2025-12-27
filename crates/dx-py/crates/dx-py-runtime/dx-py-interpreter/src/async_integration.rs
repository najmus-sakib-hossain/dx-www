//! Async Integration for the Interpreter
//!
//! This module wires the interpreter to the async reactor, enabling
//! async/await support in Python code.

use dx_py_reactor::{Reactor, ReactorPool, PyFuture, ReactorStats};
use std::sync::Arc;
use parking_lot::Mutex;

/// Async runtime integration for the interpreter
pub struct AsyncRuntime {
    /// The reactor pool for async I/O
    pool: Option<Arc<ReactorPool>>,
    /// Pending futures
    pending: Mutex<Vec<PendingFuture>>,
    /// Whether async is enabled
    enabled: bool,
    /// Event loop running flag
    running: Mutex<bool>,
}

/// A pending future waiting for completion
struct PendingFuture {
    /// Future ID
    id: u64,
    /// The future object
    future: PyFuture,
    /// Callback to invoke on completion
    callback: Option<Box<dyn FnOnce(FutureResult) + Send>>,
}

/// Result of a future completion
#[derive(Debug)]
pub enum FutureResult {
    /// Successful completion with value
    Ok(Vec<u8>),
    /// Error completion
    Err(AsyncError),
    /// Cancelled
    Cancelled,
}

impl AsyncRuntime {
    /// Create a new async runtime
    pub fn new() -> Self {
        Self {
            pool: None,
            pending: Mutex::new(Vec::new()),
            enabled: false,
            running: Mutex::new(false),
        }
    }
    
    /// Initialize the reactor pool
    pub fn init(&mut self, num_reactors: usize) -> Result<(), AsyncError> {
        if self.pool.is_some() {
            return Err(AsyncError::AlreadyInitialized);
        }
        
        let pool = ReactorPool::new(num_reactors)
            .map_err(|e| AsyncError::InitFailed(e.to_string()))?;
        
        self.pool = Some(Arc::new(pool));
        self.enabled = true;
        Ok(())
    }
    
    /// Check if async is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Start the event loop
    pub fn start(&self) -> Result<(), AsyncError> {
        if !self.enabled {
            return Err(AsyncError::NotInitialized);
        }
        
        let mut running = self.running.lock();
        if *running {
            return Err(AsyncError::AlreadyRunning);
        }
        
        *running = true;
        Ok(())
    }
    
    /// Stop the event loop
    pub fn stop(&self) {
        let mut running = self.running.lock();
        *running = false;
    }
    
    /// Check if event loop is running
    pub fn is_running(&self) -> bool {
        *self.running.lock()
    }
    
    /// Submit an async file read operation
    pub fn read_file(&self, path: &str) -> Result<u64, AsyncError> {
        let pool = self.pool.as_ref()
            .ok_or(AsyncError::NotInitialized)?;
        
        let future = PyFuture::new();
        let id = future.id();
        
        // Submit to reactor pool
        pool.submit_read(path, future.clone())
            .map_err(|e| AsyncError::SubmitFailed(e.to_string()))?;
        
        self.pending.lock().push(PendingFuture {
            id,
            future,
            callback: None,
        });
        
        Ok(id)
    }
    
    /// Submit an async file write operation
    pub fn write_file(&self, path: &str, data: &[u8]) -> Result<u64, AsyncError> {
        let pool = self.pool.as_ref()
            .ok_or(AsyncError::NotInitialized)?;
        
        let future = PyFuture::new();
        let id = future.id();
        
        pool.submit_write(path, data, future.clone())
            .map_err(|e| AsyncError::SubmitFailed(e.to_string()))?;
        
        self.pending.lock().push(PendingFuture {
            id,
            future,
            callback: None,
        });
        
        Ok(id)
    }
    
    /// Poll for completed futures
    pub fn poll(&self) -> Vec<(u64, FutureResult)> {
        let mut pending = self.pending.lock();
        let mut completed = Vec::new();
        
        pending.retain(|pf| {
            if pf.future.is_complete() {
                let result = if pf.future.is_cancelled() {
                    FutureResult::Cancelled
                } else if let Some(err) = pf.future.error() {
                    FutureResult::Err(AsyncError::IoError(err))
                } else {
                    FutureResult::Ok(pf.future.result().unwrap_or_default())
                };
                completed.push((pf.id, result));
                false
            } else {
                true
            }
        });
        
        completed
    }
    
    /// Wait for a specific future to complete
    pub fn wait(&self, future_id: u64) -> Result<FutureResult, AsyncError> {
        loop {
            let completed = self.poll();
            for (id, result) in completed {
                if id == future_id {
                    return Ok(result);
                }
            }
            
            // Small sleep to avoid busy waiting
            std::thread::sleep(std::time::Duration::from_micros(100));
        }
    }
    
    /// Cancel a pending future
    pub fn cancel(&self, future_id: u64) -> bool {
        let pending = self.pending.lock();
        for pf in pending.iter() {
            if pf.id == future_id {
                pf.future.cancel();
                return true;
            }
        }
        false
    }
    
    /// Get the number of pending futures
    pub fn pending_count(&self) -> usize {
        self.pending.lock().len()
    }
    
    /// Get reactor statistics
    pub fn stats(&self) -> Option<ReactorStats> {
        self.pool.as_ref().map(|p| p.stats())
    }
    
    /// Shutdown the async runtime
    pub fn shutdown(&mut self) {
        self.stop();
        
        // Cancel all pending futures
        let pending = self.pending.lock();
        for pf in pending.iter() {
            pf.future.cancel();
        }
        drop(pending);
        
        self.pool = None;
        self.enabled = false;
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AsyncRuntime {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Async runtime errors
#[derive(Debug, thiserror::Error)]
pub enum AsyncError {
    #[error("Async runtime not initialized")]
    NotInitialized,
    
    #[error("Async runtime already initialized")]
    AlreadyInitialized,
    
    #[error("Event loop already running")]
    AlreadyRunning,
    
    #[error("Initialization failed: {0}")]
    InitFailed(String),
    
    #[error("Submit failed: {0}")]
    SubmitFailed(String),
    
    #[error("I/O error: {0}")]
    IoError(String),
    
    #[error("Future not found")]
    FutureNotFound,
    
    #[error("Timeout")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_async_runtime_creation() {
        let runtime = AsyncRuntime::new();
        assert!(!runtime.is_enabled());
        assert!(!runtime.is_running());
    }
    
    #[test]
    fn test_not_initialized_error() {
        let runtime = AsyncRuntime::new();
        assert!(matches!(
            runtime.read_file("test.txt"),
            Err(AsyncError::NotInitialized)
        ));
    }
    
    #[test]
    fn test_pending_count() {
        let runtime = AsyncRuntime::new();
        assert_eq!(runtime.pending_count(), 0);
    }
}
