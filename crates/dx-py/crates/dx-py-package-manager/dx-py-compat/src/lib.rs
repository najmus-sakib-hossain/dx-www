//! dx-py-compat: Compatibility layer for Python project formats
//!
//! This crate provides:
//! - pyproject.toml parsing and serialization
//! - Binary pyproject.dx format conversion
//! - Round-trip conversion between formats

pub mod pyproject;

pub use dx_py_core::{Error, Result};
pub use pyproject::{PyProjectToml, ProjectSection, BuildSystem, convert_to_binary, convert_from_binary};
