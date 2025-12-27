//! Bytecode interpreter for DX-Py runtime
//!
//! Implements the dispatch loop for DPB bytecode execution.
//!
//! ## Features
//!
//! - Bytecode dispatch with computed goto optimization
//! - JIT integration for tiered compilation
//! - Async integration for async/await support
//! - Error propagation with Python exception semantics

pub mod dispatch;
pub mod vm;
pub mod opcodes;
pub mod jit_integration;
pub mod async_integration;

pub use dispatch::Dispatcher;
pub use vm::VirtualMachine;
pub use jit_integration::{JitIntegration, JitError, JitStats};
pub use async_integration::{AsyncRuntime, AsyncError, FutureResult};

use dx_py_core::RuntimeError;

/// Interpreter error types
#[derive(Debug, thiserror::Error)]
pub enum InterpreterError {
    #[error("Runtime error: {0}")]
    Runtime(String),
    
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("Name error: {0}")]
    NameError(String),
    
    #[error("Value error: {0}")]
    ValueError(String),
    
    #[error("Index error: {0}")]
    IndexError(String),
    
    #[error("Key error: {0}")]
    KeyError(String),
    
    #[error("Attribute error: {0}")]
    AttributeError(String),
    
    #[error("Import error: {0}")]
    ImportError(String),
    
    #[error("Stop iteration")]
    StopIteration,
    
    #[error("System exit: {0}")]
    SystemExit(i32),
    
    #[error("Core error: {0}")]
    Core(#[from] dx_py_core::CoreError),
    
    #[error("{0}")]
    RuntimeError(#[from] RuntimeError),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;
