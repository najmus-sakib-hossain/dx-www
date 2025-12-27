//! Report generator (stub for future implementation)

use crate::models::ValidationReport;

/// Generator for validation reports
pub struct ReportGenerator;

impl ReportGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn to_json(&self, report: &ValidationReport) -> String {
        serde_json::to_string_pretty(report).unwrap_or_default()
    }
    
    pub fn to_terminal(&self, report: &ValidationReport) -> String {
        let mut output = String::new();
        
        output.push_str(&format!(
            "\n=== Validation Report ===\n\n\
             Total crates: {}\n\
             Compliant: {}\n\
             Violations: {}\n\
             Critical: {}\n\
             Auto-fixable: {}\n\n",
            report.summary.total_crates,
            report.summary.compliant_crates,
            report.summary.total_violations,
            report.summary.critical_violations,
            report.summary.auto_fixable,
        ));
        
        for violation in &report.violations {
            output.push_str(&format!(
                "[{}] {}: {}\n",
                violation.severity.display_name(),
                violation.crate_name,
                violation.message
            ));
        }
        
        output
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}
