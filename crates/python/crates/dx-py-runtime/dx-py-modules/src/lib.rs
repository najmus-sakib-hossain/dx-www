//! DX-Py Modules (DPM) - Binary Module Format
//!
//! This crate implements the Binary Module Format for pre-compiled Python modules
//! with O(1) symbol lookup via perfect hashing.

pub mod format;
pub mod export_table;
pub mod loader;
pub mod compiler;

pub use format::*;
pub use export_table::ExportTable;
pub use loader::DpmLoader;
pub use compiler::DpmCompiler;
