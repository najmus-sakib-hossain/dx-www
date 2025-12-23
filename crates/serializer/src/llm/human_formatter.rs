//! Human format formatter
//!
//! Formats DxDocument to beautiful human-readable format with:
//! - Unicode box-drawing tables
//! - Expanded key names
//! - Section headers with box-drawing characters
//! - Auto-generated summaries
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

/// Table style for human format output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableStyle {
    /// Unicode box-drawing: ┌─┬─┐ │ ├─┼─┤ └─┴─┘
    Unicode,
    /// ASCII: +-+-+ | +-+-+ +-+-+
    Ascii,
    /// Markdown: | --- | --- |
    Markdown,
    /// No borders
    Minimal,
}

impl Default for TableStyle {
    fn default() -> Self {
        TableStyle::Unicode
    }
}

/// Configuration for human format output
#[derive(Debug, Clone)]
pub struct HumanFormatConfig {
    /// Table rendering style
    pub table_style: TableStyle,
    /// Indentation size (spaces)
    pub indent_size: usize,
    /// Maximum line width
    pub max_width: usize,
    /// Show reference comments
    pub show_references: bool,
    /// Show summary footers
    pub show_summaries: bool,
    /// Expand abbreviated keys
    pub expand_abbreviations: bool,
}


impl Default for HumanFormatConfig {
    fn default() -> Self {
        Self {
            table_style: TableStyle::Unicode,
            indent_size: 4,
            max_width: 80,
            show_references: true,
            show_summaries: true,
            expand_abbreviations: true,
        }
    }
}

impl HumanFormatConfig {
    /// Create a new config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the table style
    pub fn with_table_style(mut self, style: TableStyle) -> Self {
        self.table_style = style;
        self
    }

    /// Set the indent size
    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    /// Set the max width
    pub fn with_max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }
}

/// Format DxDocument to beautiful human-readable format
pub struct HumanFormatter {
    config: HumanFormatConfig,
    abbrev: AbbrevDict,
}

impl HumanFormatter {
    /// Create a new formatter with default config
    pub fn new() -> Self {
        Self {
            config: HumanFormatConfig::default(),
            abbrev: AbbrevDict::new(),
        }
    }

    /// Create a formatter with custom config
    pub fn with_config(config: HumanFormatConfig) -> Self {
        Self {
            config,
            abbrev: AbbrevDict::new(),
        }
    }

    /// Format DxDocument to human-readable string
    pub fn format(&self, doc: &DxDocument) -> String {
        let mut output = String::new();

        // Format context section if present
        if !doc.context.is_empty() {
            output.push_str(&self.format_section_header("Configuration"));
            output.push('\n');
            output.push_str(&self.format_config(&doc.context, &doc.refs));
            output.push('\n');
        }

        // Format references section if present and show_references is enabled
        if !doc.refs.is_empty() && self.config.show_references {
            output.push_str(&self.format_section_header("References"));
            output.push('\n');
            output.push_str(&self.format_references(&doc.refs));
            output.push('\n');
        }

        // Format data sections
        let mut section_ids: Vec<_> = doc.sections.keys().collect();
        section_ids.sort();
        for id in section_ids {
            if let Some(section) = doc.sections.get(id) {
                let section_name = self.section_id_to_name(*id);
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
        let width = self.config.max_width;
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


    /// Format config section with key-value pairs
    fn format_config(
        &self,
        context: &HashMap<String, DxLlmValue>,
        refs: &HashMap<String, String>,
    ) -> String {
        let mut output = String::new();
        let indent = " ".repeat(self.config.indent_size);

        output.push_str("[config]\n");

        // Sort keys and find max key length for alignment
        let mut keys: Vec<_> = context.keys().collect();
        keys.sort();

        let max_key_len = keys
            .iter()
            .map(|k| {
                if self.config.expand_abbreviations {
                    self.abbrev.expand(k, "config").len()
                } else {
                    k.len()
                }
            })
            .max()
            .unwrap_or(0);

        for key in keys {
            if let Some(value) = context.get(key) {
                let display_key = if self.config.expand_abbreviations {
                    self.abbrev.expand(key, "config")
                } else {
                    key.clone()
                };
                let padding = " ".repeat(max_key_len - display_key.len());
                let formatted_value = self.format_config_value(value, refs);
                output.push_str(&format!(
                    "{}{}{} = {}\n",
                    indent, display_key, padding, formatted_value
                ));
            }
        }

        output
    }

    /// Format references section
    fn format_references(&self, refs: &HashMap<String, String>) -> String {
        let mut output = String::new();
        let indent = " ".repeat(self.config.indent_size);

        output.push_str("[references]\n");

        let mut keys: Vec<_> = refs.keys().collect();
        keys.sort();

        let max_key_len = keys.iter().map(|k| k.len()).max().unwrap_or(0);

        for key in keys {
            if let Some(value) = refs.get(key) {
                let padding = " ".repeat(max_key_len - key.len());
                output.push_str(&format!("{}{}{} = \"{}\"\n", indent, key, padding, value));
            }
        }

        output
    }

    /// Format a config value for display
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
            DxLlmValue::Arr(items) => {
                let formatted: Vec<String> = items
                    .iter()
                    .map(|item| self.format_config_value(item, refs))
                    .collect();
                format!("[{}]", formatted.join(", "))
            }
        }
    }

    /// Convert section ID to human-readable name
    fn section_id_to_name(&self, id: char) -> String {
        match id {
            'd' => "Data".to_string(),
            'h' => "Hikes".to_string(),
            'o' => "Orders".to_string(),
            'p' => "Products".to_string(),
            'u' => "Users".to_string(),
            'i' => "Items".to_string(),
            't' => "Tasks".to_string(),
            'e' => "Events".to_string(),
            _ => format!("Section {}", id.to_uppercase()),
        }
    }


    /// Format data section as table
    fn format_data_section(
        &self,
        id: char,
        section: &DxSection,
        refs: &HashMap<String, String>,
    ) -> String {
        let mut output = String::new();
        let context = self.section_id_to_name(id).to_lowercase();

        output.push_str(&format!("[{}]\n", context));
        output.push_str(&self.build_table(section, &context, refs));

        if self.config.show_summaries && !section.rows.is_empty() {
            output.push('\n');
            output.push_str(&self.generate_summary(section, &context));
        }

        output
    }

    /// Build Unicode box-drawn table
    fn build_table(
        &self,
        section: &DxSection,
        context: &str,
        refs: &HashMap<String, String>,
    ) -> String {
        let indent = " ".repeat(self.config.indent_size);

        // Calculate column widths
        let col_widths = self.calculate_column_widths(section, context, refs);

        match self.config.table_style {
            TableStyle::Unicode => self.build_unicode_table(section, context, refs, &col_widths, &indent),
            TableStyle::Ascii => self.build_ascii_table(section, context, refs, &col_widths, &indent),
            TableStyle::Markdown => self.build_markdown_table(section, context, refs, &col_widths, &indent),
            TableStyle::Minimal => self.build_minimal_table(section, context, refs, &col_widths, &indent),
        }
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
                if self.config.expand_abbreviations {
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

    /// Build Unicode box-drawn table
    fn build_unicode_table(
        &self,
        section: &DxSection,
        context: &str,
        refs: &HashMap<String, String>,
        col_widths: &[usize],
        indent: &str,
    ) -> String {
        let mut output = String::new();

        // Top border: ┌──────┬──────┬──────┐
        output.push_str(indent);
        output.push('┌');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(*width + 2));
            if i < col_widths.len() - 1 {
                output.push('┬');
            }
        }
        output.push_str("┐\n");

        // Header row: │ Col1 │ Col2 │ Col3 │
        output.push_str(indent);
        output.push('│');
        for (i, col) in section.schema.iter().enumerate() {
            let display_col = if self.config.expand_abbreviations {
                self.abbrev.expand(col, context)
            } else {
                col.clone()
            };
            let padding = col_widths[i] - display_col.chars().count();
            output.push_str(&format!(" {}{} │", display_col, " ".repeat(padding)));
        }
        output.push('\n');

        // Header separator: ├──────┼──────┼──────┤
        output.push_str(indent);
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
            output.push_str(indent);
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
                        output.push_str(&format!(" {}{}{} │", " ".repeat(left_pad), cell, " ".repeat(right_pad)));
                    }
                    _ => {
                        output.push_str(&format!(" {}{} │", cell, " ".repeat(padding)));
                    }
                }
            }
            output.push('\n');
        }

        // Bottom border: └──────┴──────┴──────┘
        output.push_str(indent);
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


    /// Build ASCII table
    fn build_ascii_table(
        &self,
        section: &DxSection,
        context: &str,
        refs: &HashMap<String, String>,
        col_widths: &[usize],
        indent: &str,
    ) -> String {
        let mut output = String::new();

        // Top border: +------+------+------+
        output.push_str(indent);
        output.push('+');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"-".repeat(*width + 2));
            if i < col_widths.len() - 1 {
                output.push('+');
            }
        }
        output.push_str("+\n");

        // Header row
        output.push_str(indent);
        output.push('|');
        for (i, col) in section.schema.iter().enumerate() {
            let display_col = if self.config.expand_abbreviations {
                self.abbrev.expand(col, context)
            } else {
                col.clone()
            };
            let padding = col_widths[i] - display_col.chars().count();
            output.push_str(&format!(" {}{} |", display_col, " ".repeat(padding)));
        }
        output.push('\n');

        // Header separator
        output.push_str(indent);
        output.push('+');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"-".repeat(*width + 2));
            if i < col_widths.len() - 1 {
                output.push('+');
            }
        }
        output.push_str("+\n");

        // Data rows
        for row in &section.rows {
            output.push_str(indent);
            output.push('|');
            for (i, value) in row.iter().enumerate() {
                let cell = self.format_cell_value(value, refs);
                let width = col_widths.get(i).copied().unwrap_or(3);
                let padding = width.saturating_sub(cell.chars().count());
                output.push_str(&format!(" {}{} |", cell, " ".repeat(padding)));
            }
            output.push('\n');
        }

        // Bottom border
        output.push_str(indent);
        output.push('+');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"-".repeat(*width + 2));
            if i < col_widths.len() - 1 {
                output.push('+');
            }
        }
        output.push_str("+\n");

        output
    }

    /// Build Markdown table
    fn build_markdown_table(
        &self,
        section: &DxSection,
        context: &str,
        refs: &HashMap<String, String>,
        col_widths: &[usize],
        indent: &str,
    ) -> String {
        let mut output = String::new();

        // Header row
        output.push_str(indent);
        output.push('|');
        for (i, col) in section.schema.iter().enumerate() {
            let display_col = if self.config.expand_abbreviations {
                self.abbrev.expand(col, context)
            } else {
                col.clone()
            };
            let padding = col_widths[i] - display_col.chars().count();
            output.push_str(&format!(" {}{} |", display_col, " ".repeat(padding)));
        }
        output.push('\n');

        // Separator
        output.push_str(indent);
        output.push('|');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&format!(" {} ", "-".repeat(*width)));
            if i < col_widths.len() - 1 {
                output.push('|');
            }
        }
        output.push_str("|\n");

        // Data rows
        for row in &section.rows {
            output.push_str(indent);
            output.push('|');
            for (i, value) in row.iter().enumerate() {
                let cell = self.format_cell_value(value, refs);
                let width = col_widths.get(i).copied().unwrap_or(3);
                let padding = width.saturating_sub(cell.chars().count());
                output.push_str(&format!(" {}{} |", cell, " ".repeat(padding)));
            }
            output.push('\n');
        }

        output
    }

    /// Build minimal table (no borders)
    fn build_minimal_table(
        &self,
        section: &DxSection,
        context: &str,
        refs: &HashMap<String, String>,
        col_widths: &[usize],
        indent: &str,
    ) -> String {
        let mut output = String::new();

        // Header row
        output.push_str(indent);
        for (i, col) in section.schema.iter().enumerate() {
            let display_col = if self.config.expand_abbreviations {
                self.abbrev.expand(col, context)
            } else {
                col.clone()
            };
            let padding = col_widths[i] - display_col.chars().count();
            output.push_str(&format!("{}{}", display_col, " ".repeat(padding + 2)));
        }
        output.push('\n');

        // Data rows
        for row in &section.rows {
            output.push_str(indent);
            for (i, value) in row.iter().enumerate() {
                let cell = self.format_cell_value(value, refs);
                let width = col_widths.get(i).copied().unwrap_or(3);
                let padding = width.saturating_sub(cell.chars().count());
                output.push_str(&format!("{}{}", cell, " ".repeat(padding + 2)));
            }
            output.push('\n');
        }

        output
    }


    /// Format cell value for display
    ///
    /// Uses special symbols:
    /// - ✓ for boolean true
    /// - ✗ for boolean false
    /// - — for null
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
                format!("[{}]", formatted.join(", "))
            }
        }
    }

    /// Generate summary footer for table
    fn generate_summary(&self, section: &DxSection, _context: &str) -> String {
        let indent = " ".repeat(self.config.indent_size);
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
                let col_name = if self.config.expand_abbreviations {
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

        format!("{}{}\n", indent, summary_parts.join(" | "))
    }

    /// Get the config
    pub fn config(&self) -> &HumanFormatConfig {
        &self.config
    }

    /// Get the abbreviation dictionary
    pub fn abbrev(&self) -> &AbbrevDict {
        &self.abbrev
    }
}

impl Default for HumanFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_empty_document() {
        let formatter = HumanFormatter::new();
        let doc = DxDocument::new();
        let output = formatter.format(&doc);
        assert!(output.is_empty());
    }

    #[test]
    fn test_format_section_header() {
        let formatter = HumanFormatter::new();
        let header = formatter.format_section_header("Test Section");
        assert!(header.contains("═"));
        assert!(header.contains("TEST SECTION"));
    }

    #[test]
    fn test_format_config() {
        let formatter = HumanFormatter::new();
        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));
        doc.context.insert("ct".to_string(), DxLlmValue::Num(42.0));

        let output = formatter.format(&doc);
        assert!(output.contains("[config]"));
        assert!(output.contains("name")); // nm expanded to name
        assert!(output.contains("count")); // ct expanded to count
        assert!(output.contains("\"Test\""));
        assert!(output.contains("42"));
    }

    #[test]
    fn test_format_boolean_values() {
        let formatter = HumanFormatter::new();
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
        let formatter = HumanFormatter::new();
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
    fn test_format_unicode_table() {
        let formatter = HumanFormatter::new();
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string(), "nm".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Str("Alpha".to_string()),
        ]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        assert!(output.contains("┌"));
        assert!(output.contains("┐"));
        assert!(output.contains("│"));
        assert!(output.contains("├"));
        assert!(output.contains("┤"));
        assert!(output.contains("└"));
        assert!(output.contains("┘"));
    }

    #[test]
    fn test_format_summary() {
        let formatter = HumanFormatter::new();
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
        assert!(output.contains("Total: 2 rows"));
        assert!(output.contains("sum: 300")); // amount sum
    }

    #[test]
    fn test_format_references() {
        let formatter = HumanFormatter::new();
        let mut doc = DxDocument::new();
        doc.refs.insert("A".to_string(), "Shared Value".to_string());

        let mut section = DxSection::new(vec!["id".to_string(), "value".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Ref("A".to_string()),
        ]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        assert!(output.contains("[references]"));
        assert!(output.contains("Shared Value"));
    }

    #[test]
    fn test_ascii_table_style() {
        let config = HumanFormatConfig::new().with_table_style(TableStyle::Ascii);
        let formatter = HumanFormatter::with_config(config);
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string()]);
        section.rows.push(vec![DxLlmValue::Num(1.0)]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        assert!(output.contains("+"));
        assert!(output.contains("|"));
        assert!(output.contains("-"));
    }

    #[test]
    fn test_markdown_table_style() {
        let config = HumanFormatConfig::new().with_table_style(TableStyle::Markdown);
        let formatter = HumanFormatter::with_config(config);
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string()]);
        section.rows.push(vec![DxLlmValue::Num(1.0)]);
        doc.sections.insert('d', section);

        let output = formatter.format(&doc);
        assert!(output.contains("|"));
        assert!(output.contains("---"));
    }
}
