//! Crate scanner for discovering and parsing workspace crates

use crate::models::{CargoToml, CrateInfo, CrateType, WorkspaceConfig, WorkspacePackage};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

/// Errors that can occur during crate scanning
#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Workspace root not found at {0}")]
    WorkspaceNotFound(PathBuf),
    
    #[error("Failed to read file {path}: {source}")]
    FileReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Failed to parse Cargo.toml at {path}: {source}")]
    TomlParseError {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    
    #[error("No [workspace] section found in root Cargo.toml")]
    NotAWorkspace,
}

/// Scanner for discovering crates in a workspace
pub struct CrateScanner {
    workspace_root: PathBuf,
    workspace_cargo: CargoToml,
    excluded_paths: HashSet<PathBuf>,
}

impl CrateScanner {
    /// Create a new scanner for the given workspace root
    pub fn new(workspace_root: impl AsRef<Path>) -> Result<Self, ScanError> {
        let workspace_root = workspace_root.as_ref().to_path_buf();
        let cargo_path = workspace_root.join("Cargo.toml");
        
        if !cargo_path.exists() {
            return Err(ScanError::WorkspaceNotFound(workspace_root));
        }
        
        let content = fs::read_to_string(&cargo_path).map_err(|e| ScanError::FileReadError {
            path: cargo_path.clone(),
            source: e,
        })?;
        
        let workspace_cargo = CargoToml::parse(&content).map_err(|e| ScanError::TomlParseError {
            path: cargo_path,
            source: e,
        })?;
        
        if !workspace_cargo.is_workspace_root() {
            return Err(ScanError::NotAWorkspace);
        }
        
        // Build excluded paths set
        let excluded_paths = workspace_cargo
            .workspace
            .as_ref()
            .map(|w| {
                w.exclude
                    .iter()
                    .map(|p| workspace_root.join(p))
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(Self {
            workspace_root,
            workspace_cargo,
            excluded_paths,
        })
    }
    
    /// Get the workspace root path
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }
    
    /// Get the workspace configuration
    pub fn workspace_config(&self) -> Option<&WorkspaceConfig> {
        self.workspace_cargo.workspace.as_ref()
    }
    
    /// Get workspace package defaults
    pub fn workspace_package(&self) -> Option<&WorkspacePackage> {
        self.workspace_config().and_then(|w| w.package.as_ref())
    }
    
    /// Get workspace members
    pub fn workspace_members(&self) -> Vec<&str> {
        self.workspace_config()
            .map(|w| w.members.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    
    /// Scan the workspace and return all discovered crates
    pub fn scan(&self) -> Result<Vec<CrateInfo>, ScanError> {
        let mut crates = Vec::new();
        
        // First, scan explicit workspace members
        if let Some(workspace) = &self.workspace_cargo.workspace {
            for member_pattern in &workspace.members {
                let member_crates = self.scan_member_pattern(member_pattern)?;
                crates.extend(member_crates);
            }
        }
        
        Ok(crates)
    }
    
    /// Scan a member pattern (may contain globs)
    fn scan_member_pattern(&self, pattern: &str) -> Result<Vec<CrateInfo>, ScanError> {
        let mut crates = Vec::new();
        
        // Handle simple paths (no glob)
        let member_path = self.workspace_root.join(pattern);
        
        if member_path.is_dir() {
            if let Some(crate_info) = self.scan_crate_dir(&member_path)? {
                crates.push(crate_info);
            }
        }
        
        Ok(crates)
    }
    
    /// Scan a single crate directory
    fn scan_crate_dir(&self, path: &Path) -> Result<Option<CrateInfo>, ScanError> {
        // Check if excluded
        if self.is_excluded(path) {
            return Ok(None);
        }
        
        let cargo_path = path.join("Cargo.toml");
        if !cargo_path.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&cargo_path).map_err(|e| ScanError::FileReadError {
            path: cargo_path.clone(),
            source: e,
        })?;
        
        let cargo_toml = CargoToml::parse(&content).map_err(|e| ScanError::TomlParseError {
            path: cargo_path,
            source: e,
        })?;
        
        // Skip workspace roots (nested workspaces) and non-package Cargo.toml files
        if !cargo_toml.is_package() {
            return Ok(None);
        }
        
        // Determine crate type from path
        let relative_path = path.strip_prefix(&self.workspace_root).unwrap_or(path);
        let crate_type = CrateType::from_path(relative_path);
        
        // Collect files in the crate directory
        let files = self.collect_crate_files(path);
        
        let package = cargo_toml.package.as_ref().unwrap();
        let crate_info = CrateInfo::new(
            package.name.clone(),
            path.to_path_buf(),
            cargo_toml,
            crate_type,
        )
        .with_files(files);
        
        Ok(Some(crate_info))
    }
    
    /// Check if a path is excluded
    fn is_excluded(&self, path: &Path) -> bool {
        self.excluded_paths.iter().any(|excluded| {
            path.starts_with(excluded)
        })
    }
    
    /// Collect all files in a crate directory (non-recursive for top-level)
    fn collect_crate_files(&self, path: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        // Collect top-level files
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    files.push(entry_path);
                }
            }
        }
        
        // Also check for src directory
        let src_dir = path.join("src");
        if src_dir.is_dir() {
            for entry in WalkDir::new(&src_dir).max_depth(1) {
                if let Ok(entry) = entry {
                    if entry.path().is_file() {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }
        
        files
    }
    
    /// Scan all crates including nested ones (for comprehensive analysis)
    pub fn scan_all(&self) -> Result<Vec<CrateInfo>, ScanError> {
        let mut crates = Vec::new();
        let crates_dir = self.workspace_root.join("crates");
        
        if !crates_dir.exists() {
            return self.scan();
        }
        
        // Walk the crates directory looking for Cargo.toml files
        for entry in WalkDir::new(&crates_dir)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e.path()))
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            
            if entry.file_name() == "Cargo.toml" && entry.path().is_file() {
                let crate_dir = entry.path().parent().unwrap();
                
                // Skip target directories
                if crate_dir.to_string_lossy().contains("target") {
                    continue;
                }
                
                if let Ok(Some(crate_info)) = self.scan_crate_dir(crate_dir) {
                    crates.push(crate_info);
                }
            }
        }
        
        Ok(crates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_workspace() -> TempDir {
        let temp = TempDir::new().unwrap();
        
        // Create root Cargo.toml
        let root_cargo = r#"
[workspace]
resolver = "2"
members = ["crates/test-crate"]

[workspace.package]
version = "0.1.0"
edition = "2024"
"#;
        fs::write(temp.path().join("Cargo.toml"), root_cargo).unwrap();
        
        // Create crates directory
        let crate_dir = temp.path().join("crates/test-crate");
        fs::create_dir_all(&crate_dir).unwrap();
        fs::create_dir_all(crate_dir.join("src")).unwrap();
        
        // Create crate Cargo.toml
        let crate_cargo = r#"
[package]
name = "test-crate"
version.workspace = true
edition.workspace = true
"#;
        fs::write(crate_dir.join("Cargo.toml"), crate_cargo).unwrap();
        fs::write(crate_dir.join("src/lib.rs"), "// lib").unwrap();
        fs::write(crate_dir.join("README.md"), "# Test").unwrap();
        
        temp
    }

    #[test]
    fn test_scanner_creation() {
        let temp = create_test_workspace();
        let scanner = CrateScanner::new(temp.path()).unwrap();
        
        assert!(scanner.workspace_config().is_some());
        assert_eq!(scanner.workspace_members(), vec!["crates/test-crate"]);
    }

    #[test]
    fn test_scanner_scan() {
        let temp = create_test_workspace();
        let scanner = CrateScanner::new(temp.path()).unwrap();
        let crates = scanner.scan().unwrap();
        
        assert_eq!(crates.len(), 1);
        assert_eq!(crates[0].name, "test-crate");
        assert!(crates[0].cargo_toml.package.as_ref().unwrap().version.is_workspace());
    }

    #[test]
    fn test_scanner_not_workspace() {
        let temp = TempDir::new().unwrap();
        
        // Create a non-workspace Cargo.toml
        let cargo = r#"
[package]
name = "not-workspace"
version = "0.1.0"
"#;
        fs::write(temp.path().join("Cargo.toml"), cargo).unwrap();
        
        let result = CrateScanner::new(temp.path());
        assert!(matches!(result, Err(ScanError::NotAWorkspace)));
    }

    #[test]
    fn test_crate_files_collection() {
        let temp = create_test_workspace();
        let scanner = CrateScanner::new(temp.path()).unwrap();
        let crates = scanner.scan().unwrap();
        
        assert!(!crates[0].files.is_empty());
        assert!(crates[0].has_file("README.md"));
        assert!(crates[0].has_file("Cargo.toml"));
    }
}
