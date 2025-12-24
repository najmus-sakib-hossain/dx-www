//! LLM format serializer
//!
//! Serializes DxDocument to token-optimized LLM format.
//!
//! ## Features
//!
//! - Automatic reference creation for repeated strings
//! - Compact boolean representation (+/-)
//! - Null as ~
//! - Inline arrays as *a,b,c
//! - Key abbreviation support

use crate::llm::abbrev::AbbrevDict;
use crate::llm::types::{DxDocument, DxLlmValue, DxSection};
use std::collections::HashMap;

/// Serialize DxDocument to LLM-optimized format
pub struct LlmSerializer {
    abbrev: AbbrevDict,
    /// Minimum string length to consider for reference creation
    min_ref_length: usize,
    /// Minimum occurrences to create a reference
    min_ref_count: usize,
}

impl LlmSerializer {
    /// Create a new serializer with default settings
    pub fn new() -> Self {
        Self {
            abbrev: AbbrevDict::new(),
            min_ref_length: 5,
            min_ref_count: 2,
        }
    }

    /// Create a serializer with custom abbreviation dictionary
    pub fn with_abbrev(abbrev: AbbrevDict) -> Self {
        Self {
            abbrev,
            min_ref_length: 5,
            min_ref_count: 2,
        }
    }

    /// Serialize DxDocument to LLM format string
    pub fn serialize(&self, doc: &DxDocument) -> String {
        let mut output = String::new();

        // Find repeated strings for reference optimization
        let auto_refs = self.find_repeated_strings(doc);

        // Merge auto-refs with existing refs
        let mut all_refs = doc.refs.clone();
        for (key, value) in auto_refs {
            if !all_refs.contains_key(&key) {
                all_refs.insert(key, value);
            }
        }

        // Serialize context section
        if !doc.context.is_empty() {
            output.push_str(&self.serialize_context(&doc.context, &all_refs));
            output.push('\n');
        }

        // Serialize reference definitions
        for (key, value) in &all_refs {
            output.push_str(&format!("#:{}|{}\n", key, value));
        }

        // Serialize data sections
        let mut section_ids: Vec<_> = doc.sections.keys().collect();
        section_ids.sort();
        for id in section_ids {
            if let Some(section) = doc.sections.get(id) {
                output.push_str(&self.serialize_section(*id, section, &all_refs));
            }
        }

        output.trim_end().to_string()
    }


    /// Find repeated strings for reference optimization
    ///
    /// Returns a map of reference keys to values for strings that appear
    /// multiple times and are long enough to benefit from referencing.
    fn find_repeated_strings(&self, doc: &DxDocument) -> HashMap<String, String> {
        let mut string_counts: HashMap<String, usize> = HashMap::new();

        // Count strings in context
        for value in doc.context.values() {
            self.count_strings(value, &mut string_counts);
        }

        // Count strings in sections
        for section in doc.sections.values() {
            for row in &section.rows {
                for value in row {
                    self.count_strings(value, &mut string_counts);
                }
            }
        }

        // Create references for repeated strings
        let mut refs = HashMap::new();
        let mut ref_counter = 0u8;

        for (string, count) in string_counts {
            if string.len() >= self.min_ref_length && count >= self.min_ref_count {
                // Generate reference key (A, B, C, ... Z, AA, AB, ...)
                let key = self.generate_ref_key(ref_counter);
                refs.insert(key, string);
                ref_counter += 1;
            }
        }

        refs
    }

    /// Count string occurrences in a value
    fn count_strings(&self, value: &DxLlmValue, counts: &mut HashMap<String, usize>) {
        match value {
            DxLlmValue::Str(s) => {
                *counts.entry(s.clone()).or_insert(0) += 1;
            }
            DxLlmValue::Arr(items) => {
                for item in items {
                    self.count_strings(item, counts);
                }
            }
            _ => {}
        }
    }

    /// Generate a reference key from a counter
    fn generate_ref_key(&self, n: u8) -> String {
        if n < 26 {
            ((b'A' + n) as char).to_string()
        } else {
            format!("{}{}", ((b'A' + (n / 26) - 1) as char), ((b'A' + (n % 26)) as char))
        }
    }

    /// Serialize context section
    fn serialize_context(
        &self,
        context: &HashMap<String, DxLlmValue>,
        refs: &HashMap<String, String>,
    ) -> String {
        if context.is_empty() {
            return String::new();
        }

        let mut pairs: Vec<String> = Vec::new();

        // Sort keys for consistent output
        let mut keys: Vec<_> = context.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(value) = context.get(key) {
                let compressed_key = self.abbrev.compress(key);
                let serialized_value = self.serialize_value(value, refs);
                pairs.push(format!("{}|{}", compressed_key, serialized_value));
            }
        }

        format!("#c:{}", pairs.join(";"))
    }

    /// Serialize a data section
    fn serialize_section(
        &self,
        id: char,
        section: &DxSection,
        refs: &HashMap<String, String>,
    ) -> String {
        let mut output = String::new();

        // Serialize schema
        let compressed_schema: Vec<String> = section
            .schema
            .iter()
            .map(|col| self.abbrev.compress(col))
            .collect();
        output.push_str(&format!("#{}({})\n", id, compressed_schema.join("|")));

        // Serialize rows
        for row in &section.rows {
            let values: Vec<String> = row
                .iter()
                .map(|v| self.serialize_value(v, refs))
                .collect();
            output.push_str(&values.join("|"));
            output.push('\n');
        }

        output
    }


    /// Serialize a single value
    fn serialize_value(&self, value: &DxLlmValue, refs: &HashMap<String, String>) -> String {
        match value {
            DxLlmValue::Bool(true) => "+".to_string(),
            DxLlmValue::Bool(false) => "-".to_string(),
            DxLlmValue::Null => "~".to_string(),
            DxLlmValue::Ref(key) => format!("^{}", key),
            DxLlmValue::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            DxLlmValue::Str(s) => {
                // Check if this string should be a reference
                for (ref_key, ref_value) in refs {
                    if ref_value == s {
                        return format!("^{}", ref_key);
                    }
                }
                s.clone()
            }
            DxLlmValue::Arr(items) => {
                let serialized: Vec<String> = items
                    .iter()
                    .map(|item| self.serialize_value(item, refs))
                    .collect();
                format!("*{}", serialized.join(","))
            }
        }
    }

    /// Get the abbreviation dictionary
    pub fn abbrev(&self) -> &AbbrevDict {
        &self.abbrev
    }
}

impl Default for LlmSerializer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_empty() {
        let serializer = LlmSerializer::new();
        let doc = DxDocument::new();
        let output = serializer.serialize(&doc);
        assert!(output.is_empty());
    }

    #[test]
    fn test_serialize_context() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();
        doc.context.insert("name".to_string(), DxLlmValue::Str("Test".to_string()));
        doc.context.insert("count".to_string(), DxLlmValue::Num(42.0));

        let output = serializer.serialize(&doc);
        assert!(output.contains("#c:"));
        assert!(output.contains("ct|42")); // count -> ct
        assert!(output.contains("nm|Test")); // name -> nm
    }

    #[test]
    fn test_serialize_booleans() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();
        doc.context.insert("active".to_string(), DxLlmValue::Bool(true));
        doc.context.insert("deleted".to_string(), DxLlmValue::Bool(false));

        let output = serializer.serialize(&doc);
        assert!(output.contains("|+"));
        assert!(output.contains("|-"));
    }

    #[test]
    fn test_serialize_null() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();
        doc.context.insert("value".to_string(), DxLlmValue::Null);

        let output = serializer.serialize(&doc);
        assert!(output.contains("|~"));
    }

    #[test]
    fn test_serialize_section() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string(), "name".to_string(), "active".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Str("Alpha".to_string()),
            DxLlmValue::Bool(true),
        ]);
        section.rows.push(vec![
            DxLlmValue::Num(2.0),
            DxLlmValue::Str("Beta".to_string()),
            DxLlmValue::Bool(false),
        ]);
        doc.sections.insert('d', section);

        let output = serializer.serialize(&doc);
        assert!(output.contains("#d(id|nm|ac)")); // name -> nm, active -> ac
        assert!(output.contains("1|Alpha|+"));
        assert!(output.contains("2|Beta|-"));
    }

    #[test]
    fn test_serialize_array() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();
        doc.context.insert(
            "tags".to_string(),
            DxLlmValue::Arr(vec![
                DxLlmValue::Str("a".to_string()),
                DxLlmValue::Str("b".to_string()),
                DxLlmValue::Str("c".to_string()),
            ]),
        );

        let output = serializer.serialize(&doc);
        assert!(output.contains("*a,b,c"));
    }

    #[test]
    fn test_serialize_references() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();
        doc.refs.insert("A".to_string(), "Shared Value".to_string());

        let mut section = DxSection::new(vec!["id".to_string(), "value".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Ref("A".to_string()),
        ]);
        doc.sections.insert('d', section);

        let output = serializer.serialize(&doc);
        assert!(output.contains("#:A|Shared Value"));
        assert!(output.contains("^A"));
    }

    #[test]
    fn test_auto_reference_creation() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();

        // Create a section with repeated long strings
        let mut section = DxSection::new(vec!["id".to_string(), "value".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Str("This is a long repeated string".to_string()),
        ]);
        section.rows.push(vec![
            DxLlmValue::Num(2.0),
            DxLlmValue::Str("This is a long repeated string".to_string()),
        ]);
        doc.sections.insert('d', section);

        let output = serializer.serialize(&doc);
        // Should create a reference for the repeated string
        assert!(output.contains("#:"));
        assert!(output.contains("^"));
    }

    #[test]
    fn test_generate_ref_key() {
        let serializer = LlmSerializer::new();
        assert_eq!(serializer.generate_ref_key(0), "A");
        assert_eq!(serializer.generate_ref_key(1), "B");
        assert_eq!(serializer.generate_ref_key(25), "Z");
        assert_eq!(serializer.generate_ref_key(26), "AA");
        assert_eq!(serializer.generate_ref_key(27), "AB");
    }
}
