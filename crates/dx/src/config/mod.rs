//! Configuration module
//!
//! Provides configuration loading and management for DX projects.

pub mod dx_config;

// Re-export configuration types for public API
// These are intentionally exported for library consumers even if not used internally
#[allow(unused_imports)]
pub use dx_config::{
    BuildConfig, DevConfig, DxConfig, FontToolConfig, IconToolConfig, MediaToolConfig,
    ProjectConfig, RuntimeConfig, StyleToolConfig, ToolsConfig, DEFAULT_CONFIG_FILE,
};
