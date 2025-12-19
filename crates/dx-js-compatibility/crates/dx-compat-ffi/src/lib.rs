//! # dx-compat-ffi
//!
//! Foreign Function Interface compatibility layer.

#![warn(missing_docs)]

mod error;
mod library;
mod types;

pub use error::{FfiError, FfiResult};
pub use library::{dlopen, DynamicLibrary};
pub use types::FfiType;

/// Pointer operations module.
pub mod ptr;
