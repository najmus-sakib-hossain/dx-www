//! Documentation validator (stub for future implementation)

use crate::models::{CrateInfo, Violation};

/// Validator for documentation files
pub struct DocumentationValidator;

impl DocumentationValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn validate(&self, _crate_info: &CrateInfo) -> Vec<Violation> {
        // TODO: Implement in Task 7
        Vec::new()
    }
}

impl Default for DocumentationValidator {
    fn default() -> Self {
        Self::new()
    }
}
