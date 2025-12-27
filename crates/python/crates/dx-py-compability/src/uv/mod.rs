//! uv configuration compatibility module
//!
//! Provides functionality for parsing and applying uv configuration files.

mod config;

pub use config::{UvConfig, UvConfigLoader, PythonPreference, MergedConfig};
