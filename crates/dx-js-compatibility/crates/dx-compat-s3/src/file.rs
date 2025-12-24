//! S3 file handle implementation.
//!
//! Provides lazy-loading file operations similar to Bun.file().

use crate::error::{S3Error, S3Result};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use std::sync::Arc;

/// S3 file handle for lazy-loading object operations.
///
/// Similar to Bun.file() but for S3 objects.
pub struct S3File {
    client: Arc<Client>,
    bucket: String,
    key: String,
}

impl S3File {
    /// Create a new S3 file handle.
    pub(crate) fn new(client: Arc<Client>, bucket: String, key: String) -> Self {
        Self { client, bucket, key }
    }

    /// Get the object key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the bucket name.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Check if the object exists.
    pub async fn exists(&self) -> S3Result<bool> {
        match self.client.head_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                let service_error = e.into_service_error();
                if service_error.is_not_found() {
                    Ok(false)
                } else {
                    Err(S3Error::Network(service_error.to_string()))
                }
            }
        }
    }

    /// Get object size in bytes.
    pub async fn size(&self) -> S3Result<u64> {
        let response = self.client.head_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .send()
            .await
            .map_err(|e| {
                let service_error = e.into_service_error();
                if service_error.is_not_found() {
                    S3Error::NoSuchKey(self.key.clone())
                } else {
                    S3Error::Network(service_error.to_string())
                }
            })?;

        Ok(response.content_length().unwrap_or(0) as u64)
    }

    /// Get content type.
    pub async fn content_type(&self) -> S3Result<Option<String>> {
        let response = self.client.head_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(response.content_type().map(|s| s.to_string()))
    }

    /// Read file as text (UTF-8).
    pub async fn text(&self) -> S3Result<String> {
        let bytes = self.array_buffer().await?;
        String::from_utf8(bytes)
            .map_err(|e| S3Error::Network(format!("Invalid UTF-8: {}", e)))
    }

    /// Read file as bytes.
    pub async fn array_buffer(&self) -> S3Result<Vec<u8>> {
        let response = self.client.get_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .send()
            .await
            .map_err(|e| {
                let service_error = e.into_service_error();
                if service_error.is_no_such_key() {
                    S3Error::NoSuchKey(self.key.clone())
                } else {
                    S3Error::Network(service_error.to_string())
                }
            })?;

        let bytes = response.body.collect().await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(bytes.to_vec())
    }

    /// Read file as JSON.
    pub async fn json<T: serde::de::DeserializeOwned>(&self) -> S3Result<T> {
        let text = self.text().await?;
        serde_json::from_str(&text)
            .map_err(|e| S3Error::Network(format!("Invalid JSON: {}", e)))
    }

    /// Get a byte stream for streaming reads.
    pub async fn stream(&self) -> S3Result<ByteStream> {
        let response = self.client.get_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(response.body)
    }

    /// Read a range of bytes.
    pub async fn slice(&self, start: u64, end: u64) -> S3Result<Vec<u8>> {
        let range = format!("bytes={}-{}", start, end - 1);

        let response = self.client.get_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .range(range)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        let bytes = response.body.collect().await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(bytes.to_vec())
    }

    /// Write data to the object.
    pub async fn write(&self, data: impl Into<Bytes>) -> S3Result<()> {
        let bytes: Bytes = data.into();
        let body = ByteStream::from(bytes);

        self.client.put_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .body(body)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(())
    }

    /// Write text to the object.
    pub async fn write_text(&self, text: &str) -> S3Result<()> {
        self.write(Bytes::from(text.to_string())).await
    }

    /// Write JSON to the object.
    pub async fn write_json<T: serde::Serialize>(&self, value: &T) -> S3Result<()> {
        let json = serde_json::to_string(value)
            .map_err(|e| S3Error::Network(format!("JSON serialization error: {}", e)))?;
        self.write_text(&json).await
    }

    /// Delete the object.
    pub async fn delete(&self) -> S3Result<()> {
        self.client.delete_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(())
    }

    /// Copy to another key.
    pub async fn copy_to(&self, dest_key: &str) -> S3Result<S3File> {
        let source = format!("{}/{}", self.bucket, self.key);

        self.client.copy_object()
            .bucket(&self.bucket)
            .key(dest_key)
            .copy_source(source)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(S3File::new(
            Arc::clone(&self.client),
            self.bucket.clone(),
            dest_key.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a running S3-compatible service
    // They are marked as ignored by default

    #[test]
    fn test_file_key() {
        // Create a mock client for testing
        // In real tests, we'd use a mock or localstack
    }
}
