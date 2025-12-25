//! Python version management
//!
//! Provides functionality for discovering, installing, and managing Python versions.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Error, Result};

/// Information about a Python installation
#[derive(Debug, Clone)]
pub struct PythonInstall {
    /// Path to the Python executable
    pub path: PathBuf,
    /// Python version string (e.g., "3.12.0")
    pub version: String,
    /// Whether this is a system Python
    pub is_system: bool,
    /// Whether this is managed by dx-py
    pub is_managed: bool,
}

impl PythonInstall {
    /// Create a new Python installation info
    pub fn new(path: PathBuf, version: String) -> Self {
        Self {
            path,
            version,
            is_system: false,
            is_managed: false,
        }
    }

    /// Mark as system Python
    pub fn system(mut self) -> Self {
        self.is_system = true;
        self
    }

    /// Mark as managed by dx-py
    pub fn managed(mut self) -> Self {
        self.is_managed = true;
        self
    }
}

/// Python version manager
///
/// Handles discovery, installation, and management of Python versions.
pub struct PythonManager {
    /// Directory where managed Python versions are installed
    install_dir: PathBuf,
    /// URL for pre-built Python binaries
    builds_url: String,
    /// Cache of discovered Python installations
    discovered: HashMap<String, PythonInstall>,
}

impl PythonManager {
    /// Create a new Python manager with default settings
    pub fn new() -> Self {
        let install_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("dx-py")
            .join("python");

        Self {
            install_dir,
            builds_url: "https://github.com/indygreg/python-build-standalone/releases/download".to_string(),
            discovered: HashMap::new(),
        }
    }

    /// Create a Python manager with a custom install directory
    pub fn with_install_dir(install_dir: PathBuf) -> Self {
        Self {
            install_dir,
            builds_url: "https://github.com/indygreg/python-build-standalone/releases/download".to_string(),
            discovered: HashMap::new(),
        }
    }

    /// Get the install directory
    pub fn install_dir(&self) -> &Path {
        &self.install_dir
    }

    /// Discover system Python installations
    pub fn discover(&mut self) -> Vec<PythonInstall> {
        let mut found = Vec::new();

        // Check common locations based on platform
        #[cfg(unix)]
        let paths = vec![
            "/usr/bin/python3",
            "/usr/local/bin/python3",
            "/opt/homebrew/bin/python3",
        ];

        #[cfg(windows)]
        let paths = vec![
            "C:\\Python312\\python.exe",
            "C:\\Python311\\python.exe",
            "C:\\Python310\\python.exe",
            "C:\\Python39\\python.exe",
        ];

        for path_str in paths {
            let path = PathBuf::from(path_str);
            if path.exists() {
                if let Ok(version) = self.get_version(&path) {
                    let install = PythonInstall::new(path.clone(), version.clone()).system();
                    self.discovered.insert(version, install.clone());
                    found.push(install);
                }
            }
        }

        // Check PATH
        if let Ok(path_var) = std::env::var("PATH") {
            #[cfg(unix)]
            let separator = ':';
            #[cfg(windows)]
            let separator = ';';

            for dir in path_var.split(separator) {
                #[cfg(unix)]
                let python_path = PathBuf::from(dir).join("python3");
                #[cfg(windows)]
                let python_path = PathBuf::from(dir).join("python.exe");

                if python_path.exists() {
                    if let Ok(version) = self.get_version(&python_path) {
                        if !self.discovered.contains_key(&version) {
                            let install = PythonInstall::new(python_path, version.clone()).system();
                            self.discovered.insert(version, install.clone());
                            found.push(install);
                        }
                    }
                }
            }
        }

        // Check pyenv
        if let Ok(pyenv_root) = std::env::var("PYENV_ROOT") {
            let versions_dir = PathBuf::from(&pyenv_root).join("versions");
            if versions_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&versions_dir) {
                    for entry in entries.flatten() {
                        let version_dir = entry.path();
                        #[cfg(unix)]
                        let python_path = version_dir.join("bin").join("python");
                        #[cfg(windows)]
                        let python_path = version_dir.join("python.exe");

                        if python_path.exists() {
                            if let Ok(version) = self.get_version(&python_path) {
                                if !self.discovered.contains_key(&version) {
                                    let install = PythonInstall::new(python_path, version.clone());
                                    self.discovered.insert(version, install.clone());
                                    found.push(install);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check managed installations
        if self.install_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&self.install_dir) {
                for entry in entries.flatten() {
                    let version_dir = entry.path();
                    #[cfg(unix)]
                    let python_path = version_dir.join("bin").join("python3");
                    #[cfg(windows)]
                    let python_path = version_dir.join("python.exe");

                    if python_path.exists() {
                        if let Ok(version) = self.get_version(&python_path) {
                            if !self.discovered.contains_key(&version) {
                                let install = PythonInstall::new(python_path, version.clone()).managed();
                                self.discovered.insert(version, install.clone());
                                found.push(install);
                            }
                        }
                    }
                }
            }
        }

        found
    }

    /// Get the version of a Python executable
    pub fn get_version(&self, python_path: &Path) -> Result<String> {
        let output = Command::new(python_path)
            .args(["--version"])
            .output()
            .map_err(|e| Error::PythonNotFound(format!("Failed to run Python: {}", e)))?;

        if !output.status.success() {
            return Err(Error::PythonNotFound(format!(
                "Python returned error: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let version_str = String::from_utf8_lossy(&output.stdout);
        // Parse "Python 3.12.0" -> "3.12.0"
        let version = version_str
            .trim()
            .strip_prefix("Python ")
            .unwrap_or(version_str.trim())
            .to_string();

        Ok(version)
    }

    /// Find a Python installation matching the version constraint
    pub fn find(&self, version_constraint: &str) -> Option<&PythonInstall> {
        // Simple matching for now - exact version or prefix match
        let constraint = version_constraint.trim();

        // Try exact match first
        if let Some(install) = self.discovered.get(constraint) {
            return Some(install);
        }

        // Try prefix match (e.g., "3.12" matches "3.12.0")
        for (version, install) in &self.discovered {
            if version.starts_with(constraint) {
                return Some(install);
            }
        }

        None
    }

    /// Get the path where a version would be installed
    pub fn version_path(&self, version: &str) -> PathBuf {
        self.install_dir.join(version)
    }

    /// Check if a version is installed
    pub fn is_installed(&self, version: &str) -> bool {
        let version_dir = self.version_path(version);
        #[cfg(unix)]
        let python_path = version_dir.join("bin").join("python3");
        #[cfg(windows)]
        let python_path = version_dir.join("python.exe");

        python_path.exists()
    }

    /// Get the Python executable path for a version
    pub fn python_path(&self, version: &str) -> PathBuf {
        let version_dir = self.version_path(version);
        #[cfg(unix)]
        {
            version_dir.join("bin").join("python3")
        }
        #[cfg(windows)]
        {
            version_dir.join("python.exe")
        }
    }

    /// Pin a Python version for a project
    pub fn pin(&self, project_dir: &Path, version: &str) -> Result<()> {
        let pin_file = project_dir.join(".python-version");
        std::fs::write(&pin_file, format!("{}\n", version))
            .map_err(|e| Error::Io(e))?;
        Ok(())
    }

    /// Read the pinned Python version for a project
    pub fn read_pin(&self, project_dir: &Path) -> Result<Option<String>> {
        let pin_file = project_dir.join(".python-version");
        if !pin_file.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&pin_file)
            .map_err(|e| Error::Io(e))?;
        Ok(Some(content.trim().to_string()))
    }

    /// List all discovered Python installations
    pub fn list(&self) -> Vec<&PythonInstall> {
        self.discovered.values().collect()
    }
}

impl Default for PythonManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_python_manager_new() {
        let manager = PythonManager::new();
        assert!(manager.install_dir().to_string_lossy().contains("dx-py"));
    }

    #[test]
    fn test_python_manager_with_install_dir() {
        let temp_dir = TempDir::new().unwrap();
        let manager = PythonManager::with_install_dir(temp_dir.path().to_path_buf());
        assert_eq!(manager.install_dir(), temp_dir.path());
    }

    #[test]
    fn test_version_path() {
        let temp_dir = TempDir::new().unwrap();
        let manager = PythonManager::with_install_dir(temp_dir.path().to_path_buf());
        let path = manager.version_path("3.12.0");
        assert!(path.ends_with("3.12.0"));
    }

    #[test]
    fn test_pin_and_read() {
        let temp_dir = TempDir::new().unwrap();
        let manager = PythonManager::new();

        manager.pin(temp_dir.path(), "3.12.0").unwrap();
        let pinned = manager.read_pin(temp_dir.path()).unwrap();
        assert_eq!(pinned, Some("3.12.0".to_string()));
    }

    #[test]
    fn test_read_pin_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let manager = PythonManager::new();

        let pinned = manager.read_pin(temp_dir.path()).unwrap();
        assert_eq!(pinned, None);
    }
}
