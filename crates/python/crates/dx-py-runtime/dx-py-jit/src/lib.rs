//! DX-Py JIT - Tiered JIT Compiler with Cranelift Backend
//!
//! This crate implements a 4-tier JIT compilation strategy for the DX-Py runtime,
//! providing progressive optimization based on execution profiles.
//!
//! ## Compilation Tiers
//!
//! | Tier | Name | Threshold | Description |
//! |------|------|-----------|-------------|
//! | 0 | Interpreter | 0 | Bytecode interpretation with profiling |
//! | 1 | Baseline JIT | 100 calls | Fast compile, moderate speedup |
//! | 2 | Optimizing JIT | 1000 calls | Type-specialized with guards |
//! | 3 | AOT Optimized | 10000 calls | Profile-guided, persistent |
//!
//! ## Features
//!
//! - [`CompilationTier`]: Tier definitions and thresholds
//! - [`FunctionProfile`]: Execution profiling for tier promotion
//! - [`TypeFeedback`]: Type observation for specialization
//! - [`TieredJit`]: Main JIT compiler interface
//! - [`OsrManager`]: On-stack replacement for hot loops
//!
//! ## Usage
//!
//! ```rust
//! use dx_py_jit::{TieredJit, FunctionId, CompilationTier};
//!
//! let jit = TieredJit::new();
//! let func_id = FunctionId(1);
//!
//! // Get or create a profile
//! let profile = jit.get_profile(func_id, 100, 5);
//!
//! // Record calls
//! for _ in 0..100 {
//!     profile.record_call();
//! }
//!
//! // Check for tier promotion
//! if let Some(tier) = jit.check_promotion(func_id) {
//!     println!("Promote to {:?}", tier);
//! }
//! ```
//!
//! ## Type Feedback
//!
//! The JIT collects type information at each bytecode location:
//!
//! - **Monomorphic**: Single type observed - can emit specialized code
//! - **Polymorphic**: 2-4 types - emit type guards with fallback
//! - **Megamorphic**: Too many types - use generic code
//!
//! ## Deoptimization
//!
//! When type guards fail, the JIT deoptimizes back to the interpreter:
//!
//! 1. Save live values from registers/stack
//! 2. Reconstruct interpreter state
//! 3. Continue execution in interpreter
//! 4. Re-profile for better specialization

pub mod tier;
pub mod profile;
pub mod compiler;
pub mod osr;

pub use tier::CompilationTier;
pub use profile::{FunctionProfile, TypeFeedback, PyType};
pub use compiler::{TieredJit, FunctionId, CompiledFunction};
pub use osr::OsrManager;
