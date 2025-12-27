//! Configuration module
//!
//! Provides configuration types and serialization for dx-py.

mod types;
mod serde_impl;

pub use types::DxPyConfig;
pub use serde_impl::{ConfigError, validate_config, parse_and_validate};
