//! S3 file handle implementation.

use crate::error::S3Result;

/// S3 file handle.
pub struct S3File {
    _key: String,
}

impl S3File {
    /// Create a new S3 file handle.
    pub fn new(key: String) -> Self {
        Self { _key: key }
    }

    /// Read file as text.
    pub async fn text(&self) -> S3Result<String> {
        // TODO: Implement
        Ok(String::new())
    }

    /// Read file as bytes.
    pub async fn array_buffer(&self) -> S3Result<Vec<u8>> {
        // TODO: Implement
        Ok(Vec::new())
    }
}
