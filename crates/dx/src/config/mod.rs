//! Configuration module
//!
//! Provides configuration loading and management for DX projects.

pub mod dx_config;

pub use dx_config::{
    BuildConfig, DevConfig, DxConfig, FontToolConfig, IconToolConfig, MediaToolConfig,
    ProjectConfig, RuntimeConfig, StyleToolConfig, ToolsConfig, DEFAULT_CONFIG_FILE,
};
