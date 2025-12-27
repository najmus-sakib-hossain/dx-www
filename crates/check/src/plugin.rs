//! Plugin System for Custom Rules
//!
//! Enables users to create and load custom lint rules via plugins.
//!
//! # Architecture
//!
//! Plugins can be written in:
//! 1. Rust (compiled to shared libraries)
//! 2. WASM (portable, sandboxed)
//! 3. JavaScript (via dx-js-runtime)
//!
//! # Plugin Discovery
//!
//! Plugins are discovered from:
//! 1. `./plugins/` directory in workspace
//! 2. `~/.dx-check/plugins/` global directory
//! 3. npm packages with `dx-check-plugin-*` prefix
//!
//! # Example Plugin (Rust)
//!
//! ```rust,ignore
//! use dx_check::plugin::{Plugin, PluginMeta, Rule};
//!
//! #[dx_check::plugin]
//! pub struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn meta(&self) -> PluginMeta {
//!         PluginMeta {
//!             name: "my-plugin",
//!             version: "1.0.0",
//!             rules: vec!["my-rule-1", "my-rule-2"],
//!         }
//!     }
//!
//!     fn rules(&self) -> Vec<Box<dyn Rule>> {
//!         vec![Box::new(MyRule1), Box::new(MyRule2)]
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::rules::Rule;

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginMeta {
    /// Plugin name (unique identifier)
    pub name: String,
    /// Plugin version (semver)
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Author name
    pub author: Option<String>,
    /// Homepage URL
    pub homepage: Option<String>,
    /// List of rule names provided by this plugin
    pub rules: Vec<String>,
    /// Plugin type
    pub plugin_type: PluginType,
}

/// Plugin type determines how the plugin is loaded
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    /// Native Rust plugin (shared library)
    Native,
    /// WebAssembly plugin (portable, sandboxed)
    Wasm,
    /// JavaScript plugin (via dx-js-runtime)
    JavaScript,
    /// Built-in plugin (compiled into dx-check)
    Builtin,
}

impl Default for PluginType {
    fn default() -> Self {
        Self::Builtin
    }
}

/// Plugin trait - implement this to create a custom plugin
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn meta(&self) -> PluginMeta;

    /// Get all rules provided by this plugin
    fn rules(&self) -> Vec<Box<dyn Rule>>;

    /// Called when plugin is loaded
    fn on_load(&self) {}

    /// Called when plugin is unloaded
    fn on_unload(&self) {}

    /// Get configuration schema (JSON Schema)
    fn config_schema(&self) -> Option<String> {
        None
    }
}

/// Plugin instance with loaded state
pub struct LoadedPlugin {
    /// Plugin metadata
    pub meta: PluginMeta,
    /// Plugin implementation
    plugin: Arc<dyn Plugin>,
    /// Rules from this plugin
    rules: Vec<Box<dyn Rule>>,
    /// Whether plugin is enabled
    pub enabled: bool,
    /// Plugin source path
    pub source_path: Option<PathBuf>,
}

impl LoadedPlugin {
    /// Create a new loaded plugin
    pub fn new(plugin: Arc<dyn Plugin>) -> Self {
        let meta = plugin.meta();
        let rules = plugin.rules();
        plugin.on_load();
        
        Self {
            meta,
            plugin,
            rules,
            enabled: true,
            source_path: None,
        }
    }

    /// Get rules from this plugin
    pub fn rules(&self) -> &[Box<dyn Rule>] {
        &self.rules
    }

    /// Enable or disable the plugin
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Drop for LoadedPlugin {
    fn drop(&mut self) {
        self.plugin.on_unload();
    }
}

/// Plugin loader - discovers and loads plugins
pub struct PluginLoader {
    /// Search paths for plugins
    search_paths: Vec<PathBuf>,
    /// Loaded plugins
    plugins: HashMap<String, LoadedPlugin>,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new() -> Self {
        Self {
            search_paths: Vec::new(),
            plugins: HashMap::new(),
        }
    }

    /// Add a search path for plugins
    pub fn add_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.push(path.into());
    }

    /// Add default search paths
    pub fn with_default_paths(mut self) -> Self {
        // Current directory plugins
        self.search_paths.push(PathBuf::from("./plugins"));
        self.search_paths.push(PathBuf::from("./.dx-check/plugins"));

        // Home directory plugins
        if let Some(home) = dirs::home_dir() {
            self.search_paths.push(home.join(".dx-check").join("plugins"));
        }

        // XDG config plugins
        if let Some(config) = dirs::config_dir() {
            self.search_paths.push(config.join("dx-check").join("plugins"));
        }

        self
    }

    /// Discover plugins in search paths
    pub fn discover(&mut self) -> Vec<PluginMeta> {
        let mut discovered = Vec::new();

        for path in &self.search_paths {
            if !path.exists() {
                continue;
            }

            // Look for plugin manifest files
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    
                    // Check for dx-plugin.toml manifest
                    let manifest_path = if entry_path.is_dir() {
                        entry_path.join("dx-plugin.toml")
                    } else if entry_path.extension().map_or(false, |e| e == "toml") {
                        entry_path.clone()
                    } else {
                        continue;
                    };

                    if manifest_path.exists() {
                        if let Ok(meta) = self.parse_manifest(&manifest_path) {
                            discovered.push(meta);
                        }
                    }
                }
            }
        }

        discovered
    }

    /// Parse plugin manifest file
    fn parse_manifest(&self, path: &Path) -> Result<PluginMeta, PluginError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PluginError::ManifestRead(e.to_string()))?;
        
        let manifest: PluginManifest = toml::from_str(&content)
            .map_err(|e| PluginError::ManifestParse(e.to_string()))?;

        Ok(PluginMeta {
            name: manifest.plugin.name,
            version: manifest.plugin.version,
            description: manifest.plugin.description.unwrap_or_default(),
            author: manifest.plugin.author,
            homepage: manifest.plugin.homepage,
            rules: manifest.plugin.rules.unwrap_or_default(),
            plugin_type: match manifest.plugin.plugin_type.as_deref() {
                Some("wasm") => PluginType::Wasm,
                Some("javascript") | Some("js") => PluginType::JavaScript,
                Some("native") | Some("rust") => PluginType::Native,
                _ => PluginType::Native,
            },
        })
    }

    /// Load a plugin by name
    pub fn load(&mut self, name: &str) -> Result<&LoadedPlugin, PluginError> {
        // Check if already loaded
        if self.plugins.contains_key(name) {
            return Ok(self.plugins.get(name).unwrap());
        }

        // Find plugin in search paths
        for path in &self.search_paths.clone() {
            let plugin_dir = path.join(name);
            let manifest_path = plugin_dir.join("dx-plugin.toml");

            if manifest_path.exists() {
                let meta = self.parse_manifest(&manifest_path)?;
                
                // Load based on plugin type
                match meta.plugin_type {
                    PluginType::Native => {
                        // For now, native plugins are not dynamically loadable
                        // They must be compiled into dx-check
                        return Err(PluginError::NotSupported(
                            "Native plugins must be compiled into dx-check".to_string()
                        ));
                    }
                    PluginType::Wasm => {
                        return self.load_wasm_plugin(&plugin_dir, meta);
                    }
                    PluginType::JavaScript => {
                        return self.load_js_plugin(&plugin_dir, meta);
                    }
                    PluginType::Builtin => {
                        return Err(PluginError::NotSupported(
                            "Cannot dynamically load builtin plugins".to_string()
                        ));
                    }
                }
            }
        }

        Err(PluginError::NotFound(name.to_string()))
    }

    /// Load a WASM plugin
    fn load_wasm_plugin(&mut self, _dir: &Path, meta: PluginMeta) -> Result<&LoadedPlugin, PluginError> {
        // WASM plugin loading will be implemented with wasmer/wasmtime
        // For now, return a placeholder
        let plugin = Arc::new(WasmPluginStub { meta: meta.clone() });
        let loaded = LoadedPlugin::new(plugin);
        
        self.plugins.insert(meta.name.clone(), loaded);
        Ok(self.plugins.get(&meta.name).unwrap())
    }

    /// Load a JavaScript plugin
    fn load_js_plugin(&mut self, _dir: &Path, meta: PluginMeta) -> Result<&LoadedPlugin, PluginError> {
        // JavaScript plugin loading will be implemented with dx-js-runtime
        // For now, return a placeholder
        let plugin = Arc::new(JsPluginStub { meta: meta.clone() });
        let loaded = LoadedPlugin::new(plugin);
        
        self.plugins.insert(meta.name.clone(), loaded);
        Ok(self.plugins.get(&meta.name).unwrap())
    }

    /// Register a built-in plugin
    pub fn register_builtin(&mut self, plugin: Arc<dyn Plugin>) {
        let meta = plugin.meta();
        let name = meta.name.clone();
        let loaded = LoadedPlugin::new(plugin);
        self.plugins.insert(name, loaded);
    }

    /// Get all loaded plugins
    pub fn plugins(&self) -> impl Iterator<Item = &LoadedPlugin> {
        self.plugins.values()
    }

    /// Get a loaded plugin by name
    pub fn get(&self, name: &str) -> Option<&LoadedPlugin> {
        self.plugins.get(name)
    }

    /// Unload a plugin
    pub fn unload(&mut self, name: &str) -> bool {
        self.plugins.remove(name).is_some()
    }

    /// Get all rules from all enabled plugins
    pub fn all_rules(&self) -> Vec<&dyn Rule> {
        self.plugins
            .values()
            .filter(|p| p.enabled)
            .flat_map(|p| p.rules().iter().map(|r| r.as_ref()))
            .collect()
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin manifest file format (dx-plugin.toml)
#[derive(Debug, serde::Deserialize)]
struct PluginManifest {
    plugin: PluginSection,
}

#[derive(Debug, serde::Deserialize)]
struct PluginSection {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    homepage: Option<String>,
    #[serde(rename = "type")]
    plugin_type: Option<String>,
    rules: Option<Vec<String>>,
}

/// Plugin loading errors
#[derive(Debug, Clone)]
pub enum PluginError {
    /// Plugin not found
    NotFound(String),
    /// Failed to read manifest
    ManifestRead(String),
    /// Failed to parse manifest
    ManifestParse(String),
    /// Plugin type not supported
    NotSupported(String),
    /// Plugin load failed
    LoadFailed(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "Plugin not found: {}", name),
            Self::ManifestRead(e) => write!(f, "Failed to read manifest: {}", e),
            Self::ManifestParse(e) => write!(f, "Failed to parse manifest: {}", e),
            Self::NotSupported(msg) => write!(f, "Not supported: {}", msg),
            Self::LoadFailed(e) => write!(f, "Plugin load failed: {}", e),
        }
    }
}

impl std::error::Error for PluginError {}

/// Stub for WASM plugins (placeholder until wasmer integration)
struct WasmPluginStub {
    meta: PluginMeta,
}

impl Plugin for WasmPluginStub {
    fn meta(&self) -> PluginMeta {
        self.meta.clone()
    }

    fn rules(&self) -> Vec<Box<dyn Rule>> {
        Vec::new() // Will be populated from WASM exports
    }
}

/// Stub for JavaScript plugins (placeholder until dx-js-runtime integration)
struct JsPluginStub {
    meta: PluginMeta,
}

impl Plugin for JsPluginStub {
    fn meta(&self) -> PluginMeta {
        self.meta.clone()
    }

    fn rules(&self) -> Vec<Box<dyn Rule>> {
        Vec::new() // Will be populated from JS exports
    }
}

/// Built-in plugin with core rules
pub struct BuiltinPlugin;

impl Plugin for BuiltinPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "builtin".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Core dx-check rules".to_string(),
            author: Some("DX Team".to_string()),
            homepage: Some("https://dx.dev/check".to_string()),
            rules: vec![
                "no-console".to_string(),
                "no-debugger".to_string(),
                "no-unused-vars".to_string(),
                "eqeqeq".to_string(),
                "prefer-const".to_string(),
                "no-var".to_string(),
                "no-eval".to_string(),
                "no-with".to_string(),
            ],
            plugin_type: PluginType::Builtin,
        }
    }

    fn rules(&self) -> Vec<Box<dyn Rule>> {
        use crate::rules::builtin::*;
        
        vec![
            Box::new(NoConsole::new(vec![])),
            Box::<NoDebugger>::default(),
            Box::<NoUnusedVars>::default(),
            Box::new(Eqeqeq::new(false)),
            Box::<PreferConst>::default(),
            Box::<NoVar>::default(),
            Box::<NoEval>::default(),
            Box::<NoWith>::default(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_loader_creation() {
        let loader = PluginLoader::new();
        assert_eq!(loader.plugins().count(), 0);
    }

    #[test]
    fn test_builtin_plugin() {
        let plugin = BuiltinPlugin;
        let meta = plugin.meta();
        assert_eq!(meta.name, "builtin");
        assert_eq!(meta.plugin_type, PluginType::Builtin);
        assert!(!meta.rules.is_empty());
    }

    #[test]
    fn test_register_builtin() {
        let mut loader = PluginLoader::new();
        loader.register_builtin(Arc::new(BuiltinPlugin));
        assert_eq!(loader.plugins().count(), 1);
        assert!(loader.get("builtin").is_some());
    }
}
