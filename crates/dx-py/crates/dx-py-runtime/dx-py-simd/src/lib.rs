//! DX-Py SIMD - SIMD-Accelerated String Operations
//!
//! This crate provides SIMD-accelerated string operations using AVX2/AVX-512/NEON
//! instructions for 8-15x speedup over scalar implementations.

pub mod engine;
pub mod avx2;
pub mod scalar;
pub mod dispatcher;

#[cfg(target_arch = "aarch64")]
pub mod neon;

pub use engine::SimdStringEngine;
pub use dispatcher::SimdDispatcher;

/// Get the best available SIMD engine for the current CPU
pub fn get_engine() -> Box<dyn SimdStringEngine> {
    SimdDispatcher::new().get_engine()
}
