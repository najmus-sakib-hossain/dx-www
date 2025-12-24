//! Path utilities

use std::path::PathBuf;

/// Find the project root by looking for dx.toml
#[allow(dead_code)]
pub fn find_project_root() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;

    loop {
        if current.join("dx.toml").exists() {
            return Some(current);
        }

        if current.join("package.json").exists() {
            return Some(current);
        }

        if !current.pop() {
            return None;
        }
    }
}

/// Get the DX home directory (~/.dx)
#[allow(dead_code)]
pub fn dx_home() -> PathBuf {
    home::home_dir().map(|h| h.join(".dx")).unwrap_or_else(|| PathBuf::from(".dx"))
}

/// Get the cache directory
#[allow(dead_code)]
pub fn cache_dir() -> PathBuf {
    dx_home().join("cache")
}

/// Get the global bin directory
#[allow(dead_code)]
pub fn bin_dir() -> PathBuf {
    dx_home().join("bin")
}
