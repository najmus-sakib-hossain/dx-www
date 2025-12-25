//! dx-py-package-manager: High-performance Python package manager
//!
//! This crate provides binary package operations including DPP format handling,
//! dependency resolution, and zero-copy installation.

pub mod cache;
pub mod formats;
pub mod installer;
pub mod resolver;

pub use dx_py_core::{Error, Result};
pub use formats::{DplBuilder, DplLockFile, DppPackage};
