//! Dependency graph for smart change detection
//!
//! This crate builds and maintains an import graph of Python files
//! to identify which tests are affected by file changes.

pub use dx_py_core::{GraphError, TestId};
