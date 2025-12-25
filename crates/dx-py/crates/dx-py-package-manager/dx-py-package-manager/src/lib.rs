//! dx-py-package-manager: High-performance Python package manager
//!
//! This crate provides binary package operations including DPP format handling,
//! dependency resolution, and zero-copy installation.

pub mod cache;
pub mod converter;
pub mod formats;
pub mod installer;
pub mod registry;
pub mod resolver;

pub use dx_py_core::{Error, Result};
pub use cache::GlobalCache;
pub use converter::{DppBuilder, WheelFile};
pub use formats::{DplBuilder, DplLockFile, DppPackage};
pub use installer::{InstallPackage, InstallFile, InstallResult, InstallStrategy, Installer};
pub use registry::{PyPiClient, PyPiPackageInfo, DependencySpec};
pub use resolver::{
    Dependency, HintCache, InMemoryProvider, Resolution, ResolvedPackage, Resolver,
    ResolutionSnapshot, VersionConstraint, VersionProvider,
};
