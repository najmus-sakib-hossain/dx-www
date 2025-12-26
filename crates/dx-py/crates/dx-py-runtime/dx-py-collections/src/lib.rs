//! DX-Py Collections - SIMD-Accelerated Collections
//!
//! This crate implements SIMD-optimized collection types:
//! - SimdList: Homogeneous list with SIMD operations
//! - SwissDict: Dictionary with SIMD probe

pub mod simd_list;
pub mod simd_storage;
pub mod swiss_dict;

pub use simd_list::SimdList;
pub use simd_storage::SimdStorage;
pub use swiss_dict::SwissDict;
