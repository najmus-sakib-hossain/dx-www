//! HTTP client with rate limiting and retry logic.
//!
//! Provides a shared HTTP client for all provider implementations.

use crate::USER_AGENT;
use crate::error::{DxError, Result};
use crate::types::RateLimitConfig;
use reqwest::{Client, Response, StatusCode};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// HTTP client with built-in rate limiting and retry logic.
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    rate_limiter: Arc<RateLimiter>,
    max_retries: u32,
    #[allow(dead_code)] // stored for potential future use
    timeout: Duration,
}

impl HttpClient {
    /// Create a new HTTP client with default settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be created.
    pub fn new() -> Result<Self> {
        Self::with_config(RateLimitConfig::default(), 3, Duration::from_secs(30))
    }

    /// Create a new HTTP client with custom configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be created.
    pub fn with_config(
        rate_limit: RateLimitConfig,
        max_retries: u32,
        timeout: Duration,
    ) -> Result<Self> {
        use reqwest::header::{ACCEPT_LANGUAGE, HeaderMap, HeaderValue};
        
        let mut headers = HeaderMap::new();
        // NOTE: Don't set Accept header globally - let each request specify it
        // API providers need application/json while scrapers need text/html
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        
        // PERFORMANCE OPTIMIZATIONS:
        // - pool_max_idle_per_host: Keep 10 connections warm per API host
        // - pool_idle_timeout: Keep connections alive for 30s between requests
        // - tcp_nodelay: Disable Nagle's algorithm for faster small requests
        // - http2_adaptive_window: Optimize HTTP/2 flow control
        // - connection_verbose: Disabled for production
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .timeout(timeout)
            .connect_timeout(Duration::from_secs(5))  // Fast connection or fail
            .pool_max_idle_per_host(10)               // Keep 10 connections warm per host
            .pool_idle_timeout(Duration::from_secs(30)) // Connections stay alive 30s
            .tcp_nodelay(true)                        // Disable Nagle's algorithm
            .gzip(true)
            .brotli(true)
            .build()
            .map_err(|e| DxError::http(e.to_string()))?;

        Ok(Self {
            client,
            rate_limiter: Arc::new(RateLimiter::new(rate_limit)),
            max_retries,
            timeout,
        })
    }

    /// Create a client with a specific rate limit.
    #[must_use]
    pub fn with_rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.rate_limiter = Arc::new(RateLimiter::new(config));
        self
    }

    /// Execute a GET request with rate limiting and retries.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails after all retries.
    pub async fn get(&self, url: &str) -> Result<Response> {
        self.request_with_retry(|| self.client.get(url)).await
    }

    /// Execute a GET request with custom headers.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails after all retries.
    pub async fn get_with_headers(&self, url: &str, headers: &[(&str, &str)]) -> Result<Response> {
        self.request_with_retry(|| {
            let mut req = self.client.get(url);
            for (key, value) in headers {
                req = req.header(*key, *value);
            }
            req
        })
        .await
    }

    /// Execute a GET request with query parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails after all retries.
    pub async fn get_with_query<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        query: &T,
        headers: &[(&str, &str)],
    ) -> Result<Response> {
        self.request_with_retry(|| {
            let mut req = self.client.get(url).query(query);
            for (key, value) in headers {
                // Convert to HeaderName and HeaderValue for proper handling
                if let (Ok(name), Ok(val)) = (
                    reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                    reqwest::header::HeaderValue::from_str(value),
                ) {
                    req = req.header(name, val);
                }
            }
            req
        })
        .await
    }

    /// Execute a raw GET request (no automatic JSON parsing).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails after all retries.
    pub async fn get_raw(&self, url: &str) -> Result<Response> {
        self.request_with_retry(|| self.client.get(url)).await
    }

    /// Execute a POST request with JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails after all retries.
    pub async fn post_json<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<Response> {
        self.request_with_retry(|| self.client.post(url).json(body)).await
    }

    /// Execute a request with rate limiting and retry logic.
    async fn request_with_retry<F>(&self, build_request: F) -> Result<Response>
    where
        F: Fn() -> reqwest::RequestBuilder,
    {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            // Wait for rate limit
            self.rate_limiter.acquire().await;

            let request = build_request();
            debug!(
                "HTTP request attempt {}/{}",
                attempt + 1,
                self.max_retries + 1
            );

            match request.send().await {
                Ok(response) => {
                    let status = response.status();

                    // Handle rate limiting
                    if status == StatusCode::TOO_MANY_REQUESTS {
                        let retry_after = response
                            .headers()
                            .get("retry-after")
                            .and_then(|h| h.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(60);

                        warn!("Rate limited, waiting {} seconds", retry_after);
                        sleep(Duration::from_secs(retry_after)).await;
                        continue;
                    }

                    // Handle server errors with retry
                    if status.is_server_error() && attempt < self.max_retries {
                        let delay = Self::exponential_backoff(attempt);
                        warn!("Server error {}, retrying in {:?}", status.as_u16(), delay);
                        sleep(delay).await;
                        continue;
                    }

                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(e);

                    if attempt < self.max_retries {
                        let delay = Self::exponential_backoff(attempt);
                        warn!("Request failed, retrying in {:?}", delay);
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error
            .map(DxError::from)
            .unwrap_or_else(|| DxError::http("Request failed after all retries")))
    }

    /// Calculate exponential backoff delay.
    fn exponential_backoff(attempt: u32) -> Duration {
        let base_delay_ms = 1000u64; // 1 second
        let delay = base_delay_ms * 2u64.pow(attempt);
        // Simple jitter without external rand crate
        let jitter = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u64)
            % 500;
        Duration::from_millis(delay + jitter)
    }

    /// Get the underlying reqwest client.
    #[must_use]
    pub fn inner(&self) -> &Client {
        &self.client
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RATE LIMITER
// ═══════════════════════════════════════════════════════════════════════════════

/// Simple token bucket rate limiter.
#[derive(Debug)]
struct RateLimiter {
    config: RateLimitConfig,
    last_request: AtomicU64,
}

impl RateLimiter {
    fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            last_request: AtomicU64::new(0),
        }
    }

    async fn acquire(&self) {
        let delay_ms = self.config.delay_ms();
        if delay_ms == 0 {
            return;
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let last = self.last_request.load(Ordering::Relaxed);
        let elapsed = now.saturating_sub(last);

        if elapsed < delay_ms {
            let wait = delay_ms - elapsed;
            sleep(Duration::from_millis(wait)).await;
        }

        self.last_request.store(now, Ordering::Relaxed);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESPONSE HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Extension trait for response handling.
pub trait ResponseExt {
    /// Check if the response indicates success and return body as JSON.
    fn json_or_error<T: serde::de::DeserializeOwned>(
        self,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
}

impl ResponseExt for Response {
    async fn json_or_error<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let status = self.status();

        if !status.is_success() {
            let error_body = self.text().await.unwrap_or_default();
            return Err(DxError::Http {
                message: format!("HTTP {}: {}", status.as_u16(), error_body),
                status_code: Some(status.as_u16()),
                source: None,
            });
        }

        self.json::<T>().await.map_err(|e| DxError::JsonParse {
            message: e.to_string(),
            source: None,
        })
    }
}
