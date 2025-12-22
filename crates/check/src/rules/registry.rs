//! Rule Registry
//!
//! Manages all available rules and their configurations.

use super::builtin;
use super::{Category, Rule, RuleId, RuleMeta, Severity};
use crate::config::{RuleConfig, RuleConfigs, RuleSeverity};
use std::collections::HashMap;

/// Registry of all available lint rules
pub struct RuleRegistry {
    /// Rules indexed by ID
    rules_by_id: HashMap<u16, Box<dyn Rule>>,
    /// Rules indexed by name
    rules_by_name: HashMap<String, u16>,
    /// Enabled rules with their configured severity
    enabled: HashMap<u16, Severity>,
}

impl RuleRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            rules_by_id: HashMap::new(),
            rules_by_name: HashMap::new(),
            enabled: HashMap::new(),
        }
    }

    /// Create a registry with all built-in rules
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();

        for rule in builtin::all_rules() {
            registry.register(rule);
        }

        registry
    }

    /// Create a registry configured from RuleConfigs
    pub fn from_config(config: &RuleConfigs) -> Self {
        let mut registry = Self::with_builtins();

        // Enable recommended rules by default
        if config.recommended {
            for rule in builtin::recommended_rules() {
                let id = rule.meta().id.0;
                registry.enabled.insert(id, rule.meta().default_severity);
            }
        }

        // Apply individual rule configurations
        for (name, rule_config) in &config.rules {
            if let Some(&id) = registry.rules_by_name.get(name) {
                match rule_config.severity {
                    RuleSeverity::Off => {
                        registry.enabled.remove(&id);
                    }
                    RuleSeverity::Warn => {
                        registry.enabled.insert(id, Severity::Warn);
                    }
                    RuleSeverity::Error => {
                        registry.enabled.insert(id, Severity::Error);
                    }
                }
            }
        }

        registry
    }

    /// Register a rule
    pub fn register(&mut self, rule: Box<dyn Rule>) {
        let meta = rule.meta();
        let id = meta.id.0;
        let name = meta.name.to_string();

        self.rules_by_name.insert(name, id);
        self.rules_by_id.insert(id, rule);
    }

    /// Enable a rule with a specific severity
    pub fn enable(&mut self, name: &str, severity: Severity) {
        if let Some(&id) = self.rules_by_name.get(name) {
            self.enabled.insert(id, severity);
        }
    }

    /// Disable a rule
    pub fn disable(&mut self, name: &str) {
        if let Some(&id) = self.rules_by_name.get(name) {
            self.enabled.remove(&id);
        }
    }

    /// Get all enabled rules
    pub fn enabled_rules(&self) -> impl Iterator<Item = (&Box<dyn Rule>, Severity)> {
        self.enabled.iter().filter_map(move |(id, severity)| {
            self.rules_by_id.get(id).map(|rule| (rule, *severity))
        })
    }

    /// Get a rule by name
    pub fn get(&self, name: &str) -> Option<&Box<dyn Rule>> {
        self.rules_by_name
            .get(name)
            .and_then(|id| self.rules_by_id.get(id))
    }

    /// Get a rule by ID
    pub fn get_by_id(&self, id: RuleId) -> Option<&Box<dyn Rule>> {
        self.rules_by_id.get(&id.0)
    }

    /// Check if a rule is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        self.rules_by_name
            .get(name)
            .map(|id| self.enabled.contains_key(id))
            .unwrap_or(false)
    }

    /// Get all registered rule names
    pub fn rule_names(&self) -> impl Iterator<Item = &str> {
        self.rules_by_name.keys().map(|s| s.as_str())
    }

    /// Get count of registered rules
    pub fn len(&self) -> usize {
        self.rules_by_id.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.rules_by_id.is_empty()
    }

    /// Get count of enabled rules
    pub fn enabled_count(&self) -> usize {
        self.enabled.len()
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_with_builtins() {
        let registry = RuleRegistry::with_builtins();
        assert!(!registry.is_empty());
        assert!(registry.get("no-console").is_some());
        assert!(registry.get("no-debugger").is_some());
    }

    #[test]
    fn test_enable_disable() {
        let mut registry = RuleRegistry::with_builtins();
        registry.enable("no-console", Severity::Error);
        assert!(registry.is_enabled("no-console"));

        registry.disable("no-console");
        assert!(!registry.is_enabled("no-console"));
    }
}
