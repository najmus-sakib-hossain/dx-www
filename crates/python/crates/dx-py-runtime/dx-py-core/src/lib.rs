//! Core object model for DX-Py runtime
//!
//! Provides the fundamental Python object types and runtime structures.
//!
//! ## Features
//!
//! - Lock-free reference counting via PyObjectHeader
//! - Core Python types: int, str, list, dict, tuple, function
//! - Stack frames for execution
//! - Built-in functions (print, len, type, range, etc.)
//! - Standard library modules (sys, os, io, json)
//! - Debugging support with line tables and tracebacks
//!
//! ## Error Handling
//!
//! All operations return `RuntimeResult<T>` for graceful error handling.

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
pub mod stdlib;
pub mod debug;
pub mod error;

pub use header::PyObjectHeader;
pub use types::PyType;
pub use pyint::PyInt;
pub use pystr::PyStr;
pub use pylist::PyList;
pub use pydict::PyDict;
pub use pytuple::PyTuple;
pub use pyfunction::PyFunction;
pub use pyframe::PyFrame;
pub use debug::{Traceback, TracebackFrame, ExceptionInfo, LineTable, Debugger};
pub use error::{RuntimeError, RuntimeResult};

/// Legacy error types (deprecated, use RuntimeError instead)
#[derive(Debug, thiserror::Error)]
#[deprecated(since = "0.2.0", note = "Use RuntimeError instead")]
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

#[allow(deprecated)]
pub type CoreResult<T> = Result<T, CoreError>;
