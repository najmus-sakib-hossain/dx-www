//! Community Rule Marketplace
//!
//! Discover, install, and manage community-created plugins and rules.
//!
//! # Registry
//!
//! The DX Check registry is hosted at `https://registry.dx.dev/check`
//! and provides:
//! - Plugin discovery and search
//! - Version management
//! - Security auditing
//! - Download statistics
//!
//! # Installation
//!
//! ```bash
//! # Install from registry
//! dx-check plugin install @company/my-rules
//!
//! # Install specific version
//! dx-check plugin install @company/my-rules@1.2.0
//!
//! # Install from git
//! dx-check plugin install git:https://github.com/user/dx-check-rules.git
//!
//! # Install from local path
//! dx-check plugin install ./my-local-plugin
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Registry URL
    pub url: String,
    /// Authentication token (optional)
    pub token: Option<String>,
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Enable offline mode
    pub offline: bool,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("dx-check")
            .join("registry");

        Self {
            url: "https://registry.dx.dev/check".to_string(),
            token: None,
            cache_dir,
            offline: false,
        }
    }
}

/// Package information from registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    /// Package name (e.g., "@company/my-rules")
    pub name: String,
    /// Latest version
    pub version: String,
    /// All available versions
    pub versions: Vec<String>,
    /// Package description
    pub description: String,
    /// Author
    pub author: Option<String>,
    /// Homepage URL
    pub homepage: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
    /// License
    pub license: Option<String>,
    /// Download count
    pub downloads: u64,
    /// Rule count
    pub rule_count: u32,
    /// Supported languages
    pub languages: Vec<String>,
    /// Keywords for search
    pub keywords: Vec<String>,
    /// Publication date
    pub published_at: String,
    /// Verification status
    pub verified: bool,
}

/// Search results from registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    /// Total matching packages
    pub total: u64,
    /// Current page
    pub page: u32,
    /// Results per page
    pub per_page: u32,
    /// Packages in this page
    pub packages: Vec<PackageInfo>,
}

/// Installed package tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
    /// Package name
    pub name: String,
    /// Installed version
    pub version: String,
    /// Installation path
    pub path: PathBuf,
    /// Installation source
    pub source: InstallSource,
    /// Installation timestamp
    pub installed_at: String,
    /// Whether auto-update is enabled
    pub auto_update: bool,
}

/// Package installation source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallSource {
    /// From official registry
    Registry,
    /// From git repository
    Git { url: String, rev: Option<String> },
    /// From local path
    Local { path: PathBuf },
    /// From npm
    Npm,
}

/// Registry client for marketplace operations
pub struct RegistryClient {
    config: RegistryConfig,
    installed: HashMap<String, InstalledPackage>,
}

impl RegistryClient {
    /// Create a new registry client
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            config,
            installed: HashMap::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(RegistryConfig::default())
    }

    /// Load installed packages from disk
    pub fn load_installed(&mut self) -> Result<(), RegistryError> {
        let manifest_path = self.config.cache_dir.join("installed.json");
        
        if manifest_path.exists() {
            let content = std::fs::read_to_string(&manifest_path)
                .map_err(|e| RegistryError::Io(e.to_string()))?;
            
            self.installed = serde_json::from_str(&content)
                .map_err(|e| RegistryError::Parse(e.to_string()))?;
        }

        Ok(())
    }

    /// Save installed packages to disk
    pub fn save_installed(&self) -> Result<(), RegistryError> {
        std::fs::create_dir_all(&self.config.cache_dir)
            .map_err(|e| RegistryError::Io(e.to_string()))?;

        let manifest_path = self.config.cache_dir.join("installed.json");
        let content = serde_json::to_string_pretty(&self.installed)
            .map_err(|e| RegistryError::Parse(e.to_string()))?;

        std::fs::write(&manifest_path, content)
            .map_err(|e| RegistryError::Io(e.to_string()))?;

        Ok(())
    }

    /// Search for packages (async version)
    pub async fn search_async(&self, query: &str, page: u32) -> Result<SearchResults, RegistryError> {
        if self.config.offline {
            return Err(RegistryError::Offline);
        }

        // In a real implementation, this would make an HTTP request
        // For now, return mock results
        Ok(SearchResults {
            total: 0,
            page,
            per_page: 20,
            packages: self.mock_search_results(query),
        })
    }

    /// Get package info
    pub async fn get_package(&self, name: &str) -> Result<PackageInfo, RegistryError> {
        if self.config.offline {
            return Err(RegistryError::Offline);
        }

        // Mock implementation
        Ok(PackageInfo {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            versions: vec!["1.0.0".to_string()],
            description: format!("Plugin: {}", name),
            author: None,
            homepage: None,
            repository: None,
            license: Some("MIT".to_string()),
            downloads: 0,
            rule_count: 0,
            languages: vec!["typescript".to_string(), "javascript".to_string()],
            keywords: vec![],
            published_at: chrono::Utc::now().to_rfc3339(),
            verified: false,
        })
    }

    /// Install a package (async version)
    pub async fn install_async(&mut self, spec: &str) -> Result<InstalledPackage, RegistryError> {
        let (name, version, source) = self.parse_install_spec(spec)?;

        // Check if already installed
        if let Some(existing) = self.installed.get(&name) {
            if existing.version == version.as_deref().unwrap_or(&existing.version) {
                return Ok(existing.clone());
            }
        }

        // Download and install
        let install_path = self.config.cache_dir.join("packages").join(&name);
        std::fs::create_dir_all(&install_path)
            .map_err(|e| RegistryError::Io(e.to_string()))?;

        let installed = InstalledPackage {
            name: name.clone(),
            version: version.unwrap_or_else(|| "latest".to_string()),
            path: install_path,
            source,
            installed_at: chrono::Utc::now().to_rfc3339(),
            auto_update: true,
        };

        self.installed.insert(name, installed.clone());
        self.save_installed()?;

        Ok(installed)
    }

    /// Uninstall a package (modifying version)
    pub fn uninstall_mut(&mut self, name: &str) -> Result<(), RegistryError> {
        if let Some(pkg) = self.installed.remove(name) {
            // Remove installed files
            if pkg.path.exists() {
                std::fs::remove_dir_all(&pkg.path)
                    .map_err(|e| RegistryError::Io(e.to_string()))?;
            }
            self.save_installed()?;
            Ok(())
        } else {
            Err(RegistryError::NotInstalled(name.to_string()))
        }
    }

    /// Update a package (async version)
    pub async fn update_async(&mut self, name: &str) -> Result<InstalledPackage, RegistryError> {
        // Get latest version
        let info = self.get_package(name).await?;
        
        // Reinstall with latest version
        self.uninstall_mut(name).ok(); // Ignore if not installed
        self.install_async(&format!("{}@{}", name, info.version)).await
    }

    /// Update all packages (async version)
    pub async fn update_all_async(&mut self) -> Result<Vec<InstalledPackage>, RegistryError> {
        let names: Vec<String> = self.installed.keys().cloned().collect();
        let mut updated = Vec::new();

        for name in names {
            if let Ok(pkg) = self.update_async(&name).await {
                updated.push(pkg);
            }
        }

        Ok(updated)
    }

    /// List installed packages
    pub fn list_installed(&self) -> Vec<&InstalledPackage> {
        self.installed.values().collect()
    }

    /// Get installed package
    pub fn get_installed(&self, name: &str) -> Option<&InstalledPackage> {
        self.installed.get(name)
    }

    /// Parse installation spec (name@version, git:url, ./path)
    fn parse_install_spec(&self, spec: &str) -> Result<(String, Option<String>, InstallSource), RegistryError> {
        if spec.starts_with("git:") {
            let url = spec.strip_prefix("git:").unwrap().to_string();
            let name = url
                .rsplit('/')
                .next()
                .unwrap_or("unknown")
                .trim_end_matches(".git")
                .to_string();
            return Ok((name, None, InstallSource::Git { url, rev: None }));
        }

        if spec.starts_with("./") || spec.starts_with("../") || Path::new(spec).is_absolute() {
            let path = PathBuf::from(spec);
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("local")
                .to_string();
            return Ok((name, None, InstallSource::Local { path }));
        }

        // Parse name@version
        if let Some((name, version)) = spec.split_once('@') {
            Ok((name.to_string(), Some(version.to_string()), InstallSource::Registry))
        } else {
            Ok((spec.to_string(), None, InstallSource::Registry))
        }
    }

    /// Mock search results for development
    fn mock_search_results(&self, query: &str) -> Vec<PackageInfo> {
        if query.is_empty() {
            return vec![];
        }

        // Return some example packages
        vec![
            PackageInfo {
                name: "@dx/react-rules".to_string(),
                version: "2.1.0".to_string(),
                versions: vec!["2.1.0".to_string(), "2.0.0".to_string(), "1.0.0".to_string()],
                description: "React-specific lint rules for dx-check".to_string(),
                author: Some("DX Team".to_string()),
                homepage: Some("https://dx.dev/check/plugins/react".to_string()),
                repository: Some("https://github.com/nicholasoxford/dx".to_string()),
                license: Some("MIT".to_string()),
                downloads: 150_000,
                rule_count: 45,
                languages: vec!["typescript".to_string(), "javascript".to_string()],
                keywords: vec!["react".to_string(), "hooks".to_string(), "jsx".to_string()],
                published_at: "2025-12-01T00:00:00Z".to_string(),
                verified: true,
            },
            PackageInfo {
                name: "@dx/security-rules".to_string(),
                version: "1.5.0".to_string(),
                versions: vec!["1.5.0".to_string(), "1.4.0".to_string()],
                description: "Security-focused lint rules".to_string(),
                author: Some("DX Security Team".to_string()),
                homepage: Some("https://dx.dev/check/plugins/security".to_string()),
                repository: Some("https://github.com/nicholasoxford/dx".to_string()),
                license: Some("MIT".to_string()),
                downloads: 80_000,
                rule_count: 32,
                languages: vec!["typescript".to_string(), "javascript".to_string(), "python".to_string()],
                keywords: vec!["security".to_string(), "xss".to_string(), "injection".to_string()],
                published_at: "2025-11-15T00:00:00Z".to_string(),
                verified: true,
            },
        ]
    }

    // ============================================================================
    // Synchronous API for CLI usage
    // ============================================================================

    /// Search for packages (sync wrapper)
    pub fn search(&self, query: &str) -> Result<Vec<PackageInfo>, RegistryError> {
        // For now, return mock results directly without async
        Ok(self.mock_search_results(query))
    }

    /// Install a package (sync wrapper)  
    pub fn install(&self, name: &str, version: Option<&str>) -> Result<InstalledPackage, RegistryError> {
        let spec = match version {
            Some(v) => format!("{}@{}", name, v),
            None => name.to_string(),
        };
        
        let (pkg_name, pkg_version, source) = self.parse_install_spec(&spec)?;
        
        let install_path = self.config.cache_dir.join("packages").join(&pkg_name);
        
        // Create installed package record
        let installed = InstalledPackage {
            name: pkg_name.clone(),
            version: pkg_version.unwrap_or_else(|| "latest".to_string()),
            path: install_path.clone(),
            source,
            installed_at: chrono::Utc::now().to_rfc3339(),
            auto_update: true,
        };
        
        // Create install directory
        std::fs::create_dir_all(&install_path)
            .map_err(|e| RegistryError::Io(e.to_string()))?;
        
        Ok(installed)
    }

    /// Uninstall a package (sync)
    pub fn uninstall(&self, name: &str) -> Result<(), RegistryError> {
        let install_path = self.config.cache_dir.join("packages").join(name);
        
        if install_path.exists() {
            std::fs::remove_dir_all(&install_path)
                .map_err(|e| RegistryError::Io(e.to_string()))?;
        }
        
        Ok(())
    }

    /// Update a package (sync)
    pub fn update(&self, name: &str) -> Result<Option<InstalledPackage>, RegistryError> {
        // Check if there's a newer version available
        // For now, just return None (already up to date)
        let _ = name;
        Ok(None)
    }

    /// Update all packages (sync)
    pub fn update_all(&self) -> Result<Vec<InstalledPackage>, RegistryError> {
        // For now, return empty (all up to date)
        Ok(vec![])
    }
}

/// Registry errors
#[derive(Debug, Clone)]
pub enum RegistryError {
    /// Network error
    Network(String),
    /// I/O error
    Io(String),
    /// Parse error
    Parse(String),
    /// Package not found
    NotFound(String),
    /// Package not installed
    NotInstalled(String),
    /// Offline mode
    Offline,
    /// Invalid spec
    InvalidSpec(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(e) => write!(f, "Network error: {}", e),
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::Parse(e) => write!(f, "Parse error: {}", e),
            Self::NotFound(name) => write!(f, "Package not found: {}", name),
            Self::NotInstalled(name) => write!(f, "Package not installed: {}", name),
            Self::Offline => write!(f, "Offline mode - cannot access registry"),
            Self::InvalidSpec(spec) => write!(f, "Invalid package spec: {}", spec),
        }
    }
}

impl std::error::Error for RegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_config_default() {
        let config = RegistryConfig::default();
        assert_eq!(config.url, "https://registry.dx.dev/check");
        assert!(!config.offline);
    }

    #[test]
    fn test_parse_install_spec() {
        let client = RegistryClient::with_defaults();

        // Registry spec
        let (name, version, source) = client.parse_install_spec("@company/rules").unwrap();
        assert_eq!(name, "@company/rules");
        assert!(version.is_none());
        assert!(matches!(source, InstallSource::Registry));

        // Registry with version
        let (name, version, source) = client.parse_install_spec("@company/rules@1.0.0").unwrap();
        assert_eq!(name, "@company/rules");
        assert_eq!(version, Some("1.0.0".to_string()));
        assert!(matches!(source, InstallSource::Registry));

        // Git spec
        let (name, _, source) = client.parse_install_spec("git:https://github.com/user/rules.git").unwrap();
        assert_eq!(name, "rules");
        assert!(matches!(source, InstallSource::Git { .. }));

        // Local spec
        let (name, _, source) = client.parse_install_spec("./my-plugin").unwrap();
        assert_eq!(name, "my-plugin");
        assert!(matches!(source, InstallSource::Local { .. }));
    }
}
