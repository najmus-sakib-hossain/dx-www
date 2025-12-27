//! Dependency validator (stub for future implementation)

use crate::models::{CrateInfo, Violation};

/// Validator for dependency consistency
pub struct DependencyValidator;

impl DependencyValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn validate(&self, _crate_info: &CrateInfo) -> Vec<Violation> {
        // TODO: Implement in Task 10
        Vec::new()
    }
}

impl Default for DependencyValidator {
    fn default() -> Self {
        Self::new()
    }
}
