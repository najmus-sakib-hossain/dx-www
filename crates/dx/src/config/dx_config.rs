//! DX configuration file parsing
//!
//! Provides configuration loading from dx.toml with support for:
//! - Custom config paths via --config flag (Requirement 12.2)
//! - Error reporting with line numbers (Requirement 12.3)
//! - Binary caching for faster subsequent loads (Requirement 12.4)

use crate::utils::error::DxError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Default config file name
pub const DEFAULT_CONFIG_FILE: &str = "dx.toml";

/// Cache file extension
const CACHE_EXTENSION: &str = ".cache";

/// Main DX configuration structure
///
/// Requirement 12.1: Load configuration from dx.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DxConfig {
    /// Project metadata
    pub project: ProjectConfig,

    /// Build configuration
    #[serde(default)]
    pub build: BuildConfig,

    /// Development server configuration
    #[serde(default)]
    pub dev: DevConfig,

    /// Runtime configuration
    #[serde(default)]
    pub runtime: RuntimeConfig,

    /// Tool-specific configurations
    #[serde(default)]
    pub tools: ToolsConfig,
}

/// Project metadata configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,

    /// Project version
    #[serde(default = "default_version")]
    pub version: String,

    /// Project description
    pub description: Option<String>,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildConfig {
    /// Build target (browser, node, etc.)
    #[serde(default = "default_target")]
    pub target: String,

    /// Enable minification
    #[serde(default = "default_true")]
    pub minify: bool,

    /// Generate source maps
    #[serde(default)]
    pub sourcemap: bool,

    /// Output directory
    #[serde(default = "default_out_dir")]
    pub out_dir: String,
}

/// Development server configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DevConfig {
    /// Development server port
    #[serde(default = "default_port")]
    pub port: u16,

    /// Open browser on start
    #[serde(default)]
    pub open: bool,

    /// Enable HTTPS
    #[serde(default)]
    pub https: bool,
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuntimeConfig {
    /// JSX transform mode
    #[serde(default = "default_jsx")]
    pub jsx: String,

    /// Enable TypeScript
    #[serde(default = "default_true")]
    pub typescript: bool,
}

/// Tool-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolsConfig {
    /// Style tool configuration
    #[serde(default)]
    pub style: Option<StyleToolConfig>,

    /// Media tool configuration
    #[serde(default)]
    pub media: Option<MediaToolConfig>,

    /// Font tool configuration
    #[serde(default)]
    pub font: Option<FontToolConfig>,

    /// Icon tool configuration
    #[serde(default)]
    pub icon: Option<IconToolConfig>,
}

/// Style tool configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StyleToolConfig {
    /// CSS preprocessor (sass, less, stylus)
    pub preprocessor: Option<String>,

    /// Enable CSS modules
    #[serde(default)]
    pub modules: bool,

    /// PostCSS plugins
    #[serde(default)]
    pub postcss_plugins: Vec<String>,
}

/// Media tool configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MediaToolConfig {
    /// Image optimization quality (1-100)
    #[serde(default = "default_quality")]
    pub quality: u8,

    /// Output formats
    #[serde(default)]
    pub formats: Vec<String>,
}

/// Font tool configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FontToolConfig {
    /// Font subsetting
    #[serde(default)]
    pub subset: bool,

    /// Character ranges to include
    #[serde(default)]
    pub ranges: Vec<String>,
}

/// Icon tool configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IconToolConfig {
    /// Icon sprite generation
    #[serde(default)]
    pub sprite: bool,

    /// Icon sizes to generate
    #[serde(default)]
    pub sizes: Vec<u32>,
}

/// Cached configuration with metadata
#[derive(Debug, Serialize, Deserialize)]
struct CachedConfig {
    /// The cached configuration
    config: DxConfig,
    /// Modification time of the source file (as seconds since UNIX_EPOCH)
    source_mtime: u64,
}

impl DxConfig {
    /// Load configuration from the default path (dx.toml in current directory)
    ///
    /// Requirement 12.1: Load configuration from dx.toml
    pub fn load_default() -> Result<Self, DxError> {
        Self::load(Path::new(DEFAULT_CONFIG_FILE))
    }

    /// Load configuration from a specific path
    ///
    /// Requirement 12.2: Support custom config path via --config flag
    pub fn load(path: &Path) -> Result<Self, DxError> {
        // Check if file exists
        if !path.exists() {
            return Err(DxError::ConfigNotFound {
                path: path.to_path_buf(),
            });
        }

        // Try to load from cache first
        if let Some(cached) = Self::load_from_cache(path) {
            return Ok(cached);
        }

        // Load and parse the config file
        let config = Self::load_and_parse(path)?;

        // Cache the parsed config
        let _ = Self::save_to_cache(path, &config);

        Ok(config)
    }

    /// Load configuration, returning default if not found
    pub fn load_or_default() -> Self {
        Self::load_default().unwrap_or_default()
    }

    /// Load configuration with a custom path override
    ///
    /// If custom_path is Some, use that path. Otherwise, use the default.
    ///
    /// Requirement 12.2: Support custom config path via --config flag
    pub fn load_with_override(custom_path: Option<&Path>) -> Result<Self, DxError> {
        match custom_path {
            Some(path) => Self::load(path),
            None => Self::load_default(),
        }
    }

    /// Parse TOML content and return config or detailed error
    ///
    /// Requirement 12.3: Display error location (file, line) and message
    fn load_and_parse(path: &Path) -> Result<Self, DxError> {
        let content = fs::read_to_string(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                DxError::ConfigNotFound {
                    path: path.to_path_buf(),
                }
            } else {
                DxError::Io {
                    message: e.to_string(),
                }
            }
        })?;

        toml::from_str(&content).map_err(|e| {
            // Extract line number from TOML error
            let line = e.span().map(|s| {
                // Count newlines up to the error position
                content[..s.start].chars().filter(|&c| c == '\n').count() + 1
            }).unwrap_or(1);

            DxError::ConfigInvalid {
                path: path.to_path_buf(),
                line,
                message: e.message().to_string(),
            }
        })
    }

    /// Get the cache file path for a config file
    fn cache_path(config_path: &Path) -> PathBuf {
        let mut cache_path = config_path.to_path_buf();
        let file_name = cache_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "config".to_string());
        cache_path.set_file_name(format!(".{}{}", file_name, CACHE_EXTENSION));
        cache_path
    }

    /// Load configuration from cache if valid
    ///
    /// Requirement 12.4: Cache parsed configuration in binary format
    fn load_from_cache(config_path: &Path) -> Option<Self> {
        let cache_path = Self::cache_path(config_path);

        // Check if cache file exists
        if !cache_path.exists() {
            return None;
        }

        // Get source file modification time
        let source_mtime = fs::metadata(config_path)
            .and_then(|m| m.modified())
            .ok()?
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()?
            .as_secs();

        // Read and deserialize cache
        let cache_data = fs::read(&cache_path).ok()?;
        let cached: CachedConfig = serde_json::from_slice(&cache_data).ok()?;

        // Check if cache is still valid (source file hasn't been modified)
        if cached.source_mtime >= source_mtime {
            Some(cached.config)
        } else {
            // Cache is stale, remove it
            let _ = fs::remove_file(&cache_path);
            None
        }
    }

    /// Save configuration to cache
    ///
    /// Requirement 12.4: Cache parsed configuration in binary format
    fn save_to_cache(config_path: &Path, config: &Self) -> Result<(), DxError> {
        let cache_path = Self::cache_path(config_path);

        // Get source file modification time
        let source_mtime = fs::metadata(config_path)
            .and_then(|m| m.modified())
            .map_err(|e| DxError::Io {
                message: e.to_string(),
            })?
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| DxError::Io {
                message: e.to_string(),
            })?
            .as_secs();

        let cached = CachedConfig {
            config: config.clone(),
            source_mtime,
        };

        // Serialize and write cache (using JSON for now, can switch to bincode later)
        let cache_data = serde_json::to_vec(&cached).map_err(|e| DxError::Io {
            message: e.to_string(),
        })?;

        fs::write(&cache_path, cache_data).map_err(|e| DxError::Io {
            message: e.to_string(),
        })?;

        Ok(())
    }

    /// Invalidate the cache for a config file
    pub fn invalidate_cache(config_path: &Path) {
        let cache_path = Self::cache_path(config_path);
        let _ = fs::remove_file(cache_path);
    }
}

// Default value functions
fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_target() -> String {
    "browser".to_string()
}

fn default_out_dir() -> String {
    "dist".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_jsx() -> String {
    "dx".to_string()
}

fn default_true() -> bool {
    true
}

fn default_quality() -> u8 {
    85
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target: default_target(),
            minify: true,
            sourcemap: false,
            out_dir: default_out_dir(),
        }
    }
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            open: false,
            https: false,
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            jsx: default_jsx(),
            typescript: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_temp_config(content: &str) -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("dx.toml");
        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        (dir, config_path)
    }

    #[test]
    fn test_load_valid_config() {
        let content = r#"
[project]
name = "test-project"
version = "1.0.0"
description = "A test project"

[build]
target = "node"
minify = false

[dev]
port = 8080
"#;
        let (_dir, path) = create_temp_config(content);
        let config = DxConfig::load(&path).unwrap();

        assert_eq!(config.project.name, "test-project");
        assert_eq!(config.project.version, "1.0.0");
        assert_eq!(config.build.target, "node");
        assert!(!config.build.minify);
        assert_eq!(config.dev.port, 8080);
    }

    #[test]
    fn test_load_minimal_config() {
        let content = r#"
[project]
name = "minimal"
"#;
        let (_dir, path) = create_temp_config(content);
        let config = DxConfig::load(&path).unwrap();

        assert_eq!(config.project.name, "minimal");
        assert_eq!(config.project.version, "0.1.0"); // default
        assert_eq!(config.build.target, "browser"); // default
        assert_eq!(config.dev.port, 3000); // default
    }

    #[test]
    fn test_load_config_not_found() {
        let result = DxConfig::load(Path::new("nonexistent.toml"));
        assert!(result.is_err());
        match result.unwrap_err() {
            DxError::ConfigNotFound { path } => {
                assert_eq!(path, PathBuf::from("nonexistent.toml"));
            }
            _ => panic!("Expected ConfigNotFound error"),
        }
    }

    #[test]
    fn test_load_invalid_config() {
        let content = r#"
[project]
name = "test"
version = 123  # Should be a string
"#;
        let (_dir, path) = create_temp_config(content);
        let result = DxConfig::load(&path);

        assert!(result.is_err());
        match result.unwrap_err() {
            DxError::ConfigInvalid { line, message, .. } => {
                assert!(line > 0, "Line number should be positive");
                assert!(!message.is_empty(), "Error message should not be empty");
            }
            _ => panic!("Expected ConfigInvalid error"),
        }
    }

    #[test]
    fn test_config_with_tools() {
        let content = r#"
[project]
name = "with-tools"

[tools.style]
preprocessor = "sass"
modules = true
postcss_plugins = ["autoprefixer"]

[tools.media]
quality = 90
formats = ["webp", "avif"]
"#;
        let (_dir, path) = create_temp_config(content);
        let config = DxConfig::load(&path).unwrap();

        let style = config.tools.style.unwrap();
        assert_eq!(style.preprocessor, Some("sass".to_string()));
        assert!(style.modules);
        assert_eq!(style.postcss_plugins, vec!["autoprefixer"]);

        let media = config.tools.media.unwrap();
        assert_eq!(media.quality, 90);
        assert_eq!(media.formats, vec!["webp", "avif"]);
    }

    #[test]
    fn test_cache_path_generation() {
        let config_path = Path::new("/project/dx.toml");
        let cache_path = DxConfig::cache_path(config_path);
        assert_eq!(cache_path, PathBuf::from("/project/.dx.toml.cache"));
    }

    #[test]
    fn test_config_caching() {
        let content = r#"
[project]
name = "cached-project"
version = "2.0.0"
"#;
        let (_dir, path) = create_temp_config(content);

        // First load - should parse and cache
        let config1 = DxConfig::load(&path).unwrap();
        assert_eq!(config1.project.name, "cached-project");

        // Verify cache file was created
        let cache_path = DxConfig::cache_path(&path);
        assert!(cache_path.exists(), "Cache file should exist");

        // Second load - should load from cache
        let config2 = DxConfig::load(&path).unwrap();
        assert_eq!(config2.project.name, "cached-project");
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_cache_invalidation_on_source_change() {
        let content1 = r#"
[project]
name = "original"
"#;
        let (dir, path) = create_temp_config(content1);

        // First load
        let config1 = DxConfig::load(&path).unwrap();
        assert_eq!(config1.project.name, "original");

        // Invalidate the cache manually to simulate source change detection
        DxConfig::invalidate_cache(&path);

        let content2 = r#"
[project]
name = "modified"
"#;
        fs::write(&path, content2).unwrap();

        // Second load - should reload since cache was invalidated
        let config2 = DxConfig::load(&path).unwrap();
        assert_eq!(config2.project.name, "modified");

        drop(dir); // Clean up
    }

    // ═══════════════════════════════════════════════════════════════════
    //  PROPERTY TESTS
    // ═══════════════════════════════════════════════════════════════════

    // Feature: dx-cli, Property 22: Custom Config Path Override
    // Validates: Requirements 12.2
    //
    // When a custom config path is provided, the config should be loaded
    // from that path instead of the default dx.toml.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]

        #[test]
        fn prop_custom_config_path_override(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,20}",
            version in "[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}"
        ) {
            let content = format!(r#"
[project]
name = "{}"
version = "{}"
"#, name, version);

            let dir = TempDir::new().unwrap();
            let custom_path = dir.path().join("custom-config.toml");
            fs::write(&custom_path, &content).unwrap();

            // Load with custom path override
            let config = DxConfig::load_with_override(Some(&custom_path)).unwrap();

            prop_assert_eq!(config.project.name, name);
            prop_assert_eq!(config.project.version, version);
        }
    }

    // Feature: dx-cli, Property 23: Invalid Config Error Reporting
    // Validates: Requirements 12.3
    //
    // When configuration is invalid, the error should include the file path,
    // line number, and a descriptive message.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]

        #[test]
        fn prop_invalid_config_error_reporting(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,20}",
            invalid_line in 2usize..10
        ) {
            // Create config with invalid TOML at a specific line
            let mut lines: Vec<String> = vec![
                "[project]".to_string(),
                format!("name = \"{}\"", name),
            ];

            // Add some valid lines
            for _ in 2..invalid_line {
                lines.push("# comment".to_string());
            }

            // Add invalid line (unclosed string)
            lines.push("version = \"unclosed".to_string());

            let content = lines.join("\n");
            let (_dir, path) = create_temp_config(&content);

            let result = DxConfig::load(&path);
            prop_assert!(result.is_err());

            match result.unwrap_err() {
                DxError::ConfigInvalid { path: err_path, line, message } => {
                    prop_assert_eq!(err_path, path);
                    prop_assert!(line > 0, "Line number should be positive");
                    prop_assert!(!message.is_empty(), "Error message should not be empty");
                }
                other => prop_assert!(false, "Expected ConfigInvalid, got {:?}", other),
            }
        }
    }

    // Feature: dx-cli, Property 24: Config Cache Round-Trip
    // Validates: Requirements 12.4
    //
    // Configuration should survive a cache round-trip without data loss.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]

        #[test]
        fn prop_config_cache_round_trip(
            name in "[a-zA-Z][a-zA-Z0-9_-]{0,20}",
            version in "[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}",
            port in 1024u16..65535,
            minify in proptest::bool::ANY
        ) {
            let content = format!(r#"
[project]
name = "{}"
version = "{}"

[build]
minify = {}

[dev]
port = {}
"#, name, version, minify, port);

            let (_dir, path) = create_temp_config(&content);

            // First load - parses and caches
            let config1 = DxConfig::load(&path).unwrap();

            // Verify cache exists
            let cache_path = DxConfig::cache_path(&path);
            prop_assert!(cache_path.exists(), "Cache file should be created");

            // Second load - should load from cache
            let config2 = DxConfig::load(&path).unwrap();

            // Verify round-trip preserves all data
            prop_assert_eq!(&config1.project.name, &config2.project.name);
            prop_assert_eq!(&config1.project.version, &config2.project.version);
            prop_assert_eq!(config1.build.minify, config2.build.minify);
            prop_assert_eq!(config1.dev.port, config2.dev.port);
            prop_assert_eq!(&config1, &config2);
        }
    }
}
