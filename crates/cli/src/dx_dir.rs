//! DX Directory Management
//!
//! Manages the `.dx` folder structure for caching and runtime data.

use anyhow::Result;
use std::path::{Path, PathBuf};

/// All DX cache subdirectories
pub const DX_SUBDIRS: &[&str] = &[
    "www",
    "extension",
    "cli",
    "cache",
    "runtime",
    "package-manager",
    "workspace",
    "test-runner",
    "compatibility",
    "serializer",
    "forge",
    "style",
    "ui",
    "font",
    "media",
    "icon",
    "i18n",
    "auth",
    "test",
    "driven",
    "generator",
];

/// DX directory manager
pub struct DxDir {
    root: PathBuf,
}

impl DxDir {
    /// Create a new DxDir manager for the given project root
    pub fn new(project_root: impl AsRef<Path>) -> Self {
        Self {
            root: project_root.as_ref().join(".dx"),
        }
    }

    /// Get the .dx directory path
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Initialize the .dx directory structure
    pub fn init(&self) -> Result<()> {
        // Create root .dx directory
        std::fs::create_dir_all(&self.root)?;

        // Create all subdirectories
        for subdir in DX_SUBDIRS {
            let path = self.root.join(subdir);
            std::fs::create_dir_all(&path)?;
        }

        // Create .gitignore in .dx folder
        let gitignore_path = self.root.join(".gitignore");
        if !gitignore_path.exists() {
            std::fs::write(&gitignore_path, Self::default_gitignore())?;
        }

        Ok(())
    }

    /// Get path to a specific subdirectory
    pub fn subdir(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    // Convenience methods for each subdirectory
    pub fn www(&self) -> PathBuf { self.subdir("www") }
    pub fn extension(&self) -> PathBuf { self.subdir("extension") }
    pub fn cli(&self) -> PathBuf { self.subdir("cli") }
    pub fn cache(&self) -> PathBuf { self.subdir("cache") }
    pub fn runtime(&self) -> PathBuf { self.subdir("runtime") }
    pub fn package_manager(&self) -> PathBuf { self.subdir("package-manager") }
    pub fn workspace(&self) -> PathBuf { self.subdir("workspace") }
    pub fn test_runner(&self) -> PathBuf { self.subdir("test-runner") }
    pub fn compatibility(&self) -> PathBuf { self.subdir("compatibility") }
    pub fn serializer(&self) -> PathBuf { self.subdir("serializer") }
    pub fn forge(&self) -> PathBuf { self.subdir("forge") }
    pub fn style(&self) -> PathBuf { self.subdir("style") }
    pub fn ui(&self) -> PathBuf { self.subdir("ui") }
    pub fn font(&self) -> PathBuf { self.subdir("font") }
    pub fn media(&self) -> PathBuf { self.subdir("media") }
    pub fn icon(&self) -> PathBuf { self.subdir("icon") }
    pub fn i18n(&self) -> PathBuf { self.subdir("i18n") }
    pub fn auth(&self) -> PathBuf { self.subdir("auth") }
    pub fn test(&self) -> PathBuf { self.subdir("test") }
    pub fn driven(&self) -> PathBuf { self.subdir("driven") }
    pub fn generator(&self) -> PathBuf { self.subdir("generator") }

    /// Check if .dx directory exists
    pub fn exists(&self) -> bool {
        self.root.exists()
    }

    /// Get default .gitignore content
    fn default_gitignore() -> &'static str {
        r#"# DX Cache Directory
# This folder contains cached data and should generally be ignored

# Cache files
cache/
*.cache
*.tmp

# Runtime data
runtime/
*.pid
*.sock

# Build artifacts
www/dist/
www/build/
style/dist/

# Package manager cache
package-manager/

# Test artifacts
test-runner/coverage/
test-runner/reports/
test/results/

# Forge daemon data
forge/*.log
forge/*.pid

# Generated files
generator/output/

# Keep structure files
!.gitkeep
"#
    }

    /// Clean all cache directories
    pub fn clean_all(&self) -> Result<()> {
        for subdir in DX_SUBDIRS {
            self.clean_subdir(subdir)?;
        }
        Ok(())
    }

    /// Clean a specific subdirectory
    pub fn clean_subdir(&self, name: &str) -> Result<()> {
        let path = self.subdir(name);
        if path.exists() {
            // Remove contents but keep the directory
            for entry in std::fs::read_dir(&path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(&path)?;
                } else {
                    std::fs::remove_file(&path)?;
                }
            }
        }
        Ok(())
    }

    /// Get total size of .dx directory
    pub fn total_size(&self) -> Result<u64> {
        dir_size(&self.root)
    }

    /// Get size of a specific subdirectory
    pub fn subdir_size(&self, name: &str) -> Result<u64> {
        dir_size(&self.subdir(name))
    }

    /// Get status of all subdirectories
    pub fn status(&self) -> Vec<DxSubdirStatus> {
        DX_SUBDIRS
            .iter()
            .map(|name| {
                let path = self.subdir(name);
                let exists = path.exists();
                let size = if exists {
                    dir_size(&path).unwrap_or(0)
                } else {
                    0
                };
                let file_count = if exists {
                    count_files(&path).unwrap_or(0)
                } else {
                    0
                };
                DxSubdirStatus {
                    name: name.to_string(),
                    exists,
                    size,
                    file_count,
                }
            })
            .collect()
    }
}

/// Status of a .dx subdirectory
#[derive(Debug, Clone)]
pub struct DxSubdirStatus {
    pub name: String,
    pub exists: bool,
    pub size: u64,
    pub file_count: usize,
}

/// Calculate directory size recursively
fn dir_size(path: &Path) -> Result<u64> {
    let mut size = 0;
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                size += dir_size(&path)?;
            } else {
                size += entry.metadata()?.len();
            }
        }
    }
    Ok(size)
}

/// Count files in directory recursively
fn count_files(path: &Path) -> Result<usize> {
    let mut count = 0;
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                count += count_files(&path)?;
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// Format bytes as human-readable string
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
