//! DX Bundle Core - Arena allocator, core types, and zero-allocation infrastructure
//! 
//! This crate provides the foundation for 3x faster bundling than Bun:
//! - Arena allocator for zero-allocation transforms
//! - Core types using binary representations
//! - Thread-local arenas for parallel processing

#![allow(unsafe_code)]

pub mod arena;
pub mod types;
pub mod error;
pub mod config;
pub mod hash;

pub use arena::{BundleArena, ArenaOutput, with_arena};
pub use types::*;
pub use error::BundleError;
pub use config::BundleConfig;
pub use hash::ContentHash;
