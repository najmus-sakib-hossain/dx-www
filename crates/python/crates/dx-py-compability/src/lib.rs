//! dx-py-compability: Python runtime detection, uv configuration compatibility, and platform support
//!
//! This crate provides compatibility functionality for dx-py including:
//! - Python runtime detection and version management
//! - uv configuration file parsing and compatibility
//! - PEP 508 environment marker evaluation
//! - Platform detection and wheel tag generation
//! - Virtual environment creation (PEP 405 compliant)
//! - Configuration serialization and validation
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use dx_py_compability::{RuntimeDetector, VenvBuilder, VenvOptions};
//!
//! // Detect Python installations
//! let detector = RuntimeDetector::new();
//! let runtimes = detector.detect_all().unwrap();
//!
//! // Create a virtual environment
//! if let Some(python) = runtimes.first() {
//!     let builder = VenvBuilder::new(python.clone())
//!         .with_options(VenvOptions::new().system_site_packages(false));
//!     let venv = builder.build(std::path::Path::new(".venv")).unwrap();
//!     println!("Created venv at: {:?}", venv.path);
//! }
//! ```
//!
//! # Modules
//!
//! - [`config`] - Configuration types and serialization
//! - [`markers`] - PEP 508 environment marker evaluation
//! - [`platform`] - Platform detection and wheel tag generation
//! - [`runtime`] - Python runtime detection
//! - [`uv`] - uv configuration compatibility
//! - [`venv`] - Virtual environment creation

pub mod config;
pub mod markers;
pub mod platform;
pub mod runtime;
pub mod uv;
pub mod venv;

// Re-export main types from config
pub use config::{DxPyConfig, ConfigError, validate_config, parse_and_validate};

// Re-export main types from markers
pub use markers::{MarkerEnvironment, MarkerEvaluator, MarkerError, MarkerExpr, MarkerOp};

// Re-export main types from platform
pub use platform::{Platform, PlatformDetector, Os, Architecture, Libc, WheelTag, WheelTagGenerator, ManylinuxTag, MusllinuxTag};

// Re-export main types from runtime
pub use runtime::{PythonRuntime, PythonVersion, PreRelease, VersionParseError, RuntimeDetector, InstallationSource, RuntimeCapabilities, DetectionError};

// Re-export main types from uv
pub use uv::{UvConfig, UvConfigLoader, PythonPreference};

// Re-export main types from venv
pub use venv::{VenvBuilder, VenvOptions, VirtualEnvironment, VenvError, PyvenvCfg, ActivationScripts};
