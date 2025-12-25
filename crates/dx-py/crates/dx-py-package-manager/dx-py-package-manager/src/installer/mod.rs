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


use std::io::{BufRead, BufReader, Read};
use zip::ZipArchive;
use crate::Error;

/// Installed package information
#[derive(Debug, Clone)]
pub struct InstalledPackage {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// List of installed files
    pub files: Vec<PathBuf>,
    /// Path to .dist-info directory
    pub dist_info: PathBuf,
}

/// RECORD file entry
#[derive(Debug, Clone)]
pub struct RecordEntry {
    /// File path relative to site-packages
    pub path: String,
    /// Hash algorithm and digest (e.g., "sha256=...")
    pub hash: Option<String>,
    /// File size in bytes
    pub size: Option<u64>,
}

impl RecordEntry {
    /// Parse a RECORD line
    pub fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.is_empty() || parts[0].is_empty() {
            return None;
        }

        let path = parts[0].to_string();
        let hash = parts.get(1).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let size = parts.get(2).and_then(|s| s.parse().ok());

        Some(Self { path, hash, size })
    }
}

/// Real wheel installer that extracts wheel files to site-packages
pub struct WheelInstaller {
    /// Global package cache
    cache: GlobalCache,
    /// Site-packages directory
    site_packages: PathBuf,
    /// Installation strategy
    strategy: InstallStrategy,
}

impl WheelInstaller {
    /// Create a new wheel installer
    pub fn new(cache: GlobalCache, site_packages: PathBuf) -> Self {
        Self {
            cache,
            site_packages,
            strategy: InstallStrategy::default(),
        }
    }

    /// Create a wheel installer with a specific strategy
    pub fn with_strategy(cache: GlobalCache, site_packages: PathBuf, strategy: InstallStrategy) -> Self {
        Self {
            cache,
            site_packages,
            strategy,
        }
    }

    /// Get the site-packages directory
    pub fn site_packages(&self) -> &Path {
        &self.site_packages
    }

    /// Install a wheel file from bytes
    pub fn install_wheel(&self, wheel_data: &[u8]) -> Result<InstalledPackage> {
        use std::io::Cursor;
        
        let cursor = Cursor::new(wheel_data);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| Error::Cache(format!("Failed to open wheel: {}", e)))?;

        // Find the .dist-info directory
        let dist_info_name = self.find_dist_info(&mut archive)?;
        let (name, version) = self.parse_dist_info_name(&dist_info_name)?;

        let mut installed_files = Vec::new();

        // Extract all files
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| Error::Cache(format!("Failed to read wheel entry: {}", e)))?;

            let file_path = file.name().to_string();
            
            // Skip directories
            if file_path.ends_with('/') {
                continue;
            }

            // Handle .data directory specially
            let dest_path = if file_path.contains(".data/") {
                self.handle_data_file(&file_path, &name)?
            } else {
                self.site_packages.join(&file_path)
            };

            // Create parent directories
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Read and write file content
            let mut content = Vec::new();
            file.read_to_end(&mut content)
                .map_err(|e| Error::Cache(format!("Failed to read file: {}", e)))?;

            // Write atomically using temp file
            let temp_path = dest_path.with_extension("tmp");
            fs::write(&temp_path, &content)?;
            fs::rename(&temp_path, &dest_path)?;

            installed_files.push(dest_path);
        }

        let dist_info_path = self.site_packages.join(&dist_info_name);

        Ok(InstalledPackage {
            name,
            version,
            files: installed_files,
            dist_info: dist_info_path,
        })
    }

    /// Install a wheel from the cache
    pub fn install_from_cache(&self, hash: &[u8; 32]) -> Result<InstalledPackage> {
        let data = self.cache.get(hash)?;
        self.install_wheel(&data)
    }

    /// Find the .dist-info directory name in the wheel
    fn find_dist_info<R: Read + std::io::Seek>(&self, archive: &mut ZipArchive<R>) -> Result<String> {
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                let name = file.name().to_string();
                if name.contains(".dist-info/") {
                    // Extract the dist-info directory name
                    let parts: Vec<&str> = name.split('/').collect();
                    if !parts.is_empty() && parts[0].ends_with(".dist-info") {
                        return Ok(parts[0].to_string());
                    }
                }
            }
        }
        Err(Error::Cache("No .dist-info directory found in wheel".to_string()))
    }

    /// Parse package name and version from dist-info directory name
    fn parse_dist_info_name(&self, dist_info: &str) -> Result<(String, String)> {
        // Format: {name}-{version}.dist-info
        let without_suffix = dist_info.strip_suffix(".dist-info")
            .ok_or_else(|| Error::Cache("Invalid dist-info name".to_string()))?;

        // Find the last hyphen that separates name from version
        let parts: Vec<&str> = without_suffix.rsplitn(2, '-').collect();
        if parts.len() != 2 {
            return Err(Error::Cache("Invalid dist-info name format".to_string()));
        }

        let version = parts[0].to_string();
        let name = parts[1].to_string();

        Ok((name, version))
    }

    /// Handle files in the .data directory
    fn handle_data_file(&self, file_path: &str, _package_name: &str) -> Result<PathBuf> {
        // .data directory structure: {name}-{version}.data/{category}/{path}
        // Categories: scripts, headers, data, purelib, platlib
        
        let parts: Vec<&str> = file_path.split('/').collect();
        if parts.len() < 3 {
            return Ok(self.site_packages.join(file_path));
        }

        let category = parts[1];
        let rest: PathBuf = parts[2..].iter().collect();

        match category {
            "scripts" => {
                // Scripts go to bin directory (parent of site-packages)
                let bin_dir = self.site_packages.parent()
                    .map(|p| p.join("Scripts"))
                    .unwrap_or_else(|| self.site_packages.join("Scripts"));
                Ok(bin_dir.join(rest))
            }
            "headers" => {
                // Headers go to include directory
                let include_dir = self.site_packages.parent()
                    .map(|p| p.join("include"))
                    .unwrap_or_else(|| self.site_packages.join("include"));
                Ok(include_dir.join(rest))
            }
            "data" => {
                // Data files go to the root of the environment
                let data_dir = self.site_packages.parent()
                    .unwrap_or(&self.site_packages);
                Ok(data_dir.join(rest))
            }
            "purelib" | "platlib" => {
                // These go directly to site-packages
                Ok(self.site_packages.join(rest))
            }
            _ => {
                // Unknown category, put in site-packages
                Ok(self.site_packages.join(file_path))
            }
        }
    }

    /// Read RECORD file from an installed package
    pub fn read_record(&self, dist_info: &Path) -> Result<Vec<RecordEntry>> {
        let record_path = dist_info.join("RECORD");
        if !record_path.exists() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(&record_path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Some(entry) = RecordEntry::parse(&line) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Uninstall a package using its RECORD file
    pub fn uninstall(&self, package_name: &str) -> Result<u64> {
        let normalized = package_name.replace('-', "_").to_lowercase();
        let mut removed = 0;

        // Find the .dist-info directory
        let dist_info = self.find_installed_dist_info(&normalized)?;

        // Read RECORD file
        let records = self.read_record(&dist_info)?;

        // Remove all files listed in RECORD
        for entry in &records {
            let file_path = self.site_packages.join(&entry.path);
            if file_path.exists() {
                if file_path.is_file() {
                    fs::remove_file(&file_path)?;
                    removed += 1;
                }
            }
        }

        // Remove the .dist-info directory
        if dist_info.exists() {
            let count = self.count_files_in_dir(&dist_info)?;
            fs::remove_dir_all(&dist_info)?;
            removed += count;
        }

        // Clean up empty directories
        self.cleanup_empty_dirs(&self.site_packages)?;

        Ok(removed)
    }

    /// Find the .dist-info directory for an installed package
    fn find_installed_dist_info(&self, normalized_name: &str) -> Result<PathBuf> {
        for entry in fs::read_dir(&self.site_packages)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy().to_lowercase();

            if name_str.starts_with(&format!("{}-", normalized_name))
                && name_str.ends_with(".dist-info")
            {
                return Ok(entry.path());
            }
        }

        Err(Error::PackageNotFound(normalized_name.to_string()))
    }

    /// Count files in a directory
    fn count_files_in_dir(&self, dir: &Path) -> Result<u64> {
        let mut count = 0;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.path().is_file() {
                count += 1;
            } else if entry.path().is_dir() {
                count += self.count_files_in_dir(&entry.path())?;
            }
        }
        Ok(count)
    }

    /// Clean up empty directories
    fn cleanup_empty_dirs(&self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.cleanup_empty_dirs(&path)?;
                // Try to remove if empty
                let _ = fs::remove_dir(&path);
            }
        }
        Ok(())
    }

    /// Generate entry point scripts
    pub fn generate_scripts(&self, dist_info: &Path) -> Result<Vec<PathBuf>> {
        let entry_points_path = dist_info.join("entry_points.txt");
        if !entry_points_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&entry_points_path)?;
        let mut scripts = Vec::new();
        let mut in_console_scripts = false;

        for line in content.lines() {
            let line = line.trim();
            if line == "[console_scripts]" {
                in_console_scripts = true;
                continue;
            }
            if line.starts_with('[') {
                in_console_scripts = false;
                continue;
            }
            if in_console_scripts && line.contains('=') {
                if let Some(script_path) = self.create_script(line)? {
                    scripts.push(script_path);
                }
            }
        }

        Ok(scripts)
    }

    /// Create a single entry point script
    fn create_script(&self, entry: &str) -> Result<Option<PathBuf>> {
        let parts: Vec<&str> = entry.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Ok(None);
        }

        let script_name = parts[0].trim();
        let target = parts[1].trim();

        // Parse target: module:function
        let target_parts: Vec<&str> = target.splitn(2, ':').collect();
        if target_parts.len() != 2 {
            return Ok(None);
        }

        let module = target_parts[0].trim();
        let function = target_parts[1].trim();

        // Get scripts directory
        let scripts_dir = self.site_packages.parent()
            .map(|p| p.join("Scripts"))
            .unwrap_or_else(|| self.site_packages.join("Scripts"));
        fs::create_dir_all(&scripts_dir)?;

        // Generate script content
        let wrapper = format!(
            r#"#!python
import sys
from {} import {}
if __name__ == '__main__':
    sys.exit({}())
"#,
            module, function, function
        );

        #[cfg(windows)]
        {
            // On Windows, create a .py file that can be run
            let py_path = scripts_dir.join(format!("{}-script.py", script_name));
            fs::write(&py_path, &wrapper)?;
            // The .exe would need to be a launcher, for now just create the .py
            return Ok(Some(py_path));
        }

        #[cfg(not(windows))]
        {
            let script_path = scripts_dir.join(script_name);
            fs::write(&script_path, &wrapper)?;
            // Make executable on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&script_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&script_path, perms)?;
            }
            Ok(Some(script_path))
        }
    }
}
