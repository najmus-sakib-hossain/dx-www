//! Resource Manager for DX CLI
//!
//! Provides system resource management including:
//! - Temporary file tracking and cleanup
//! - Child process management with limits
//! - Disk space checking
//! - Graceful shutdown handling
//!
//! Requirements: 9.1, 9.4, 9.5, 9.6, 9.7

use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::Semaphore;

use crate::utils::error::DxError;

// ═══════════════════════════════════════════════════════════════════════════
//  CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// Default maximum number of concurrent child processes
pub const DEFAULT_MAX_PROCESSES: usize = 4;

/// Minimum disk space warning threshold (100MB)
pub const MIN_DISK_SPACE_BYTES: u64 = 100 * 1024 * 1024;

/// Graceful shutdown timeout before SIGKILL (5 seconds)
pub const GRACEFUL_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

// ═══════════════════════════════════════════════════════════════════════════
//  RESOURCE MANAGER
// ═══════════════════════════════════════════════════════════════════════════

/// Manages system resources for the DX CLI
///
/// Tracks temporary files and child processes, ensuring cleanup on exit.
/// Limits concurrent processes to prevent resource exhaustion.
///
/// Requirement 9.1: Limit concurrent processes (default: 4)
/// Requirement 9.4, 9.7: Track temp files for cleanup
/// Requirement 9.5: Terminate child processes gracefully
pub struct ResourceManager {
    /// Tracked temporary files for cleanup
    temp_files: Arc<Mutex<Vec<PathBuf>>>,
    /// Tracked child processes for cleanup
    child_processes: Arc<Mutex<Vec<u32>>>,
    /// Semaphore for limiting concurrent processes
    process_semaphore: Arc<Semaphore>,
    /// Maximum number of concurrent processes
    max_processes: usize,
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_PROCESSES)
    }
}

impl ResourceManager {
    /// Create a new ResourceManager with the specified process limit
    ///
    /// Requirement 9.1: Limit concurrent processes to prevent resource exhaustion
    pub fn new(max_processes: usize) -> Self {
        Self {
            temp_files: Arc::new(Mutex::new(Vec::new())),
            child_processes: Arc::new(Mutex::new(Vec::new())),
            process_semaphore: Arc::new(Semaphore::new(max_processes)),
            max_processes,
        }
    }

    /// Get the maximum number of concurrent processes
    pub fn max_processes(&self) -> usize {
        self.max_processes
    }

    /// Get the number of available process slots
    pub fn available_slots(&self) -> usize {
        self.process_semaphore.available_permits()
    }

    /// Register a temporary file for cleanup
    ///
    /// Requirement 9.4, 9.7: Track all temp files for cleanup
    pub fn register_temp_file(&self, path: PathBuf) {
        if let Ok(mut files) = self.temp_files.lock() {
            files.push(path);
        }
    }

    /// Create and register a temporary file
    ///
    /// Creates a temp file with the given prefix and registers it for cleanup.
    /// Requirement 9.4: Track temp files for cleanup
    pub fn create_temp_file(&self, prefix: &str) -> Result<PathBuf, DxError> {
        let temp_dir = std::env::temp_dir();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        
        let filename = format!("{}_{}.tmp", prefix, timestamp);
        let path = temp_dir.join(filename);
        
        // Create the file
        std::fs::File::create(&path).map_err(|e| DxError::Io {
            message: format!("Failed to create temp file: {}", e),
        })?;
        
        // Register for cleanup
        self.register_temp_file(path.clone());
        
        Ok(path)
    }
