//! DX-Py Parallel - Thread-Per-Core Parallel Executor
//!
//! This crate implements a high-performance parallel executor with:
//! - One thread per physical CPU core
//! - Core pinning for cache locality
//! - Work-stealing for load balancing

pub mod worker;
pub mod executor;
pub mod task;
pub mod parallel_object;

pub use worker::Worker;
pub use executor::ParallelExecutor;
pub use task::Task;
pub use parallel_object::ParallelPyObject;
