//! # dx-compat-macro
//!
//! Compile-time macros compatibility layer.

#![warn(missing_docs)]

mod error;

pub use error::{MacroError, MacroResult};

/// Macro execution context.
pub struct MacroContext {
    _placeholder: (),
}

impl MacroContext {
    /// Create a new macro context.
    pub fn new() -> Self {
        Self { _placeholder: () }
    }

    /// Execute a macro function.
    pub fn execute<T>(&self, _func: impl Fn() -> T) -> MacroResult<T> {
        // TODO: Implement isolated execution
        Err(MacroError::ExecutionFailed(
            "Not implemented".to_string(),
        ))
    }
}

impl Default for MacroContext {
    fn default() -> Self {
        Self::new()
    }
}
