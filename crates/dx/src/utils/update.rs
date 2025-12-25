//! Self-update system for the DX CLI
//!
//! Provides update checking and delta patching functionality.
//! - Requirement 5.1: Verify Ed25519 signature before applying
//! - Requirement 5.2: Return SignatureInvalid on failure
//! - Requirement 5.3: Create backup of current binary before replacement
//! - Requirement 5.4: Restore from backup if update fails
//! - Requirement 5.5: Prefer delta updates over full binary downloads
//! - Requirement 5.7: Use atomic rename operations
//! - Requirement 5.8: Display both version numbers and release notes

use crate::utils::error::DxError;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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

// ═══════════════════════════════════════════════════════════════════════════
//  SIGNATURE VERIFICATION
// ═══════════════════════════════════════════════════════════════════════════

/// Verify Ed25519 signature of update binary
///
/// Requirement 5.1: Verify Ed25519 signature before applying
/// Requirement 5.2: Return SignatureInvalid on failure
///
/// # Arguments
/// * `data` - The binary data to verify
/// * `signature_bytes` - The 64-byte Ed25519 signature
/// * `public_key_bytes` - The 32-byte Ed25519 public key
///
/// # Returns
/// * `Ok(())` - If signature is valid
/// * `Err(SignatureInvalid)` - If signature verification fails
pub fn verify_signature(
    data: &[u8],
    signature_bytes: &[u8],
    public_key_bytes: &[u8],
) -> Result<(), DxError> {
    // Parse the public key
    let public_key = VerifyingKey::from_bytes(
        public_key_bytes
            .try_into()
            .map_err(|_| DxError::SignatureInvalid)?,
    )
    .map_err(|_| DxError::SignatureInvalid)?;

    // Parse the signature
    let signature = Signature::from_bytes(
        signature_bytes
            .try_into()
            .map_err(|_| DxError::SignatureInvalid)?,
    );

    // Verify the signature
    public_key
        .verify(data, &signature)
        .map_err(|_| DxError::SignatureInvalid)
}

/// Verify signature from hex-encoded strings
///
/// Convenience wrapper for verify_signature that accepts hex-encoded inputs.
pub fn verify_signature_hex(
    data: &[u8],
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<(), DxError> {
    let signature_bytes = hex_decode(signature_hex)?;
    let public_key_bytes = hex_decode(public_key_hex)?;
    verify_signature(data, &signature_bytes, &public_key_bytes)
}

/// Decode hex string to bytes
fn hex_decode(hex: &str) -> Result<Vec<u8>, DxError> {
    let hex = hex.trim();
    if hex.len() % 2 != 0 {
        return Err(DxError::SignatureInvalid);
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| DxError::SignatureInvalid)
        })
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════
//  UPDATE APPLICATION
// ═══════════════════════════════════════════════════════════════════════════

/// Update applier for the DX CLI
///
/// Handles backup, atomic replacement, and restore on failure.
pub struct UpdateApplier {
    /// Path to the current binary
    binary_path: PathBuf,
    /// Path to the backup file
    backup_path: PathBuf,
}

impl UpdateApplier {
    /// Create a new update applier for the given binary path
    pub fn new(binary_path: impl Into<PathBuf>) -> Self {
        let binary_path = binary_path.into();
        let backup_path = binary_path.with_extension("bak");
        Self {
            binary_path,
            backup_path,
        }
    }

    /// Create an update applier for the current executable
    pub fn for_current_exe() -> Result<Self, DxError> {
        let binary_path = std::env::current_exe().map_err(|e| DxError::Io {
            message: format!("Failed to get current executable path: {}", e),
        })?;
        Ok(Self::new(binary_path))
    }

    /// Create a backup of the current binary
    ///
    /// Requirement 5.3: Create backup of current binary before replacement
    pub fn create_backup(&self) -> Result<(), DxError> {
        if self.binary_path.exists() {
            fs::copy(&self.binary_path, &self.backup_path).map_err(|e| DxError::Io {
                message: format!(
                    "Failed to create backup at {}: {}",
                    self.backup_path.display(),
                    e
                ),
            })?;
        }
        Ok(())
    }

    /// Restore from backup
    ///
    /// Requirement 5.4: Restore from backup if update fails
    pub fn restore_from_backup(&self) -> Result<(), DxError> {
        if self.backup_path.exists() {
            fs::copy(&self.backup_path, &self.binary_path).map_err(|e| DxError::Io {
                message: format!(
                    "Failed to restore from backup {}: {}",
                    self.backup_path.display(),
                    e
                ),
            })?;
        }
        Ok(())
    }

    /// Remove the backup file
    pub fn remove_backup(&self) -> Result<(), DxError> {
        if self.backup_path.exists() {
            fs::remove_file(&self.backup_path).map_err(|e| DxError::Io {
                message: format!(
                    "Failed to remove backup {}: {}",
                    self.backup_path.display(),
                    e
                ),
            })?;
        }
        Ok(())
    }

    /// Apply update with atomic replacement
    ///
    /// Requirement 5.7: Use atomic rename operations
    /// Requirement 12.4: Atomic rename to prevent partial updates
    ///
    /// # Arguments
    /// * `new_binary` - The new binary data to install
    /// * `signature` - The Ed25519 signature of the new binary
    /// * `public_key` - The Ed25519 public key for verification
    ///
    /// # Returns
    /// * `Ok(())` - If update was applied successfully
    /// * `Err(...)` - If update failed (backup will be restored)
    pub fn apply_update(
        &self,
        new_binary: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<(), DxError> {
        // Step 1: Verify signature before doing anything
        verify_signature(new_binary, signature, public_key)?;

        // Step 2: Create backup
        self.create_backup()?;

        // Step 3: Write to temp file
        let temp_path = self.binary_path.with_extension("tmp");
        let result = self.write_and_replace(new_binary, &temp_path);

        // Step 4: On failure, restore from backup
        if result.is_err() {
            let _ = self.restore_from_backup();
            // Clean up temp file if it exists
            let _ = fs::remove_file(&temp_path);
        }

        result
    }

    /// Write new binary to temp file and atomically replace
    fn write_and_replace(&self, new_binary: &[u8], temp_path: &Path) -> Result<(), DxError> {
        // Write to temp file
        fs::write(temp_path, new_binary).map_err(|e| DxError::Io {
            message: format!("Failed to write temp file {}: {}", temp_path.display(), e),
        })?;

        // Set executable permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o755);
            fs::set_permissions(temp_path, perms).map_err(|e| DxError::Io {
                message: format!("Failed to set permissions on {}: {}", temp_path.display(), e),
            })?;
        }

        // Atomic rename
        fs::rename(temp_path, &self.binary_path).map_err(|e| DxError::Io {
            message: format!(
                "Failed to replace binary {} with {}: {}",
                self.binary_path.display(),
                temp_path.display(),
                e
            ),
        })?;

        Ok(())
    }

    /// Get the backup path
    pub fn backup_path(&self) -> &Path {
        &self.backup_path
    }

    /// Check if a backup exists
    pub fn has_backup(&self) -> bool {
        self.backup_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use proptest::prelude::*;
    use rand::rngs::OsRng;

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

    // ═══════════════════════════════════════════════════════════════════
    //  SIGNATURE VERIFICATION TESTS
    // ═══════════════════════════════════════════════════════════════════

    /// Helper to generate a valid keypair and signature for testing
    fn generate_test_signature(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
        use ed25519_dalek::Signer;
        let signing_key = SigningKey::generate(&mut OsRng);
        let signature = signing_key.sign(data);
        let public_key = signing_key.verifying_key();
        (signature.to_bytes().to_vec(), public_key.to_bytes().to_vec())
    }

    #[test]
    fn test_valid_signature_verification() {
        let data = b"test binary data";
        let (signature, public_key) = generate_test_signature(data);
        
        let result = verify_signature(data, &signature, &public_key);
        assert!(result.is_ok(), "Valid signature should verify successfully");
    }

    #[test]
    fn test_invalid_signature_fails() {
        let data = b"test binary data";
        let (mut signature, public_key) = generate_test_signature(data);
        
        // Corrupt the signature
        signature[0] ^= 0xFF;
        
        let result = verify_signature(data, &signature, &public_key);
        assert!(matches!(result, Err(DxError::SignatureInvalid)));
    }

    #[test]
    fn test_wrong_data_fails() {
        let data = b"test binary data";
        let (signature, public_key) = generate_test_signature(data);
        
        // Verify with different data
        let wrong_data = b"different data";
        let result = verify_signature(wrong_data, &signature, &public_key);
        assert!(matches!(result, Err(DxError::SignatureInvalid)));
    }

    #[test]
    fn test_wrong_public_key_fails() {
        let data = b"test binary data";
        let (signature, _) = generate_test_signature(data);
        
        // Generate a different keypair
        let (_, wrong_public_key) = generate_test_signature(b"other data");
        
        let result = verify_signature(data, &signature, &wrong_public_key);
        assert!(matches!(result, Err(DxError::SignatureInvalid)));
    }

    #[test]
    fn test_invalid_signature_length() {
        let data = b"test binary data";
        let (_, public_key) = generate_test_signature(data);
        
        // Too short signature
        let short_sig = vec![0u8; 32];
        let result = verify_signature(data, &short_sig, &public_key);
        assert!(matches!(result, Err(DxError::SignatureInvalid)));
    }

    #[test]
    fn test_invalid_public_key_length() {
        let data = b"test binary data";
        let (signature, _) = generate_test_signature(data);
        
        // Too short public key
        let short_key = vec![0u8; 16];
        let result = verify_signature(data, &signature, &short_key);
        assert!(matches!(result, Err(DxError::SignatureInvalid)));
    }

    #[test]
    fn test_hex_decode() {
        assert_eq!(hex_decode("00").unwrap(), vec![0u8]);
        assert_eq!(hex_decode("ff").unwrap(), vec![255u8]);
        assert_eq!(hex_decode("0102").unwrap(), vec![1u8, 2u8]);
        assert_eq!(hex_decode("AABB").unwrap(), vec![170u8, 187u8]);
        
        // Invalid hex
        assert!(hex_decode("0").is_err()); // Odd length
        assert!(hex_decode("GG").is_err()); // Invalid chars
    }

    // Feature: dx-cli-hardening, Property 15: Signature Verification Gates Updates
    // **Validates: Requirements 5.1, 5.2**
    //
    // For any update payload, if the Ed25519 signature verification fails,
    // the update SHALL be aborted and a SignatureInvalid error SHALL be returned.
    // Valid signatures SHALL allow the update to proceed.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_valid_signature_allows_update(data in proptest::collection::vec(any::<u8>(), 1..1000)) {
            let (signature, public_key) = generate_test_signature(&data);
            let result = verify_signature(&data, &signature, &public_key);
            prop_assert!(result.is_ok(), "Valid signature should allow update");
        }

        #[test]
        fn prop_invalid_signature_blocks_update(
            data in proptest::collection::vec(any::<u8>(), 1..1000),
            corrupt_byte in 0usize..64,
            corrupt_value in any::<u8>()
        ) {
            let (mut signature, public_key) = generate_test_signature(&data);
            
            // Corrupt the signature (ensure we actually change it)
            let original = signature[corrupt_byte];
            signature[corrupt_byte] = if corrupt_value == original {
                corrupt_value.wrapping_add(1)
            } else {
                corrupt_value
            };
            
            let result = verify_signature(&data, &signature, &public_key);
            prop_assert!(
                matches!(result, Err(DxError::SignatureInvalid)),
                "Corrupted signature should return SignatureInvalid"
            );
        }

        #[test]
        fn prop_wrong_data_blocks_update(
            data1 in proptest::collection::vec(any::<u8>(), 1..500),
            data2 in proptest::collection::vec(any::<u8>(), 1..500)
        ) {
            // Only test when data is actually different
            prop_assume!(data1 != data2);
            
            let (signature, public_key) = generate_test_signature(&data1);
            let result = verify_signature(&data2, &signature, &public_key);
            prop_assert!(
                matches!(result, Err(DxError::SignatureInvalid)),
                "Signature for different data should fail"
            );
        }
    }

    // Feature: dx-cli-hardening, Property 16: Delta Patch Preference
    // **Validates: Requirements 5.5**
    //
    // When a delta patch is available, the update system SHALL choose the delta patch.
    // When only full download is available, it SHALL use that.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_delta_preferred_when_available(
            full_size in 1_000_000u64..100_000_000,
            delta_size in 100_000u64..1_000_000
        ) {
            let info = UpdateInfo {
                current_version: "1.0.0".to_string(),
                new_version: "1.1.0".to_string(),
                release_notes: String::new(),
                download_url: "https://example.com/dx".to_string(),
                delta_url: Some("https://example.com/dx.patch".to_string()),
                full_size,
                delta_size: Some(delta_size),
                signature: String::new(),
            };

            prop_assert!(info.has_delta(), "Should report delta available");
            prop_assert!(
                info.preferred_url().contains("patch"),
                "Should prefer delta URL when available"
            );
            prop_assert_eq!(
                info.preferred_size(),
                delta_size,
                "Should report delta size when delta available"
            );
        }

        #[test]
        fn prop_full_used_when_no_delta(full_size in 1_000_000u64..100_000_000) {
            let info = UpdateInfo {
                current_version: "1.0.0".to_string(),
                new_version: "1.1.0".to_string(),
                release_notes: String::new(),
                download_url: "https://example.com/dx".to_string(),
                delta_url: None,
                full_size,
                delta_size: None,
                signature: String::new(),
            };

            prop_assert!(!info.has_delta(), "Should report no delta");
            prop_assert!(
                !info.preferred_url().contains("patch"),
                "Should use full URL when no delta"
            );
            prop_assert_eq!(
                info.preferred_size(),
                full_size,
                "Should report full size when no delta"
            );
        }
    }

    // Feature: dx-cli-hardening, Property 17: Update Version Display
    // **Validates: Requirements 5.8**
    //
    // For any available update, the display output SHALL contain both the current
    // version and the new version, plus a non-empty release notes summary if available.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_version_display_contains_both_versions(
            major1 in 0u32..100,
            minor1 in 0u32..100,
            patch1 in 0u32..100,
            major2 in 0u32..100,
            minor2 in 0u32..100,
            patch2 in 0u32..100
        ) {
            let current = format!("{}.{}.{}", major1, minor1, patch1);
            let new_ver = format!("{}.{}.{}", major2, minor2, patch2);

            let info = UpdateInfo {
                current_version: current.clone(),
                new_version: new_ver.clone(),
                release_notes: "Bug fixes and improvements".to_string(),
                download_url: String::new(),
                delta_url: None,
                full_size: 0,
                delta_size: None,
                signature: String::new(),
            };

            let display = info.version_display();

            prop_assert!(
                display.contains(&current),
                "Display should contain current version: {} not in {}",
                current,
                display
            );
            prop_assert!(
                display.contains(&new_ver),
                "Display should contain new version: {} not in {}",
                new_ver,
                display
            );
        }

        #[test]
        fn prop_release_notes_preserved(notes in "[a-zA-Z0-9 ]{1,100}") {
            let info = UpdateInfo {
                current_version: "1.0.0".to_string(),
                new_version: "1.1.0".to_string(),
                release_notes: notes.clone(),
                download_url: String::new(),
                delta_url: None,
                full_size: 0,
                delta_size: None,
                signature: String::new(),
            };

            prop_assert_eq!(
                info.release_notes,
                notes,
                "Release notes should be preserved"
            );
        }
    }
}
