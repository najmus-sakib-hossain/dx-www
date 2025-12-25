//! Package installer with hard link optimization
//!
//! Installs DPP packages to site-packages using hard links when possible,
//! falling back to copy when hard links aren't supported (e.g., cross-filesystem).

use std::fs;
use std::path::{Path, PathBuf};

use crate::cache::GlobalCache;
use crate::Result;

/// Installation strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStrategy {
    /// Hard links from cache - fast, deduplication
    HardLink,
    /// Copy files - fallback, always works
    Copy,
}

impl Default for InstallStrategy {
    fn default() -> Self {
        Self::HardLink
    }
}

/// File entry for installation
#[derive(Debug, Clone)]
pub struct InstallFile {
    /// Relative path within the package
    pub path: String,
    /// File content
    pub content: Vec<u8>,
}

/// Package to install
#[derive(Debug, Clone)]
pub struct InstallPackage {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Content hash (BLAKE3)
    pub hash: [u8; 32],
    /// Files to install
    pub files: Vec<InstallFile>,
}

/// Installation result
#[derive(Debug, Default)]
pub struct InstallResult {
    /// Number of files installed
    pub files_installed: u64,
    /// Number of hard links created
    pub hard_links: u64,
    /// Number of files copied
    pub copies: u64,
    /// Total bytes installed
    pub bytes_installed: u64,
}

/// Zero-copy installer
pub struct Installer {
    /// Global package cache
    cache: GlobalCache,
    /// Installation strategy
    strategy: InstallStrategy,
}

impl Installer {
    /// Create a new installer with the given cache
    pub fn new(cache: GlobalCache) -> Self {
        Self {
            cache,
            strategy: InstallStrategy::default(),
        }
    }

    /// Create a new installer with a specific strategy
    pub fn with_strategy(cache: GlobalCache, strategy: InstallStrategy) -> Self {
        Self { cache, strategy }
    }

    /// Get the cache
    pub fn cache(&self) -> &GlobalCache {
        &self.cache
    }

    /// Get the installation strategy
    pub fn strategy(&self) -> InstallStrategy {
        self.strategy
    }

    /// Install a package to site-packages
    pub fn install(&self, package: &InstallPackage, site_packages: &Path) -> Result<InstallResult> {
        // First, ensure package is in cache
        let cache_path = self.ensure_cached(package)?;

        // Install based on strategy
        match self.strategy {
            InstallStrategy::HardLink => {
                self.install_hardlink(package, &cache_path, site_packages)
            }
            InstallStrategy::Copy => self.install_copy(package, site_packages),
        }
    }

    /// Install multiple packages
    pub fn install_all(
        &self,
        packages: &[InstallPackage],
        site_packages: &Path,
    ) -> Result<InstallResult> {
        let mut total = InstallResult::default();

        for package in packages {
            let result = self.install(package, site_packages)?;
            total.files_installed += result.files_installed;
            total.hard_links += result.hard_links;
            total.copies += result.copies;
            total.bytes_installed += result.bytes_installed;
        }

        Ok(total)
    }

    /// Ensure package is in cache, return cache path
    fn ensure_cached(&self, package: &InstallPackage) -> Result<PathBuf> {
        // Check if already cached
        if self.cache.contains(&package.hash) {
            return Ok(self.cache.get_path(&package.hash));
        }

        // Build package data and store in cache
        // Note: We use store() not store_verified() because the hash was computed
        // from the original package content, not our serialization format
        let mut data = Vec::new();

        // Simple format: file count + entries
        data.extend_from_slice(&(package.files.len() as u32).to_le_bytes());

        for file in &package.files {
            let path_bytes = file.path.as_bytes();
            data.extend_from_slice(&(path_bytes.len() as u16).to_le_bytes());
            data.extend_from_slice(path_bytes);
            data.extend_from_slice(&(file.content.len() as u64).to_le_bytes());
            data.extend_from_slice(&file.content);
        }

        self.cache.store(&package.hash, &data)
    }

    /// Install using hard links from cache
    fn install_hardlink(
        &self,
        package: &InstallPackage,
        cache_path: &Path,
        site_packages: &Path,
    ) -> Result<InstallResult> {
        let mut result = InstallResult::default();

        // Extract files from cache to a temp location first
        let cache_extract_dir = cache_path.with_extension("extracted");
        if !cache_extract_dir.exists() {
            self.extract_to_dir(package, &cache_extract_dir)?;
        }

        // Create hard links to site-packages
        for file in &package.files {
            let src = cache_extract_dir.join(&file.path);
            let dst = site_packages.join(&file.path);

            // Create parent directories
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent)?;
            }

            // Try hard link first, fall back to copy
            if src.exists() {
                match fs::hard_link(&src, &dst) {
                    Ok(()) => {
                        result.hard_links += 1;
                    }
                    Err(_) => {
                        // Fall back to copy (cross-filesystem or other issue)
                        fs::copy(&src, &dst)?;
                        result.copies += 1;
                    }
                }
            } else {
                // Source doesn't exist in extracted cache, write directly
                fs::write(&dst, &file.content)?;
                result.copies += 1;
            }

            result.files_installed += 1;
            result.bytes_installed += file.content.len() as u64;
        }

        Ok(result)
    }

    /// Install by copying files
    fn install_copy(&self, package: &InstallPackage, site_packages: &Path) -> Result<InstallResult> {
        let mut result = InstallResult::default();

        for file in &package.files {
            let dst = site_packages.join(&file.path);

            // Create parent directories
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent)?;
            }

            // Write file
            fs::write(&dst, &file.content)?;

            result.files_installed += 1;
            result.copies += 1;
            result.bytes_installed += file.content.len() as u64;
        }

        Ok(result)
    }

    /// Extract package files to a directory
    fn extract_to_dir(&self, package: &InstallPackage, dir: &Path) -> Result<()> {
        fs::create_dir_all(dir)?;

        for file in &package.files {
            let path = dir.join(&file.path);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&path, &file.content)?;
        }

        Ok(())
    }

    /// Uninstall a package from site-packages
    pub fn uninstall(&self, package_name: &str, site_packages: &Path) -> Result<u64> {
        let mut removed = 0;

        // Look for package directory (normalized name)
        let normalized = package_name.replace('-', "_");
        let pkg_dir = site_packages.join(&normalized);

        if pkg_dir.exists() {
            removed += self.count_files(&pkg_dir)?;
            fs::remove_dir_all(&pkg_dir)?;
        }

        // Also check for .dist-info directory
        for entry in fs::read_dir(site_packages)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with(&format!("{}-", normalized))
                && name_str.ends_with(".dist-info")
            {
                removed += self.count_files(&entry.path())?;
                fs::remove_dir_all(entry.path())?;
            }
        }

        Ok(removed)
    }

    /// Count files in a directory recursively
    fn count_files(&self, dir: &Path) -> Result<u64> {
        let mut count = 0;

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                count += self.count_files(&path)?;
            } else {
                count += 1;
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_package(name: &str, version: &str) -> InstallPackage {
        let normalized = name.replace('-', "_");
        let files = vec![
            InstallFile {
                path: format!("{}/__init__.py", normalized),
                content: b"# init".to_vec(),
            },
            InstallFile {
                path: format!("{}/main.py", normalized),
                content: b"def main(): pass".to_vec(),
            },
        ];

        // Compute hash from files
        let mut hasher = blake3::Hasher::new();
        for file in &files {
            hasher.update(file.path.as_bytes());
            hasher.update(&file.content);
        }
        let hash = *hasher.finalize().as_bytes();

        InstallPackage {
            name: name.to_string(),
            version: version.to_string(),
            hash,
            files,
        }
    }

    #[test]
    fn test_install_copy() {
        let cache_dir = TempDir::new().unwrap();
        let site_packages = TempDir::new().unwrap();

        let cache = GlobalCache::new(cache_dir.path()).unwrap();
        let installer = Installer::with_strategy(cache, InstallStrategy::Copy);

        let package = create_test_package("test-pkg", "1.0.0");
        let result = installer.install(&package, site_packages.path()).unwrap();

        assert_eq!(result.files_installed, 2);
        assert_eq!(result.copies, 2);
        assert_eq!(result.hard_links, 0);

        // Verify files exist
        assert!(site_packages.path().join("test_pkg/__init__.py").exists());
        assert!(site_packages.path().join("test_pkg/main.py").exists());
    }

    #[test]
    fn test_install_hardlink() {
        let cache_dir = TempDir::new().unwrap();
        let site_packages = TempDir::new().unwrap();

        let cache = GlobalCache::new(cache_dir.path()).unwrap();
        let installer = Installer::with_strategy(cache, InstallStrategy::HardLink);

        let package = create_test_package("test-pkg", "1.0.0");
        let result = installer.install(&package, site_packages.path()).unwrap();

        assert_eq!(result.files_installed, 2);
        // Hard links may or may not work depending on filesystem
        assert!(result.hard_links + result.copies == 2);

        // Verify files exist
        assert!(site_packages.path().join("test_pkg/__init__.py").exists());
        assert!(site_packages.path().join("test_pkg/main.py").exists());
    }

    #[test]
    fn test_install_multiple() {
        let cache_dir = TempDir::new().unwrap();
        let site_packages = TempDir::new().unwrap();

        let cache = GlobalCache::new(cache_dir.path()).unwrap();
        let installer = Installer::with_strategy(cache, InstallStrategy::Copy);

        let packages = vec![
            create_test_package("pkg-a", "1.0.0"),
            create_test_package("pkg-b", "2.0.0"),
        ];

        let result = installer.install_all(&packages, site_packages.path()).unwrap();

        assert_eq!(result.files_installed, 4);

        // Verify files exist
        assert!(site_packages.path().join("pkg_a/__init__.py").exists());
        assert!(site_packages.path().join("pkg_b/__init__.py").exists());
    }

    #[test]
    fn test_uninstall() {
        let cache_dir = TempDir::new().unwrap();
        let site_packages = TempDir::new().unwrap();

        let cache = GlobalCache::new(cache_dir.path()).unwrap();
        let installer = Installer::with_strategy(cache, InstallStrategy::Copy);

        let package = create_test_package("test-pkg", "1.0.0");
        installer.install(&package, site_packages.path()).unwrap();

        // Verify installed
        assert!(site_packages.path().join("test_pkg/__init__.py").exists());

        // Uninstall
        let removed = installer.uninstall("test-pkg", site_packages.path()).unwrap();
        assert_eq!(removed, 2);

        // Verify removed
        assert!(!site_packages.path().join("test_pkg").exists());
    }
}
