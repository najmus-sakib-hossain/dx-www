//! Cross-Process Shared Objects (Entangled Objects) for DX-Py runtime
//!
//! Provides shared memory objects that can be accessed across processes
//! with optimistic concurrency control.

pub mod region;
pub mod object;
pub mod handle;
pub mod array;

pub use region::SharedMemoryRegion;
pub use object::EntangledObject;
pub use handle::EntangledHandle;
pub use array::EntangledArray;

/// Entangled object error types
#[derive(Debug, thiserror::Error)]
pub enum EntangledError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Region not found: {0}")]
    RegionNotFound(String),
    
    #[error("Object not found")]
    ObjectNotFound,
    
    #[error("Concurrency conflict: version mismatch")]
    ConcurrencyConflict,
    
    #[error("Region full")]
    RegionFull,
    
    #[error("Invalid handle")]
    InvalidHandle,
    
    #[error("Type mismatch")]
    TypeMismatch,
}
