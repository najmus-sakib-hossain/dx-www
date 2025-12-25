//! Self-update system for the DX CLI
//!
//! Provides update checking and delta patching functionality.
//! - Requirement 6.1: Check for updates from GitHub releases API
//! - Requirement 6.2: Display current and new version numbers
//! - Requirement 6.3: Prefer delta updates over full binary downloads
//! - Requirement 6.4: Verify Ed25519 signatures on updates
//! - Requirement 6.7: Display release notes summary

use crate::utils::error::DxError;
use serde::{Deserialize, Serialize};

/// Current version of the DX CLI
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub repository for releases
pub const GITHUB_REPO: &str = "user/dx";

/// GitHub releases API URL
pub const RELEASES_API_URL: &str = "https://api.github.com/repos/user/dx/releases/latest";

/// Represents an available update
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateInfo {
    /// Current installed version
    pub current_version: String,
    /// New available version
    pub new_version: String,
    /// Release notes summary
    pub release_notes: String,
    /// Download URL for the full binary
    pub download_url: String,
    /// Download URL for delta patch (if available)
    pub delta_url: Option<String>,
    /// Size of the full binary in bytes
    pub full_size: u64,
    /// Size of the delta patch in bytes (if available)
    pub delta_size: Option<u64>,
    /// Ed25519 signature for verification
    pub signature: String,
}

impl UpdateInfo {
    /// Check if a delta patch is available
    pub fn has_delta(&self) -> bool {
        self.delta_url.is_some()
    }

    /// Get the preferred download URL (delta if available, otherwise full)
    ///
    /// Requirement 6.3: Prefer delta updates over full binary downloads
    pub fn preferred_url(&self) -> &str {
        self.delta_url.as_deref().unwrap_or(&self.download_url)
    }

    /// Get the size of the preferred download
    pub fn preferred_size(&self) -> u64 {
        self.delta_size.unwrap_or(self.full_size)
    }

    /// Format the version display string
    ///
    /// Requirement 6.2: Display current and new version numbers
    pub fn version_display(&self) -> String {
        format!("{} → {}", self.current_version, self.new_version)
    }
}

/// GitHub release asset information
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubAsset {
    /// Asset name
    pub name: String,
    /// Download URL
    pub browser_download_url: String,
    /// Size in bytes
    pub size: u64,
}

/// GitHub release information
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubRelease {
    /// Release tag name (version)
    pub tag_name: String,
    /// Release name
    pub name: Option<String>,
    /// Release body (notes)
    pub body: Option<String>,
    /// Release assets
    pub assets: Vec<GitHubAsset>,
    /// Whether this is a prerelease
    pub prerelease: bool,
    /// Whether this is a draft
    pub draft: bool,
}

/// Update checker for the DX CLI
///
/// Requirement 6.1: Check for updates from GitHub releases API
pub struct UpdateChecker {
    /// GitHub releases API URL
    api_url: String,
    /// Current version
    current_version: String,
}

impl UpdateChecker {
    /// Create a new update checker
    pub fn new() -> Self {
        Self {
            api_url: RELEASES_API_URL.to_string(),
            current_version: CURRENT_VERSION.to_string(),
        }
    }

    /// Create an update checker with custom settings (for testing)
    #[cfg(test)]
    pub fn with_config(api_url: String, current_version: String) -> Self {
        Self {
            api_url,
            current_version,
        }
    }

    /// Get the current version
    pub fn current_version(&self) -> &str {
        &self.current_version
    }

    /// Check for available updates
    ///
    /// Requirement 6.1: Check for updates from GitHub releases API
    /// Requirement 6.2: Display current and new version numbers
    /// Requirement 6.3: Prefer delta updates over full binary downloads
    pub async fn check(&self) -> Result<Option<UpdateInfo>, DxError> {
        // Fetch latest release from GitHub API
        let release = self.fetch_latest_release().await?;

        // Skip prereleases and drafts
        if release.prerelease || release.draft {
            return Ok(None);
        }

        // Parse version from tag
        let new_version = release.tag_name.trim_start_matches('v').to_string();

        // Compare versions
        if !is_newer_version(&new_version, &self.current_version) {
            return Ok(None);
        }

        // Find platform-appropriate assets
        let platform = get_platform_identifier();
        let (download_url, full_size) = self.find_binary_asset(&release.assets, &platform)?;
        let (delta_url, delta_size) = self.find_delta_asset(&release.assets, &platform);
        let signature = self.find_signature(&release.assets, &platform)?;

        // Extract release notes summary
        let release_notes = release
            .body
            .as_deref()
            .map(|b| summarize_release_notes(b))
            .unwrap_or_default();

        Ok(Some(UpdateInfo {
            current_version: self.current_version.clone(),
            new_version,
            release_notes,
            download_url,
            delta_url,
            full_size,
            delta_size,
            signature,
        }))
    }

    /// Fetch the latest release from GitHub API
    async fn fetch_latest_release(&self) -> Result<GitHubRelease, DxError> {
        // In a real implementation, this would use reqwest or similar
        // For now, return a placeholder error
        Err(DxError::Network {
            message: format!("Update check not yet implemented (would fetch from {})", self.api_url),
        })
    }

    /// Find the binary asset for the current platform
    fn find_binary_asset(
        &self,
        assets: &[GitHubAsset],
        platform: &str,
    ) -> Result<(String, u64), DxError> {
        // Look for asset matching platform pattern: dx-{platform}.{ext}
        let patterns = [
            format!("dx-{}.exe", platform),
            format!("dx-{}.tar.gz", platform),
            format!("dx-{}.zip", platform),
            format!("dx-{}", platform),
        ];

        for pattern in &patterns {
            if let Some(asset) = assets.iter().find(|a| a.name == *pattern) {
                return Ok((asset.browser_download_url.clone(), asset.size));
            }
        }

        Err(DxError::UpdateDownloadFailed {
            message: format!("No binary found for platform: {}", platform),
        })
    }

    /// Find the delta patch asset for the current platform
    fn find_delta_asset(&self, assets: &[GitHubAsset], platform: &str) -> (Option<String>, Option<u64>) {
        // Look for delta patch: dx-{platform}.patch or dx-{platform}.delta
        let patterns = [
            format!("dx-{}.patch", platform),
            format!("dx-{}.delta", platform),
            format!("dx-{}-{}.patch", platform, self.current_version),
        ];

        for pattern in &patterns {
            if let Some(asset) = assets.iter().find(|a| a.name == *pattern) {
                return (Some(asset.browser_download_url.clone()), Some(asset.size));
            }
        }

        (None, None)
    }

    /// Find the signature file for the binary
    fn find_signature(&self, assets: &[GitHubAsset], platform: &str) -> Result<String, DxError> {
        // Look for signature: dx-{platform}.sig or dx-{platform}.asc
        let patterns = [
            format!("dx-{}.sig", platform),
            format!("dx-{}.asc", platform),
        ];

        for pattern in &patterns {
            if let Some(asset) = assets.iter().find(|a| a.name == *pattern) {
                return Ok(asset.browser_download_url.clone());
            }
        }

        // Signature is required for security
        Err(DxError::UpdateDownloadFailed {
            message: format!("No signature found for platform: {}", platform),
        })
    }
}

impl Default for UpdateChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the platform identifier for asset matching
fn get_platform_identifier() -> String {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        "unknown"
    };

    format!("{}-{}", os, arch)
}

/// Compare two semantic versions
///
/// Returns true if `new` is newer than `current`
fn is_newer_version(new: &str, current: &str) -> bool {
    let parse_version = |v: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = v
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };

    let new_v = parse_version(new);
    let current_v = parse_version(current);

    new_v > current_v
}

/// Summarize release notes to first paragraph or 200 chars
///
/// Requirement 6.7: Display release notes summary
fn summarize_release_notes(notes: &str) -> String {
    // Take first paragraph
    let first_para = notes
        .split("\n\n")
        .next()
        .unwrap_or(notes)
        .trim();

    // Limit to 200 characters
    if first_para.len() > 200 {
        format!("{}...", &first_para[..197])
    } else {
        first_para.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_version_comparison() {
        assert!(is_newer_version("1.0.1", "1.0.0"));
        assert!(is_newer_version("1.1.0", "1.0.0"));
        assert!(is_newer_version("2.0.0", "1.9.9"));
        assert!(!is_newer_version("1.0.0", "1.0.0"));
        assert!(!is_newer_version("1.0.0", "1.0.1"));
        assert!(!is_newer_version("0.9.0", "1.0.0"));
    }

    #[test]
    fn test_platform_identifier() {
        let platform = get_platform_identifier();
        assert!(!platform.is_empty());
        assert!(platform.contains('-'));
    }

    #[test]
    fn test_release_notes_summary() {
        let notes = "This is the first paragraph.\n\nThis is the second paragraph.";
        let summary = summarize_release_notes(notes);
        assert_eq!(summary, "This is the first paragraph.");

        let long_notes = "A".repeat(300);
        let summary = summarize_release_notes(&long_notes);
        assert_eq!(summary.len(), 200);
        assert!(summary.ends_with("..."));
    }

    #[test]
    fn test_update_info_has_delta() {
        let info = UpdateInfo {
            current_version: "1.0.0".to_string(),
            new_version: "1.1.0".to_string(),
            release_notes: "Test".to_string(),
            download_url: "https://example.com/dx".to_string(),
            delta_url: Some("https://example.com/dx.patch".to_string()),
            full_size: 10_000_000,
            delta_size: Some(500_000),
            signature: "sig".to_string(),
        };

        assert!(info.has_delta());
        assert_eq!(info.preferred_url(), "https://example.com/dx.patch");
        assert_eq!(info.preferred_size(), 500_000);
    }

    #[test]
    fn test_update_info_no_delta() {
        let info = UpdateInfo {
            current_version: "1.0.0".to_string(),
            new_version: "1.1.0".to_string(),
            release_notes: "Test".to_string(),
            download_url: "https://example.com/dx".to_string(),
            delta_url: None,
            full_size: 10_000_000,
            delta_size: None,
            signature: "sig".to_string(),
        };

        assert!(!info.has_delta());
        assert_eq!(info.preferred_url(), "https://example.com/dx");
        assert_eq!(info.preferred_size(), 10_000_000);
    }

    #[test]
    fn test_version_display() {
        let info = UpdateInfo {
            current_version: "1.0.0".to_string(),
            new_version: "1.1.0".to_string(),
            release_notes: String::new(),
            download_url: String::new(),
            delta_url: None,
            full_size: 0,
            delta_size: None,
            signature: String::new(),
        };

        assert_eq!(info.version_display(), "1.0.0 → 1.1.0");
    }

    // ═══════════════════════════════════════════════════════════════════
    //  PROPERTY TESTS
    // ═══════════════════════════════════════════════════════════════════

    // Feature: dx-cli, Property 6: Update Version Display
    // Validates: Requirements 6.2
    //
    // The version display should always show both current and new versions
    // in the format "current → new".
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn prop_version_display_format(
            major1 in 0u32..100,
            minor1 in 0u32..100,
            patch1 in 0u32..100,
            major2 in 0u32..100,
            minor2 in 0u32..100,
            patch2 in 0u32..100
        ) {
            let current = format!("{}.{}.{}", major1, minor1, patch1);
            let new = format!("{}.{}.{}", major2, minor2, patch2);

            let info = UpdateInfo {
                current_version: current.clone(),
                new_version: new.clone(),
                release_notes: String::new(),
                download_url: String::new(),
                delta_url: None,
                full_size: 0,
                delta_size: None,
                signature: String::new(),
            };

            let display = info.version_display();

            // Should contain both versions
            prop_assert!(display.contains(&current), "Display should contain current version");
            prop_assert!(display.contains(&new), "Display should contain new version");

            // Should contain arrow separator
            prop_assert!(display.contains("→"), "Display should contain arrow separator");

            // Format should be "current → new"
            prop_assert_eq!(display, format!("{} → {}", current, new));
        }
    }

    // Feature: dx-cli, Property 7: Delta Patch Preference
    // Validates: Requirements 6.3
    //
    // When a delta patch is available, it should be preferred over full download.
    // When no delta is available, full download should be used.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn prop_delta_patch_preference(
            full_size in 1_000_000u64..100_000_000,
            delta_size in 100_000u64..1_000_000,
            has_delta in proptest::bool::ANY
        ) {
            let delta_url = if has_delta {
                Some("https://example.com/dx.patch".to_string())
            } else {
                None
            };

            let delta_size_opt = if has_delta {
                Some(delta_size)
            } else {
                None
            };

            let info = UpdateInfo {
                current_version: "1.0.0".to_string(),
                new_version: "1.1.0".to_string(),
                release_notes: String::new(),
                download_url: "https://example.com/dx".to_string(),
                delta_url,
                full_size,
                delta_size: delta_size_opt,
                signature: String::new(),
            };

            if has_delta {
                // Delta should be preferred
                prop_assert!(info.has_delta(), "Should report delta available");
                prop_assert!(info.preferred_url().contains("patch"), "Should prefer delta URL");
                prop_assert_eq!(info.preferred_size(), delta_size, "Should report delta size");
            } else {
                // Full download should be used
                prop_assert!(!info.has_delta(), "Should report no delta");
                prop_assert!(!info.preferred_url().contains("patch"), "Should use full URL");
                prop_assert_eq!(info.preferred_size(), full_size, "Should report full size");
            }
        }
    }

    // Property test for version comparison
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_version_comparison_reflexive(
            major in 0u32..100,
            minor in 0u32..100,
            patch in 0u32..100
        ) {
            let version = format!("{}.{}.{}", major, minor, patch);
            // A version is never newer than itself
            prop_assert!(!is_newer_version(&version, &version));
        }

        #[test]
        fn prop_version_comparison_major_dominates(
            major1 in 0u32..50,
            major2 in 51u32..100,
            minor1 in 0u32..100,
            minor2 in 0u32..100,
            patch1 in 0u32..100,
            patch2 in 0u32..100
        ) {
            let v1 = format!("{}.{}.{}", major1, minor1, patch1);
            let v2 = format!("{}.{}.{}", major2, minor2, patch2);
            // Higher major version is always newer
            prop_assert!(is_newer_version(&v2, &v1));
            prop_assert!(!is_newer_version(&v1, &v2));
        }
    }
}
