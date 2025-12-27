//! dx-crate-lint: Validation tool for DX ecosystem crate standards
//!
//! This crate provides validation and linting for Rust crates in the DX ecosystem,
//! ensuring consistent metadata, naming conventions, documentation, and structure.

pub mod models;
pub mod scanner;
pub mod validators;
pub mod report;

pub use models::*;
pub use scanner::*;
pub use validators::*;
pub use report::*;
