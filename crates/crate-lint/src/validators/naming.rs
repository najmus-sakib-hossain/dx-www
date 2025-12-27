//! Naming convention validator for DX ecosystem crates

use crate::models::{CrateInfo, CrateType, Violation, ViolationCategory, Severity, Fix};
use regex::Regex;
use std::collections::HashSet;

/// Forbidden generic names that must have dx- prefix
const FORBIDDEN_GENERIC_NAMES: &[&str] = &[
    "serializer",
    "workspace",
    "generator",
    "font",
    "icon",
    "style",
    "media",
    "i18n",
    "driven",
    "forge",
    "check",
];

/// Validator for naming conventions
pub struct NamingValidator {
    tool_pattern: Regex,
    www_pattern: Regex,
    js_pattern: Regex,
    lib_name_pattern: Regex,
    forbidden_names: HashSet<&'static str>,
}

impl NamingValidator {
    /// Create a new naming validator
    pub fn new() -> Self {
        Self {
            // dx-{name} pattern for top-level tools
            tool_pattern: Regex::new(r"^dx-[a-z][a-z0-9-]*$").unwrap(),
            // dx-www-{name} pattern for www modules
            www_pattern: Regex::new(r"^dx-www-[a-z][a-z0-9-]*$").unwrap(),
            // dx-js-{name} pattern for javascript modules
            js_pattern: Regex::new(r"^dx-js-[a-z][a-z0-9-]*$").unwrap(),
            // Library names should use underscores
            lib_name_pattern: Regex::new(r"^dx_[a-z][a-z0-9_]*$").unwrap(),
            forbidden_names: FORBIDDEN_GENERIC_NAMES.iter().copied().collect(),
        }
    }
    
    /// Validate a crate's naming conventions
    pub fn validate(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check package name
        violations.extend(self.validate_package_name(crate_info));
        
        // Check library name if present
        violations.extend(self.validate_lib_name(crate_info));
        
        // Check binary names if present
        violations.extend(self.validate_bin_names(crate_info));
        
        // Check for forbidden generic names
        violations.extend(self.validate_forbidden_names(crate_info));
        
        violations
    }
    
    /// Validate package name follows the correct pattern for its type
    pub fn validate_package_name(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let name = &crate_info.package().name;
        
        let (is_valid, expected_pattern) = match crate_info.crate_type {
            CrateType::TopLevelTool => {
                (self.tool_pattern.is_match(name), "dx-{name}")
            }
            CrateType::WwwModule => {
                (self.www_pattern.is_match(name), "dx-www-{name}")
            }
            CrateType::JavaScriptModule => {
                (self.js_pattern.is_match(name), "dx-js-{name}")
            }
            CrateType::Library => {
                // Libraries should also use dx- prefix
                (self.tool_pattern.is_match(name), "dx-{name}")
            }
            CrateType::Nested => {
                // Nested crates have more flexibility
                (true, "")
            }
        };
        
        if !is_valid {
            let suggested = crate_info.expected_package_name();
            violations.push(
                Violation::new(
                    "naming-package-pattern",
                    &crate_info.name,
                    ViolationCategory::Naming,
                    Severity::Error,
                    format!(
                        "Package name '{}' does not follow pattern '{}'. Suggested: '{}'",
                        name, expected_pattern, suggested
                    ),
                )
                .with_file(crate_info.cargo_toml_path())
                .with_fix(Fix::new(
                    format!("Rename package to '{}'", suggested),
                    false, // Renaming requires careful consideration
                )),
            );
        }
        
        violations
    }
    
    /// Validate library name uses underscores
    pub fn validate_lib_name(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        if let Some(lib) = &crate_info.cargo_toml.lib {
            if let Some(lib_name) = &lib.name {
                // Library names should use underscores, not hyphens
                if lib_name.contains('-') {
                    let suggested = lib_name.replace('-', "_");
                    violations.push(
                        Violation::new(
                            "naming-lib-underscore",
                            &crate_info.name,
                            ViolationCategory::Naming,
                            Severity::Error,
                            format!(
                                "Library name '{}' should use underscores, not hyphens. Suggested: '{}'",
                                lib_name, suggested
                            ),
                        )
                        .with_file(crate_info.cargo_toml_path())
                        .with_fix(Fix::new(
                            format!("Change lib name to '{}'", suggested),
                            true,
                        )),
                    );
                }
                
                // Library names should have dx_ prefix
                if !lib_name.starts_with("dx_") && crate_info.crate_type != CrateType::Nested {
                    let suggested = crate_info.expected_lib_name();
                    violations.push(
                        Violation::new(
                            "naming-lib-prefix",
                            &crate_info.name,
                            ViolationCategory::Naming,
                            Severity::Warning,
                            format!(
                                "Library name '{}' should have 'dx_' prefix. Suggested: '{}'",
                                lib_name, suggested
                            ),
                        )
                        .with_file(crate_info.cargo_toml_path())
                        .with_fix(Fix::new(
                            format!("Change lib name to '{}'", suggested),
                            false,
                        )),
                    );
                }
            }
        }
        
        violations
    }
    
    /// Validate binary names
    pub fn validate_bin_names(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        for bin in &crate_info.cargo_toml.bins {
            // Binary names should match package name pattern
            if crate_info.crate_type == CrateType::TopLevelTool {
                if !self.tool_pattern.is_match(&bin.name) {
                    violations.push(
                        Violation::new(
                            "naming-bin-pattern",
                            &crate_info.name,
                            ViolationCategory::Naming,
                            Severity::Warning,
                            format!(
                                "Binary name '{}' should follow 'dx-{{name}}' pattern",
                                bin.name
                            ),
                        )
                        .with_file(crate_info.cargo_toml_path()),
                    );
                }
            }
        }
        
        violations
    }
    
    /// Check for forbidden generic names
    pub fn validate_forbidden_names(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let name = &crate_info.package().name;
        
        // Check if the name is a forbidden generic name without prefix
        if self.forbidden_names.contains(name.as_str()) {
            let suggested = format!("dx-{}", name);
            violations.push(
                Violation::new(
                    "naming-generic-forbidden",
                    &crate_info.name,
                    ViolationCategory::Naming,
                    Severity::Error,
                    format!(
                        "Package name '{}' is too generic and must have 'dx-' prefix. Suggested: '{}'",
                        name, suggested
                    ),
                )
                .with_file(crate_info.cargo_toml_path())
                .with_fix(Fix::new(
                    format!("Rename package to '{}'", suggested),
                    false,
                )),
            );
        }
        
        violations
    }
    
    /// Suggest a compliant name for a crate
    pub fn suggest_name(&self, crate_info: &CrateInfo) -> String {
        crate_info.expected_package_name()
    }
    
    /// Check if a package name is valid for a given crate type
    pub fn is_valid_name(&self, name: &str, crate_type: CrateType) -> bool {
        match crate_type {
            CrateType::TopLevelTool | CrateType::Library => self.tool_pattern.is_match(name),
            CrateType::WwwModule => self.www_pattern.is_match(name),
            CrateType::JavaScriptModule => self.js_pattern.is_match(name),
            CrateType::Nested => true,
        }
    }
}

impl Default for NamingValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CargoToml;
    use std::path::PathBuf;

    fn create_test_crate(name: &str, crate_type: CrateType) -> CrateInfo {
        let toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
"#, name);
        let cargo_toml = CargoToml::parse(&toml_content).unwrap();
        CrateInfo::new(
            name.to_string(),
            PathBuf::from(format!("crates/{}", name)),
            cargo_toml,
            crate_type,
        )
    }

    #[test]
    fn test_valid_tool_names() {
        let validator = NamingValidator::new();
        
        assert!(validator.is_valid_name("dx-cli", CrateType::TopLevelTool));
        assert!(validator.is_valid_name("dx-crate-lint", CrateType::TopLevelTool));
        assert!(validator.is_valid_name("dx-forge", CrateType::TopLevelTool));
        
        assert!(!validator.is_valid_name("cli", CrateType::TopLevelTool));
        assert!(!validator.is_valid_name("crate-lint", CrateType::TopLevelTool));
    }

    #[test]
    fn test_valid_www_names() {
        let validator = NamingValidator::new();
        
        assert!(validator.is_valid_name("dx-www-core", CrateType::WwwModule));
        assert!(validator.is_valid_name("dx-www-dom", CrateType::WwwModule));
        
        assert!(!validator.is_valid_name("dx-core", CrateType::WwwModule));
        assert!(!validator.is_valid_name("www-core", CrateType::WwwModule));
    }

    #[test]
    fn test_forbidden_generic_names() {
        let validator = NamingValidator::new();
        
        let crate_info = create_test_crate("serializer", CrateType::Library);
        let violations = validator.validate_forbidden_names(&crate_info);
        
        assert!(!violations.is_empty());
        assert!(violations.iter().any(|v| v.id == "naming-generic-forbidden"));
    }

    #[test]
    fn test_package_name_validation() {
        let validator = NamingValidator::new();
        
        // Valid name
        let crate_info = create_test_crate("dx-serializer", CrateType::Library);
        let violations = validator.validate_package_name(&crate_info);
        assert!(violations.is_empty());
        
        // Invalid name
        let crate_info = create_test_crate("serializer", CrateType::Library);
        let violations = validator.validate_package_name(&crate_info);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_lib_name_validation() {
        let validator = NamingValidator::new();
        
        // Test lib name with hyphens
        let toml_content = r#"
[package]
name = "dx-test"
version = "0.1.0"

[lib]
name = "dx-test"
"#;
        let cargo_toml = CargoToml::parse(toml_content).unwrap();
        let crate_info = CrateInfo::new(
            "dx-test".to_string(),
            PathBuf::from("crates/test"),
            cargo_toml,
            CrateType::Library,
        );
        
        let violations = validator.validate_lib_name(&crate_info);
        assert!(violations.iter().any(|v| v.id == "naming-lib-underscore"));
    }
}
