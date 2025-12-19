//! S3 client implementation.
//!
//! Provides S3-compatible object storage with support for:
//! - AWS S3, Cloudflare R2, MinIO, and other S3-compatible services
//! - Presigned URL generation
//! - Multipart uploads for large files

use crate::error::{S3Error, S3Result};
use crate::file::S3File;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use std::sync::Arc;
use std::time::Duration;

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
    /// Session token (optional)
    pub session_token: Option<String>,
}

impl S3Config {
    /// Create a new S3 configuration.
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            access_key_id: String::new(),
            secret_access_key: String::new(),
            endpoint: None,
            region: None,
            bucket: bucket.into(),
            session_token: None,
        }
    }

    /// Set credentials.
    pub fn with_credentials(
        mut self,
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
    ) -> Self {
        self.access_key_id = access_key_id.into();
        self.secret_access_key = secret_access_key.into();
        self
    }

    /// Set custom endpoint (for R2, MinIO, etc.).
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Set region.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
}

/// S3 client for object storage operations.
pub struct S3Client {
    client: Client,
    bucket: String,
}

impl S3Client {
    /// Create a new S3 client.
    pub async fn new(config: S3Config) -> S3Result<Self> {
        let credentials = Credentials::new(
            &config.access_key_id,
            &config.secret_access_key,
            config.session_token.clone(),
            None,
            "dx-compat-s3",
        );

        let region = Region::new(config.region.clone().unwrap_or_else(|| "us-east-1".to_string()));

        let mut s3_config_builder = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .credentials_provider(credentials)
            .region(region);

        if let Some(endpoint) = &config.endpoint {
            s3_config_builder = s3_config_builder
                .endpoint_url(endpoint)
                .force_path_style(true);
        }

        let s3_config = s3_config_builder.build();
        let client = Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket: config.bucket,
        })
    }

    /// Get a file handle for the given key.
    pub fn file(&self, key: &str) -> S3File {
        S3File::new(Arc::new(self.client.clone()), self.bucket.clone(), key.to_string())
    }

    /// Check if an object exists.
    pub async fn exists(&self, key: &str) -> S3Result<bool> {
        match self.client.head_object()
            .bucket(&self.bucket)
            .key(key)
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

    /// Delete an object.
    pub async fn delete(&self, key: &str) -> S3Result<()> {
        self.client.delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;
        Ok(())
    }

    /// List objects with optional prefix.
    pub async fn list(&self, prefix: Option<&str>) -> S3Result<Vec<S3ObjectInfo>> {
        let mut request = self.client.list_objects_v2().bucket(&self.bucket);

        if let Some(p) = prefix {
            request = request.prefix(p);
        }

        let response = request.send().await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        let objects = response.contents()
            .iter()
            .map(|obj| S3ObjectInfo {
                key: obj.key().unwrap_or_default().to_string(),
                size: obj.size().unwrap_or(0) as u64,
                last_modified: obj.last_modified()
                    .map(|t| t.secs())
                    .unwrap_or(0),
                etag: obj.e_tag().map(|s| s.to_string()),
            })
            .collect();

        Ok(objects)
    }

    /// Write data to an object.
    pub async fn write(&self, key: &str, data: impl Into<Bytes>) -> S3Result<()> {
        let bytes: Bytes = data.into();
        let body = ByteStream::from(bytes);

        self.client.put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(())
    }

    /// Generate a presigned URL for downloading.
    pub async fn presign_get(&self, key: &str, expires_in: Duration) -> S3Result<String> {
        let presigning_config = PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| S3Error::Config(e.to_string()))?;

        let presigned = self.client.get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(presigned.uri().to_string())
    }

    /// Generate a presigned URL for uploading.
    pub async fn presign_put(&self, key: &str, expires_in: Duration) -> S3Result<String> {
        let presigning_config = PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| S3Error::Config(e.to_string()))?;

        let presigned = self.client.put_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(presigned.uri().to_string())
    }

    /// Start a multipart upload for large files.
    pub async fn create_multipart_upload(&self, key: &str) -> S3Result<MultipartUpload> {
        let response = self.client.create_multipart_upload()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        let upload_id = response.upload_id()
            .ok_or_else(|| S3Error::Network("No upload ID returned".to_string()))?
            .to_string();

        Ok(MultipartUpload {
            client: self.client.clone(),
            bucket: self.bucket.clone(),
            key: key.to_string(),
            upload_id,
            parts: Vec::new(),
        })
    }
}

/// Information about an S3 object.
#[derive(Debug, Clone)]
pub struct S3ObjectInfo {
    /// Object key
    pub key: String,
    /// Object size in bytes
    pub size: u64,
    /// Last modified timestamp (Unix seconds)
    pub last_modified: i64,
    /// ETag (optional)
    pub etag: Option<String>,
}

/// Multipart upload handle.
pub struct MultipartUpload {
    client: Client,
    bucket: String,
    key: String,
    upload_id: String,
    parts: Vec<CompletedPart>,
}

/// Completed part information.
#[derive(Clone)]
struct CompletedPart {
    part_number: i32,
    e_tag: String,
}

impl MultipartUpload {
    /// Upload a part.
    ///
    /// Part numbers must be between 1 and 10,000.
    /// Each part (except the last) must be at least 5MB.
    pub async fn upload_part(&mut self, part_number: i32, data: impl Into<Bytes>) -> S3Result<()> {
        let bytes: Bytes = data.into();
        let body = ByteStream::from(bytes);

        let response = self.client.upload_part()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .part_number(part_number)
            .body(body)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        let e_tag = response.e_tag()
            .ok_or_else(|| S3Error::Network("No ETag returned".to_string()))?
            .to_string();

        self.parts.push(CompletedPart { part_number, e_tag });
        Ok(())
    }

    /// Complete the multipart upload.
    pub async fn complete(self) -> S3Result<()> {
        use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart as AwsCompletedPart};

        let parts: Vec<AwsCompletedPart> = self.parts
            .iter()
            .map(|p| {
                AwsCompletedPart::builder()
                    .part_number(p.part_number)
                    .e_tag(&p.e_tag)
                    .build()
            })
            .collect();

        let completed = CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();

        self.client.complete_multipart_upload()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .multipart_upload(completed)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(())
    }

    /// Abort the multipart upload.
    pub async fn abort(self) -> S3Result<()> {
        self.client.abort_multipart_upload()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .send()
            .await
            .map_err(|e| S3Error::Network(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = S3Config::new("my-bucket")
            .with_credentials("access_key", "secret_key")
            .with_endpoint("http://localhost:9000")
            .with_region("us-west-2");

        assert_eq!(config.bucket, "my-bucket");
        assert_eq!(config.access_key_id, "access_key");
        assert_eq!(config.secret_access_key, "secret_key");
        assert_eq!(config.endpoint, Some("http://localhost:9000".to_string()));
        assert_eq!(config.region, Some("us-west-2".to_string()));
    }
}
