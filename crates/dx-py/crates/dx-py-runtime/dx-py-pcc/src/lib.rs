//! Persistent Compilation Cache (PCC) for DX-Py runtime
//!
//! Provides persistent storage for JIT-compiled code across sessions.

pub mod signature;
pub mod artifact;
pub mod cache;
pub mod storage;

pub use signature::FunctionSignature;
pub use artifact::{CachedArtifact, CompilationTier, Relocation};
pub use cache::PersistentCompilationCache;
pub use storage::CodeStorage;

/// PCC error types
#[derive(Debug, thiserror::Error)]
pub enum PccError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Cache not found")]
    NotFound,
    
    #[error("Cache corrupted")]
    Corrupted,
    
    #[error("Version mismatch")]
    VersionMismatch,
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Cache full")]
    CacheFull,
}
