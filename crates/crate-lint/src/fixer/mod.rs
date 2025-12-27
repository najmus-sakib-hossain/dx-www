//! Auto-fixer module for applying fixes to crate violations
//!
//! This module provides templates and auto-fix functionality for common violations.

mod templates;
mod cargo_fixer;
mod doc_fixer;
mod auto_fixer;

pub use templates::*;
pub use cargo_fixer::*;
pub use doc_fixer::*;
pub use auto_fixer::*;
