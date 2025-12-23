//! Human format parser
//!
//! Parses beautiful human-readable format back to DxDocument.
//!
//! ## Human Format Syntax
//!
//! ```text
//! # ═══════════════════════════════════════════════════════════════════════════════
//! #                                   SECTION NAME
//! # ═══════════════════════════════════════════════════════════════════════════════
//!
//! [config]
//!     key      = "value"
//!     long_key = "another value"
//!
//! [references]
//!     A = "Some Value"
//!
//! [data_section]
//!     ┌──────┬──────┬──────┐
//!     │ Col1 │ Col2 │ Col3 │
//!     ├──────┼──────┼──────┤
//!     │ val  │ val  │  ✓   │
//!     │ val  │ val  │  ✗   │
//!     └──────┴──────┴──────┘
//!
//!     Total: 2 rows | Summary info
//! ```

use crate::llm::abbrev::AbbrevDict;
use crate::llm::types::{DxDocument, DxLlmValue, DxSection};
use std::collections::HashMap;
use thiserror::Error;

/// Parse errors for Human format
#[derive(Debug, Error)]
pub enum HumanParseError {
    #[error("Invalid section header: {msg}")]
    InvalidSectionHeader { msg: String },

    #[error("Invalid key-value pair: {msg}")]
    InvalidKeyValue { msg: String },

    #[error("Invalid table format at line {line}: {msg}")]
    InvalidTable { line: usize, msg: String },

    #[error("Unexpected content: {msg}")]
    UnexpectedContent { msg: String },
}

/// Parse human-readable format back to DxDocument
pub struct HumanParser {
    abbrev: AbbrevDict,
}


impl HumanParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            abbrev: AbbrevDict::new(),
        }
    }

    /// Parse human format string into DxDocument
    pub fn parse(&self, input: &str) -> Result<DxDocument, HumanParseError> {
        let mut doc = DxDocument::new();
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // Skip empty lines and comment headers (═══)
            if line.is_empty() || line.starts_with("# ═") || line.starts_with("#") && !line.starts_with("[") {
                i += 1;
                continue;
            }

            // Check for section header
            if let Some(section_name) = self.parse_section_header(line) {
                i += 1;
                match section_name.to_lowercase().as_str() {
                    "config" | "configuration" => {
                        let (context, consumed) = self.parse_config_section(&lines[i..])?;
                        doc.context = context;
                        i += consumed;
                    }
                    "references" | "refs" => {
                        let (refs, consumed) = self.parse_references_section(&lines[i..])?;
                        doc.refs = refs;
                        i += consumed;
                    }
                    _ => {
                        // Data section
                        let section_id = self.section_name_to_id(&section_name);
                        let (section, consumed) = self.parse_data_section(&lines[i..], &section_name)?;
                        doc.sections.insert(section_id, section);
                        i += consumed;
                    }
                }
            } else {
                i += 1;
            }
        }

        Ok(doc)
    }

    /// Parse section header: [section_name]
    fn parse_section_header(&self, line: &str) -> Option<String> {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let name = line[1..line.len() - 1].trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
        None
    }

    /// Parse config section with key-value pairs
    fn parse_config_section(
        &self,
        lines: &[&str],
    ) -> Result<(HashMap<String, DxLlmValue>, usize), HumanParseError> {
        let mut context = HashMap::new();
        let mut consumed = 0;

        for line in lines {
            let line = line.trim();

            // Stop at next section or empty line followed by section
            if line.starts_with('[') || line.starts_with("# ═") {
                break;
            }

            consumed += 1;

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse key = value
            if let Some((key, value)) = self.parse_key_value(line)? {
                // Compress key back to abbreviation
                let compressed_key = self.abbrev.compress(&key);
                context.insert(compressed_key, value);
            }
        }

        Ok((context, consumed))
    }

    /// Parse references section
    fn parse_references_section(
        &self,
        lines: &[&str],
    ) -> Result<(HashMap<String, String>, usize), HumanParseError> {
        let mut refs = HashMap::new();
        let mut consumed = 0;

        for line in lines {
            let line = line.trim();

            // Stop at next section
            if line.starts_with('[') || line.starts_with("# ═") {
                break;
            }

            consumed += 1;

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse key = "value"
            if let Some((key, value)) = self.parse_key_value(line)? {
                if let DxLlmValue::Str(s) = value {
                    refs.insert(key, s);
                }
            }
        }

        Ok((refs, consumed))
    }


    /// Parse key-value pair: key = "value" or key = value
    fn parse_key_value(&self, line: &str) -> Result<Option<(String, DxLlmValue)>, HumanParseError> {
        let line = line.trim();

        // Skip comments
        if line.starts_with('#') {
            return Ok(None);
        }

        // Find the = separator
        let eq_pos = line.find('=');
        if eq_pos.is_none() {
            return Ok(None);
        }

        let eq_pos = eq_pos.unwrap();
        let key = line[..eq_pos].trim().to_string();
        let mut value_str = line[eq_pos + 1..].trim();

        // Remove trailing comment (# ref: ...)
        if let Some(comment_pos) = value_str.find("  #") {
            value_str = value_str[..comment_pos].trim();
        }

        let value = self.parse_config_value(value_str)?;
        Ok(Some((key, value)))
    }

    /// Parse a config value (string, number, bool, null, array)
    fn parse_config_value(&self, s: &str) -> Result<DxLlmValue, HumanParseError> {
        let s = s.trim();

        // Quoted string
        if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
            return Ok(DxLlmValue::Str(s[1..s.len() - 1].to_string()));
        }

        // Boolean
        if s == "true" {
            return Ok(DxLlmValue::Bool(true));
        }
        if s == "false" {
            return Ok(DxLlmValue::Bool(false));
        }

        // Null
        if s == "null" {
            return Ok(DxLlmValue::Null);
        }

        // Array
        if s.starts_with('[') && s.ends_with(']') {
            let inner = s[1..s.len() - 1].trim();
            if inner.is_empty() {
                return Ok(DxLlmValue::Arr(vec![]));
            }
            let items: Result<Vec<DxLlmValue>, _> = inner
                .split(',')
                .map(|item| self.parse_config_value(item.trim()))
                .collect();
            return Ok(DxLlmValue::Arr(items?));
        }

        // Number
        if let Ok(n) = s.parse::<f64>() {
            return Ok(DxLlmValue::Num(n));
        }

        // Default to string (unquoted)
        Ok(DxLlmValue::Str(s.to_string()))
    }

    /// Parse data section with table
    fn parse_data_section(
        &self,
        lines: &[&str],
        section_name: &str,
    ) -> Result<(DxSection, usize), HumanParseError> {
        let mut consumed = 0;
        let mut table_lines: Vec<&str> = Vec::new();
        let mut in_table = false;

        for line in lines {
            let trimmed = line.trim();

            // Stop at next section
            if trimmed.starts_with('[') && !in_table {
                break;
            }

            // Stop at section header
            if trimmed.starts_with("# ═") {
                break;
            }

            consumed += 1;

            // Skip empty lines and summary lines before table
            if trimmed.is_empty() {
                if in_table {
                    // End of table
                    break;
                }
                continue;
            }

            // Skip summary lines
            if trimmed.starts_with("Total:") {
                continue;
            }

            // Detect table start
            if trimmed.starts_with('┌') || trimmed.starts_with('+') || trimmed.starts_with('|') {
                in_table = true;
            }

            if in_table {
                table_lines.push(trimmed);
                // Detect table end
                if trimmed.starts_with('└') || (trimmed.starts_with('+') && table_lines.len() > 2) {
                    break;
                }
            }
        }

        let section = self.parse_table(&table_lines, section_name)?;
        Ok((section, consumed))
    }


    /// Parse Unicode or ASCII table
    fn parse_table(
        &self,
        lines: &[&str],
        context: &str,
    ) -> Result<DxSection, HumanParseError> {
        if lines.is_empty() {
            return Ok(DxSection::new(vec![]));
        }

        // Detect table style
        let is_unicode = lines[0].contains('┌') || lines[0].contains('│');
        let is_markdown = lines[0].starts_with('|') && !lines[0].contains('┌');

        let (schema, rows) = if is_unicode {
            self.parse_unicode_table(lines, context)?
        } else if is_markdown {
            self.parse_markdown_table(lines, context)?
        } else {
            self.parse_ascii_table(lines, context)?
        };

        let mut section = DxSection::new(schema);
        for row in rows {
            section.rows.push(row);
        }

        Ok(section)
    }

    /// Parse Unicode box-drawn table
    fn parse_unicode_table(
        &self,
        lines: &[&str],
        _context: &str,
    ) -> Result<(Vec<String>, Vec<Vec<DxLlmValue>>), HumanParseError> {
        let mut schema = Vec::new();
        let mut rows = Vec::new();
        let mut header_found = false;
        let mut separator_found = false;

        for line in lines {
            let line = line.trim();

            // Skip top border and separator lines
            if line.starts_with('┌') || line.starts_with('├') || line.starts_with('└') {
                if line.starts_with('├') {
                    separator_found = true;
                }
                continue;
            }

            // Parse row with │ separators
            if line.starts_with('│') && line.ends_with('│') {
                let cells: Vec<&str> = line[3..line.len() - 3]
                    .split('│')
                    .map(|s| s.trim())
                    .collect();

                if !header_found {
                    // This is the header row
                    schema = cells
                        .iter()
                        .map(|col| self.abbrev.compress(col))
                        .collect();
                    header_found = true;
                } else if separator_found {
                    // This is a data row
                    let row: Vec<DxLlmValue> = cells
                        .iter()
                        .map(|cell| self.parse_cell_value(cell))
                        .collect();
                    rows.push(row);
                }
            }
        }

        Ok((schema, rows))
    }

    /// Parse ASCII table
    fn parse_ascii_table(
        &self,
        lines: &[&str],
        _context: &str,
    ) -> Result<(Vec<String>, Vec<Vec<DxLlmValue>>), HumanParseError> {
        let mut schema = Vec::new();
        let mut rows = Vec::new();
        let mut header_found = false;
        let mut separator_count = 0;

        for line in lines {
            let line = line.trim();

            // Skip border lines
            if line.starts_with('+') {
                separator_count += 1;
                continue;
            }

            // Parse row with | separators
            if line.starts_with('|') && line.ends_with('|') {
                let cells: Vec<&str> = line[1..line.len() - 1]
                    .split('|')
                    .map(|s| s.trim())
                    .collect();

                if !header_found {
                    schema = cells
                        .iter()
                        .map(|col| self.abbrev.compress(col))
                        .collect();
                    header_found = true;
                } else if separator_count >= 2 {
                    let row: Vec<DxLlmValue> = cells
                        .iter()
                        .map(|cell| self.parse_cell_value(cell))
                        .collect();
                    rows.push(row);
                }
            }
        }

        Ok((schema, rows))
    }

    /// Parse Markdown table
    fn parse_markdown_table(
        &self,
        lines: &[&str],
        _context: &str,
    ) -> Result<(Vec<String>, Vec<Vec<DxLlmValue>>), HumanParseError> {
        let mut schema = Vec::new();
        let mut rows = Vec::new();
        let mut header_found = false;
        let mut separator_found = false;

        for line in lines {
            let line = line.trim();

            // Skip separator line (| --- | --- |)
            if line.contains("---") {
                separator_found = true;
                continue;
            }

            // Parse row with | separators
            if line.starts_with('|') && line.ends_with('|') {
                let cells: Vec<&str> = line[1..line.len() - 1]
                    .split('|')
                    .map(|s| s.trim())
                    .collect();

                if !header_found {
                    schema = cells
                        .iter()
                        .map(|col| self.abbrev.compress(col))
                        .collect();
                    header_found = true;
                } else if separator_found {
                    let row: Vec<DxLlmValue> = cells
                        .iter()
                        .map(|cell| self.parse_cell_value(cell))
                        .collect();
                    rows.push(row);
                }
            }
        }

        Ok((schema, rows))
    }


    /// Parse table cell value
    ///
    /// Recognizes special symbols:
    /// - ✓ → boolean true
    /// - ✗ → boolean false
    /// - — → null
    fn parse_cell_value(&self, s: &str) -> DxLlmValue {
        let s = s.trim();

        // Boolean true
        if s == "✓" || s == "true" {
            return DxLlmValue::Bool(true);
        }

        // Boolean false
        if s == "✗" || s == "false" {
            return DxLlmValue::Bool(false);
        }

        // Null
        if s == "—" || s == "null" || s == "-" && s.len() == 1 {
            return DxLlmValue::Null;
        }

        // Array
        if s.starts_with('[') && s.ends_with(']') {
            let inner = s[1..s.len() - 1].trim();
            if inner.is_empty() {
                return DxLlmValue::Arr(vec![]);
            }
            let items: Vec<DxLlmValue> = inner
                .split(',')
                .map(|item| self.parse_cell_value(item.trim()))
                .collect();
            return DxLlmValue::Arr(items);
        }

        // Number
        if let Ok(n) = s.parse::<f64>() {
            return DxLlmValue::Num(n);
        }

        // Default to string
        DxLlmValue::Str(s.to_string())
    }

    /// Convert section name to single-character ID
    fn section_name_to_id(&self, name: &str) -> char {
        match name.to_lowercase().as_str() {
            "data" => 'd',
            "hikes" => 'h',
            "orders" => 'o',
            "products" => 'p',
            "users" => 'u',
            "items" => 'i',
            "tasks" => 't',
            "events" => 'e',
            _ => name.chars().next().unwrap_or('x').to_ascii_lowercase(),
        }
    }

    /// Get the abbreviation dictionary
    pub fn abbrev(&self) -> &AbbrevDict {
        &self.abbrev
    }
}

impl Default for HumanParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let parser = HumanParser::new();
        let doc = parser.parse("").unwrap();
        assert!(doc.is_empty());
    }

    #[test]
    fn test_parse_section_header() {
        let parser = HumanParser::new();
        assert_eq!(parser.parse_section_header("[config]"), Some("config".to_string()));
        assert_eq!(parser.parse_section_header("[data]"), Some("data".to_string()));
        assert_eq!(parser.parse_section_header("not a header"), None);
    }

    #[test]
    fn test_parse_config_section() {
        let parser = HumanParser::new();
        let input = r#"
[config]
    name  = "Test"
    count = 42
    active = true
"#;
        let doc = parser.parse(input).unwrap();
        assert_eq!(doc.context.len(), 3);
        assert_eq!(doc.context.get("nm").unwrap().as_str(), Some("Test"));
        assert_eq!(doc.context.get("ct").unwrap().as_num(), Some(42.0));
        assert_eq!(doc.context.get("ac").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_parse_references_section() {
        let parser = HumanParser::new();
        let input = r#"
[references]
    A = "Shared Value"
    B = "Another Value"
"#;
        let doc = parser.parse(input).unwrap();
        assert_eq!(doc.refs.len(), 2);
        assert_eq!(doc.refs.get("A"), Some(&"Shared Value".to_string()));
        assert_eq!(doc.refs.get("B"), Some(&"Another Value".to_string()));
    }

    #[test]
    fn test_parse_cell_values() {
        let parser = HumanParser::new();
        
        assert_eq!(parser.parse_cell_value("✓"), DxLlmValue::Bool(true));
        assert_eq!(parser.parse_cell_value("✗"), DxLlmValue::Bool(false));
        assert_eq!(parser.parse_cell_value("—"), DxLlmValue::Null);
        assert_eq!(parser.parse_cell_value("42"), DxLlmValue::Num(42.0));
        assert_eq!(parser.parse_cell_value("hello"), DxLlmValue::Str("hello".to_string()));
    }

    #[test]
    fn test_parse_unicode_table() {
        let parser = HumanParser::new();
        let input = r#"
[data]
    ┌─────┬───────┬────────┐
    │ id  │ name  │ active │
    ├─────┼───────┼────────┤
    │ 1   │ Alpha │   ✓    │
    │ 2   │ Beta  │   ✗    │
    └─────┴───────┴────────┘
"#;
        let doc = parser.parse(input).unwrap();
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
    fn test_parse_markdown_table() {
        let parser = HumanParser::new();
        let input = r#"
[data]
    | id  | name  | active |
    | --- | ----- | ------ |
    | 1   | Alpha | true   |
    | 2   | Beta  | false  |
"#;
        let doc = parser.parse(input).unwrap();
        assert!(doc.sections.contains_key(&'d'));
        
        let section = doc.sections.get(&'d').unwrap();
        assert_eq!(section.rows.len(), 2);
    }

    #[test]
    fn test_parse_full_document() {
        let parser = HumanParser::new();
        let input = r#"
# ════════════════════════════════════════════════════════════════════════════════
#                                  CONFIGURATION
# ════════════════════════════════════════════════════════════════════════════════

[config]
    name    = "Test Document"
    version = 1.0

# ════════════════════════════════════════════════════════════════════════════════
#                                    REFERENCES
# ════════════════════════════════════════════════════════════════════════════════

[references]
    A = "Shared Value"

# ════════════════════════════════════════════════════════════════════════════════
#                                       DATA
# ════════════════════════════════════════════════════════════════════════════════

[data]
    ┌─────┬───────┐
    │ id  │ value │
    ├─────┼───────┤
    │ 1   │ Alpha │
    └─────┴───────┘

    Total: 1 rows
"#;
        let doc = parser.parse(input).unwrap();
        
        assert!(!doc.context.is_empty());
        assert!(!doc.refs.is_empty());
        assert!(!doc.sections.is_empty());
    }

    #[test]
    fn test_section_name_to_id() {
        let parser = HumanParser::new();
        assert_eq!(parser.section_name_to_id("data"), 'd');
        assert_eq!(parser.section_name_to_id("hikes"), 'h');
        assert_eq!(parser.section_name_to_id("orders"), 'o');
        assert_eq!(parser.section_name_to_id("unknown"), 'u');
    }
}
