//! LLM format parser
//!
//! Parses token-optimized LLM format into DxDocument.
//!
//! ## LLM Format Syntax
//!
//! ```text
//! # Root-level context (key|value pairs without prefix)
//! nm|dx
//! v|0.0.1
//! tt|Enhanced Developing Experience
//!
//! # Reference definitions
//! #:<ref_key>|<ref_value>
//!
//! # Data section with schema
//! #<id>(<col>|<col>|<col>)
//! <val>|<val>|<val>
//! <val>|<val>|<val>
//!
//! # Special values
//! +       → boolean true
//! -       → boolean false
//! ~       → null
//! ^<key>  → reference pointer
//! *a,b,c  → inline array
//! ```

use crate::llm::types::{DxDocument, DxLlmValue, DxSection};
use std::collections::HashMap;
use thiserror::Error;

/// Parse errors for LLM format
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid sigil at position {pos}: expected #c, #:, or #<letter>")]
    InvalidSigil { pos: usize },

    #[error("Malformed context section: {msg}")]
    MalformedContext { msg: String },

    #[error("Malformed reference: {msg}")]
    MalformedReference { msg: String },

    #[error("Malformed section header: {msg}")]
    MalformedSectionHeader { msg: String },

    #[error("Undefined reference: ^{key}")]
    UndefinedReference { key: String },

    #[error("Invalid value format: {value}")]
    InvalidValue { value: String },

    #[error("Schema mismatch: expected {expected} columns, got {got}")]
    SchemaMismatch { expected: usize, got: usize },

    #[error("No active section for data row")]
    NoActiveSection,

    #[error("Invalid UTF-8 at byte offset {offset}")]
    Utf8Error { offset: usize },
}

/// Parse LLM-optimized format into DxDocument
pub struct LlmParser;

impl LlmParser {
    /// Parse LLM format string into DxDocument
    pub fn parse(input: &str) -> Result<DxDocument, ParseError> {
        let mut doc = DxDocument::new();
        let mut current_section: Option<char> = None;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            Self::parse_line(line, &mut doc, &mut current_section)?;
        }

        Ok(doc)
    }

    /// Parse LLM format from bytes with UTF-8 validation
    pub fn parse_bytes(input: &[u8]) -> Result<DxDocument, ParseError> {
        let input_str = std::str::from_utf8(input).map_err(|e| ParseError::Utf8Error {
            offset: e.valid_up_to(),
        })?;
        Self::parse(input_str)
    }


    /// Parse a single line
    fn parse_line(
        line: &str,
        doc: &mut DxDocument,
        current_section: &mut Option<char>,
    ) -> Result<(), ParseError> {
        // Skip comment lines (# followed by space or decorative characters)
        if Self::is_comment_line(line) {
            return Ok(());
        }

        // Legacy support: Context section with #c: prefix
        if let Some(content) = line.strip_prefix("#c:") {
            let context = Self::parse_context_legacy(content)?;
            doc.context.extend(context);
            *current_section = None;
        } else if let Some(content) = line.strip_prefix("#:") {
            // Reference definition: #:key|value
            let (key, value) = Self::parse_reference(content)?;
            doc.refs.insert(key, value);
        } else if line.starts_with('#') && line.len() >= 2 {
            // Check for data section header: #x(col|col|col)
            let id_char = line.chars().nth(1).unwrap();
            if id_char.is_ascii_alphabetic() && line.contains('(') {
                let section = Self::parse_section_header(id_char, line)?;
                doc.sections.insert(id_char, section);
                *current_section = Some(id_char);
            } else {
                // Unknown sigil - skip as comment for forward compatibility
                return Ok(());
            }
        } else if let Some(section_id) = current_section {
            // Data row for current section
            if let Some(section) = doc.sections.get_mut(section_id) {
                let row = Self::parse_row(line, &section.schema)?;
                section.rows.push(row);
            }
        } else if line.contains('|') && !line.starts_with('#') {
            // Root-level key|value pair (context) - new format without #c: prefix
            let (key, value) = Self::parse_context_pair(line)?;
            doc.context.insert(key, value);
        }

        Ok(())
    }

    /// Check if a line is a comment
    ///
    /// Comments are lines starting with:
    /// - '# ' (hash followed by space)
    /// - '# ═' or similar decorative characters
    /// - Lines that are just '#'
    fn is_comment_line(line: &str) -> bool {
        if !line.starts_with('#') {
            return false;
        }

        // Single '#' is a comment
        if line.len() == 1 {
            return true;
        }

        let second_char = line.chars().nth(1).unwrap();

        // '# ' (hash + space) is a comment
        if second_char == ' ' {
            return true;
        }

        // Decorative lines like '# ═══' are comments
        if second_char == '═' || second_char == '─' || second_char == '━' {
            return true;
        }

        false
    }

    /// Parse context section: #c:key|val;key|val (legacy format)
    fn parse_context_legacy(content: &str) -> Result<HashMap<String, DxLlmValue>, ParseError> {
        let mut context = HashMap::new();

        if content.is_empty() {
            return Ok(context);
        }

        for pair in content.split(';') {
            let pair = pair.trim();
            if pair.is_empty() {
                continue;
            }

            let parts: Vec<&str> = pair.splitn(2, '|').collect();
            if parts.len() != 2 {
                return Err(ParseError::MalformedContext {
                    msg: format!("Expected key|value, got: {}", pair),
                });
            }

            let key = parts[0].trim().to_string();
            let value = Self::parse_value(parts[1].trim());
            context.insert(key, value);
        }

        Ok(context)
    }

    /// Parse a single root-level context pair: key|value (new format)
    fn parse_context_pair(line: &str) -> Result<(String, DxLlmValue), ParseError> {
        let parts: Vec<&str> = line.splitn(2, '|').collect();
        if parts.len() != 2 {
            return Err(ParseError::MalformedContext {
                msg: format!("Expected key|value, got: {}", line),
            });
        }

        let key = parts[0].trim().to_string();
        let value = Self::parse_value(parts[1].trim());
        Ok((key, value))
    }

    /// Parse reference: #:key|value
    fn parse_reference(content: &str) -> Result<(String, String), ParseError> {
        let parts: Vec<&str> = content.splitn(2, '|').collect();
        if parts.len() != 2 {
            return Err(ParseError::MalformedReference {
                msg: format!("Expected key|value, got: {}", content),
            });
        }

        Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
    }

    /// Parse data section header: #x(col|col|col)
    fn parse_section_header(id: char, line: &str) -> Result<DxSection, ParseError> {
        // Find the schema part between parentheses
        let start = line.find('(').ok_or_else(|| ParseError::MalformedSectionHeader {
            msg: format!("Missing opening parenthesis in: {}", line),
        })?;
        let end = line.find(')').ok_or_else(|| ParseError::MalformedSectionHeader {
            msg: format!("Missing closing parenthesis in: {}", line),
        })?;

        if start >= end {
            return Err(ParseError::MalformedSectionHeader {
                msg: format!("Invalid parentheses in: {}", line),
            });
        }

        let schema_str = &line[start + 1..end];
        let schema: Vec<String> = schema_str
            .split('|')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if schema.is_empty() {
            return Err(ParseError::MalformedSectionHeader {
                msg: format!("Empty schema in section {}", id),
            });
        }

        Ok(DxSection::new(schema))
    }


    /// Parse data row: val|val|val
    fn parse_row(line: &str, schema: &[String]) -> Result<Vec<DxLlmValue>, ParseError> {
        let values: Vec<DxLlmValue> = line
            .split('|')
            .map(|s| Self::parse_value(s.trim()))
            .collect();

        if values.len() != schema.len() {
            return Err(ParseError::SchemaMismatch {
                expected: schema.len(),
                got: values.len(),
            });
        }

        Ok(values)
    }

    /// Parse a single value
    ///
    /// Handles special values:
    /// - `+` → boolean true
    /// - `-` → boolean false
    /// - `~` → null
    /// - `^<key>` → reference pointer
    /// - `*a,b,c` → inline array
    /// - Numbers (integers and floats)
    /// - Strings (everything else)
    pub fn parse_value(s: &str) -> DxLlmValue {
        let s = s.trim();

        // Boolean true
        if s == "+" {
            return DxLlmValue::Bool(true);
        }

        // Boolean false
        if s == "-" {
            return DxLlmValue::Bool(false);
        }

        // Null
        if s == "~" {
            return DxLlmValue::Null;
        }

        // Reference pointer
        if s.starts_with('^') {
            return DxLlmValue::Ref(s[1..].to_string());
        }

        // Inline array
        if s.starts_with('*') {
            let items: Vec<DxLlmValue> = s[1..]
                .split(',')
                .map(|item| Self::parse_value(item.trim()))
                .collect();
            return DxLlmValue::Arr(items);
        }

        // Try to parse as number
        if let Ok(n) = s.parse::<f64>() {
            return DxLlmValue::Num(n);
        }

        // Default to string
        DxLlmValue::Str(s.to_string())
    }

    /// Resolve all references in a document
    ///
    /// Replaces `DxLlmValue::Ref` with the actual referenced value.
    pub fn resolve_refs(doc: &DxDocument) -> Result<DxDocument, ParseError> {
        let mut resolved = doc.clone();

        // Resolve refs in context
        for value in resolved.context.values_mut() {
            Self::resolve_value(value, &doc.refs)?;
        }

        // Resolve refs in sections
        for section in resolved.sections.values_mut() {
            for row in &mut section.rows {
                for value in row.iter_mut() {
                    Self::resolve_value(value, &doc.refs)?;
                }
            }
        }

        Ok(resolved)
    }

    /// Resolve a single value's references
    fn resolve_value(
        value: &mut DxLlmValue,
        refs: &HashMap<String, String>,
    ) -> Result<(), ParseError> {
        match value {
            DxLlmValue::Ref(key) => {
                if let Some(ref_value) = refs.get(key) {
                    *value = DxLlmValue::Str(ref_value.clone());
                } else {
                    return Err(ParseError::UndefinedReference { key: key.clone() });
                }
            }
            DxLlmValue::Arr(items) => {
                for item in items.iter_mut() {
                    Self::resolve_value(item, refs)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let doc = LlmParser::parse("").unwrap();
        assert!(doc.is_empty());
    }

    #[test]
    fn test_parse_context_new_format() {
        // New format: root-level key|value pairs without #c: prefix
        let input = "nm|Test\nst|active\nct|42";
        let doc = LlmParser::parse(input).unwrap();

        assert_eq!(doc.context.len(), 3);
        assert_eq!(doc.context.get("nm").unwrap().as_str(), Some("Test"));
        assert_eq!(doc.context.get("st").unwrap().as_str(), Some("active"));
        assert_eq!(doc.context.get("ct").unwrap().as_num(), Some(42.0));
    }

    #[test]
    fn test_parse_context_legacy_format() {
        // Legacy format: #c:key|val;key|val (still supported for backward compatibility)
        let input = "#c:nm|Test;st|active;ct|42";
        let doc = LlmParser::parse(input).unwrap();

        assert_eq!(doc.context.len(), 3);
        assert_eq!(doc.context.get("nm").unwrap().as_str(), Some("Test"));
        assert_eq!(doc.context.get("st").unwrap().as_str(), Some("active"));
        assert_eq!(doc.context.get("ct").unwrap().as_num(), Some(42.0));
    }

    #[test]
    fn test_parse_references() {
        let input = "#:A|Some Value\n#:B|Another Value";
        let doc = LlmParser::parse(input).unwrap();

        assert_eq!(doc.refs.len(), 2);
        assert_eq!(doc.refs.get("A"), Some(&"Some Value".to_string()));
        assert_eq!(doc.refs.get("B"), Some(&"Another Value".to_string()));
    }

    #[test]
    fn test_parse_section() {
        let input = "#d(id|nm|ac)\n1|Alpha|+\n2|Beta|-";
        let doc = LlmParser::parse(input).unwrap();

        assert!(doc.sections.contains_key(&'d'));
        let section = doc.sections.get(&'d').unwrap();
        assert_eq!(section.schema, vec!["id", "nm", "ac"]);
        assert_eq!(section.rows.len(), 2);

        // First row
        assert_eq!(section.rows[0][0].as_num(), Some(1.0));
        assert_eq!(section.rows[0][1].as_str(), Some("Alpha"));
        assert_eq!(section.rows[0][2].as_bool(), Some(true));

        // Second row
        assert_eq!(section.rows[1][0].as_num(), Some(2.0));
        assert_eq!(section.rows[1][1].as_str(), Some("Beta"));
        assert_eq!(section.rows[1][2].as_bool(), Some(false));
    }

    #[test]
    fn test_parse_special_values() {
        assert_eq!(LlmParser::parse_value("+"), DxLlmValue::Bool(true));
        assert_eq!(LlmParser::parse_value("-"), DxLlmValue::Bool(false));
        assert_eq!(LlmParser::parse_value("~"), DxLlmValue::Null);
        assert_eq!(
            LlmParser::parse_value("^A"),
            DxLlmValue::Ref("A".to_string())
        );
        assert_eq!(LlmParser::parse_value("42"), DxLlmValue::Num(42.0));
        assert_eq!(LlmParser::parse_value("3.14"), DxLlmValue::Num(3.14));
        assert_eq!(
            LlmParser::parse_value("hello"),
            DxLlmValue::Str("hello".to_string())
        );
    }

    #[test]
    fn test_parse_array() {
        let value = LlmParser::parse_value("*a,b,c");
        if let DxLlmValue::Arr(items) = value {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0].as_str(), Some("a"));
            assert_eq!(items[1].as_str(), Some("b"));
            assert_eq!(items[2].as_str(), Some("c"));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_parse_full_document() {
        let input = r#"
#c:task|Demo;ver|1.0
#:A|Shared Value
#d(id|nm|ref)
1|Item1|^A
2|Item2|^A
"#;
        let doc = LlmParser::parse(input).unwrap();

        assert_eq!(doc.context.len(), 2);
        assert_eq!(doc.refs.len(), 1);
        assert_eq!(doc.sections.len(), 1);

        let section = doc.sections.get(&'d').unwrap();
        assert_eq!(section.rows.len(), 2);
    }

    #[test]
    fn test_resolve_refs() {
        let input = r#"
#:A|Resolved Value
#d(id|val)
1|^A
"#;
        let doc = LlmParser::parse(input).unwrap();
        let resolved = LlmParser::resolve_refs(&doc).unwrap();

        let section = resolved.sections.get(&'d').unwrap();
        assert_eq!(section.rows[0][1].as_str(), Some("Resolved Value"));
    }

    #[test]
    fn test_is_comment_line() {
        // Comments
        assert!(LlmParser::is_comment_line("# This is a comment"));
        assert!(LlmParser::is_comment_line("# ════════════════════"));
        assert!(LlmParser::is_comment_line("# ────────────────────"));
        assert!(LlmParser::is_comment_line("#"));
        assert!(LlmParser::is_comment_line("# "));

        // Not comments (valid sigils)
        assert!(!LlmParser::is_comment_line("#c:key|value"));
        assert!(!LlmParser::is_comment_line("#:ref|value"));
        assert!(!LlmParser::is_comment_line("#d(id|name)"));
    }

    #[test]
    fn test_parse_with_comments() {
        let input = r#"
# This is a comment
# ════════════════════════════════════════════════════════════════════════════════
#                                  CONFIGURATION
# ════════════════════════════════════════════════════════════════════════════════
#c:nm|Test;vr|1.0
# Another comment
#d(id|nm)
1|Alpha
# Comment in data section
2|Beta
"#;
        let doc = LlmParser::parse(input).unwrap();

        assert_eq!(doc.context.len(), 2);
        assert_eq!(doc.context.get("nm").unwrap().as_str(), Some("Test"));
        
        let section = doc.sections.get(&'d').unwrap();
        assert_eq!(section.rows.len(), 2);
    }

    #[test]
    fn test_parse_bytes_valid_utf8() {
        let input = b"#c:nm|Test";
        let doc = LlmParser::parse_bytes(input).unwrap();
        assert_eq!(doc.context.get("nm").unwrap().as_str(), Some("Test"));
    }

    #[test]
    fn test_parse_bytes_invalid_utf8() {
        // Invalid UTF-8 sequence at position 5
        let input = &[0x23, 0x63, 0x3a, 0x6e, 0x6d, 0xFF, 0x7c, 0x54]; // "#c:nm" + invalid + "|T"
        let err = LlmParser::parse_bytes(input).unwrap_err();
        if let ParseError::Utf8Error { offset } = err {
            assert_eq!(offset, 5);
        } else {
            panic!("Expected Utf8Error, got {:?}", err);
        }
    }
}
