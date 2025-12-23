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
