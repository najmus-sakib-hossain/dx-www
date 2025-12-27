//! License validator (stub for future implementation)

use crate::models::{CrateInfo, Violation};

/// Validator for license files
pub struct LicenseValidator;

impl LicenseValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn validate(&self, _crate_info: &CrateInfo) -> Vec<Violation> {
        // TODO: Implement in Task 8
        Vec::new()
    }
}

impl Default for LicenseValidator {
    fn default() -> Self {
        Self::new()
    }
}
