//! Deflater: Human-Pretty → LLM-Dense Transformation
//!
//! Transforms beautiful human-readable format back to token-efficient LLM format.
//!
//! ## Human-Pretty Format Examples:
//! ```text
//! // Database config
//! ▼ server
//!     host: localhost
//!     port: 5432
//!     ssl:  ✓
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
//!
//! ## LLM-Dense Output (Hologram format):
//! ```text
//! !Database config!server#host:localhost#port:5432#ssl:1
//! items@3>apple|banana|cherry
//! data@2=id^name^active
//! >1|Alice|1
//! >2|Bob|0
//! ```
//!
//! ## LLM-Dense Output (DX format with abbreviated keys):
//! ```text
//! c.n:dx
//! ^v:0.0.1
//! ws>frontend|backend
//! ```

use super::types::{HologramConfig, PrettyElement};
use crate::mappings::Mappings;
use std::fmt::Write;

/// Deflater: Transforms human-pretty format to LLM-dense format
pub struct Deflater {
    config: HologramConfig,
}

impl Deflater {
    /// Create a new Deflater with the given configuration
    pub fn new(config: HologramConfig) -> Self {
        Self { config }
    }

    /// Create a Deflater with default configuration
    pub fn default_config() -> Self {
        Self::new(HologramConfig::default())
    }

    /// Deflate human-pretty format to LLM-dense format
    pub fn deflate(&self, pretty: &str) -> String {
        let mut output = String::with_capacity(pretty.len());
        let lines: Vec<&str> = pretty.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // Empty line - skip
            if line.is_empty() {
                i += 1;
                continue;
            }

            // Parse the line
            let element = self.parse_pretty_line(line);

            match element {
                PrettyElement::Comment { text } => {
                    // Look ahead to see if there's content to anchor to
                    let next_content = self.find_next_content(&lines, i + 1);
                    if next_content.is_some() {
                        // Store comment for anchoring
                        let (section_output, consumed) =
                            self.deflate_section_with_comment(&lines, i + 1, Some(&text));
                        output.push_str(&section_output);
                        i += 1 + consumed;
                    } else {
                        // Standalone comment
                        writeln!(output, "!{}!", text).unwrap();
                        i += 1;
                    }
                }
                PrettyElement::Section { key: _, metadata: _ } => {
                    let (section_output, consumed) =
                        self.deflate_section_with_comment(&lines, i, None);
                    output.push_str(&section_output);
                    i += consumed;
                }
                PrettyElement::Field { key, value } => {
                    // Check if this is an array marked with @ARRAY@
                    if let Some(items_str) = value.strip_prefix("@ARRAY@") {
                        // Skip @ARRAY@
                        let compressed_key = self.compress_full_key(&key);
                        writeln!(output, "{}>{}", compressed_key, items_str).unwrap();
                    } else {
                        // Simple key:value at root level - compress the key
                        let compressed_key = self.compress_full_key(&key);
                        let dense_val = self.deflate_value(&value);
                        writeln!(output, "{}:{}", compressed_key, dense_val).unwrap();
                    }
                    i += 1;
                }
                PrettyElement::TableBorder | PrettyElement::Empty => {
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        output.trim_end().to_string()
    }

    /// Parse a single line of human-pretty format
    fn parse_pretty_line(&self, line: &str) -> PrettyElement {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return PrettyElement::Empty;
        }

        // Comment: // text
        if trimmed.starts_with("//") {
            let text = trimmed[2..].trim().to_string();
            return PrettyElement::Comment { text };
        }

        // Section marker: ▼ key or ▾ key or > key (ASCII mode)
        if trimmed.starts_with('▼') || trimmed.starts_with('▾') || trimmed.starts_with("> ") {
            let content = if trimmed.starts_with("> ") {
                &trimmed[2..]
            } else {
                &trimmed[3..] // Skip UTF-8 char + space
            }
            .trim();

            // Parse: key (N items) or key (N columns × M rows)
            if let Some(paren_idx) = content.find('(') {
                let key = content[..paren_idx].trim().to_string();
                let metadata = content[paren_idx..].trim().to_string();
                return PrettyElement::Section {
                    key,
                    metadata: Some(metadata),
                };
            }

            return PrettyElement::Section {
                key: content.to_string(),
                metadata: None,
            };
        }

        // Table border: ┌─── or ├─── or └───
        if trimmed.starts_with('┌')
            || trimmed.starts_with('├')
            || trimmed.starts_with('└')
            || trimmed.chars().all(|c| "─┬┼┴┌├└┐┤┘".contains(c))
        {
            return PrettyElement::TableBorder;
        }

        // Table row: │ cell │ cell │
        if trimmed.starts_with('│') && trimmed.ends_with('│') {
            let cells: Vec<String> = trimmed[3..trimmed.len() - 3]
                .split('│')
                .map(|s| s.trim().to_string())
                .collect();

            // Check if it's a separator row
            if cells.iter().all(|c| c.chars().all(|ch| ch == '─')) {
                return PrettyElement::TableBorder;
            }

            // Could be header or data - we'll determine by context
            return PrettyElement::TableRow { values: cells };
        }

        // Bullet item: • item or - item or · item
        if trimmed.starts_with('•') || trimmed.starts_with("- ") || trimmed.starts_with('·') {
            let value = if trimmed.starts_with("- ") {
                trimmed[2..].trim()
            } else {
                trimmed[3..].trim() // Skip UTF-8 char + space
            }
            .to_string();
            return PrettyElement::Bullet { value };
        }

        // DX Array format: key > item | item | item (note: > with spaces)
        if let Some(arrow_idx) = trimmed.find(" > ") {
            let key = trimmed[..arrow_idx].trim().to_string();
            let items_str = trimmed[arrow_idx + 3..].trim();
            let items: Vec<String> = items_str
                .split(" | ")
                .map(|s| s.trim().to_string())
                .collect();
            
            // Return as a field with array-like value for now
            // We'll handle this specially in deflate()
            return PrettyElement::Field { 
                key, 
                value: format!("@ARRAY@{}", items.join("|")) 
            };
        }

        // Field: key: value (with possible alignment spaces)
        if let Some(colon_idx) = trimmed.find(':') {
            let key = trimmed[..colon_idx].trim().to_string();
            let value = trimmed[colon_idx + 1..].trim().to_string();
            return PrettyElement::Field { key, value };
        }

        // Unknown - treat as empty
        PrettyElement::Empty
    }

    /// Deflate a section (object, array, or table) with optional comment
    fn deflate_section_with_comment(
        &self,
        lines: &[&str],
        start_idx: usize,
        pending_comment: Option<&str>,
    ) -> (String, usize) {
        if start_idx >= lines.len() {
            return (String::new(), 0);
        }

        let line = lines[start_idx].trim();

        // Parse section header
        let element = self.parse_pretty_line(line);

        match element {
            PrettyElement::Section { key, metadata } => {
                // Determine section type from metadata
                let is_table = metadata
                    .as_ref()
                    .map(|m| m.contains("columns") || m.contains("×"))
                    .unwrap_or(false);
                let is_array = metadata
                    .as_ref()
                    .map(|m| m.contains("items"))
                    .unwrap_or(false);

                if is_table {
                    self.deflate_table(lines, start_idx, &key, pending_comment)
                } else if is_array {
                    self.deflate_array(lines, start_idx, &key, pending_comment)
                } else {
                    self.deflate_object(lines, start_idx, &key, pending_comment)
                }
            }
            _ => (String::new(), 1),
        }
    }

    /// Deflate an object section
    fn deflate_object(
        &self,
        lines: &[&str],
        start_idx: usize,
        key: &str,
        pending_comment: Option<&str>,
    ) -> (String, usize) {
        let mut output = String::new();
        let mut fields: Vec<(String, String)> = Vec::new();
        let mut i = start_idx + 1;

        // Collect fields
        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // End of section: empty line, new section, or comment
            if trimmed.is_empty() {
                break;
            }
            if trimmed.starts_with('▼')
                || trimmed.starts_with('▾')
                || trimmed.starts_with("> ")
                || trimmed.starts_with("//")
            {
                break;
            }

            // Check if this is an indented field
            if line.starts_with(' ') || line.starts_with('\t') {
                if let Some(colon_idx) = trimmed.find(':') {
                    let field_name = trimmed[..colon_idx].trim().to_string();
                    let field_value = trimmed[colon_idx + 1..].trim().to_string();
                    fields.push((field_name, self.deflate_value(&field_value)));
                }
            } else {
                // Not part of this section
                break;
            }

            i += 1;
        }

        // Build dense output
        if let Some(comment) = pending_comment {
            write!(output, "!{}!", comment).unwrap();
        }

        // Compress key and field names
        let compressed_key = self.compress_full_key(key);
        write!(output, "{}", compressed_key).unwrap();
        for (name, value) in &fields {
            let compressed_name = self.compress_full_key(name);
            write!(output, "#{}:{}", compressed_name, value).unwrap();
        }
        writeln!(output).unwrap();

        (output, i - start_idx)
    }

    /// Deflate an array section
    fn deflate_array(
        &self,
        lines: &[&str],
        start_idx: usize,
        key: &str,
        pending_comment: Option<&str>,
    ) -> (String, usize) {
        let mut output = String::new();
        let mut items: Vec<String> = Vec::new();
        let mut i = start_idx + 1;

        // Collect items
        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // End of section
            if trimmed.is_empty() {
                break;
            }
            if trimmed.starts_with('▼')
                || trimmed.starts_with('▾')
                || trimmed.starts_with("> ")
                || trimmed.starts_with("//")
            {
                break;
            }

            // Bullet item
            if trimmed.starts_with('•') || trimmed.starts_with("- ") || trimmed.starts_with('·') {
                let value = if trimmed.starts_with("- ") {
                    trimmed[2..].trim()
                } else {
                    // Handle multi-byte chars
                    let char_len = trimmed.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                    trimmed[char_len..].trim()
                };
                items.push(self.deflate_value(value));
            } else if line.starts_with(' ') || line.starts_with('\t') {
                // Indented item without bullet
                items.push(self.deflate_value(trimmed));
            } else {
                break;
            }

            i += 1;
        }

        // Build dense output
        if let Some(comment) = pending_comment {
            write!(output, "!{}!", comment).unwrap();
        }

        // Compress key
        let compressed_key = self.compress_full_key(key);
        writeln!(output, "{}@{}>{}", compressed_key, items.len(), items.join("|")).unwrap();

        (output, i - start_idx)
    }

    /// Deflate a table section
    fn deflate_table(
        &self,
        lines: &[&str],
        start_idx: usize,
        key: &str,
        pending_comment: Option<&str>,
    ) -> (String, usize) {
        let mut output = String::new();
        let mut columns: Vec<String> = Vec::new();
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut i = start_idx + 1;
        let mut header_found = false;

        // Collect table data
        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // End of section
            if trimmed.is_empty() {
                break;
            }
            if trimmed.starts_with('▼')
                || trimmed.starts_with('▾')
                || trimmed.starts_with("> ")
                || trimmed.starts_with("//")
            {
                break;
            }

            // Skip borders
            if trimmed.starts_with('┌')
                || trimmed.starts_with('├')
                || trimmed.starts_with('└')
                || trimmed.starts_with('+')
            {
                i += 1;
                continue;
            }

            // Table row: │ cell │ cell │ or simple: cell | cell
            if trimmed.starts_with('│') && trimmed.ends_with('│') {
                let cells: Vec<String> = trimmed[3..trimmed.len() - 3]
                    .split('│')
                    .map(|s| s.trim().to_string())
                    .collect();

                // Skip separator rows
                if cells.iter().all(|c| c.chars().all(|ch| ch == '─')) {
                    i += 1;
                    continue;
                }

                if !header_found {
                    columns = cells;
                    header_found = true;
                } else {
                    let deflated_cells: Vec<String> =
                        cells.iter().map(|c| self.deflate_value(c)).collect();
                    rows.push(deflated_cells);
                }
            } else if trimmed.contains(" | ") {
                // Simple ASCII table
                let cells: Vec<String> = trimmed.split(" | ").map(|s| s.trim().to_string()).collect();

                if !header_found {
                    columns = cells;
                    header_found = true;
                } else if !cells.iter().all(|c| c.chars().all(|ch| ch == '-')) {
                    let deflated_cells: Vec<String> =
                        cells.iter().map(|c| self.deflate_value(c)).collect();
                    rows.push(deflated_cells);
                }
            }

            i += 1;
        }

        // Build dense output
        if let Some(comment) = pending_comment {
            write!(output, "!{}!", comment).unwrap();
        }

        // Compress key and column names
        let compressed_key = self.compress_full_key(key);
        let compressed_columns: Vec<String> = columns
            .iter()
            .map(|c| self.compress_full_key(c))
            .collect();
        
        writeln!(output, "{}@{}={}", compressed_key, rows.len(), compressed_columns.join("^")).unwrap();

        for row in &rows {
            writeln!(output, ">{}", row.join("|")).unwrap();
        }

        (output, i - start_idx)
    }

    /// Deflate a single value (e.g., ✓ → 1, — → ~)
    fn deflate_value(&self, value: &str) -> String {
        let v = value.trim();

        // Boolean symbols
        match v {
            "✓" | "true" | "yes" | "True" | "Yes" | "TRUE" | "YES" => return "1".to_string(),
            "✗" | "false" | "no" | "False" | "No" | "FALSE" | "NO" => return "0".to_string(),
            _ => {}
        }

        // Null symbols
        if v == "—" || v == "null" || v == "none" || v == "∅" || v == "Null" || v == "None" {
            return "~".to_string();
        }

        // Reference arrows: →ref → *ref
        if v.starts_with('→') {
            return format!("*{}", &v[3..]); // UTF-8: → is 3 bytes
        }

        // String with spaces or special chars - quote it
        if (v.contains(' ') || v.contains('#') || v.contains('|') || v.contains('^'))
            && !v.starts_with('"') && !v.starts_with('\'') {
                return format!("\"{}\"", v);
            }

        v.to_string()
    }

    /// Compress a full key (handles dotted and prefix keys)
    /// e.g., "context.name" → "c.n", "^version" → "^v"
    fn compress_full_key(&self, key: &str) -> String {
        let mappings = Mappings::get();
        
        // Handle prefix inheritance (^key)
        let (prefix, rest) = if key.starts_with('^') {
            ("^", &key[1..])
        } else {
            ("", key)
        };
        
        // Handle dotted keys: each part compressed independently
        let compressed = if rest.contains('.') {
            rest.split('.')
                .map(|part| mappings.compress_key(part))
                .collect::<Vec<_>>()
                .join(".")
        } else if rest.contains('_') {
            rest.split('_')
                .map(|part| mappings.compress_key(part))
                .collect::<Vec<_>>()
                .join("_")
        } else {
            mappings.compress_key(rest)
        };
        
        format!("{}{}", prefix, compressed)
    }

    /// Find next non-empty, non-comment content
    fn find_next_content<'a>(&self, lines: &[&'a str], from_idx: usize) -> Option<&'a str> {
        for i in from_idx..lines.len() {
            let line = lines[i].trim();
            if !line.is_empty() && !line.starts_with("//") {
                return Some(line);
            }
        }
        None
    }

    /// Convert JSON object to LLM-dense format
    pub fn json_to_dense(&self, json: &str) -> Result<String, String> {
        // Parse JSON
        let parsed: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("JSON parse error: {}", e))?;

        Ok(self.value_to_dense(&parsed, None))
    }

    /// Convert a JSON value to dense format
    fn value_to_dense(&self, value: &serde_json::Value, key: Option<&str>) -> String {
        use serde_json::Value;

        match value {
            Value::Object(map) => {
                let fields: Vec<String> = map
                    .iter()
                    .map(|(k, v)| {
                        if v.is_object() || v.is_array() {
                            // Nested - recurse with key
                            self.value_to_dense(v, Some(k))
                        } else {
                            format!("{}:{}", k, self.json_value_to_string(v))
                        }
                    })
                    .collect();

                if let Some(k) = key {
                    // Check if all fields are simple
                    let all_simple = map.values().all(|v| !v.is_object() && !v.is_array());
                    if all_simple {
                        format!("{}#{}", k, fields.join("#"))
                    } else {
                        fields.join("\n")
                    }
                } else {
                    fields.join("\n")
                }
            }
            Value::Array(arr) => {
                if arr.is_empty() {
                    return String::new();
                }

                // Check if array of objects (table)
                if arr.iter().all(|v| v.is_object()) {
                    let first = arr[0].as_object().unwrap();
                    let columns: Vec<&str> = first.keys().map(|s| s.as_str()).collect();

                    let mut output = String::new();
                    if let Some(k) = key {
                        writeln!(output, "{}@{}={}", k, arr.len(), columns.join("^")).unwrap();
                    } else {
                        writeln!(output, "@{}={}", arr.len(), columns.join("^")).unwrap();
                    }

                    for item in arr {
                        let obj = item.as_object().unwrap();
                        let values: Vec<String> = columns
                            .iter()
                            .map(|&col| {
                                obj.get(col)
                                    .map(|v| self.json_value_to_string(v))
                                    .unwrap_or_else(|| "~".to_string())
                            })
                            .collect();
                        writeln!(output, ">{}", values.join("|")).unwrap();
                    }

                    output
                } else {
                    // Simple array
                    let items: Vec<String> =
                        arr.iter().map(|v| self.json_value_to_string(v)).collect();

                    if let Some(k) = key {
                        format!("{}@{}>{}", k, items.len(), items.join("|"))
                    } else {
                        format!("@{}>{}", items.len(), items.join("|"))
                    }
                }
            }
            _ => self.json_value_to_string(value),
        }
    }

    /// Convert a simple JSON value to string
    fn json_value_to_string(&self, value: &serde_json::Value) -> String {
        use serde_json::Value;

        match value {
            Value::Null => "~".to_string(),
            Value::Bool(b) => if *b { "1" } else { "0" }.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => {
                if s.contains(' ') || s.contains('#') || s.contains('|') || s.contains('^') {
                    format!("\"{}\"", s)
                } else {
                    s.clone()
                }
            }
            Value::Array(_) | Value::Object(_) => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hologram::Inflater;

    #[test]
    fn test_deflate_object() {
        let deflater = Deflater::default_config();
        let pretty = "▼ server\n    host: localhost\n    port: 5432";
        let dense = deflater.deflate(pretty);

        assert!(dense.contains("server#"));
        assert!(dense.contains("host:localhost"));
        assert!(dense.contains("port:5432"));
    }

    #[test]
    fn test_deflate_array() {
        let deflater = Deflater::default_config();
        let pretty = "▼ colors (3 items)\n    • red\n    • green\n    • blue";
        let dense = deflater.deflate(pretty);

        assert!(dense.contains("colors@3>"));
        assert!(dense.contains("red|green|blue"));
    }

    #[test]
    fn test_deflate_boolean_symbols() {
        let deflater = Deflater::default_config();
        let pretty = "▼ config\n    debug: ✓\n    prod: ✗";
        let dense = deflater.deflate(pretty);

        assert!(dense.contains("debug:1"));
        assert!(dense.contains("prod:0"));
    }

    #[test]
    fn test_deflate_comment() {
        let deflater = Deflater::default_config();
        let pretty = "// Database settings\n▼ db\n    host: localhost";
        let dense = deflater.deflate(pretty);

        assert!(dense.contains("!Database settings!"));
        assert!(dense.contains("db#host:localhost"));
    }

    #[test]
    fn test_round_trip_object() {
        let config = HologramConfig::default();
        let inflater = Inflater::new(config.clone());
        let deflater = Deflater::new(config);

        let original = "server#host:localhost#port:5432#ssl:1";
        let pretty = inflater.inflate(original);
        let back = deflater.deflate(&pretty);

        assert!(back.contains("server#"));
        assert!(back.contains("host:localhost"));
        assert!(back.contains("port:5432"));
        assert!(back.contains("ssl:1"));
    }

    #[test]
    fn test_round_trip_array() {
        let config = HologramConfig::default();
        let inflater = Inflater::new(config.clone());
        let deflater = Deflater::new(config);

        let original = "items@3>apple|banana|cherry";
        let pretty = inflater.inflate(original);
        let back = deflater.deflate(&pretty);

        assert!(back.contains("items@3>"));
        assert!(back.contains("apple|banana|cherry"));
    }

    #[test]
    fn test_json_to_dense() {
        let deflater = Deflater::default_config();
        let json = r#"{"host": "localhost", "port": 5432, "ssl": true}"#;
        let dense = deflater.json_to_dense(json).unwrap();

        assert!(dense.contains("host:localhost"));
        assert!(dense.contains("port:5432"));
        assert!(dense.contains("ssl:1"));
    }

    #[test]
    fn test_json_array_to_dense() {
        let deflater = Deflater::default_config();
        let json = r#"{"items": ["a", "b", "c"]}"#;
        let dense = deflater.json_to_dense(json).unwrap();

        assert!(dense.contains("items@3>a|b|c"));
    }
}
