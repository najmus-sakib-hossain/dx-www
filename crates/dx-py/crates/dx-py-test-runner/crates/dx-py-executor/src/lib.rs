//! Work-stealing parallel executor
//!
//! This crate implements a work-stealing executor that distributes
//! tests across workers with dynamic load balancing.

pub use dx_py_core::{ExecutionError, TestCase, TestResult};
