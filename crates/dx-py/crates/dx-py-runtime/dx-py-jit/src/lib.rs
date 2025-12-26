//! DX-Py JIT - Tiered JIT Compiler with Cranelift Backend
//!
//! This crate implements a 4-tier JIT compilation strategy:
//! - Tier 0: Interpreter with profiling
//! - Tier 1: Baseline JIT (100 calls)
//! - Tier 2: Optimizing JIT with type specialization (1000 calls)
//! - Tier 3: AOT with profile-guided optimization (10000 calls)

pub mod tier;
pub mod profile;
pub mod compiler;
pub mod osr;

pub use tier::CompilationTier;
pub use profile::{FunctionProfile, TypeFeedback};
pub use compiler::TieredJit;
pub use osr::OsrManager;
