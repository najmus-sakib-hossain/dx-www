//! # dx-compat-plugin
//!
//! Plugin system compatibility layer.

#![warn(missing_docs)]

mod builder;
mod error;
mod loader;

pub use builder::PluginBuilder;
pub use error::{PluginError, PluginResult};
pub use loader::Loader;

/// Plugin definition.
pub struct Plugin {
    /// Plugin name
    pub name: String,
    /// Setup function
    pub setup: Box<dyn Fn(&mut PluginBuilder) + Send + Sync>,
}

/// Register a plugin.
pub fn register(_plugin: Plugin) {
    // TODO: Implement plugin registration
}

/// Load handler result.
#[derive(Debug, Clone)]
pub struct OnLoadResult {
    /// File contents
    pub contents: String,
    /// Loader to use
    pub loader: Loader,
}

/// Resolve handler result.
#[derive(Debug, Clone)]
pub struct OnResolveResult {
    /// Resolved path
    pub path: String,
    /// Namespace
    pub namespace: Option<String>,
}
