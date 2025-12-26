//! dx-py-project-manager: Python project lifecycle management
//!
//! This crate provides project management functionality including:
//! - Python version management (discovery, installation, pinning)
//! - Virtual environment creation and management
//! - Workspace/monorepo support
//! - Global tool management

pub mod python;
pub mod tool;
pub mod venv;
pub mod workspace;

pub use dx_py_core::{Error, Result};
pub use python::{PythonManager, RealPythonManager, PythonInstall, PythonRelease};
pub use tool::{ToolManager, InstalledTool};
pub use venv::{VenvManager, RealVenvManager, Venv};
pub use workspace::{WorkspaceManager, WorkspaceConfig, WorkspaceMember, PathDependency};
