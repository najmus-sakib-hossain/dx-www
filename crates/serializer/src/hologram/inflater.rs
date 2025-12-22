//! Inflater: LLM-Dense → Human-Pretty Transformation
//!
//! Transforms token-efficient LLM format into beautiful, readable format.
//!
//! ## LLM-Dense Format Examples:
//!
//! ### Hologram Format (inline objects):
//! - Object: `server#host:localhost#port:5432`
//! - Array: `items@3>apple|banana|cherry`
//! - Table: `data@2=id^name^active` + `>1|Alice|1` + `>2|Bob|0`
//! - Comment: `!Database config!db#host:localhost`
//!
//! ### DX Format (one key:value per line with abbreviated keys):
//! - Key:Value: `c.n:dx` → `context.name: dx`
//! - Prefix: `^v:0.0.1` → `^version: 0.0.1`
//! - Array: `tags>rust|wasm|performance` → `tags > rust | wasm | performance`
//!
//! ## Human-Pretty Output:
//! ```text
//! // Database config
//! ▼ server
//!     host: localhost
//!     port: 5432
//!
//! ▼ items (3 items)
//!     • apple
//!     • banana
//!     • cherry
//!
//! ▼ data (2 columns × 2 rows)
//!     ┌────┬───────┬────────┐
//!     │ id │ name  │ active │
//!     ├────┼───────┼────────┤
//!     │ 1  │ Alice │ ✓      │
//!     │ 2  │ Bob   │ ✗      │
//!     └────┴───────┴────────┘
//! ```

use super::types::{CommentAnchor, DenseElement, HologramConfig};
use crate::mappings::Mappings;
use std::fmt::Write;

/// Inflater: Transforms LLM-dense format to human-pretty format
pub struct Inflater {
    config: HologramConfig,
}

impl Inflater {
    /// Create a new Inflater with the given configuration
    pub fn new(config: HologramConfig) -> Self {
        Self { config }
    }

    /// Create an Inflater with default configuration
    pub fn default_config() -> Self {
        Self::new(HologramConfig::default())
    }

    /// Inflate LLM-dense format to human-pretty format
    pub fn inflate(&self, dense: &str) -> String {
        let mut output = String::with_capacity(dense.len() * 3);
        let mut table_context: Option<TableContext> = None;

        for line in dense.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                // Close any pending table
                if let Some(ctx) = table_context.take() {
                    self.close_table(&mut output, &ctx);
                }
                output.push('\n');
                continue;
            }

            // Parse the line
            let element = self.parse_dense_line(trimmed);

            match element {
                DenseElement::TableRow { values } => {
                    // Add row to current table context
                    if let Some(ref mut ctx) = table_context {
                        ctx.rows.push(values);
                    }
                }
                _ => {
                    // Close any pending table before processing new element
                    if let Some(ctx) = table_context.take() {
                        self.close_table(&mut output, &ctx);
                    }

                    match element {
                        DenseElement::Object {
                            key,
                            fields,
                            comment,
                        } => {
                            self.write_object(&mut output, &key, &fields, comment.as_ref());
                        }
                        DenseElement::Array {
                            key,
                            items,
                            comment,
                        } => {
                            self.write_array(&mut output, &key, &items, comment.as_ref());
                        }
                        DenseElement::TableHeader {
                            key,
                            columns,
                            row_count,
                            comment,
                        } => {
                            table_context = Some(TableContext {
                                key,
                                columns,
                                expected_rows: row_count,
                                rows: Vec::new(),
                                comment,
                            });
                        }
                        DenseElement::KeyValue {
                            key,
                            value,
                            comment,
                        } => {
                            self.write_key_value(&mut output, &key, &value, comment.as_ref());
                        }
                        DenseElement::Comment(anchor) => {
                            writeln!(output, "{}", anchor.to_human()).unwrap();
                        }
                        DenseElement::Empty | DenseElement::TableRow { .. } => {}
                    }
                }
            }
        }

        // Close any remaining table
        if let Some(ctx) = table_context {
            self.close_table(&mut output, &ctx);
        }

        output
    }

    /// Parse a single line of LLM-dense format
    fn parse_dense_line(&self, line: &str) -> DenseElement {
        let mut trimmed = line.trim();

        if trimmed.is_empty() {
            return DenseElement::Empty;
        }

        // Check for anchored comment: !text!content
        let comment = if trimmed.starts_with('!') {
            if let Some((anchor, rest)) = CommentAnchor::from_dense(trimmed) {
                trimmed = rest.trim();
                if trimmed.is_empty() {
                    return DenseElement::Comment(anchor);
                }
                Some(anchor)
            } else {
                None
            }
        } else {
            None
        };

        // Table row: >val|val|val
        if trimmed.starts_with('>') {
            let values = trimmed[1..]
                .split('|')
                .map(|s| s.trim().to_string())
                .collect();
            return DenseElement::TableRow { values };
        }

        // Table header: key@N=col^col^col
        if let Some(at_idx) = trimmed.find('@') {
            if let Some(eq_idx) = trimmed.find('=') {
                if eq_idx > at_idx {
                    let key = trimmed[..at_idx].to_string();
                    let count_str = &trimmed[at_idx + 1..eq_idx];
                    let row_count = count_str.parse().unwrap_or(0);
                    let columns = trimmed[eq_idx + 1..]
                        .split('^')
                        .map(|s| s.trim().to_string())
                        .collect();

                    return DenseElement::TableHeader {
                        key,
                        columns,
                        row_count,
                        comment,
                    };
                }
            }
        }

        // Array: key@N>item|item|item or key>item|item|item (DX format)
        // Note: Table rows starting with '>' are already handled above
        if let Some(arrow_idx) = trimmed.find('>') {
            // Skip if it's a table row (handled earlier)
            if arrow_idx == 0 {
                // Already handled above, but just in case
                let values = trimmed[1..]
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .collect();
                return DenseElement::TableRow { values };
            }
            
            let key_part = &trimmed[..arrow_idx];
            let items_part = &trimmed[arrow_idx + 1..];
            
            // Check if there's a count: key@N>
            let key = if let Some(at_idx) = key_part.find('@') {
                key_part[..at_idx].to_string()
            } else {
                key_part.to_string()
            };
            
            let items = items_part
                .split('|')
                .map(|s| s.trim().to_string())
                .collect();

            return DenseElement::Array {
                key,
                items,
                comment,
            };
        }

        // Object: key#field:val#field:val
        if trimmed.contains('#') {
            let parts: Vec<&str> = trimmed.split('#').collect();
            let key = parts[0].to_string();
            let mut fields = Vec::new();

            for part in &parts[1..] {
                if let Some(colon_idx) = part.find(':') {
                    let field_name = part[..colon_idx].to_string();
                    let field_value = part[colon_idx + 1..].to_string();
                    fields.push((field_name, field_value));
                }
            }

            if !fields.is_empty() {
                return DenseElement::Object {
                    key,
                    fields,
                    comment,
                };
            }
        }

        // Simple key:value
        if let Some(colon_idx) = trimmed.find(':') {
            let key = trimmed[..colon_idx].to_string();
            let value = trimmed[colon_idx + 1..].to_string();
            return DenseElement::KeyValue {
                key,
                value,
                comment,
            };
        }

        // Unknown format - treat as comment
        DenseElement::Comment(CommentAnchor::new(trimmed))
    }

    /// Write an object section to output
    fn write_object(
        &self,
        output: &mut String,
        key: &str,
        fields: &[(String, String)],
        comment: Option<&CommentAnchor>,
    ) {
        // Write comment if present
        if let Some(anchor) = comment {
            writeln!(output, "{}", anchor.to_human()).unwrap();
        }

        // Expand abbreviated key
        let expanded_key = self.expand_full_key(key);

        // Expand field names
        let expanded_fields: Vec<(String, String)> = fields
            .iter()
            .map(|(k, v)| (self.expand_full_key(k), v.clone()))
            .collect();

        // DX format: output each field as key.field : value
        if self.config.use_dx_format {
            let max_len = 20usize;
            let mut is_first = true;
            
            for (name, value) in &expanded_fields {
                let full_key = if is_first {
                    format!("{}.{}", expanded_key, name)
                } else {
                    format!("^{}", name)
                };
                is_first = false;
                
                let padding = max_len.saturating_sub(full_key.len());
                let pretty_value = self.inflate_value(value);
                writeln!(output, "{}{} : {}", full_key, " ".repeat(padding), pretty_value).unwrap();
            }
            return;
        }

        // Hologram format: collapsible section
        writeln!(output, "{} {}", self.config.section_marker, expanded_key).unwrap();

        let max_len = if self.config.align_values {
            expanded_fields.iter().map(|(k, _)| k.len()).max().unwrap_or(0)
        } else {
            0
        };

        // Write fields
        let indent = " ".repeat(self.config.indent_size);
        for (name, value) in &expanded_fields {
            let padding = if self.config.align_values {
                " ".repeat(max_len - name.len())
            } else {
                String::new()
            };
            let pretty_value = self.inflate_value(value);
            writeln!(output, "{}{}:{} {}", indent, name, padding, pretty_value).unwrap();
        }
    }

    /// Write an array section to output
    fn write_array(
        &self,
        output: &mut String,
        key: &str,
        items: &[String],
        comment: Option<&CommentAnchor>,
    ) {
        // Write comment if present
        if let Some(anchor) = comment {
            writeln!(output, "{}", anchor.to_human()).unwrap();
        }

        // Expand abbreviated key
        let expanded_key = self.expand_full_key(key);

        // DX format: key > item | item | item (flat, single line)
        if self.config.use_dx_format {
            let items_str: Vec<String> = items
                .iter()
                .map(|item| self.inflate_value(item))
                .collect();
            
            // Calculate padding for alignment (20 chars for key)
            let padding = 20usize.saturating_sub(expanded_key.len());
            writeln!(output, "{}{} > {}", expanded_key, " ".repeat(padding), items_str.join(" | ")).unwrap();
            return;
        }

        // Hologram format: collapsible section with bullets
        writeln!(
            output,
            "{} {} ({} items)",
            self.config.section_marker,
            expanded_key,
            items.len()
        )
        .unwrap();

        // Write bullet points
        let indent = " ".repeat(self.config.indent_size);
        for item in items {
            let pretty_value = self.inflate_value(item);
            writeln!(output, "{}{} {}", indent, self.config.bullet_char, pretty_value).unwrap();
        }
    }

    /// Write a key-value pair
    fn write_key_value(
        &self,
        output: &mut String,
        key: &str,
        value: &str,
        comment: Option<&CommentAnchor>,
    ) {
        if let Some(anchor) = comment {
            writeln!(output, "{}", anchor.to_human()).unwrap();
        }
        
        // Expand abbreviated keys using mappings
        let expanded_key = self.expand_full_key(key);
        let pretty_value = self.inflate_value(value);
        
        // Format with proper alignment (20 chars for key)
        let padding = 20usize.saturating_sub(expanded_key.len());
        writeln!(output, "{}{}: {}", expanded_key, " ".repeat(padding), pretty_value).unwrap();
    }

    /// Expand a full key (handles dotted and prefix keys)
    /// e.g., "c.n" → "context.name", "^v" → "^version"
    fn expand_full_key(&self, key: &str) -> String {
        let mappings = Mappings::get();
        
        // Handle prefix inheritance (^key)
        let (prefix, rest) = if key.starts_with('^') {
            ("^", &key[1..])
        } else {
            ("", key)
        };
        
        // Handle dotted keys: each part expanded independently
        let expanded = if rest.contains('.') {
            rest.split('.')
                .map(|part| mappings.expand_key(part))
                .collect::<Vec<_>>()
                .join(".")
        } else if rest.contains('_') {
            rest.split('_')
                .map(|part| mappings.expand_key(part))
                .collect::<Vec<_>>()
                .join("_")
        } else {
            mappings.expand_key(rest)
        };
        
        format!("{}{}", prefix, expanded)
    }

    /// Close a table and write it to output
    fn close_table(&self, output: &mut String, ctx: &TableContext) {
        // Write comment if present
        if let Some(ref anchor) = ctx.comment {
            writeln!(output, "{}", anchor.to_human()).unwrap();
        }

        // Expand abbreviated key and column names
        let expanded_key = self.expand_full_key(&ctx.key);
        let expanded_columns: Vec<String> = ctx.columns
            .iter()
            .map(|c| self.expand_full_key(c))
            .collect();

        // Section header
        writeln!(
            output,
            "{} {} ({} columns × {} rows)",
            self.config.section_marker,
            expanded_key,
            expanded_columns.len(),
            ctx.rows.len()
        )
        .unwrap();

        let indent = " ".repeat(self.config.indent_size);

        // Calculate column widths using expanded column names
        let mut col_widths: Vec<usize> = expanded_columns.iter().map(|c| c.len()).collect();
        for row in &ctx.rows {
            for (i, val) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let pretty = self.inflate_value(val);
                    col_widths[i] = col_widths[i].max(pretty.len());
                }
            }
        }

        if self.config.use_box_drawing {
            self.write_table_boxed(output, &expanded_columns, &ctx.rows, &col_widths, &indent);
        } else {
            self.write_table_simple(output, &expanded_columns, &ctx.rows, &col_widths, &indent);
        }
    }

    /// Write table with Unicode box-drawing characters
    fn write_table_boxed(
        &self,
        output: &mut String,
        columns: &[String],
        rows: &[Vec<String>],
        col_widths: &[usize],
        indent: &str,
    ) {
        // Top border: ┌─────┬─────┐
        write!(output, "{}┌", indent).unwrap();
        for (i, width) in col_widths.iter().enumerate() {
            if i > 0 {
                write!(output, "┬").unwrap();
            }
            write!(output, "{}", "─".repeat(width + 2)).unwrap();
        }
        writeln!(output, "┐").unwrap();

        // Header row: │ col │ col │
        write!(output, "{}│", indent).unwrap();
        for (i, col) in columns.iter().enumerate() {
            let width = col_widths.get(i).copied().unwrap_or(col.len());
            write!(output, " {:width$} │", col, width = width).unwrap();
        }
        writeln!(output).unwrap();

        // Header separator: ├─────┼─────┤
        write!(output, "{}├", indent).unwrap();
        for (i, width) in col_widths.iter().enumerate() {
            if i > 0 {
                write!(output, "┼").unwrap();
            }
            write!(output, "{}", "─".repeat(width + 2)).unwrap();
        }
        writeln!(output, "┤").unwrap();

        // Data rows: │ val │ val │
        for row in rows {
            write!(output, "{}│", indent).unwrap();
            for (i, val) in row.iter().enumerate() {
                let width = col_widths.get(i).copied().unwrap_or(val.len());
                let pretty = self.inflate_value(val);
                write!(output, " {:width$} │", pretty, width = width).unwrap();
            }
            writeln!(output).unwrap();
        }

        // Bottom border: └─────┴─────┘
        write!(output, "{}└", indent).unwrap();
        for (i, width) in col_widths.iter().enumerate() {
            if i > 0 {
                write!(output, "┴").unwrap();
            }
            write!(output, "{}", "─".repeat(width + 2)).unwrap();
        }
        writeln!(output, "┘").unwrap();
    }

    /// Write table with simple ASCII characters
    fn write_table_simple(
        &self,
        output: &mut String,
        columns: &[String],
        rows: &[Vec<String>],
        col_widths: &[usize],
        indent: &str,
    ) {
        // Header row
        write!(output, "{}", indent).unwrap();
        for (i, col) in columns.iter().enumerate() {
            if i > 0 {
                write!(output, " | ").unwrap();
            }
            let width = col_widths.get(i).copied().unwrap_or(col.len());
            write!(output, "{:width$}", col, width = width).unwrap();
        }
        writeln!(output).unwrap();

        // Separator
        write!(output, "{}", indent).unwrap();
        for (i, width) in col_widths.iter().enumerate() {
            if i > 0 {
                write!(output, "-+-").unwrap();
            }
            write!(output, "{}", "-".repeat(*width)).unwrap();
        }
        writeln!(output).unwrap();

        // Data rows
        for row in rows {
            write!(output, "{}", indent).unwrap();
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(output, " | ").unwrap();
                }
                let width = col_widths.get(i).copied().unwrap_or(val.len());
                let pretty = self.inflate_value(val);
                write!(output, "{:width$}", pretty, width = width).unwrap();
            }
            writeln!(output).unwrap();
        }
    }

    /// Inflate a single value (e.g., 1 → ✓, ~ → —)
    fn inflate_value(&self, value: &str) -> String {
        let v = value.trim();

        if self.config.use_unicode_symbols {
            match v {
                "1" | "+" | "true" => return "✓".to_string(),
                "0" | "-" | "false" => return "✗".to_string(),
                "~" | "null" | "none" => return self.config.null_display.clone(),
                _ => {}
            }
        } else {
            match v {
                "1" | "+" => return "true".to_string(),
                "0" | "-" => return "false".to_string(),
                "~" => return "null".to_string(),
                _ => {}
            }
        }

        // Reference: *ref → →ref
        if v.starts_with('*') {
            return format!("{}{}", self.config.arrow_char, &v[1..]);
        }

        // Quoted string - show without quotes for simple strings
        if v.starts_with('"') && v.ends_with('"') && v.len() > 2 {
            let inner = &v[1..v.len() - 1];
            // Keep quotes if contains special chars
            if !inner.contains('#') && !inner.contains('|') && !inner.contains('^') {
                return inner.to_string();
            }
        }

        v.to_string()
    }
}

/// Context for building a table during inflation
struct TableContext {
    key: String,
    columns: Vec<String>,
    expected_rows: usize,
    rows: Vec<Vec<String>>,
    comment: Option<CommentAnchor>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inflate_object() {
        let inflater = Inflater::default_config();
        let dense = "server#host:localhost#port:5432#ssl:1";
        let pretty = inflater.inflate(dense);

        assert!(pretty.contains("▼ server"));
        assert!(pretty.contains("host:"));
        assert!(pretty.contains("localhost"));
        assert!(pretty.contains("port:"));
        assert!(pretty.contains("5432"));
        assert!(pretty.contains("ssl:"));
        assert!(pretty.contains("✓"));
    }

    #[test]
    fn test_inflate_array() {
        let inflater = Inflater::default_config();
        let dense = "colors@3>red|green|blue";
        let pretty = inflater.inflate(dense);

        assert!(pretty.contains("▼ colors"));
        assert!(pretty.contains("(3 items)"));
        assert!(pretty.contains("• red"));
        assert!(pretty.contains("• green"));
        assert!(pretty.contains("• blue"));
    }

    #[test]
    fn test_inflate_table() {
        let inflater = Inflater::default_config();
        let dense = "users@2=id^name^active\n>1|Alice|1\n>2|Bob|0";
        let pretty = inflater.inflate(dense);

        assert!(pretty.contains("▼ users"));
        assert!(pretty.contains("3 columns × 2 rows"));
        assert!(pretty.contains("id"));
        assert!(pretty.contains("name"));
        assert!(pretty.contains("active"));
        assert!(pretty.contains("Alice"));
        assert!(pretty.contains("Bob"));
        assert!(pretty.contains("✓")); // true
        assert!(pretty.contains("✗")); // false
    }

    #[test]
    fn test_inflate_with_comment() {
        let inflater = Inflater::default_config();
        let dense = "!Database settings!db#host:localhost";
        let pretty = inflater.inflate(dense);

        assert!(pretty.contains("// Database settings"));
        assert!(pretty.contains("▼ db"));
        assert!(pretty.contains("localhost"));
    }

    #[test]
    fn test_inflate_null() {
        let inflater = Inflater::default_config();
        let dense = "config#value:~";
        let pretty = inflater.inflate(dense);

        assert!(pretty.contains("—"));
    }

    #[test]
    fn test_inflate_reference() {
        let inflater = Inflater::default_config();
        let dense = "link#target:*users.1";
        let pretty = inflater.inflate(dense);

        assert!(pretty.contains("→users.1"));
    }

    #[test]
    fn test_ascii_mode() {
        let inflater = Inflater::new(HologramConfig::ascii());
        let dense = "server#debug:1#prod:0";
        let pretty = inflater.inflate(dense);

        assert!(pretty.contains("> server")); // ASCII section marker
        assert!(pretty.contains("true"));
        assert!(pretty.contains("false"));
        assert!(!pretty.contains("✓"));
        assert!(!pretty.contains("✗"));
    }
}
