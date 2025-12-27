//! Object representation

use std::collections::HashMap;

/// JavaScript object
#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    /// Properties
    properties: HashMap<String, super::Value>,
    /// Prototype
    prototype: Option<Box<Object>>,
}

impl Object {
    /// Create a new empty object
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
        }
    }

    /// Get a property
    pub fn get(&self, key: &str) -> Option<&super::Value> {
        self.properties
            .get(key)
            .or_else(|| self.prototype.as_ref().and_then(|p| p.get(key)))
    }

    /// Set a property
    pub fn set(&mut self, key: String, value: super::Value) {
        self.properties.insert(key, value);
    }

    /// Check if has own property
    pub fn has_own(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Get all own keys
    pub fn keys(&self) -> Vec<&String> {
        self.properties.keys().collect()
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}
