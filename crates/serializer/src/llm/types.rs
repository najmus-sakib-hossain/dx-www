//! Core data types for LLM and Human format serialization
//!
//! This module defines the internal representation (IR) that serves as the "hub"
//! for converting between LLM, Human, and Machine formats.

use std::collections::HashMap;
use std::fmt;

/// Internal document representation (the "hub")
///
/// All formats convert through this common representation, ensuring
/// consistent round-trip behavior.
#[derive(Debug, Clone, PartialEq)]
pub struct DxDocument {
    /// Context/config section (#c)
    pub context: HashMap<String, DxLlmValue>,
    /// Reference definitions (#:)
    pub refs: HashMap<String, String>,
    /// Data sections (#<letter>)
    pub sections: HashMap<char, DxSection>,
}

impl DxDocument {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
            refs: HashMap::new(),
            sections: HashMap::new(),
        }
    }

    /// Check if the document is empty
    pub fn is_empty(&self) -> bool {
        self.context.is_empty() && self.refs.is_empty() && self.sections.is_empty()
    }
}

impl Default for DxDocument {
    fn default() -> Self {
        Self::new()
    }
}


/// A data section with schema and rows
#[derive(Debug, Clone, PartialEq)]
pub struct DxSection {
    /// Column names from schema
    pub schema: Vec<String>,
    /// Row data
    pub rows: Vec<Vec<DxLlmValue>>,
}

impl DxSection {
    /// Create a new section with the given schema
    pub fn new(schema: Vec<String>) -> Self {
        Self {
            schema,
            rows: Vec::new(),
        }
    }

    /// Add a row to the section
    pub fn add_row(&mut self, row: Vec<DxLlmValue>) -> Result<(), String> {
        if row.len() != self.schema.len() {
            return Err(format!(
                "Row length {} doesn't match schema length {}",
                row.len(),
                self.schema.len()
            ));
        }
        self.rows.push(row);
        Ok(())
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.schema.len()
    }
}

/// Value types in DX LLM/Human format
///
/// This enum represents all possible value types that can appear in
/// LLM and Human format documents.
#[derive(Debug, Clone, PartialEq)]
pub enum DxLlmValue {
    /// String value
    Str(String),
    /// Numeric value (integer or float)
    Num(f64),
    /// Boolean value (+ or - in LLM, ✓ or ✗ in Human)
    Bool(bool),
    /// Null value (~ in LLM, — in Human)
    Null,
    /// Array value (*a,b,c in LLM)
    Arr(Vec<DxLlmValue>),
    /// Reference pointer (^key)
    Ref(String),
}

impl DxLlmValue {
    /// Check if this value is null
    pub fn is_null(&self) -> bool {
        matches!(self, DxLlmValue::Null)
    }

    /// Get the type name for error messages
    pub fn type_name(&self) -> &'static str {
        match self {
            DxLlmValue::Str(_) => "string",
            DxLlmValue::Num(_) => "number",
            DxLlmValue::Bool(_) => "bool",
            DxLlmValue::Null => "null",
            DxLlmValue::Arr(_) => "array",
            DxLlmValue::Ref(_) => "ref",
        }
    }

    /// Try to get as string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DxLlmValue::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as number
    pub fn as_num(&self) -> Option<f64> {
        match self {
            DxLlmValue::Num(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get as bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DxLlmValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get as array
    pub fn as_arr(&self) -> Option<&Vec<DxLlmValue>> {
        match self {
            DxLlmValue::Arr(arr) => Some(arr),
            _ => None,
        }
    }

    /// Try to get as reference key
    pub fn as_ref(&self) -> Option<&str> {
        match self {
            DxLlmValue::Ref(key) => Some(key),
            _ => None,
        }
    }
}

impl fmt::Display for DxLlmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DxLlmValue::Str(s) => write!(f, "{}", s),
            DxLlmValue::Num(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            DxLlmValue::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            DxLlmValue::Null => write!(f, "null"),
            DxLlmValue::Arr(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            DxLlmValue::Ref(key) => write!(f, "^{}", key),
        }
    }
}

impl From<&str> for DxLlmValue {
    fn from(s: &str) -> Self {
        DxLlmValue::Str(s.to_string())
    }
}

impl From<String> for DxLlmValue {
    fn from(s: String) -> Self {
        DxLlmValue::Str(s)
    }
}

impl From<f64> for DxLlmValue {
    fn from(n: f64) -> Self {
        DxLlmValue::Num(n)
    }
}

impl From<i64> for DxLlmValue {
    fn from(n: i64) -> Self {
        DxLlmValue::Num(n as f64)
    }
}

impl From<bool> for DxLlmValue {
    fn from(b: bool) -> Self {
        DxLlmValue::Bool(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dx_document_new() {
        let doc = DxDocument::new();
        assert!(doc.is_empty());
        assert!(doc.context.is_empty());
        assert!(doc.refs.is_empty());
        assert!(doc.sections.is_empty());
    }

    #[test]
    fn test_dx_section_add_row() {
        let mut section = DxSection::new(vec!["id".to_string(), "name".to_string()]);
        
        // Valid row
        let result = section.add_row(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Str("Test".to_string()),
        ]);
        assert!(result.is_ok());
        assert_eq!(section.row_count(), 1);
        
        // Invalid row (wrong length)
        let result = section.add_row(vec![DxLlmValue::Num(2.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn test_dx_llm_value_type_name() {
        assert_eq!(DxLlmValue::Str("test".to_string()).type_name(), "string");
        assert_eq!(DxLlmValue::Num(42.0).type_name(), "number");
        assert_eq!(DxLlmValue::Bool(true).type_name(), "bool");
        assert_eq!(DxLlmValue::Null.type_name(), "null");
        assert_eq!(DxLlmValue::Arr(vec![]).type_name(), "array");
        assert_eq!(DxLlmValue::Ref("key".to_string()).type_name(), "ref");
    }

    #[test]
    fn test_dx_llm_value_display() {
        assert_eq!(format!("{}", DxLlmValue::Str("hello".to_string())), "hello");
        assert_eq!(format!("{}", DxLlmValue::Num(42.0)), "42");
        assert_eq!(format!("{}", DxLlmValue::Num(3.14)), "3.14");
        assert_eq!(format!("{}", DxLlmValue::Bool(true)), "true");
        assert_eq!(format!("{}", DxLlmValue::Bool(false)), "false");
        assert_eq!(format!("{}", DxLlmValue::Null), "null");
        assert_eq!(format!("{}", DxLlmValue::Ref("A".to_string())), "^A");
    }
}
