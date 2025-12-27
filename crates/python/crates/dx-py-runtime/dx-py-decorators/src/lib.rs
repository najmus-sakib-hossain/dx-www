//! DX-Py Decorators - Compiler-Inlined Decorators
//!
//! This crate implements compiler-inlined decorators for zero-overhead
//! decorator application at compile time.

pub mod inlineable;
pub mod inliner;
pub mod lru_cache;
pub mod dataclass;

pub use inlineable::InlineableDecorator;
pub use inliner::DecoratorInliner;
pub use lru_cache::InlineLruCache;
pub use dataclass::DataclassInfo;
