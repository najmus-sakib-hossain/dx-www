//! dx-py-compability: Python runtime detection, uv configuration compatibility, and platform support
//!
//! This crate provides compatibility functionality for dx-py including:
//! - Python runtime detection and version management
//! - uv configuration file parsing and compatibility
//! - PEP 508 environment marker evaluation
//! - Platform detection and wheel tag generation
//! - Virtual environment creation (PEP 405 compliant)
//! - Configuration serialization and validation

pub mod config;
pub mod markers;
pub mod platform;
pub mod runtime;
pub mod uv;
pub mod venv;

// Re-export main types
pub use config::{DxPyConfig, ConfigError};
pub use markers::{MarkerEnvironment, MarkerEvaluator, MarkerError, MarkerExpr, MarkerOp};
pub use platform::{Platform, PlatformDetector, Os, Architecture, Libc, WheelTag, WheelTagGenerator};
pub use runtime::{PythonRuntime, PythonVersion, RuntimeDetector, InstallationSource, RuntimeCapabilities, DetectionError};
pub use uv::{UvConfig, UvConfigLoader, PythonPreference};
pub use venv::{VenvBuilder, VenvOptions, VirtualEnvironment, VenvError};
