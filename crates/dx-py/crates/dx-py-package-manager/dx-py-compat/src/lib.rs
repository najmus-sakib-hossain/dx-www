//! dx-py-compat: Compatibility layer for Python project formats
//!
//! This crate provides:
//! - pyproject.toml parsing and serialization
//! - Binary pyproject.dx format conversion
//! - Round-trip conversion between formats
//! - PEP 508 environment marker evaluation

pub mod markers;
pub mod pyproject;

pub use dx_py_core::{Error, Result};
pub use markers::{
    CompareOp, MarkerEnvironment, MarkerEvaluator, MarkerExpr, MarkerParseError, MarkerValue,
    MarkerVar,
};
pub use pyproject::{convert_from_binary, convert_to_binary, BuildSystem, ProjectSection, PyProjectToml};
