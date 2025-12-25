//! dx-py-project-manager: Python project lifecycle management
//!
//! This crate provides project management functionality including:
//! - Python version management (discovery, installation, pinning)
//! - Virtual environment creation and management
//! - Workspace/monorepo support

pub mod python;
pub mod venv;
pub mod workspace;

pub use dx_py_core::{Error, Result};
pub use python::PythonManager;
pub use venv::VenvManager;
pub use workspace::WorkspaceManager;
