//! Structure validator (stub for future implementation)

use crate::models::{CrateInfo, Violation};

/// Validator for directory structure
pub struct StructureValidator;

impl StructureValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn validate(&self, _crate_info: &CrateInfo) -> Vec<Violation> {
        // TODO: Implement in Task 9
        Vec::new()
    }
}

impl Default for StructureValidator {
    fn default() -> Self {
        Self::new()
    }
}
