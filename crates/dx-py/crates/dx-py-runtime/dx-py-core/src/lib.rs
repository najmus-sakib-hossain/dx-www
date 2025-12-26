//! Core object model for DX-Py runtime
//!
//! Provides the fundamental Python object types and runtime structures.

pub mod header;
pub mod types;
pub mod pyint;
pub mod pystr;
pub mod pylist;
pub mod pydict;
pub mod pytuple;
pub mod pyfunction;
pub mod pyframe;
pub mod builtins;

pub use header::PyObjectHeader;
pub use types::PyType;
pub use pyint::PyInt;
pub use pystr::PyStr;
pub use pylist::PyList;
pub use pydict::PyDict;
pub use pytuple::PyTuple;
pub use pyfunction::PyFunction;
pub use pyframe::PyFrame;

/// Core error types
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("Value error: {0}")]
    ValueError(String),
    
    #[error("Index error: {0}")]
    IndexError(String),
    
    #[error("Key error: {0}")]
    KeyError(String),
    
    #[error("Attribute error: {0}")]
    AttributeError(String),
    
    #[error("Name error: {0}")]
    NameError(String),
    
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    
    #[error("Overflow error")]
    OverflowError,
}

pub type CoreResult<T> = Result<T, CoreError>;
