//! npm Registry Client - Zero Infrastructure Mode
//!
//! Uses npm's free public registry API at registry.npmjs.org
//! No custom infrastructure needed!

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;

pub mod error;
pub use error::{Error, Result};

/// npm registry client
#[derive(Clone)]
pub struct NpmClient {
    client: Client,
    registry_url: String,
    cache_dir: PathBuf,
}

/// Full package metadata from npm registry
#[derive(Debug, Clone, Deserialize)]
pub struct NpmPackageMetadata {
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, NpmVersionInfo>,
    pub time: Option<HashMap<String, String>>,
}

/// Version information
#[derive(Debug, Clone, Deserialize)]
pub struct NpmVersionInfo {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(rename = "devDependencies", default)]
    pub dev_dependencies: HashMap<String, String>,
    #[serde(rename = "peerDependencies", default)]
    pub peer_dependencies: HashMap<String, String>,
    #[serde(rename = "optionalDependencies", default)]
    pub optional_dependencies: HashMap<String, String>,
    pub dist: NpmDist,
    #[serde(default)]
    pub main: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    #[serde(default)]
    pub exports: Option<serde_json::Value>,
}

/// Distribution information (tarball URL and checksums)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmDist {
    pub tarball: String,
    pub shasum: String,
    pub integrity: Option<String>,
    #[serde(rename = "fileCount")]
    pub file_count: Option<u32>,
    #[serde(rename = "unpackedSize")]
    pub unpacked_size: Option<u64>,
}

/// Abbreviated metadata (faster, smaller response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbbreviatedMetadata {
    pub name: String,
    pub modified: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, AbbreviatedVersion>,
}

/// Abbreviated version info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbbreviatedVersion {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    pub dist: NpmDist,
}

impl NpmClient {
    /// Create new npm client with default registry
    pub fn new() -> Self {
        Self::with_registry("https://registry.npmjs.org")
    }

    /// Create npm client with custom registry URL
    pub fn with_registry(url: impl Into<String>) -> Self {
        let client = Client::builder()
            .user_agent("dx-pkg/1.0.0")
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(32)
            .build()
            .expect("Failed to build HTTP client");

        // Setup metadata cache directory
        let cache_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".dx")
            .join("metadata-cache");
        std::fs::create_dir_all(&cache_dir).ok();

        Self {
            client,
            registry_url: url.into(),
            cache_dir,
        }
    }

    /// Fetch full package metadata from registry
    pub async fn get_metadata(&self, name: &str) -> Result<NpmPackageMetadata> {
        let url = format!("{}/{}", self.registry_url, name);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(Error::PackageNotFound(name.to_string()));
        }

        let metadata = response
            .json::<NpmPackageMetadata>()
            .await
            .map_err(|e| Error::ParseError(e.to_string()))?;

        Ok(metadata)
    }

    /// Fetch abbreviated metadata (faster, for resolution)
    /// Uses npm's install-v1 format which is much smaller
    /// NOW WITH DISK CACHE! (24-hour expiration)
    pub async fn get_abbreviated(&self, name: &str) -> Result<AbbreviatedMetadata> {
        // Check cache first
        let cache_path = self.cache_dir.join(format!("{}.json", name.replace('/', "_")));
        
        if let Ok(cached) = self.load_cached_metadata(&cache_path) {
            return Ok(cached);
        }
        
        // Cache miss - fetch from npm
        let url = format!("{}/{}", self.registry_url, name);
        
        let response = self.client
            .get(&url)
            .header("Accept", "application/vnd.npm.install-v1+json")
            .send()
            .await
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(Error::PackageNotFound(name.to_string()));
        }

        let metadata = response
            .json::<AbbreviatedMetadata>()
            .await
            .map_err(|e| Error::ParseError(e.to_string()))?;

        // Save to cache (ignore errors)
        self.save_cached_metadata(&cache_path, &metadata).ok();

        Ok(metadata)
    }

    /// Load cached metadata if it exists and is fresh (< 24 hours)
    fn load_cached_metadata(&self, path: &PathBuf) -> Result<AbbreviatedMetadata> {
        if !path.exists() {
            return Err(Error::PackageNotFound("cache miss".into()));
        }

        // Check age
        let metadata = std::fs::metadata(path)
            .map_err(|_| Error::PackageNotFound("cache read error".into()))?;
        let modified = metadata.modified()
            .map_err(|_| Error::PackageNotFound("cache time error".into()))?;
        let age = SystemTime::now()
            .duration_since(modified)
            .unwrap_or(Duration::from_secs(0));

        // Expire after 24 hours
        if age > Duration::from_secs(24 * 60 * 60) {
            std::fs::remove_file(path).ok();
            return Err(Error::PackageNotFound("cache expired".into()));
        }

        // Load from disk
        let data = std::fs::read_to_string(path)
            .map_err(|_| Error::PackageNotFound("cache read error".into()))?;
        let metadata: AbbreviatedMetadata = serde_json::from_str(&data)
            .map_err(|_| Error::ParseError("cache parse error".into()))?;

        Ok(metadata)
    }

    /// Save metadata to cache
    fn save_cached_metadata(&self, path: &PathBuf, metadata: &AbbreviatedMetadata) -> Result<()> {
        let json = serde_json::to_string(metadata)
            .map_err(|e| Error::ParseError(e.to_string()))?;
        std::fs::write(path, json)
            .map_err(|e| Error::NetworkError(e.to_string()))?;
        Ok(())
    }

    /// Download tarball from npm CDN
    pub async fn download_tarball(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(Error::DownloadFailed(url.to_string()));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        Ok(bytes.to_vec())
    }

    /// Bulk fetch metadata for multiple packages (parallel)
    pub async fn get_metadata_bulk(&self, names: &[&str]) -> Vec<Result<NpmPackageMetadata>> {
        use futures::future::join_all;

        let futures: Vec<_> = names
            .iter()
            .map(|name| self.get_metadata(name))
            .collect();

        join_all(futures).await
    }

    /// Bulk fetch abbreviated metadata (parallel, faster)
    pub async fn get_abbreviated_bulk(&self, names: &[&str]) -> Vec<Result<AbbreviatedMetadata>> {
        use futures::future::join_all;

        let futures: Vec<_> = names
            .iter()
            .map(|name| self.get_abbreviated(name))
            .collect();

        join_all(futures).await
    }

    /// Download multiple tarballs in parallel
    pub async fn download_tarballs_bulk(&self, urls: &[&str]) -> Vec<Result<Vec<u8>>> {
        use futures::future::join_all;

        let futures: Vec<_> = urls
            .iter()
            .map(|url| self.download_tarball(url))
            .collect();

        join_all(futures).await
    }
}

impl Default for NpmClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_lodash_metadata() {
        let client = NpmClient::new();
        let metadata = client.get_metadata("lodash").await.unwrap();
        
        assert_eq!(metadata.name, "lodash");
        assert!(metadata.versions.len() > 0);
        assert!(metadata.dist_tags.contains_key("latest"));
    }

    #[tokio::test]
    async fn test_fetch_abbreviated() {
        let client = NpmClient::new();
        let metadata = client.get_abbreviated("express").await.unwrap();
        
        assert_eq!(metadata.name, "express");
        assert!(metadata.versions.len() > 0);
    }

    #[tokio::test]
    async fn test_bulk_fetch() {
        let client = NpmClient::new();
        let names = vec!["lodash", "express", "react"];
        let results = client.get_abbreviated_bulk(&names).await;
        
        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert!(results[2].is_ok());
    }
}
