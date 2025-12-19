//! Stack registry: manages language stacks
//!
//! The registry provides a centralized way to access language-specific stacks.

use crate::LanguageStack;
use crate::error::{StackError, StackResult};
use crate::language::Language;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

/// Global stack registry
static REGISTRY: OnceLock<StackRegistry> = OnceLock::new();

/// Registry of all available language stacks
pub struct StackRegistry {
    stacks: HashMap<Language, Arc<dyn LanguageStack>>,
}

impl StackRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            stacks: HashMap::new(),
        }
    }

    /// Create a registry with default stacks
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Register JavaScript/TypeScript stack
        let js_stack = Arc::new(crate::languages::javascript::JavaScriptStack::new());
        registry.register(Language::JavaScript, js_stack.clone());
        registry.register(Language::TypeScript, js_stack);

        registry
    }

    /// Register a language stack
    pub fn register(&mut self, language: Language, stack: Arc<dyn LanguageStack>) {
        self.stacks.insert(language, stack);
    }

    /// Get a stack for a language
    pub fn get_stack(&self, language: Language) -> StackResult<Arc<dyn LanguageStack>> {
        // First check if the language even needs a stack
        if !language.needs_stack() {
            return Err(StackError::NoStackRequired(language));
        }

        self.stacks.get(&language).cloned().ok_or_else(|| {
            StackError::NotSupported(format!("{} stack not yet implemented", language))
        })
    }

    /// Check if a stack is available for a language
    pub fn has_stack(&self, language: Language) -> bool {
        language.needs_stack() && self.stacks.contains_key(&language)
    }

    /// Get all registered languages
    pub fn languages(&self) -> Vec<Language> {
        self.stacks.keys().copied().collect()
    }

    /// Get the global registry instance
    pub fn global() -> &'static StackRegistry {
        REGISTRY.get_or_init(StackRegistry::with_defaults)
    }

    /// Get a stack from the global registry
    pub fn get(language: Language) -> StackResult<Arc<dyn LanguageStack>> {
        Self::global().get_stack(language)
    }
}

impl Default for StackRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = StackRegistry::with_defaults();
        assert!(registry.has_stack(Language::JavaScript));
        assert!(registry.has_stack(Language::TypeScript));
    }

    #[test]
    fn test_rust_no_stack() {
        let registry = StackRegistry::with_defaults();
        let result = registry.get_stack(Language::Rust);
        assert!(matches!(result, Err(StackError::NoStackRequired(_))));
    }

    #[test]
    fn test_go_no_stack() {
        let registry = StackRegistry::with_defaults();
        let result = registry.get_stack(Language::Go);
        assert!(matches!(result, Err(StackError::NoStackRequired(_))));
    }
}
