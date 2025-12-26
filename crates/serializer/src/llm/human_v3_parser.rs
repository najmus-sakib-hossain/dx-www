//! Human V3 format parser
//!
//! This module provides parsing support for Human Format V3, which is a TOML-like
//! structure where:
//! - Config values appear at the top without any section header
//! - Sections use full names like `[forge]`, `[style]`, `[media]`
//! - Nested sections use dot notation like `[i18n.locales]`, `[js.dependencies]`
//! - Arrays use pipe separators: `workspace = @/www | @/backend`
//! - The `[stack]` section contains reference definitions

use std::collections::HashMap;
use std::fmt;

use super::abbrev::AbbrevDict;
use super::section_names::SectionNameDict;
use super::types::{DxDocument, DxLlmValue, DxSection};

/// Error type for Human V3 parsing
#[derive(Debug, Clone, PartialEq)]
pub struct HumanV3ParseError {
    /// Error message
    pub message: String,
    /// Line number where the error occurred (1-indexed)
    pub line: u32,
    /// Column number where the error occurred (1-indexed)
    pub column: u32,
    /// Optional hint for how to fix the problem
    pub hint: Option<String>,
}

impl HumanV3ParseError {
    /// Create a new parse error
    pub fn new(message: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            message: message.into(),
            line,
            column,
            hint: None,
        }
    }

    /// Create a new parse error with a hint
    pub fn with_hint(message: impl Into<String>, line: u32, column: u32, hint: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line,
            column,
            hint: Some(hint.into()),
        }
    }
}

impl fmt::Display for HumanV3ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at line {}, column {}: {}", self.line, self.column, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, " (hint: {})", hint)?;
        }
        Ok(())
    }
}

impl std::error::Error for HumanV3ParseError {}

/// Section header types
#[derive(Debug, Clone, PartialEq)]
pub enum SectionHeader {
    /// Simple section header: `[forge]` -> "forge"
    Simple(String),
    /// Nested section header: `[i18n.locales]` -> ("i18n", "locales")
    Nested(String, String),
}

/// Human V3 format parser
///
/// Parses Human Format V3 into `DxDocument` for conversion to LLM format.
#[derive(Debug, Clone)]
pub struct HumanV3Parser {
    /// Key abbreviation dictionary
    abbrev: AbbrevDict,
    /// Section name dictionary
    section_names: SectionNameDict,
}

impl HumanV3Parser {
    /// Create a new Human V3 parser
    pub fn new() -> Self {
        Self {
            abbrev: AbbrevDict::new(),
            section_names: SectionNameDict::new(),
        }
    }

    /// Parse Human V3 format into DxDocument
    pub fn parse(&self, input: &str) -> Result<DxDocument, HumanV3ParseError> {
        let mut doc = DxDocument::new();
        let lines: Vec<&str> = input.lines().collect();
        let mut line_idx = 0;

        // Track nested sections for merging
        let mut nested_sections: HashMap<char, NestedSectionData> = HashMap::new();

        // Parse config section (lines before any section header)
        let (context, consumed) = self.parse_config_section(&lines, line_idx)?;
        doc.context = context;
        line_idx += consumed;

        // Parse remaining sections
        while line_idx < lines.len() {
            let line = lines[line_idx].trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                line_idx += 1;
                continue;
            }

            // Check for section header
            if let Some(header) = self.parse_section_header(line) {
                line_idx += 1;

                match header {
                    SectionHeader::Simple(name) => {
                        if name == "stack" {
                            // Parse stack section as references
                            let (refs, consumed) = self.parse_stack_section(&lines, line_idx)?;
                            doc.refs = refs;
                            line_idx += consumed;
                        } else {
                            // Parse regular data section
                            let section_id = self.section_names.name_to_id(&name);
                            let section_char = section_id.chars().next().unwrap_or('?');
                            let (section, consumed) = self.parse_data_section(&lines, line_idx, &name)?;
                            doc.sections.insert(section_char, section);
                            line_idx += consumed;
                        }
                    }
                    SectionHeader::Nested(parent, child) => {
                        // Parse nested section
                        let parent_id = self.section_names.name_to_id(&parent);
                        let parent_char = parent_id.chars().next().unwrap_or('?');
                        let (data, consumed) = self.parse_nested_section(&lines, line_idx, &child)?;
                        
                        // Add to nested sections for later merging
                        let nested = nested_sections.entry(parent_char).or_insert_with(|| {
                            NestedSectionData {
                                subsections: HashMap::new(),
                                subsection_order: Vec::new(),
                            }
                        });
                        nested.subsection_order.push(child.clone());
                        nested.subsections.insert(child, data);
                        line_idx += consumed;
                    }
                }
            } else {
                // Unexpected content
                return Err(HumanV3ParseError::with_hint(
                    format!("Unexpected content: {}", line),
                    (line_idx + 1) as u32,
                    1,
                    "Expected a section header like [forge] or a key = value pair",
                ));
            }
        }

        // Merge nested sections into parent sections
        for (parent_char, nested) in nested_sections {
            let section = self.merge_nested_sections(nested)?;
            doc.sections.insert(parent_char, section);
        }

        Ok(doc)
    }

    /// Parse value from string
    ///
    /// Handles:
    /// - Quoted strings (double and single quotes)
    /// - Null markers (`-`, `~`)
    /// - Booleans (`true`, `false`)
    /// - Numbers (integer and float)
    /// - Arrays (pipe separators)
    pub fn parse_value(&self, raw: &str) -> DxLlmValue {
        let trimmed = raw.trim();

        // Check for null markers
        if trimmed == "-" || trimmed == "~" || trimmed == "none" {
            return DxLlmValue::Null;
        }

        // Check for booleans
        if trimmed == "true" {
            return DxLlmValue::Bool(true);
        }
        if trimmed == "false" {
            return DxLlmValue::Bool(false);
        }

        // Check for arrays (pipe separators)
        if trimmed.contains(" | ") {
            let parts: Vec<DxLlmValue> = trimmed
                .split(" | ")
                .map(|p| self.parse_value(p))
                .collect();
            return DxLlmValue::Arr(parts);
        }

        // Check for quoted strings
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            let inner = &trimmed[1..trimmed.len() - 1];
            return DxLlmValue::Str(inner.to_string());
        }

        // Check for numbers
        if let Ok(n) = trimmed.parse::<f64>() {
            return DxLlmValue::Num(n);
        }

        // Check for reference pointers
        if trimmed.starts_with('^') {
            return DxLlmValue::Ref(trimmed[1..].to_string());
        }

        // Default to string
        DxLlmValue::Str(trimmed.to_string())
    }

    /// Parse config section (key-value pairs before any section header)
    fn parse_config_section(
        &self,
        lines: &[&str],
        start: usize,
    ) -> Result<(HashMap<String, DxLlmValue>, usize), HumanV3ParseError> {
        let mut context = HashMap::new();
        let mut consumed = 0;

        for (i, line) in lines.iter().enumerate().skip(start) {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                consumed += 1;
                continue;
            }

            // Stop at section header
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                break;
            }

            // Parse key-value pair
            if let Some((key, value)) = self.parse_key_value(trimmed, (start + i + 1) as u32)? {
                // Compress key to abbreviated form
                let abbrev_key = self.abbrev.compress(&key);
                context.insert(abbrev_key, value);
                consumed += 1;
            } else {
                // Not a valid key-value pair, stop config section
                break;
            }
        }

        Ok((context, consumed))
    }

    /// Parse section header
    ///
    /// Returns `Some(SectionHeader)` if the line is a valid section header,
    /// `None` otherwise.
    pub fn parse_section_header(&self, line: &str) -> Option<SectionHeader> {
        let trimmed = line.trim();

        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return None;
        }

        let inner = &trimmed[1..trimmed.len() - 1];

        // Check for nested section (contains dot)
        if let Some(dot_pos) = inner.find('.') {
            let parent = &inner[..dot_pos];
            let child = &inner[dot_pos + 1..];
            Some(SectionHeader::Nested(parent.to_string(), child.to_string()))
        } else {
            Some(SectionHeader::Simple(inner.to_string()))
        }
    }

    /// Parse stack section as reference definitions
    fn parse_stack_section(
        &self,
        lines: &[&str],
        start: usize,
    ) -> Result<(HashMap<String, String>, usize), HumanV3ParseError> {
        let mut refs = HashMap::new();
        let mut consumed = 0;

        for line in lines.iter().skip(start) {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                consumed += 1;
                continue;
            }

            // Stop at next section header
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                break;
            }

            // Parse key-value pair (preserve key names, don't abbreviate)
            if let Some(eq_pos) = trimmed.find('=') {
                let key = trimmed[..eq_pos].trim().to_string();
                let value_str = trimmed[eq_pos + 1..].trim();

                // Join pipe-separated values with |
                let value = if value_str.contains(" | ") {
                    value_str.split(" | ").collect::<Vec<_>>().join("|")
                } else {
                    value_str.to_string()
                };

                refs.insert(key, value);
                consumed += 1;
            } else {
                // Not a valid key-value pair, stop
                break;
            }
        }

        Ok((refs, consumed))
    }

    /// Parse regular data section
    fn parse_data_section(
        &self,
        lines: &[&str],
        start: usize,
        _section_name: &str,
    ) -> Result<(DxSection, usize), HumanV3ParseError> {
        let mut schema = Vec::new();
        let mut row_values = Vec::new();
        let mut consumed = 0;

        for (i, line) in lines.iter().enumerate().skip(start) {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                consumed += 1;
                continue;
            }

            // Stop at next section header
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                break;
            }

            // Parse key-value pair
            if let Some((key, value)) = self.parse_key_value(trimmed, (start + i + 1) as u32)? {
                // Compress key to abbreviated form for schema
                let abbrev_key = self.abbrev.compress(&key);
                schema.push(abbrev_key);
                row_values.push(value);
                consumed += 1;
            } else {
                // Not a valid key-value pair, stop
                break;
            }
        }

        // Create section with single row
        let mut section = DxSection::new(schema);
        if !row_values.is_empty() {
            section.rows.push(row_values);
        }

        Ok((section, consumed))
    }

    /// Parse nested section content
    fn parse_nested_section(
        &self,
        lines: &[&str],
        start: usize,
        _subsection_name: &str,
    ) -> Result<(Vec<(String, DxLlmValue)>, usize), HumanV3ParseError> {
        let mut data = Vec::new();
        let mut consumed = 0;

        for (i, line) in lines.iter().enumerate().skip(start) {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                consumed += 1;
                continue;
            }

            // Stop at next section header
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                break;
            }

            // Parse key-value pair
            if let Some((key, value)) = self.parse_key_value(trimmed, (start + i + 1) as u32)? {
                data.push((key, value));
                consumed += 1;
            } else {
                // Not a valid key-value pair, stop
                break;
            }
        }

        Ok((data, consumed))
    }

    /// Merge nested sections into a single parent section
    fn merge_nested_sections(&self, nested: NestedSectionData) -> Result<DxSection, HumanV3ParseError> {
        let mut schema = Vec::new();
        let mut row_values = Vec::new();

        // Process subsections in order
        for subsection_name in &nested.subsection_order {
            if let Some(data) = nested.subsections.get(subsection_name) {
                for (key, value) in data {
                    // Prefix key with subsection name and compress
                    let prefixed_key = format!("{}_{}", subsection_name, key);
                    let abbrev_key = self.abbrev.compress(&prefixed_key);
                    schema.push(abbrev_key);
                    row_values.push(value.clone());
                }
            }
        }

        let mut section = DxSection::new(schema);
        if !row_values.is_empty() {
            section.rows.push(row_values);
        }

        Ok(section)
    }

    /// Parse a key-value pair from a line
    fn parse_key_value(
        &self,
        line: &str,
        line_num: u32,
    ) -> Result<Option<(String, DxLlmValue)>, HumanV3ParseError> {
        // Find the equals sign
        let eq_pos = match line.find('=') {
            Some(pos) => pos,
            None => return Ok(None),
        };

        let key = line[..eq_pos].trim();
        let value_str = line[eq_pos + 1..].trim();

        // Check for unclosed quotes
        let double_quotes = value_str.chars().filter(|&c| c == '"').count();
        let single_quotes = value_str.chars().filter(|&c| c == '\'').count();

        if double_quotes % 2 != 0 {
            return Err(HumanV3ParseError::with_hint(
                "Unclosed double quote",
                line_num,
                (eq_pos + 2) as u32,
                "Add a closing \" to the value",
            ));
        }

        if single_quotes % 2 != 0 {
            return Err(HumanV3ParseError::with_hint(
                "Unclosed single quote",
                line_num,
                (eq_pos + 2) as u32,
                "Add a closing ' to the value",
            ));
        }

        let value = self.parse_value(value_str);
        Ok(Some((key.to_string(), value)))
    }
}

impl Default for HumanV3Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal structure for tracking nested sections before merging
#[derive(Debug)]
struct NestedSectionData {
    /// Subsection name → key-value pairs
    subsections: HashMap<String, Vec<(String, DxLlmValue)>>,
    /// Order of subsections for consistent output
    subsection_order: Vec<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_null() {
        let parser = HumanV3Parser::new();

        assert_eq!(parser.parse_value("-"), DxLlmValue::Null);
        assert_eq!(parser.parse_value("~"), DxLlmValue::Null);
        assert_eq!(parser.parse_value("none"), DxLlmValue::Null);
    }

    #[test]
    fn test_parse_value_boolean() {
        let parser = HumanV3Parser::new();

        assert_eq!(parser.parse_value("true"), DxLlmValue::Bool(true));
        assert_eq!(parser.parse_value("false"), DxLlmValue::Bool(false));
    }

    #[test]
    fn test_parse_value_number() {
        let parser = HumanV3Parser::new();

        assert_eq!(parser.parse_value("42"), DxLlmValue::Num(42.0));
        assert_eq!(parser.parse_value("3.14"), DxLlmValue::Num(3.14));
        assert_eq!(parser.parse_value("-10"), DxLlmValue::Num(-10.0));
        assert_eq!(parser.parse_value("0.5"), DxLlmValue::Num(0.5));
    }

    #[test]
    fn test_parse_value_quoted_string() {
        let parser = HumanV3Parser::new();

        assert_eq!(
            parser.parse_value("\"hello world\""),
            DxLlmValue::Str("hello world".to_string())
        );
        assert_eq!(
            parser.parse_value("'single quotes'"),
            DxLlmValue::Str("single quotes".to_string())
        );
    }

    #[test]
    fn test_parse_value_unquoted_string() {
        let parser = HumanV3Parser::new();

        assert_eq!(
            parser.parse_value("simple"),
            DxLlmValue::Str("simple".to_string())
        );
        assert_eq!(
            parser.parse_value("@/path/to/file"),
            DxLlmValue::Str("@/path/to/file".to_string())
        );
    }

    #[test]
    fn test_parse_value_array() {
        let parser = HumanV3Parser::new();

        let result = parser.parse_value("a | b | c");
        match result {
            DxLlmValue::Arr(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], DxLlmValue::Str("a".to_string()));
                assert_eq!(arr[1], DxLlmValue::Str("b".to_string()));
                assert_eq!(arr[2], DxLlmValue::Str("c".to_string()));
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_parse_value_reference() {
        let parser = HumanV3Parser::new();

        assert_eq!(
            parser.parse_value("^js"),
            DxLlmValue::Ref("js".to_string())
        );
    }

    #[test]
    fn test_parse_section_header_simple() {
        let parser = HumanV3Parser::new();

        assert_eq!(
            parser.parse_section_header("[forge]"),
            Some(SectionHeader::Simple("forge".to_string()))
        );
        assert_eq!(
            parser.parse_section_header("[stack]"),
            Some(SectionHeader::Simple("stack".to_string()))
        );
    }

    #[test]
    fn test_parse_section_header_nested() {
        let parser = HumanV3Parser::new();

        assert_eq!(
            parser.parse_section_header("[i18n.locales]"),
            Some(SectionHeader::Nested("i18n".to_string(), "locales".to_string()))
        );
        assert_eq!(
            parser.parse_section_header("[js.dependencies]"),
            Some(SectionHeader::Nested("js".to_string(), "dependencies".to_string()))
        );
    }

    #[test]
    fn test_parse_section_header_invalid() {
        let parser = HumanV3Parser::new();

        assert_eq!(parser.parse_section_header("not a header"), None);
        assert_eq!(parser.parse_section_header("[incomplete"), None);
        assert_eq!(parser.parse_section_header("incomplete]"), None);
    }

    #[test]
    fn test_parse_config_section() {
        let parser = HumanV3Parser::new();

        let input = r#"name = dx
version = 0.0.1
title = "Enhanced Developing Experience"

[forge]
"#;
        let lines: Vec<&str> = input.lines().collect();
        let (context, consumed) = parser.parse_config_section(&lines, 0).unwrap();

        assert_eq!(consumed, 4); // 3 key-value pairs + 1 empty line
        assert_eq!(context.get("nm"), Some(&DxLlmValue::Str("dx".to_string())));
        assert_eq!(context.get("vr"), Some(&DxLlmValue::Str("0.0.1".to_string())));
        assert_eq!(
            context.get("tt"),
            Some(&DxLlmValue::Str("Enhanced Developing Experience".to_string()))
        );
    }

    #[test]
    fn test_parse_stack_section() {
        let parser = HumanV3Parser::new();

        let input = r#"js = javascript/typescript | bun | tsc | vite | bun | react
python = py | python | python | uv | pip | django

[style]
"#;
        let lines: Vec<&str> = input.lines().collect();
        let (refs, consumed) = parser.parse_stack_section(&lines, 0).unwrap();

        assert_eq!(consumed, 3); // 2 key-value pairs + 1 empty line
        assert_eq!(
            refs.get("js"),
            Some(&"javascript/typescript|bun|tsc|vite|bun|react".to_string())
        );
        assert_eq!(
            refs.get("python"),
            Some(&"py|python|python|uv|pip|django".to_string())
        );
    }

    #[test]
    fn test_parse_data_section() {
        let parser = HumanV3Parser::new();

        let input = r#"path = @/style
name = dx

[media]
"#;
        let lines: Vec<&str> = input.lines().collect();
        let (section, consumed) = parser.parse_data_section(&lines, 0, "style").unwrap();

        assert_eq!(consumed, 3); // 2 key-value pairs + 1 empty line
        assert_eq!(section.schema, vec!["pt", "nm"]); // path -> pt, name -> nm
        assert_eq!(section.rows.len(), 1);
    }

    #[test]
    fn test_parse_full_document() {
        let parser = HumanV3Parser::new();

        let input = r#"name = dx
version = 0.0.1

[stack]
js = javascript/typescript | bun

[forge]
repository = https://example.com
"#;
        let doc = parser.parse(input).unwrap();

        // Check context
        assert_eq!(doc.context.get("nm"), Some(&DxLlmValue::Str("dx".to_string())));
        assert_eq!(doc.context.get("vr"), Some(&DxLlmValue::Str("0.0.1".to_string())));

        // Check refs
        assert_eq!(doc.refs.get("js"), Some(&"javascript/typescript|bun".to_string()));

        // Check sections
        assert!(doc.sections.contains_key(&'f')); // forge -> f
    }

    #[test]
    fn test_parse_nested_sections() {
        let parser = HumanV3Parser::new();

        let input = r#"[i18n.locales]
path = @/locales
default = en-US

[i18n.ttses]
path = @/media/sounds
default = en-US
"#;
        let doc = parser.parse(input).unwrap();

        // Check that i18n section exists
        assert!(doc.sections.contains_key(&'i')); // i18n -> i

        let section = doc.sections.get(&'i').unwrap();
        // Schema should have prefixed keys
        assert!(section.schema.iter().any(|k| k.contains("locales")));
        assert!(section.schema.iter().any(|k| k.contains("ttses")));
    }

    #[test]
    fn test_parse_error_unclosed_quote() {
        let parser = HumanV3Parser::new();

        let input = r#"name = "unclosed
version = 0.0.1
"#;
        let result = parser.parse(input);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.message.contains("Unclosed"));
        assert_eq!(err.line, 1);
    }

    #[test]
    fn test_array_element_count() {
        let parser = HumanV3Parser::new();

        // N pipe separators should produce N+1 elements
        let test_cases = [
            ("a", 1),
            ("a | b", 2),
            ("a | b | c", 3),
            ("a | b | c | d", 4),
            ("a | b | c | d | e", 5),
        ];

        for (input, expected_count) in test_cases {
            let result = parser.parse_value(input);
            match result {
                DxLlmValue::Arr(arr) => {
                    assert_eq!(
                        arr.len(),
                        expected_count,
                        "Input '{}' should have {} elements",
                        input,
                        expected_count
                    );
                }
                DxLlmValue::Str(_) if expected_count == 1 => {
                    // Single element is returned as string, not array
                }
                _ => panic!("Unexpected result for input '{}'", input),
            }
        }
    }
}
