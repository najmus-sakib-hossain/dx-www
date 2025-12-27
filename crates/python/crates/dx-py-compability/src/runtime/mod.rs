//! Python runtime detection module
//!
//! Provides functionality for detecting Python installations across the system.

mod detector;
mod version;
mod capabilities;

pub use detector::{RuntimeDetector, DetectionError};
pub use version::{PythonVersion, PreRelease, VersionParseError};
pub use capabilities::RuntimeCapabilities;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Installation source for Python
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallationSource {
    /// System-installed Python
    System,
    /// pyenv-managed Python
    Pyenv,
    /// Conda-managed Python
    Conda,
    /// Homebrew-installed Python (macOS)
    Homebrew,
    /// Windows Store Python
    WindowsStore,
    /// Custom installation path
    Custom(PathBuf),
}

/// Detected Python runtime information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRuntime {
    /// Path to the Python executable
    pub executable: PathBuf,
    /// Python version (major, minor, patch)
    pub version: PythonVersion,
    /// Platform architecture (x86_64, aarch64, etc.)
    pub architecture: crate::platform::Architecture,
    /// Installation source (system, pyenv, conda, etc.)
    pub source: InstallationSource,
    /// Available capabilities
    pub capabilities: RuntimeCapabilities,
}

impl PythonRuntime {
    /// Create a new Python runtime
    pub fn new(
        executable: PathBuf,
        version: PythonVersion,
        architecture: crate::platform::Architecture,
        source: InstallationSource,
    ) -> Self {
        Self {
            executable,
            version,
            architecture,
            source,
            capabilities: RuntimeCapabilities::default(),
        }
    }

    /// Set capabilities
    pub fn with_capabilities(mut self, capabilities: RuntimeCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }
}
