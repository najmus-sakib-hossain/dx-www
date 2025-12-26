//! DX-Py SIMD - SIMD-Accelerated String Operations
//!
//! This crate provides SIMD-accelerated string operations using AVX2/AVX-512/NEON
//! instructions for 8-15x speedup over scalar implementations.
//!
//! ## Features
//!
//! - AVX2 acceleration on x86_64 (32 bytes/iteration)
//! - NEON acceleration on ARM64 (16 bytes/iteration)
//! - Automatic CPU detection and dispatch
//! - Scalar fallback for compatibility
//!
//! ## Usage
//!
//! ```rust
//! use dx_py_simd::get_engine;
//!
//! let engine = get_engine();
//! let pos = engine.find("hello world", "world");
//! assert_eq!(pos, Some(6));
//! ```

pub mod engine;
pub mod avx2;
pub mod scalar;
pub mod dispatcher;
pub mod neon;

pub use engine::SimdStringEngine;
pub use dispatcher::SimdDispatcher;
pub use neon::NeonStringEngine;

/// Get the best available SIMD engine for the current CPU
pub fn get_engine() -> Box<dyn SimdStringEngine> {
    SimdDispatcher::new().get_engine()
}
