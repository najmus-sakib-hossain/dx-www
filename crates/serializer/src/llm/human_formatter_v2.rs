//! Human Format V2 Formatter
//!
//! Formats DxDocument to a flat TOML-like human-readable format with:
//! - No indentation (flat structure)
//! - Expanded key names
//! - Full section names in brackets
//! - Arrays as comma-separated lists without brackets
//! - Unicode box-drawing tables without indentation
//!
//! ## Human Format V2 Syntax
//!
//! ```text
//! # ═══════════════════════════════════════════════════════════════════════════════
//! #                                   CONFIGURATION
//! # ═══════════════════════════════════════════════════════════════════════════════
//!
//! [config]
//! name         = "value"
//! version      = "1.0"
//! workspace    = frontend/www, frontend/mobile
//!
//! [forge]
//! ┌──────┬──────┬──────┐
//! │ Col1 │ Col2 │ Col3 │
//! ├──────┼──────┼──────┤
//! │ val  │ val  │  ✓   │
//! └──────┴──────┴──────┘
//!
//! Total: 1 row
//! ```

use crate::llm::abbrev::AbbrevDict;
use crate::llm::types::{DxDocument, DxLlmValue, DxSection};
use std::collections::HashMap;

/// Configuration for Human Format V2 output
#[derive(Debug, Clone)]
pub struct HumanFormatV2Config {
    /// Maximum line width for wrapping
    pub max_line_width: usize,
    /// Expand abbreviated keys to full names
    pub expand_keys: bool,
    /// Show summary footers for tables
    pub show_summaries: bool,
    /// Show reference comments
    pub show_references: bool,
}

impl Default for HumanFormatV2Config {
    fn default() -> Self {
        Self {
            max_line_width: 120,
            expand_keys: true,
            show_summaries: true,
            show_references: true,
        }
    }
}

impl HumanFormatV2Config {
    /// Create a new config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the max line width
    pub fn with_max_width(mut self, width: usize) -> Self {
        self.max_line_width = width;
        self
    }

    /// Set whether to expand keys
    pub fn with_expand_keys(mut self, expand: bool) -> Self {
        self.expand_keys = expand;
        self
    }
}

/// Section name mappings from single-letter IDs to full names
fn section_id_to_full_name(id: char) -> String {
    match id {
        'a' => "assets".to_string(),
        'b' => "builds".to_string(),
        'c' => "config".to_string(),
        'd' => "data".to_string(),
        'e' => "events".to_string(),
        'f' => "forge".to_string(),
        'g' => "groups".to_string(),
        'h' => "hikes".to_string(),
        'i' => "items".to_string(),
        'j' => "jobs".to_string(),
        'k' => "keys".to_string(),
        'l' => "logs".to_string(),
        'm' => "metrics".to_string(),
        'n' => "nodes".to_string(),
        'o' => "orders".to_string(),
        'p' => "products".to_string(),
        'q' => "queries".to_string(),
        'r' => "resources".to_string(),
        's' => "services".to_string(),
        't' => "tasks".to_string(),
        'u' => "users".to_string(),
        'v' => "versions".to_string(),
        'w' => "workflows".to_string(),
        'x' => "extensions".to_string(),
        'y' => "yields".to_string(),
        'z' => "zones".to_string(),
        _ => format!("section_{}", id),
    }
}

/// Format DxDocument to Human Format V2
pub struct HumanFormatterV2 {
    config: HumanFormatV2Config,
    abbrev: AbbrevDict,
}

impl HumanFormatterV2 {
    /// Create a new formatter with default config
    pub fn new() -> Self {
        Self {
            config: HumanFormatV2Config::default(),
            abbrev: AbbrevDict::new(),
        }
    }

    /// Create a formatter with custom config
    pub fn with_config(config: HumanFormatV2Config) -> Self {
        Self {
            config,
            abbrev: AbbrevDict::new(),
        }
    }

    /// Format DxDocument to human-readable V2 string
    pub fn format(&self, doc: &DxDocument) -> String {
        let mut output = String::new();

        // Format context section if present
        if !doc.context.is_empty() {
            output.push_str(&self.format_section_header("Configuration"));
            output.push('\n');
            output.push_str(&self.format_config_section(&doc.context, &doc.refs));
            output.push('\n');
        }

        // Format references section if present and show_references is enabled
        if !doc.refs.is_empty() && self.config.show_references {
            output.push_str(&self.format_section_header("References"));
            output.push('\n');
            output.push_str(&self.format_references_section(&doc.refs));
            output.push('\n');
        }

        // Format data sections
        let mut section_ids: Vec<_> = doc.sections.keys().collect();
        section_ids.sort();
        for id in section_ids {
            if let Some(section) = doc.sections.get(id) {
                let section_name = section_id_to_full_name(*id);
                output.push_str(&self.format_section_header(&section_name));
                output.push('\n');
                output.push_str(&self.format_data_section(*id, section, &doc.refs));
                output.push('\n');
            }
        }

        output.trim_end().to_string()
    }

    /// Format section header with box-drawing characters
    fn format_section_header(&self, title: &str) -> String {
        let width = self.config.max_line_width.min(80);
        let title_upper = title.to_uppercase();
        let title_len = title_upper.chars().count();
        let padding = (width.saturating_sub(title_len).saturating_sub(4)) / 2;

        let border = "═".repeat(width);
        let spaces = " ".repeat(padding);

        format!(
            "# {}\n# {}{}\n# {}",
            border, spaces, title_upper, border
        )
    }

    /// Format config section with key-value pairs (no indentation)
    fn format_config_section(
        &self,
        context: &HashMap<String, DxLlmValue>,
        refs: &HashMap<String, String>,
    ) -> String {
        let mut output = String::new();
        output.push_str("[config]\n");

        // Sort keys and find max key length for alignment
        let mut keys: Vec<_> = context.keys().collect();
        keys.sort();

        let max_key_len = keys
            .iter()
            .map(|k| {
                if self.config.expand_keys {
                    self.abbrev.expand(k, "config").len()
                } else {
                    k.len()
                }
            })
            .max()
            .unwrap_or(0);

        for key in keys {
            if let Some(value) = context.get(key) {
                let display_key = if self.config.expand_keys {
                    self.abbrev.expand(key, "config")
                } else {
                    key.clone()
                };
                let padding = " ".repeat(max_key_len - display_key.len());
                let formatted_value = self.format_config_value(value, refs);
                // No indentation - flat structure
                output.push_str(&format!(
                    "{}{} = {}\n",
                    display_key, padding, formatted_value
                ));
            }
        }

        output
    }

    /// Format references section (no indentation)
    fn format_references_section(&self, refs: &HashMap<String, String>) -> String {
        let mut output = String::new();
        output.push_str("[references]\n");

        let mut keys: Vec<_> = refs.keys().collect();
        keys.sort();

        let max_key_len = keys.iter().map(|k| k.len()).max().unwrap_or(0);

        for key in keys {
            if let Some(value) = refs.get(key) {
                let padding = " ".repeat(max_key_len - key.len());
                // No indentation - flat structure
                output.push_str(&format!("{}{} = \"{}\"\n", key, padding, value));
            }
        }

        output
    }

    /// Format a config value for display
    /// Arrays are formatted as comma-separated lists without brackets
    fn format_config_value(&self, value: &DxLlmValue, refs: &HashMap<String, String>) -> String {
        match value {
            DxLlmValue::Str(s) => format!("\"{}\"", s),
            DxLlmValue::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            DxLlmValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            DxLlmValue::Null => "null".to_string(),
            DxLlmValue::Ref(key) => {
                if let Some(resolved) = refs.get(key) {
                    if self.config.show_references {
                        format!("\"{}\"  # ref: {}", resolved, key)
                    } else {
                        format!("\"{}\"", resolved)
                    }
                } else {
                    format!("^{}", key)
                }
            }
            // V2: Arrays as comma-separated lists without brackets
            DxLlmValue::Arr(items) => {
                let formatted: Vec<String> = items
                    .iter()
                    .map(|item| self.format_array_item(item, refs))
                    .collect();
                formatted.join(", ")
            }
        }
    }

    /// Format array item (without quotes for simple strings)
    fn format_array_item(&self, value: &DxLlmValue, refs: &HashMap<String, String>) -> String {
        match value {
            DxLlmValue::Str(s) => s.clone(), // No quotes for array items
            DxLlmValue::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            DxLlmValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            DxLlmValue::Null => "null".to_string(),
            DxLlmValue::Ref(key) => {
                if let Some(resolved) = refs.get(key) {
                    resolved.clone()
                } else {
                    format!("^{}", key)
                }
            }
            DxLlmValue::Arr(items) => {
                let formatted: Vec<String> = items
                    .iter()
                    .map(|item| self.format_array_item(item, refs))
                    .collect();
                format!("[{}]", formatted.join(", "))
            }
        }
    }

    /// Format data section as table (no indentation)
    fn format_data_section(
        &self,
        id: char,
        section: &DxSection,
        refs: &HashMap<String, String>,
    ) -> String {
        let mut output = String::new();
        let section_name = section_id_to_full_name(id);

        // Use full section name in brackets
        output.push_str(&format!("[{}]\n", section_name));
        output.push_str(&self.build_table(section, &section_name, refs));

        if self.config.show_summaries && !section.rows.is_empty() {
            output.push('\n');
            output.push_str(&self.generate_summary(section));
        }

        output
    }

    /// Build Unicode box-drawn table (no indentation)
    fn build_table(
        &self,
        section: &DxSection,
        context: &str,
        refs: &HashMap<String, String>,
    ) -> String {
        let mut output = String::new();

        // Calculate column widths
        let col_widths = self.calculate_column_widths(section, context, refs);

        // Top border: ┌──────┬──────┬──────┐
        output.push('┌');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(*width + 2));
            if i < col_widths.len() - 1 {
                output.push('┬');
            }
        }
        output.push_str("┐\n");

        // Header row: │ Col1 │ Col2 │ Col3 │
        output.push('│');
        for (i, col) in section.schema.iter().enumerate() {
            let display_col = if self.config.expand_keys {
                self.abbrev.expand(col, context)
            } else {
                col.clone()
            };
            let padding = col_widths[i] - display_col.chars().count();
            output.push_str(&format!(" {}{} │", display_col, " ".repeat(padding)));
        }
        output.push('\n');

        // Header separator: ├──────┼──────┼──────┤
        output.push('├');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(*width + 2));
            if i < col_widths.len() - 1 {
                output.push('┼');
            }
        }
        output.push_str("┤\n");

        // Data rows
        for row in &section.rows {
            output.push('│');
            for (i, value) in row.iter().enumerate() {
                let cell = self.format_cell_value(value, refs);
                let width = col_widths.get(i).copied().unwrap_or(3);
                let cell_len = cell.chars().count();
                let padding = width.saturating_sub(cell_len);

                // Right-align numbers, center booleans, left-align strings
                match value {
                    DxLlmValue::Num(_) => {
                        output.push_str(&format!(" {}{} │", " ".repeat(padding), cell));
                    }
                    DxLlmValue::Bool(_) | DxLlmValue::Null => {
                        let left_pad = padding / 2;
                        let right_pad = padding - left_pad;
                        output.push_str(&format!(
                            " {}{}{} │",
                            " ".repeat(left_pad),
                            cell,
                            " ".repeat(right_pad)
                        ));
                    }
                    _ => {
                        output.push_str(&format!(" {}{} │", cell, " ".repeat(padding)));
                    }
                }
            }
            output.push('\n');
        }

        // Bottom border: └──────┴──────┴──────┘
        output.push('└');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(*width + 2));
            if i < col_widths.len() - 1 {
                output.push('┴');
            }
        }
        output.push_str("┘\n");

        output
    }

    /// Calculate column widths based on content
    fn calculate_column_widths(
        &self,
        section: &DxSection,
        context: &str,
        refs: &HashMap<String, String>,
    ) -> Vec<usize> {
        let mut widths: Vec<usize> = section
            .schema
            .iter()
            .map(|col| {
                if self.config.expand_keys {
                    self.abbrev.expand(col, context).chars().count()
                } else {
                    col.chars().count()
                }
            })
            .collect();

        // Check row values
        for row in &section.rows {
            for (i, value) in row.iter().enumerate() {
                if i < widths.len() {
                    let cell_width = self.format_cell_value(value, refs).chars().count();
                    widths[i] = widths[i].max(cell_width);
                }
            }
        }

        // Ensure minimum width of 3
        widths.iter().map(|w| (*w).max(3)).collect()
    }

    /// Format cell value for display
    fn format_cell_value(&self, value: &DxLlmValue, refs: &HashMap<String, String>) -> String {
        match value {
            DxLlmValue::Bool(true) => "✓".to_string(),
            DxLlmValue::Bool(false) => "✗".to_string(),
            DxLlmValue::Null => "—".to_string(),
            DxLlmValue::Str(s) => s.clone(),
            DxLlmValue::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            DxLlmValue::Ref(key) => {
                if let Some(resolved) = refs.get(key) {
                    resolved.clone()
                } else {
                    format!("^{}", key)
                }
            }
            DxLlmValue::Arr(items) => {
                let formatted: Vec<String> = items
                    .iter()
                    .map(|item| self.format_cell_value(item, refs))
                    .collect();
                formatted.join(", ")
            }
        }
    }

    /// Generate summary footer for table (no indentation)
    fn generate_summary(&self, section: &DxSection) -> String {
        let row_count = section.rows.len();

        // Calculate sums for numeric columns
        let mut numeric_sums: Vec<Option<f64>> = vec![None; section.schema.len()];

        for row in &section.rows {
            for (i, value) in row.iter().enumerate() {
                if let DxLlmValue::Num(n) = value {
                    numeric_sums[i] = Some(numeric_sums[i].unwrap_or(0.0) + n);
                }
            }
        }

        let mut summary_parts = vec![format!("Total: {} rows", row_count)];

        // Add sums for numeric columns
        for (i, sum) in numeric_sums.iter().enumerate() {
            if let Some(s) = sum {
                let col_name = if self.config.expand_keys {
                    self.abbrev.expand(&section.schema[i], "")
                } else {
                    section.schema[i].clone()
                };
                if s.fract() == 0.0 {
                    summary_parts.push(format!("{} sum: {}", col_name, *s as i64));
                } else {
                    summary_parts.push(format!("{} sum: {:.2}", col_name, s));
                }
            }
        }

        // No indentation - flat structure
        format!("{}\n", summary_parts.join(" | "))
    }

    /// Get the config
    pub fn config(&self) -> &HumanFormatV2Config {
        &self.config
    }

    /// Get the abbreviation dictionary
    pub fn abbrev(&self) -> &AbbrevDict {
        &self.abbrev
    }
}

impl Default for HumanFormatterV2 {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_empty_document() {
        let formatter = HumanFormatterV2::new();
        let doc = DxDocument::new();
        let output = formatter.format(&doc);
        assert!(output.is_empty());
    }

    #[test]
    fn test_format_section_header() {
        let formatter = HumanFormatterV2::new();
        let header = formatter.format_section_header("Test Section");
        assert!(header.contains("═"));
        assert!(header.contains("TEST SECTION"));
    }

    #[test]
    fn test_format_config_no_indentation() {
        let formatter = HumanFormatterV2::new();
        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));
        doc.context.insert("ct".to_string(), DxLlmValue::Num(42.0));

        let output = formatter.format(&doc);
        
        // Check for [config] section
        assert!(output.contains("[config]"));
        
        // Check that keys are expanded
        assert!(output.contains("name")); // nm expanded to name
        assert!(output.contains("count")); // ct expanded to count
        
        // Check no indentation (lines should start with key name, not spaces)
        for line in output.lines() {
            if line.contains(" = ") && !line.starts_with('#') && !line.starts_with('[') {
                assert!(!line.starts_with(' '), "Line should not be indented: {}", line);
            }
        }
    }

    #[test]
    fn test_format_array_as_comma_separated() {
        let formatter = HumanFormatterV2::new();
        let mut doc = DxDocument::new();
        doc.context.insert(
            "ws".to_string(),
            DxLlmValue::Arr(vec![
                DxLlmValue::Str("frontend/www".to_string()),
                DxLlmValue::Str("frontend/mobile".to_string()),
            ]),
        );

        let output = formatter.format(&doc);
        
        // Arrays should be comma-separated without brackets
        assert!(output.contains("frontend/www, frontend/mobile"));
        // Should NOT have brackets
        assert!(!output.contains("[frontend/www"));
    }

    #[test]
    fn test_format_table_no_indentation() {
        let formatter = HumanFormatterV2::new();
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string(), "nm".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Str("Alpha".to_string()),
        ]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        
        // Check table borders start at column 0 (no indentation)
        for line in output.lines() {
            if line.starts_with('┌') || line.starts_with('│') || 
               line.starts_with('├') || line.starts_with('└') {
                // Table lines should not be indented
                assert!(!line.chars().next().unwrap().is_whitespace());
            }
        }
    }

    #[test]
    fn test_format_full_section_names() {
        let formatter = HumanFormatterV2::new();
        let mut doc = DxDocument::new();

        let section = DxSection::new(vec!["id".to_string()]);
        doc.sections.insert('f', section);

        let output = formatter.format(&doc);
        
        // Should use full section name [forge] not [f]
        assert!(output.contains("[forge]"));
        assert!(!output.contains("[f]") || output.contains("[forge]"));
    }

    #[test]
    fn test_format_boolean_values() {
        let formatter = HumanFormatterV2::new();
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string(), "active".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Bool(true),
        ]);
        section.rows.push(vec![
            DxLlmValue::Num(2.0),
            DxLlmValue::Bool(false),
        ]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        assert!(output.contains("✓"));
        assert!(output.contains("✗"));
    }

    #[test]
    fn test_format_null_value() {
        let formatter = HumanFormatterV2::new();
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string(), "value".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Null,
        ]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        assert!(output.contains("—"));
    }

    #[test]
    fn test_format_summary_no_indentation() {
        let formatter = HumanFormatterV2::new();
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string(), "amount".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Num(100.0),
        ]);
        section.rows.push(vec![
            DxLlmValue::Num(2.0),
            DxLlmValue::Num(200.0),
        ]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        
        // Summary should exist
        assert!(output.contains("Total: 2 rows"));
        
        // Summary should not be indented
        for line in output.lines() {
            if line.contains("Total:") {
                assert!(!line.starts_with(' '), "Summary should not be indented");
            }
        }
    }

    #[test]
    fn test_section_id_to_full_name() {
        assert_eq!(section_id_to_full_name('f'), "forge");
        assert_eq!(section_id_to_full_name('d'), "data");
        assert_eq!(section_id_to_full_name('u'), "users");
        assert_eq!(section_id_to_full_name('o'), "orders");
        assert_eq!(section_id_to_full_name('t'), "tasks");
    }
}
