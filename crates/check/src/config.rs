//! Dx Check Configuration
//!
//! Zero-config by default with full customization support.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main configuration for dx-check
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CheckerConfig {
    /// Root directory for checking
    pub root: PathBuf,

    /// Files/patterns to include
    pub include: Vec<String>,

    /// Files/patterns to exclude
    pub exclude: Vec<String>,

    /// Rule configurations
    pub rules: RuleConfigs,

    /// Formatter settings
    pub format: FormatConfig,

    /// Cache settings
    pub cache: CacheConfig,

    /// Parallel execution settings
    pub parallel: ParallelConfig,

    /// Architecture boundary rules
    pub architecture: Option<ArchitectureConfig>,

    /// Project-specific overrides
    pub overrides: Vec<OverrideConfig>,
}

impl Default for CheckerConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            include: vec![
                "**/*.js".into(),
                "**/*.jsx".into(),
                "**/*.ts".into(),
                "**/*.tsx".into(),
                "**/*.mjs".into(),
                "**/*.cjs".into(),
            ],
            exclude: vec![
                "**/node_modules/**".into(),
                "**/dist/**".into(),
                "**/build/**".into(),
                "**/.git/**".into(),
                "**/coverage/**".into(),
            ],
            rules: RuleConfigs::default(),
            format: FormatConfig::default(),
            cache: CacheConfig::default(),
            parallel: ParallelConfig::default(),
            architecture: None,
            overrides: Vec::new(),
        }
    }
}

impl CheckerConfig {
    /// Create config from dx.toml if present, otherwise use defaults
    pub fn auto_detect(root: &std::path::Path) -> Self {
        let config_path = root.join("dx.toml");
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }

        // Try biome.json for compatibility
        let biome_path = root.join("biome.json");
        if biome_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&biome_path) {
                if let Ok(config) = Self::from_biome_json(&content) {
                    return config;
                }
            }
        }

        // Fall back to defaults with auto-detection
        let mut config = Self::default();
        config.root = root.to_path_buf();
        config
    }

    /// Parse Biome configuration for compatibility
    fn from_biome_json(content: &str) -> Result<Self, serde_json::Error> {
        let biome: serde_json::Value = serde_json::from_str(content)?;
        let mut config = Self::default();

        // Convert Biome rules to dx-check rules
        if let Some(linter) = biome.get("linter") {
            if let Some(rules) = linter.get("rules") {
                config.rules = RuleConfigs::from_biome(rules);
            }
        }

        Ok(config)
    }

    /// Merge with CLI overrides
    pub fn with_cli_overrides(mut self, cli: &CliOverrides) -> Self {
        if let Some(ref fix) = cli.fix {
            self.rules.auto_fix = *fix;
        }
        if let Some(threads) = cli.threads {
            self.parallel.threads = threads;
        }
        self
    }
}

/// Rule severity and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfigs {
    /// Enable recommended rules
    pub recommended: bool,

    /// Enable auto-fix
    pub auto_fix: bool,

    /// Individual rule settings: rule_id -> severity/options
    pub rules: HashMap<String, RuleConfig>,
}

impl Default for RuleConfigs {
    fn default() -> Self {
        Self {
            recommended: true,
            auto_fix: false,
            rules: HashMap::new(),
        }
    }
}

impl RuleConfigs {
    fn from_biome(value: &serde_json::Value) -> Self {
        let mut configs = Self::default();

        // Parse Biome rule categories
        if let Some(obj) = value.as_object() {
            for (category, rules) in obj {
                if let Some(rules_obj) = rules.as_object() {
                    for (rule, config) in rules_obj {
                        let rule_id = format!("{category}/{rule}");
                        if let Some(severity) = config.as_str() {
                            configs.rules.insert(
                                rule_id,
                                RuleConfig {
                                    severity: RuleSeverity::from_str(severity),
                                    options: HashMap::new(),
                                },
                            );
                        }
                    }
                }
            }
        }

        configs
    }
}

/// Individual rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    pub severity: RuleSeverity,
    pub options: HashMap<String, serde_json::Value>,
}

/// Rule severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverity {
    Off,
    Warn,
    Error,
}

impl RuleSeverity {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "off" | "0" => Self::Off,
            "warn" | "warning" | "1" => Self::Warn,
            "error" | "2" => Self::Error,
            _ => Self::Warn,
        }
    }
}

/// Formatter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatConfig {
    /// Use tabs instead of spaces
    pub use_tabs: bool,

    /// Indentation width
    pub indent_width: u8,

    /// Line width before wrapping
    pub line_width: u16,

    /// Quote style for strings
    pub quote_style: QuoteStyle,

    /// Semicolons at end of statements
    pub semicolons: Semicolons,

    /// Trailing commas in multi-line
    pub trailing_comma: TrailingComma,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            use_tabs: false,
            indent_width: 2,
            line_width: 80,
            quote_style: QuoteStyle::Double,
            semicolons: Semicolons::Always,
            trailing_comma: TrailingComma::All,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuoteStyle {
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Semicolons {
    Always,
    AsNeeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrailingComma {
    All,
    Es5,
    None,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CacheConfig {
    /// Enable AST caching
    pub enabled: bool,

    /// Cache directory (default: .dx-cache)
    pub directory: PathBuf,

    /// Maximum cache size in bytes
    pub max_size: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: PathBuf::from(".dx-cache"),
            max_size: 1024 * 1024 * 1024, // 1GB
        }
    }
}

/// Parallel execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ParallelConfig {
    /// Number of worker threads (0 = auto-detect)
    pub threads: usize,

    /// Enable work stealing
    pub work_stealing: bool,

    /// Batch size for file processing
    pub batch_size: usize,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            threads: 0, // Auto-detect
            work_stealing: true,
            batch_size: 100,
        }
    }
}

/// Architecture boundary enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureConfig {
    /// Defined layers
    pub layers: Vec<String>,

    /// Layer rules
    pub rules: Vec<LayerRule>,

    /// Glob pattern to layer mapping
    pub mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerRule {
    pub from: String,
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

/// Override configuration for specific paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideConfig {
    /// Glob patterns for files to override
    pub files: Vec<String>,

    /// Rule overrides for matched files
    pub rules: HashMap<String, RuleConfig>,
}

/// CLI overrides
#[derive(Debug, Default)]
pub struct CliOverrides {
    pub fix: Option<bool>,
    pub threads: Option<usize>,
    pub format: Option<bool>,
}

// Add toml to dependencies - we need this for config parsing
fn toml_stub() {}
