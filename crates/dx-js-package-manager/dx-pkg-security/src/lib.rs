//! Security & Sandboxing for Dx Package Manager
//!
//! Provides:
//! - Capability-based permission system
//! - Path sandboxing
//! - Integrity verification
//! - Attack vector protection

use anyhow::{Result, bail};
use dx_pkg_core::hash::ContentHash;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Security capabilities for package operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    /// Allowed file system paths (read)
    pub read_paths: HashSet<PathBuf>,
    /// Allowed file system paths (write)
    pub write_paths: HashSet<PathBuf>,
    /// Allowed network hosts
    pub network_hosts: HashSet<String>,
    /// Allow script execution
    pub allow_scripts: bool,
    /// Maximum package size (bytes)
    pub max_package_size: u64,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            read_paths: HashSet::new(),
            write_paths: HashSet::new(),
            network_hosts: HashSet::from_iter(vec!["registry.dx.dev".to_string()]),
            allow_scripts: false,
            max_package_size: 100 * 1024 * 1024, // 100MB
        }
    }
}

impl SecurityCapabilities {
    /// Create capabilities for installation
    pub fn for_install(install_dir: impl AsRef<Path>) -> Self {
        let mut caps = Self::default();
        caps.write_paths.insert(install_dir.as_ref().to_path_buf());
        caps.read_paths.insert(install_dir.as_ref().to_path_buf());
        caps
    }

    /// Check if path is allowed for reading
    pub fn can_read(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        self.read_paths.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Check if path is allowed for writing
    pub fn can_write(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        self.write_paths.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Check if network host is allowed
    pub fn can_access_network(&self, host: &str) -> bool {
        self.network_hosts.contains(host)
    }
}

/// Audit result for a package operation
#[derive(Debug, Clone)]
pub struct AuditResult {
    pub passed: bool,
    pub issues: Vec<SecurityIssue>,
    pub risk_score: u32, // 0-100
}

/// Security issue detected during audit
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: Severity,
    pub category: IssueCategory,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueCategory {
    PathTraversal,
    IntegrityViolation,
    ExcessiveSize,
    SuspiciousScript,
    UnauthorizedNetwork,
}

/// Security auditor for package operations
pub struct SecurityAuditor {
    capabilities: SecurityCapabilities,
}

impl SecurityAuditor {
    /// Create new auditor with capabilities
    pub fn new(capabilities: SecurityCapabilities) -> Self {
        Self { capabilities }
    }

    /// Audit package before installation
    pub fn audit_package(
        &self,
        path: &Path,
        _expected_hash: ContentHash,
        size: u64,
    ) -> Result<AuditResult> {
        let mut issues = Vec::new();
        let mut risk_score = 0;

        // Check 1: Path traversal protection
        if !self.is_safe_path(path) {
            issues.push(SecurityIssue {
                severity: Severity::Critical,
                category: IssueCategory::PathTraversal,
                description: format!("Path traversal attempt detected: {}", path.display()),
            });
            risk_score += 40;
        }

        // Check 2: Size limit
        if size > self.capabilities.max_package_size {
            issues.push(SecurityIssue {
                severity: Severity::High,
                category: IssueCategory::ExcessiveSize,
                description: format!(
                    "Package size {} exceeds limit {}",
                    size, self.capabilities.max_package_size
                ),
            });
            risk_score += 30;
        }

        // Check 3: Write permission
        if !self.capabilities.can_write(path) {
            issues.push(SecurityIssue {
                severity: Severity::High,
                category: IssueCategory::UnauthorizedNetwork,
                description: format!("No write permission for: {}", path.display()),
            });
            risk_score += 25;
        }

        let passed = risk_score < 50; // Threshold for blocking

        Ok(AuditResult {
            passed,
            issues,
            risk_score,
        })
    }

    /// Check if path is safe (no traversal attacks)
    fn is_safe_path(&self, path: &Path) -> bool {
        // Check for path traversal patterns
        let path_str = path.to_string_lossy();

        // Block obvious attacks
        if path_str.contains("..") || path_str.contains("~") {
            return false;
        }

        // Ensure path is within allowed directories
        self.capabilities.can_write(path)
    }

    /// Verify package integrity
    pub fn verify_integrity(&self, data: &[u8], expected: ContentHash) -> Result<()> {
        let actual = dx_pkg_core::hash::xxhash64(data);

        if u128::from(actual) != expected {
            bail!("Integrity check failed: expected {:016x}, got {:016x}", expected, actual);
        }

        Ok(())
    }

    /// Check network access permission
    pub fn check_network_access(&self, host: &str) -> Result<()> {
        if !self.capabilities.can_access_network(host) {
            bail!("Network access denied for host: {}", host);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capabilities() {
        let caps = SecurityCapabilities::default();
        assert!(!caps.allow_scripts);
        assert_eq!(caps.max_package_size, 100 * 1024 * 1024);
        assert!(caps.can_access_network("registry.dx.dev"));
        assert!(!caps.can_access_network("evil.com"));
    }

    #[test]
    fn test_install_capabilities() {
        let caps = SecurityCapabilities::for_install("./node_modules");
        assert!(caps.can_write("./node_modules"));
        assert!(caps.can_write("./node_modules/react"));
        assert!(!caps.can_write("../etc/passwd"));
    }

    #[test]
    fn test_path_traversal_detection() {
        let caps = SecurityCapabilities::for_install("./node_modules");
        let auditor = SecurityAuditor::new(caps);

        assert!(!auditor.is_safe_path(Path::new("../etc/passwd")));
        assert!(!auditor.is_safe_path(Path::new("~/secret")));
        assert!(auditor.is_safe_path(Path::new("./node_modules/react")));
    }

    #[test]
    fn test_size_limit() {
        let caps = SecurityCapabilities::default();
        let auditor = SecurityAuditor::new(caps);

        let result = auditor
            .audit_package(
                Path::new("./node_modules/huge"),
                0x1234567890abcdef,
                200 * 1024 * 1024, // 200MB (exceeds limit)
            )
            .unwrap();

        assert!(!result.passed);
        assert!(result.risk_score > 0);
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_integrity_verification() {
        let caps = SecurityCapabilities::default();
        let auditor = SecurityAuditor::new(caps);

        let data = b"hello world";
        let hash = dx_pkg_core::hash::xxhash64(data);

        // Valid hash
        assert!(auditor.verify_integrity(data, hash.into()).is_ok());

        // Invalid hash
        assert!(auditor.verify_integrity(data, 0xdeadbeef).is_err());
    }
}
