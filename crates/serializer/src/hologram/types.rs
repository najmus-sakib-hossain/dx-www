//! Hologram Types and Configuration
//!
//! Core types for the Holographic DX Architecture.

use std::fmt;

/// Result type for hologram operations
pub type HologramResult<T> = Result<T, HologramError>;

/// Errors that can occur during hologram operations
#[derive(Debug, Clone, PartialEq)]
pub enum HologramError {
    /// Invalid input format
    ParseError(String),
    /// Malformed table structure
    TableError(String),
    /// Comment anchor mismatch
    CommentError(String),
    /// Encoding/decoding failure
    EncodingError(String),
}

impl fmt::Display for HologramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HologramError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            HologramError::TableError(msg) => write!(f, "Table error: {}", msg),
            HologramError::CommentError(msg) => write!(f, "Comment error: {}", msg),
            HologramError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
        }
    }
}

impl std::error::Error for HologramError {}

/// Configuration for hologram inflate/deflate operations
#[derive(Debug, Clone)]
pub struct HologramConfig {
    /// Number of spaces for indentation in pretty mode
    pub indent_size: usize,

    /// Preserve comments during deflation
    pub preserve_comments: bool,

    /// Use Unicode symbols (✓/✗ instead of 1/0)
    pub use_unicode_symbols: bool,

    /// Use box-drawing characters for tables
    pub use_box_drawing: bool,

    /// Section marker character (▼ or ▾)
    pub section_marker: char,

    /// Bullet character for arrays (• or -)
    pub bullet_char: char,

    /// Arrow character for references (→ or >)
    pub arrow_char: char,

    /// Null representation (— or null)
    pub null_display: String,

    /// Align field values in sections
    pub align_values: bool,

    /// Maximum line width for table formatting
    pub max_line_width: usize,
}

impl Default for HologramConfig {
    fn default() -> Self {
        Self {
            indent_size: 4,
            preserve_comments: true,
            use_unicode_symbols: true,
            use_box_drawing: true,
            section_marker: '▼',
            bullet_char: '•',
            arrow_char: '→',
            null_display: "—".to_string(),
            align_values: true,
            max_line_width: 120,
        }
    }
}

impl HologramConfig {
    /// Create a minimal ASCII-only configuration
    pub fn ascii() -> Self {
        Self {
            indent_size: 2,
            preserve_comments: true,
            use_unicode_symbols: false,
            use_box_drawing: false,
            section_marker: '>',
            bullet_char: '-',
            arrow_char: '>',
            null_display: "null".to_string(),
            align_values: true,
            max_line_width: 80,
        }
    }

    /// Create a compact configuration for smaller displays
    pub fn compact() -> Self {
        Self {
            indent_size: 2,
            preserve_comments: true,
            use_unicode_symbols: true,
            use_box_drawing: false,
            section_marker: '▾',
            bullet_char: '·',
            arrow_char: '→',
            null_display: "∅".to_string(),
            align_values: false,
            max_line_width: 60,
        }
    }
}

/// Represents a comment anchored to a specific element
#[derive(Debug, Clone, PartialEq)]
pub struct CommentAnchor {
    /// The comment text (without delimiters)
    pub text: String,
    /// Position: before (true) or after (false) the anchored element
    pub is_before: bool,
}

impl CommentAnchor {
    /// Create a new comment anchor
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            is_before: true,
        }
    }

    /// Format as LLM-dense comment: !text!
    pub fn to_dense(&self) -> String {
        format!("!{}!", self.text)
    }

    /// Format as human-readable comment: // text
    pub fn to_human(&self) -> String {
        format!("// {}", self.text)
    }

    /// Parse from LLM-dense format: !text!
    pub fn from_dense(s: &str) -> Option<(Self, &str)> {
        if !s.starts_with('!') {
            return None;
        }

        let rest = &s[1..];
        if let Some(end_idx) = rest.find('!') {
            let text = &rest[..end_idx];
            let remaining = &rest[end_idx + 1..];
            Some((
                Self {
                    text: text.to_string(),
                    is_before: true,
                },
                remaining,
            ))
        } else {
            None
        }
    }
}

/// Parsed element from LLM-dense format
#[derive(Debug, Clone, PartialEq)]
pub enum DenseElement {
    /// Object: key#field:val#field:val
    Object {
        key: String,
        fields: Vec<(String, String)>,
        comment: Option<CommentAnchor>,
    },
    /// Array: key@N>item|item|item
    Array {
        key: String,
        items: Vec<String>,
        comment: Option<CommentAnchor>,
    },
    /// Table header: key@N=col^col^col
    TableHeader {
        key: String,
        columns: Vec<String>,
        row_count: usize,
        comment: Option<CommentAnchor>,
    },
    /// Table row: >val|val|val
    TableRow { values: Vec<String> },
    /// Simple key:value
    KeyValue {
        key: String,
        value: String,
        comment: Option<CommentAnchor>,
    },
    /// Standalone comment
    Comment(CommentAnchor),
    /// Empty line
    Empty,
}

/// Parsed element from human-pretty format
#[derive(Debug, Clone, PartialEq)]
pub enum PrettyElement {
    /// Section header: ▼ key or ▼ key (N items)
    Section {
        key: String,
        metadata: Option<String>,
    },
    /// Field line: key: value
    Field { key: String, value: String },
    /// Bullet item: • item
    Bullet { value: String },
    /// Table header row: │ col │ col │
    TableHeader { columns: Vec<String> },
    /// Table data row: │ val │ val │
    TableRow { values: Vec<String> },
    /// Table border: ┌─────┬─────┐
    TableBorder,
    /// Comment: // text
    Comment { text: String },
    /// Empty line
    Empty,
}

/// Value type indicators for smart formatting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    String,
    Integer,
    Float,
    Boolean,
    Null,
    Reference,
    Unknown,
}

impl ValueType {
    /// Infer type from a string value
    pub fn infer(s: &str) -> Self {
        let s = s.trim();

        // Boolean
        if s == "1" || s == "0" || s == "true" || s == "false" || s == "✓" || s == "✗" {
            return ValueType::Boolean;
        }

        // Null
        if s == "~" || s == "null" || s == "none" || s == "—" || s == "∅" {
            return ValueType::Null;
        }

        // Reference
        if s.starts_with('*') || s.starts_with('→') {
            return ValueType::Reference;
        }

        // Integer
        if s.parse::<i64>().is_ok() {
            return ValueType::Integer;
        }

        // Float
        if s.parse::<f64>().is_ok() {
            return ValueType::Float;
        }

        // Quoted string
        if (s.starts_with('"') && s.ends_with('"'))
            || (s.starts_with('\'') && s.ends_with('\''))
        {
            return ValueType::String;
        }

        ValueType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_anchor_parse() {
        let input = "!Database config!db#host:localhost";
        let (anchor, rest) = CommentAnchor::from_dense(input).unwrap();

        assert_eq!(anchor.text, "Database config");
        assert_eq!(rest, "db#host:localhost");
    }

    #[test]
    fn test_comment_anchor_format() {
        let anchor = CommentAnchor::new("Server settings");

        assert_eq!(anchor.to_dense(), "!Server settings!");
        assert_eq!(anchor.to_human(), "// Server settings");
    }

    #[test]
    fn test_value_type_inference() {
        assert_eq!(ValueType::infer("1"), ValueType::Boolean);
        assert_eq!(ValueType::infer("0"), ValueType::Boolean);
        assert_eq!(ValueType::infer("true"), ValueType::Boolean);
        assert_eq!(ValueType::infer("✓"), ValueType::Boolean);

        assert_eq!(ValueType::infer("~"), ValueType::Null);
        assert_eq!(ValueType::infer("null"), ValueType::Null);

        assert_eq!(ValueType::infer("*ref"), ValueType::Reference);
        assert_eq!(ValueType::infer("→ref"), ValueType::Reference);

        assert_eq!(ValueType::infer("42"), ValueType::Integer);
        assert_eq!(ValueType::infer("3.14"), ValueType::Float);
        assert_eq!(ValueType::infer("\"hello\""), ValueType::String);
    }

    #[test]
    fn test_config_defaults() {
        let config = HologramConfig::default();
        assert_eq!(config.indent_size, 4);
        assert!(config.use_unicode_symbols);
        assert_eq!(config.section_marker, '▼');
    }

    #[test]
    fn test_config_ascii() {
        let config = HologramConfig::ascii();
        assert!(!config.use_unicode_symbols);
        assert_eq!(config.section_marker, '>');
        assert_eq!(config.bullet_char, '-');
    }
}
