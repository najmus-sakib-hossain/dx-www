//! DX-Py FFI - Memory Teleportation (Zero-Copy) FFI
//!
//! This crate implements zero-copy data sharing with C extensions like NumPy:
//! - Direct pointer sharing for array data
//! - SIMD operations on teleported arrays
//! - GIL-free execution for pure computation
//! - C-API compatibility layer

pub mod teleport;
pub mod capi;
pub mod fast_ffi;

pub use teleport::{TeleportedArray, DType};
pub use capi::CApiCompat;
pub use fast_ffi::FastFfi;
