//! dx-py-package-manager: High-performance Python package manager
//!
//! This crate provides binary package operations including DPP format handling,
//! dependency resolution, and zero-copy installation.

pub mod cache;
pub mod converter;
pub mod download;
pub mod formats;
pub mod installer;
pub mod registry;
pub mod resolver;

pub use dx_py_core::{Error, Result};
pub use cache::GlobalCache;
pub use converter::{DppBuilder, WheelFile};
pub use download::{DownloadManager, DownloadRequest, DownloadResult, compute_sha256, verify_sha256};
pub use formats::{DplBuilder, DplLockFile, DppPackage};
pub use installer::{InstallPackage, InstallFile, InstallResult, InstallStrategy, Installer, WheelInstaller, InstalledPackage, RecordEntry};
pub use registry::{PyPiClient, AsyncPyPiClient, PyPiPackageInfo, DependencySpec, ReleaseFile, FileDigests, PackageInfo};
pub use resolver::{
    Dependency, HintCache, InMemoryProvider, PyPiResolver, Resolution, ResolvedPackage, Resolver,
    ResolutionSnapshot, VersionConstraint, VersionProvider,
};
