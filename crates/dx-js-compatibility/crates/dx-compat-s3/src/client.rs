//! S3 client implementation.

use crate::error::S3Result;
use crate::file::S3File;

/// S3 client configuration.
#[derive(Debug, Clone)]
pub struct S3Config {
    /// AWS access key ID
    pub access_key_id: String,
    /// AWS secret access key
    pub secret_access_key: String,
    /// Custom endpoint (for R2, MinIO, etc.)
    pub endpoint: Option<String>,
    /// AWS region
    pub region: Option<String>,
    /// Bucket name
    pub bucket: String,
}

/// S3 client.
pub struct S3Client {
    _bucket: String,
}

impl S3Client {
    /// Create a new S3 client.
    pub async fn new(config: S3Config) -> S3Result<Self> {
        Ok(Self {
            _bucket: config.bucket,
        })
    }

    /// Get a file handle.
    pub fn file(&self, key: &str) -> S3File {
        S3File::new(key.to_string())
    }
}
