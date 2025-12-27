//! Core types for dx-py-test-runner
//!
//! This crate defines the fundamental data structures used throughout
//! the test runner: TestCase, TestResult, TestId, and related types.

mod types;
mod errors;

pub use types::*;
pub use errors::*;

#[cfg(test)]
mod tests;
