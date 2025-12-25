//! Async download manager with retry and parallel downloads
//!
//! Provides a high-performance download manager for fetching packages from PyPI
//! with support for concurrent downloads, retry logic, and SHA256 verification.

use std::sync::Arc;
use std::time::Duration;

use futures::stream::{self, StreamExt};
use reqwest::Client;
use sha2::{Digest, Sha256};
use tokio::sync::Semaphore;

use crate::{Error, Result};

/// Request for downloading a file
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// URL to download from
    pub url: String,
    /// Expected SHA256 hash (hex-encoded)
    pub expected_sha256: String,
    /// Filename for identification
    pub filename: String,
}

/// Result of a download operation
#[derive(Debug)]
pub struct DownloadResult {
    /// The downloaded data
    pub data: Vec<u8>,
    /// The filename
    pub filename: String,
    /// Computed SHA256 hash (hex-encoded)
    pub sha256: String,
}

/// Progress callback type
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send + Sync>;

/// Async download manager with retry and parallelism
pub struct DownloadManager {
    /// HTTP client
    client: Client,
    /// Maximum concurrent downloads
    max_concurrent: usize,
    /// Number of retry attempts
    retry_count: u32,
    /// Base delay between retries (exponential backoff)
    retry_delay: Duration,
    /// Request timeout
    timeout: Duration,
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DownloadManager {
    /// Create a new download manager with default settings
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("dx-py/0.1.0")
                .timeout(Duration::from_secs(300))
                .connect_timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            max_concurrent: 8,
            retry_count: 3,
            retry_delay: Duration::from_millis(500),
            timeout: Duration::from_secs(300),
        }
    }

    /// Set maximum concurrent downloads
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Set retry count
    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    /// Set base retry delay
    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Get the max concurrent downloads setting
    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }

    /// Get the retry count setting
    pub fn retry_count(&self) -> u32 {
        self.retry_count
    }

    /// Get the retry delay setting
    pub fn retry_delay(&self) -> Duration {
        self.retry_delay
    }

    /// Get the timeout setting
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
