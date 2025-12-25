//! PyPI registry client
//!
//! Provides HTTP client for fetching package metadata and downloading packages
//! from PyPI and compatible registries.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Error, Result};

/// PyPI JSON API response for package metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PyPiPackageInfo {
    /// Package information
    pub info: PackageInfo,
    /// Available releases (version -> files)
    pub releases: HashMap<String, Vec<ReleaseFile>>,
    /// URLs for the latest version
    pub urls: Vec<ReleaseFile>,
}

/// Package metadata from PyPI
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageInfo {
    /// Package name
    pub name: String,
    /// Latest version
    pub version: String,
    /// Package summary/description
    pub summary: Option<String>,
    /// Author name
    pub author: Option<String>,
    /// Author email
    pub author_email: Option<String>,
    /// License
    pub license: Option<String>,
    /// Project homepage
    pub home_page: Option<String>,
    /// Required Python version
    pub requires_python: Option<String>,
    /// Package dependencies
    pub requires_dist: Option<Vec<String>>,
}

/// Release file information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseFile {
    /// Filename
    pub filename: String,
    /// Download URL
    pub url: String,
    /// File size in bytes
    pub size: u64,
    /// Package type (sdist, bdist_wheel, etc.)
    pub packagetype: String,
    /// Python version requirement
    pub python_version: Option<String>,
    /// SHA256 digest
    pub digests: FileDigests,
    /// Whether this requires Python
    pub requires_python: Option<String>,
}

/// File digests for integrity verification
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileDigests {
    /// MD5 hash (legacy)
    pub md5: Option<String>,
    /// SHA256 hash
    pub sha256: String,
}

/// PyPI client for fetching package metadata and downloading packages
pub struct PyPiClient {
    /// HTTP client
    client: reqwest::blocking::Client,
    /// Base URL for PyPI API
    base_url: String,
}

impl Default for PyPiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl PyPiClient {
    /// Create a new PyPI client with default settings
    pub fn new() -> Self {
        Self::with_base_url("https://pypi.org")
    }

    /// Create a new PyPI client with a custom base URL
    pub fn with_base_url(base_url: &str) -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("dx-py-package-manager/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Get package metadata from PyPI
    pub fn get_package(&self, name: &str) -> Result<PyPiPackageInfo> {
        let url = format!("{}/pypi/{}/json", self.base_url, name);

        let response = self
            .client
            .get(&url)
            .send()
            .map_err(|e| Error::Cache(format!("Network error: {}", e)))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::InvalidPackageName(format!(
                "Package not found: {}",
                name
            )));
        }

        if !response.status().is_success() {
            return Err(Error::Cache(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        response
            .json()
            .map_err(|e| Error::Cache(format!("JSON parse error: {}", e)))
    }

    /// Get package metadata for a specific version
    pub fn get_package_version(&self, name: &str, version: &str) -> Result<PyPiPackageInfo> {
        let url = format!("{}/pypi/{}/{}/json", self.base_url, name, version);

        let response = self
            .client
            .get(&url)
            .send()
            .map_err(|e| Error::Cache(format!("Network error: {}", e)))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::InvalidPackageName(format!(
                "Package version not found: {}=={}",
                name, version
            )));
        }

        if !response.status().is_success() {
            return Err(Error::Cache(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        response
            .json()
            .map_err(|e| Error::Cache(format!("JSON parse error: {}", e)))
    }

    /// Get all available versions for a package
    pub fn get_versions(&self, name: &str) -> Result<Vec<String>> {
        let info = self.get_package(name)?;
        let mut versions: Vec<String> = info.releases.keys().cloned().collect();
        versions.sort();
        Ok(versions)
    }

    /// Get dependencies for a package version
    pub fn get_dependencies(&self, name: &str, version: &str) -> Result<Vec<String>> {
        let info = self.get_package_version(name, version)?;
        Ok(info.info.requires_dist.unwrap_or_default())
    }

    /// Find the best wheel file for the current platform
    pub fn find_wheel(&self, name: &str, version: &str) -> Result<Option<ReleaseFile>> {
        let info = self.get_package_version(name, version)?;

        // Get files for this version
        let files = info.releases.get(version).cloned().unwrap_or_default();

        // Prefer wheels over sdist
        // Priority: platform-specific wheel > universal wheel > sdist
        let mut best_wheel: Option<ReleaseFile> = None;

        for file in files {
            if file.packagetype == "bdist_wheel" {
                // Check if this is a better match
                let dominated = best_wheel.as_ref().map_or(false, |best| {
                    // Prefer more specific wheels
                    file.filename.contains("any") && !best.filename.contains("any")
                });

                if !dominated {
                    best_wheel = Some(file);
                }
            }
        }

        Ok(best_wheel)
    }

    /// Download a file and verify its integrity
    pub fn download(&self, url: &str, expected_sha256: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|e| Error::Cache(format!("Download error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Cache(format!(
                "Download failed: {}",
                response.status()
            )));
        }

        let data = response
            .bytes()
            .map_err(|e| Error::Cache(format!("Read error: {}", e)))?
            .to_vec();

        // Verify SHA256
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let computed = hex::encode(hasher.finalize());

        if computed != expected_sha256 {
            return Err(Error::Cache(format!(
                "SHA256 mismatch: expected {}, got {}",
                expected_sha256, computed
            )));
        }

        Ok(data)
    }

    /// Download a wheel file for a package version
    pub fn download_wheel(&self, name: &str, version: &str) -> Result<Vec<u8>> {
        let wheel = self
            .find_wheel(name, version)?
            .ok_or_else(|| Error::Cache(format!("No wheel found for {}=={}", name, version)))?;

        self.download(&wheel.url, &wheel.digests.sha256)
    }
}

/// Parsed dependency specification
#[derive(Debug, Clone)]
pub struct DependencySpec {
    /// Package name
    pub name: String,
    /// Version constraint (e.g., ">=1.0,<2.0")
    pub version_constraint: Option<String>,
    /// Extras (e.g., ["dev", "test"])
    pub extras: Vec<String>,
    /// Environment markers (e.g., "python_version >= '3.8'")
    pub markers: Option<String>,
}

impl DependencySpec {
    /// Parse a PEP 508 dependency string
    pub fn parse(spec: &str) -> Result<Self> {
        let spec = spec.trim();

        // Simple parser for common cases
        // Full PEP 508 parsing would be more complex

        // Check for markers (after ';')
        let (spec_part, markers) = if let Some(idx) = spec.find(';') {
            (spec[..idx].trim(), Some(spec[idx + 1..].trim().to_string()))
        } else {
            (spec, None)
        };

        // Check for extras (in brackets)
        let (name_and_version, extras) = if let Some(start) = spec_part.find('[') {
            if let Some(end) = spec_part.find(']') {
                let extras_str = &spec_part[start + 1..end];
                let extras: Vec<String> = extras_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                // Combine name part with version part (after ']')
                let name_part = &spec_part[..start];
                let version_part = &spec_part[end + 1..];
                (format!("{}{}", name_part, version_part), extras)
            } else {
                (spec_part.to_string(), Vec::new())
            }
        } else {
            (spec_part.to_string(), Vec::new())
        };

        // Check for version constraint
        let (name, version_constraint) = if let Some(idx) = name_and_version.find(|c: char| {
            c == '>' || c == '<' || c == '=' || c == '!' || c == '~'
        }) {
            (
                name_and_version[..idx].trim().to_string(),
                Some(name_and_version[idx..].trim().to_string()),
            )
        } else {
            (name_and_version.trim().to_string(), None)
        };

        if name.is_empty() {
            return Err(Error::InvalidPackageName(
                "Empty package name".to_string(),
            ));
        }

        Ok(Self {
            name,
            version_constraint,
            extras,
            markers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_spec_parse_simple() {
        let spec = DependencySpec::parse("requests").unwrap();
        assert_eq!(spec.name, "requests");
        assert!(spec.version_constraint.is_none());
        assert!(spec.extras.is_empty());
        assert!(spec.markers.is_none());
    }

    #[test]
    fn test_dependency_spec_parse_with_version() {
        let spec = DependencySpec::parse("requests>=2.0").unwrap();
        assert_eq!(spec.name, "requests");
        assert_eq!(spec.version_constraint, Some(">=2.0".to_string()));
    }

    #[test]
    fn test_dependency_spec_parse_with_extras() {
        let spec = DependencySpec::parse("requests[security,socks]>=2.0").unwrap();
        assert_eq!(spec.name, "requests");
        assert_eq!(spec.extras, vec!["security", "socks"]);
        assert_eq!(spec.version_constraint, Some(">=2.0".to_string()));
    }

    #[test]
    fn test_dependency_spec_parse_with_markers() {
        let spec =
            DependencySpec::parse("requests>=2.0; python_version >= '3.8'").unwrap();
        assert_eq!(spec.name, "requests");
        assert_eq!(spec.version_constraint, Some(">=2.0".to_string()));
        assert_eq!(spec.markers, Some("python_version >= '3.8'".to_string()));
    }

    #[test]
    fn test_dependency_spec_parse_complex() {
        let spec = DependencySpec::parse(
            "urllib3[brotli,socks]>=1.21.1,<3; python_version >= '3.7'",
        )
        .unwrap();
        assert_eq!(spec.name, "urllib3");
        assert_eq!(spec.extras, vec!["brotli", "socks"]);
        assert_eq!(spec.version_constraint, Some(">=1.21.1,<3".to_string()));
        assert_eq!(spec.markers, Some("python_version >= '3.7'".to_string()));
    }
}
