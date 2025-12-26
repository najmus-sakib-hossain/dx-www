//! Test discovery engine using tree-sitter for zero-import scanning
//!
//! This crate provides Rust-based AST scanning of Python files to discover
//! test functions without importing them, achieving 100x faster discovery.

mod parser;
mod scanner;
mod index;

pub use parser::PythonParser;
pub use scanner::{TestScanner, DiscoveredTest};
pub use index::{TestIndex, TestIndexBuilder};

#[cfg(test)]
mod tests;
