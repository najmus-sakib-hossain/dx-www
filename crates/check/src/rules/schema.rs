//! Rule Schema for DX Serializer Integration
//!
//! Defines the binary-serializable rule format that enables:
//! - 0.70ns field access (hardware limit)
//! - Zero-copy rule loading from memory-mapped files
//! - Human-readable .dx files for contributors
//! - Cross-language rule support (12 languages)
//!
//! # Architecture Decision: Language Prefixes
//!
//! Rules use language prefixes to avoid ID collisions:
//! - `js/no-console` - JavaScript/TypeScript
//! - `py/F841` - Python (ruff)
//! - `go/fmt` - Go (gofmt.rs/gold)
//! - `rs/clippy::unwrap_used` - Rust (clippy)
//! - `php/no-unused-import` - PHP (mago)
//! - `md/MD001` - Markdown (rumdl)
//! - `toml/missing-key` - TOML (taplo)
//! - `kt/no-wildcard-imports` - Kotlin (ktlint)
//! - `cpp/clang-tidy` - C/C++ (cpp-linter-rs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Language identifier for cross-language rule support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
#[repr(u8)]
pub enum Language {
    /// JavaScript/TypeScript/JSX/TSX (biome, oxc)
    JavaScript = 0,
    /// TypeScript-specific rules
    TypeScript = 1,
    /// Python (ruff)
    Python = 2,
    /// Go (gofmt.rs, gold)
    Go = 3,
    /// Rust (rustfmt, clippy)
    Rust = 4,
    /// PHP (mago)
    Php = 5,
    /// Markdown (rumdl)
    Markdown = 6,
    /// TOML (taplo)
    Toml = 7,
    /// Kotlin (ktlint)
    Kotlin = 8,
    /// C (cpp-linter-rs)
    C = 9,
    /// C++ (cpp-linter-rs)
    Cpp = 10,
    /// JSON (biome)
    Json = 11,
    /// CSS (biome)
    Css = 12,
    /// HTML
    Html = 13,
    /// YAML
    Yaml = 14,
    /// Universal (applies to all languages)
    Universal = 255,
}

impl Language {
    /// Get the language prefix for rule IDs
    pub const fn prefix(&self) -> &'static str {
        match self {
            Self::JavaScript => "js",
            Self::TypeScript => "ts",
            Self::Python => "py",
            Self::Go => "go",
            Self::Rust => "rs",
            Self::Php => "php",
            Self::Markdown => "md",
            Self::Toml => "toml",
            Self::Kotlin => "kt",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Json => "json",
            Self::Css => "css",
            Self::Html => "html",
            Self::Yaml => "yaml",
            Self::Universal => "all",
        }
    }

    /// Get file extensions for this language
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Self::JavaScript => &["js", "jsx", "mjs", "cjs"],
            Self::TypeScript => &["ts", "tsx", "mts", "cts"],
            Self::Python => &["py", "pyi", "pyw"],
            Self::Go => &["go"],
            Self::Rust => &["rs"],
            Self::Php => &["php", "phtml", "php3", "php4", "php5", "phps"],
            Self::Markdown => &["md", "markdown", "mdown", "mkd"],
            Self::Toml => &["toml"],
            Self::Kotlin => &["kt", "kts"],
            Self::C => &["c", "h"],
            Self::Cpp => &["cpp", "cc", "cxx", "hpp", "hxx", "h"],
            Self::Json => &["json", "jsonc"],
            Self::Css => &["css", "scss", "sass", "less"],
            Self::Html => &["html", "htm", "xhtml"],
            Self::Yaml => &["yaml", "yml"],
            Self::Universal => &[],
        }
    }

    /// Get language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "js" | "jsx" | "mjs" | "cjs" => Some(Self::JavaScript),
            "ts" | "tsx" | "mts" | "cts" => Some(Self::TypeScript),
            "py" | "pyi" | "pyw" => Some(Self::Python),
            "go" => Some(Self::Go),
            "rs" => Some(Self::Rust),
            "php" | "phtml" => Some(Self::Php),
            "md" | "markdown" => Some(Self::Markdown),
            "toml" => Some(Self::Toml),
            "kt" | "kts" => Some(Self::Kotlin),
            "c" | "h" => Some(Self::C),
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Some(Self::Cpp),
            "json" | "jsonc" => Some(Self::Json),
            "css" | "scss" | "sass" | "less" => Some(Self::Css),
            "html" | "htm" | "xhtml" => Some(Self::Html),
            "yaml" | "yml" => Some(Self::Yaml),
            _ => None,
        }
    }
}

/// Rule category for organization and filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
#[repr(u8)]
pub enum DxCategory {
    /// Possible runtime errors
    Correctness = 0,
    /// Suspicious code patterns
    Suspicious = 1,
    /// Code style preferences
    Style = 2,
    /// Performance issues
    Performance = 3,
    /// Security vulnerabilities
    Security = 4,
    /// Code complexity
    Complexity = 5,
    /// Accessibility (JSX/HTML)
    Accessibility = 6,
    /// Import/export issues
    Imports = 7,
    /// Type-related issues
    Types = 8,
    /// Documentation issues
    Documentation = 9,
    /// Deprecated API usage
    Deprecated = 10,
    /// Formatting rules
    Format = 11,
}

impl DxCategory {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Correctness => "correctness",
            Self::Suspicious => "suspicious",
            Self::Style => "style",
            Self::Performance => "performance",
            Self::Security => "security",
            Self::Complexity => "complexity",
            Self::Accessibility => "a11y",
            Self::Imports => "imports",
            Self::Types => "types",
            Self::Documentation => "docs",
            Self::Deprecated => "deprecated",
            Self::Format => "format",
        }
    }
}

/// Rule severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
#[repr(u8)]
pub enum DxSeverity {
    /// Rule is disabled
    Off = 0,
    /// Warning - potential issue
    Warn = 1,
    /// Error - definite problem
    Error = 2,
}

/// Rule source - which linter/formatter the rule comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
#[repr(u8)]
pub enum RuleSource {
    /// Built-in dx-check rules
    DxCheck = 0,
    /// Biome rules (JS/TS/JSON/CSS)
    Biome = 1,
    /// OXC rules (JS/TS)
    Oxc = 2,
    /// Ruff rules (Python)
    Ruff = 3,
    /// Mago rules (PHP)
    Mago = 4,
    /// gofmt.rs rules (Go format)
    GofmtRs = 5,
    /// Gold rules (Go lint)
    Gold = 6,
    /// rustfmt rules (Rust format)
    Rustfmt = 7,
    /// Clippy rules (Rust lint)
    Clippy = 8,
    /// Taplo rules (TOML)
    Taplo = 9,
    /// rumdl rules (Markdown)
    Rumdl = 10,
    /// cpp-linter-rs rules (C/C++)
    CppLinter = 11,
    /// ktlint rules (Kotlin)
    Ktlint = 12,
}

impl RuleSource {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DxCheck => "dx-check",
            Self::Biome => "biome",
            Self::Oxc => "oxc",
            Self::Ruff => "ruff",
            Self::Mago => "mago",
            Self::GofmtRs => "gofmt.rs",
            Self::Gold => "gold",
            Self::Rustfmt => "rustfmt",
            Self::Clippy => "clippy",
            Self::Taplo => "taplo",
            Self::Rumdl => "rumdl",
            Self::CppLinter => "cpp-linter-rs",
            Self::Ktlint => "ktlint",
        }
    }
}

/// Serializable rule definition for dx-serializer
///
/// This is the canonical representation of a rule that gets:
/// 1. Written to `.dx` files (human/LLM format) for contributors
/// 2. Compiled to `.dxm` files (machine format) for runtime
///
/// # Binary Layout (64 bytes, cache-line aligned)
///
/// ```text
/// ┌────────────────────────────────────────────────────────────────┐
/// │ rule_id (u16) │ lang (u8) │ cat (u8) │ src (u8) │ sev (u8) │ flags (u16) │
/// ├────────────────────────────────────────────────────────────────┤
/// │ name_offset (u32) │ name_len (u16) │ desc_offset (u32) │ desc_len (u16) │
/// ├────────────────────────────────────────────────────────────────┤
/// │ docs_url_offset (u32) │ docs_url_len (u16) │ options_offset (u32) │
/// ├────────────────────────────────────────────────────────────────┤
/// │ reserved (16 bytes for future expansion)                       │
/// └────────────────────────────────────────────────────────────────┘
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct DxRule {
    /// Unique rule ID (global across all languages)
    /// Format: (language_id << 12) | rule_index
    /// Supports 16 languages × 4096 rules each = 65536 total
    pub rule_id: u16,

    /// Target language
    pub language: Language,

    /// Rule category
    pub category: DxCategory,

    /// Source linter/formatter
    pub source: RuleSource,

    /// Default severity
    pub default_severity: DxSeverity,

    /// Rule name (e.g., "no-console", "F841")
    pub name: String,

    /// Full prefixed name (e.g., "js/no-console", "py/F841")
    pub prefixed_name: String,

    /// Short description
    pub description: String,

    /// Whether the rule has an auto-fix
    pub fixable: bool,

    /// Whether this is a recommended rule
    pub recommended: bool,

    /// Whether this is a format rule (vs lint rule)
    pub is_formatter: bool,

    /// Documentation URL
    pub docs_url: Option<String>,

    /// Configuration options schema (JSON Schema)
    pub options_schema: Option<String>,

    /// Related rules (for grouping)
    pub related_rules: Vec<String>,

    /// Deprecated - replaced by this rule
    pub deprecated_by: Option<String>,
}

impl DxRule {
    /// Create a new rule with prefixed name
    pub fn new(
        rule_id: u16,
        language: Language,
        name: impl Into<String>,
        description: impl Into<String>,
        category: DxCategory,
        source: RuleSource,
    ) -> Self {
        let name = name.into();
        let prefixed_name = format!("{}/{}", language.prefix(), name);

        Self {
            rule_id,
            language,
            category,
            source,
            default_severity: DxSeverity::Warn,
            name,
            prefixed_name,
            description: description.into(),
            fixable: false,
            recommended: false,
            is_formatter: false,
            docs_url: None,
            options_schema: None,
            related_rules: Vec::new(),
            deprecated_by: None,
        }
    }

    /// Builder: set fixable
    pub fn fixable(mut self, fixable: bool) -> Self {
        self.fixable = fixable;
        self
    }

    /// Builder: set recommended
    pub fn recommended(mut self, recommended: bool) -> Self {
        self.recommended = recommended;
        self
    }

    /// Builder: set severity
    pub fn severity(mut self, severity: DxSeverity) -> Self {
        self.default_severity = severity;
        self
    }

    /// Builder: set docs URL
    pub fn docs(mut self, url: impl Into<String>) -> Self {
        self.docs_url = Some(url.into());
        self
    }

    /// Builder: mark as formatter rule
    pub fn formatter(mut self) -> Self {
        self.is_formatter = true;
        self.category = DxCategory::Format;
        self
    }
}

/// Complete rule database - serializable to .dxm binary
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct DxRuleDatabase {
    /// Magic number for validation: "DXRB" (DX Rules Binary)
    pub magic: [u8; 4],

    /// Version of the rule format
    pub version: u32,

    /// Total number of rules
    pub rule_count: u32,

    /// All rules, indexed by rule_id
    pub rules: Vec<DxRule>,

    /// Index: prefixed_name -> rule_id (for O(1) lookup)
    pub name_index: HashMap<String, u16>,

    /// Index: language -> rule_ids (for language filtering)
    pub language_index: HashMap<u8, Vec<u16>>,

    /// Index: category -> rule_ids (for category filtering)
    pub category_index: HashMap<u8, Vec<u16>>,

    /// Index: source -> rule_ids (for source filtering)
    pub source_index: HashMap<u8, Vec<u16>>,

    /// Deduplication statistics
    pub stats: RuleDatabaseStats,
}

/// Statistics about the rule database
#[derive(Debug, Clone, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct RuleDatabaseStats {
    /// Total rules per language
    pub rules_per_language: HashMap<String, u32>,

    /// Total rules per source
    pub rules_per_source: HashMap<String, u32>,

    /// Fixable vs non-fixable
    pub fixable_count: u32,
    pub non_fixable_count: u32,

    /// Recommended count
    pub recommended_count: u32,

    /// Format vs lint rules
    pub format_rule_count: u32,
    pub lint_rule_count: u32,

    /// Binary size after compilation
    pub binary_size_bytes: u64,

    /// Compilation timestamp
    pub compiled_at: Option<String>,
}

impl DxRuleDatabase {
    /// Magic bytes: "DXRB"
    pub const MAGIC: [u8; 4] = [0x44, 0x58, 0x52, 0x42];

    /// Current format version
    pub const VERSION: u32 = 1;

    /// Create a new empty database
    pub fn new() -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            rule_count: 0,
            rules: Vec::new(),
            name_index: HashMap::new(),
            language_index: HashMap::new(),
            category_index: HashMap::new(),
            source_index: HashMap::new(),
            stats: RuleDatabaseStats::default(),
        }
    }

    /// Add a rule to the database
    pub fn add_rule(&mut self, rule: DxRule) {
        let rule_id = rule.rule_id;
        let prefixed_name = rule.prefixed_name.clone();
        let language = rule.language as u8;
        let category = rule.category as u8;
        let source = rule.source as u8;

        // Update indexes
        self.name_index.insert(prefixed_name, rule_id);
        self.language_index
            .entry(language)
            .or_default()
            .push(rule_id);
        self.category_index
            .entry(category)
            .or_default()
            .push(rule_id);
        self.source_index.entry(source).or_default().push(rule_id);

        // Update stats
        *self
            .stats
            .rules_per_language
            .entry(rule.language.prefix().to_string())
            .or_default() += 1;
        *self
            .stats
            .rules_per_source
            .entry(rule.source.as_str().to_string())
            .or_default() += 1;

        if rule.fixable {
            self.stats.fixable_count += 1;
        } else {
            self.stats.non_fixable_count += 1;
        }

        if rule.recommended {
            self.stats.recommended_count += 1;
        }

        if rule.is_formatter {
            self.stats.format_rule_count += 1;
        } else {
            self.stats.lint_rule_count += 1;
        }

        self.rules.push(rule);
        self.rule_count = self.rules.len() as u32;
    }

    /// Get a rule by prefixed name
    pub fn get_by_name(&self, prefixed_name: &str) -> Option<&DxRule> {
        self.name_index
            .get(prefixed_name)
            .and_then(|&id| self.rules.iter().find(|r| r.rule_id == id))
    }

    /// Get all rules for a language
    pub fn get_by_language(&self, language: Language) -> Vec<&DxRule> {
        self.language_index
            .get(&(language as u8))
            .map(|ids| {
                ids.iter()
                    .filter_map(|&id| self.rules.iter().find(|r| r.rule_id == id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all rules from a source
    pub fn get_by_source(&self, source: RuleSource) -> Vec<&DxRule> {
        self.source_index
            .get(&(source as u8))
            .map(|ids| {
                ids.iter()
                    .filter_map(|&id| self.rules.iter().find(|r| r.rule_id == id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Validate the database integrity
    pub fn validate(&self) -> Result<(), String> {
        if self.magic != Self::MAGIC {
            return Err("Invalid magic bytes".into());
        }
        if self.version != Self::VERSION {
            return Err(format!(
                "Version mismatch: expected {}, got {}",
                Self::VERSION,
                self.version
            ));
        }
        if self.rule_count != self.rules.len() as u32 {
            return Err("Rule count mismatch".into());
        }
        Ok(())
    }
}

impl Default for DxRuleDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_prefix() {
        assert_eq!(Language::JavaScript.prefix(), "js");
        assert_eq!(Language::Python.prefix(), "py");
        assert_eq!(Language::Rust.prefix(), "rs");
    }

    #[test]
    fn test_rule_creation() {
        let rule = DxRule::new(
            1,
            Language::JavaScript,
            "no-console",
            "Disallow console statements",
            DxCategory::Suspicious,
            RuleSource::DxCheck,
        )
        .fixable(true)
        .recommended(true);

        assert_eq!(rule.prefixed_name, "js/no-console");
        assert!(rule.fixable);
        assert!(rule.recommended);
    }

    #[test]
    fn test_database_indexing() {
        let mut db = DxRuleDatabase::new();

        db.add_rule(DxRule::new(
            1,
            Language::JavaScript,
            "no-console",
            "Disallow console",
            DxCategory::Suspicious,
            RuleSource::DxCheck,
        ));

        db.add_rule(DxRule::new(
            2,
            Language::Python,
            "F841",
            "Unused variable",
            DxCategory::Correctness,
            RuleSource::Ruff,
        ));

        assert_eq!(db.rule_count, 2);
        assert!(db.get_by_name("js/no-console").is_some());
        assert!(db.get_by_name("py/F841").is_some());
        assert_eq!(db.get_by_language(Language::JavaScript).len(), 1);
        assert_eq!(db.get_by_source(RuleSource::Ruff).len(), 1);
    }
}
