//! Bun.file() and Bun.write() file operations.

use crate::error::{BunError, BunResult};
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// BunFile handle.
pub struct BunFile {
    path: PathBuf,
}

impl BunFile {
    /// Create a new file handle.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Read as text.
    pub async fn text(&self) -> BunResult<String> {
        let content = async_fs::read_to_string(&self.path).await?;
        Ok(content)
    }

    /// Parse as JSON.
    pub async fn json<T: serde::de::DeserializeOwned>(&self) -> BunResult<T> {
        let content = self.text().await?;
        serde_json::from_str(&content)
            .map_err(|e| BunError::File(e.to_string()))
    }

    /// Read as bytes.
    pub async fn array_buffer(&self) -> BunResult<Vec<u8>> {
        let content = async_fs::read(&self.path).await?;
        Ok(content)
    }

    /// Get file size.
    pub async fn size(&self) -> BunResult<u64> {
        let metadata = async_fs::metadata(&self.path).await?;
        Ok(metadata.len())
    }

    /// Get MIME type based on extension.
    pub fn type_(&self) -> &str {
        match self.path.extension().and_then(|e| e.to_str()) {
            Some("txt") => "text/plain",
            Some("html") | Some("htm") => "text/html",
            Some("css") => "text/css",
            Some("js") | Some("mjs") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("pdf") => "application/pdf",
            _ => "application/octet-stream",
        }
    }

    /// Slice file (creates a new handle with offset).
    pub fn slice(&self, _start: u64, _end: Option<u64>) -> BunFile {
        // TODO: Implement proper slicing
        BunFile::new(&self.path)
    }
}

/// Write data to file.
pub async fn write(path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> BunResult<usize> {
    let data = data.as_ref();
    async_fs::write(path, data).await?;
    Ok(data.len())
}

/// Create a file handle (Bun.file()).
pub fn file(path: impl AsRef<Path>) -> BunFile {
    BunFile::new(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_write_and_read() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");

        write(&path, b"hello world").await.unwrap();
        
        let file = BunFile::new(&path);
        let content = file.text().await.unwrap();
        
        assert_eq!(content, "hello world");
    }

    #[tokio::test]
    async fn test_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");

        write(&path, r#"{"name": "test"}"#).await.unwrap();
        
        let file = BunFile::new(&path);
        let value: serde_json::Value = file.json().await.unwrap();
        
        assert_eq!(value["name"], "test");
    }
}
