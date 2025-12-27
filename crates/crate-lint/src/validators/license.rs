//! License validator for crate license files and Cargo.toml consistency

use crate::models::{
    CrateInfo, Violation, ViolationCategory, Severity, Fix, FileChange, ChangeOperation,
};

/// Expected license identifier
const EXPECTED_LICENSE: &str = "MIT OR Apache-2.0";

/// MIT license header pattern for detection
const MIT_LICENSE_PATTERNS: &[&str] = &[
    "mit license",
    "permission is hereby granted, free of charge",
    "the mit license",
];

/// Apache license header pattern for detection
const APACHE_LICENSE_PATTERNS: &[&str] = &[
    "apache license",
    "version 2.0",
    "apache-2.0",
];

/// Validator for license files
pub struct LicenseValidator {
    expected_license: &'static str,
}

impl LicenseValidator {
    /// Create a new license validator
    pub fn new() -> Self {
        Self {
            expected_license: EXPECTED_LICENSE,
        }
    }
    
    /// Validate a crate's license
    pub fn validate(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check license file existence
        violations.extend(self.validate_license_file_exists(crate_info));
        
        // Check license content if file exists
        violations.extend(self.validate_license_content(crate_info));
        
        // Check Cargo.toml license field consistency
        violations.extend(self.validate_cargo_toml_license(crate_info));
        
        violations
    }
    
    /// Validate that a LICENSE file exists
    pub fn validate_license_file_exists(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        let has_license = crate_info.has_file("LICENSE");
        let has_license_mit = crate_info.has_file("LICENSE-MIT");
        let has_license_apache = crate_info.has_file("LICENSE-APACHE");
        
        // Either LICENSE or both LICENSE-MIT and LICENSE-APACHE should exist
        if !has_license && !(has_license_mit && has_license_apache) {
            let fix = Fix::new("Create LICENSE file with MIT OR Apache-2.0 dual license", true)
                .with_change(FileChange::new(
                    crate_info.path.join("LICENSE"),
                    ChangeOperation::Create {
                        content: self.generate_dual_license_template(),
                    },
                ));
            
            violations.push(
                Violation::new(
                    "license-file-missing",
                    &crate_info.name,
                    ViolationCategory::License,
                    Severity::Error,
                    "Crate must have a LICENSE file or LICENSE-MIT and LICENSE-APACHE files",
                )
                .with_file(crate_info.path.join("LICENSE"))
                .with_fix(fix),
            );
        }
        
        // If only one of MIT/Apache exists, warn about incomplete dual license
        if (has_license_mit && !has_license_apache) || (!has_license_mit && has_license_apache) {
            let missing = if has_license_mit { "LICENSE-APACHE" } else { "LICENSE-MIT" };
            violations.push(
                Violation::new(
                    "license-dual-incomplete",
                    &crate_info.name,
                    ViolationCategory::License,
                    Severity::Warning,
                    format!("Dual license setup incomplete: missing {} file", missing),
                )
                .with_file(crate_info.path.join(missing)),
            );
        }
        
        violations
    }
    
    /// Validate license file content matches expected license
    pub fn validate_license_content(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check main LICENSE file
        let license_path = crate_info.path.join("LICENSE");
        if crate_info.has_file("LICENSE") {
            if let Ok(content) = std::fs::read_to_string(&license_path) {
                violations.extend(self.check_license_content(crate_info, &content, "LICENSE"));
            }
        }
        
        // Check LICENSE-MIT if exists
        let mit_path = crate_info.path.join("LICENSE-MIT");
        if crate_info.has_file("LICENSE-MIT") {
            if let Ok(content) = std::fs::read_to_string(&mit_path) {
                if !self.is_mit_license(&content) {
                    violations.push(
                        Violation::new(
                            "license-mit-invalid",
                            &crate_info.name,
                            ViolationCategory::License,
                            Severity::Error,
                            "LICENSE-MIT file does not contain valid MIT license text",
                        )
                        .with_file(&mit_path),
                    );
                }
            }
        }
        
        // Check LICENSE-APACHE if exists
        let apache_path = crate_info.path.join("LICENSE-APACHE");
        if crate_info.has_file("LICENSE-APACHE") {
            if let Ok(content) = std::fs::read_to_string(&apache_path) {
                if !self.is_apache_license(&content) {
                    violations.push(
                        Violation::new(
                            "license-apache-invalid",
                            &crate_info.name,
                            ViolationCategory::License,
                            Severity::Error,
                            "LICENSE-APACHE file does not contain valid Apache 2.0 license text",
                        )
                        .with_file(&apache_path),
                    );
                }
            }
        }
        
        violations
    }
    
    /// Check license content for a specific file
    fn check_license_content(&self, crate_info: &CrateInfo, content: &str, filename: &str) -> Vec<Violation> {
        let mut violations = Vec::new();
        let file_path = crate_info.path.join(filename);
        
        let has_mit = self.is_mit_license(content);
        let has_apache = self.is_apache_license(content);
        
        // For dual license, we expect both MIT and Apache references
        if !has_mit && !has_apache {
            violations.push(
                Violation::new(
                    "license-content-invalid",
                    &crate_info.name,
                    ViolationCategory::License,
                    Severity::Error,
                    format!("{} does not contain recognizable MIT or Apache 2.0 license text", filename),
                )
                .with_file(&file_path),
            );
        }
        
        violations
    }
    
    /// Validate Cargo.toml license field matches LICENSE file
    pub fn validate_cargo_toml_license(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let package = crate_info.package();
        let cargo_path = crate_info.cargo_toml_path();
        
        // Check if license field uses workspace inheritance
        if package.license.is_workspace() {
            // Workspace inheritance is fine, no need to check further
            return violations;
        }
        
        // Check if license field is set
        if let Some(license) = package.license.as_literal() {
            // Normalize license string for comparison
            let normalized = license.to_uppercase().replace(' ', "");
            let expected_normalized = self.expected_license.to_uppercase().replace(' ', "");
            
            if normalized != expected_normalized && normalized != "MIT" && normalized != "APACHE-2.0" {
                violations.push(
                    Violation::new(
                        "license-cargo-mismatch",
                        &crate_info.name,
                        ViolationCategory::License,
                        Severity::Warning,
                        format!(
                            "Cargo.toml license '{}' should be '{}' for consistency",
                            license, self.expected_license
                        ),
                    )
                    .with_file(&cargo_path)
                    .with_fix(Fix::new(
                        format!("Change license to '{}'", self.expected_license),
                        true,
                    )),
                );
            }
        }
        
        violations
    }
    
    /// Check if content contains MIT license text
    pub fn is_mit_license(&self, content: &str) -> bool {
        let content_lower = content.to_lowercase();
        MIT_LICENSE_PATTERNS.iter().any(|p| content_lower.contains(p))
    }
    
    /// Check if content contains Apache license text
    pub fn is_apache_license(&self, content: &str) -> bool {
        let content_lower = content.to_lowercase();
        APACHE_LICENSE_PATTERNS.iter().any(|p| content_lower.contains(p))
    }
    
    /// Generate a dual license template
    fn generate_dual_license_template(&self) -> String {
        r#"Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
"#.to_string()
    }
}

impl Default for LicenseValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CargoToml, CrateType};
    use std::path::PathBuf;

    fn create_test_crate(name: &str, toml_content: &str, files: Vec<&str>) -> CrateInfo {
        let cargo_toml = CargoToml::parse(toml_content).unwrap();
        CrateInfo::new(
            name.to_string(),
            PathBuf::from(format!("crates/{}", name)),
            cargo_toml,
            CrateType::Library,
        ).with_files(files.into_iter().map(PathBuf::from).collect())
    }

    #[test]
    fn test_missing_license_file() {
        let validator = LicenseValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"
"#,
            vec!["Cargo.toml", "src/lib.rs"],
        );
        
        let violations = validator.validate_license_file_exists(&crate_info);
        assert!(violations.iter().any(|v| v.id == "license-file-missing"));
    }

    #[test]
    fn test_license_file_present() {
        let validator = LicenseValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"
"#,
            vec!["Cargo.toml", "src/lib.rs", "LICENSE"],
        );
        
        let violations = validator.validate_license_file_exists(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_dual_license_files() {
        let validator = LicenseValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"
"#,
            vec!["Cargo.toml", "src/lib.rs", "LICENSE-MIT", "LICENSE-APACHE"],
        );
        
        let violations = validator.validate_license_file_exists(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_incomplete_dual_license() {
        let validator = LicenseValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"
"#,
            vec!["Cargo.toml", "src/lib.rs", "LICENSE-MIT"],
        );
        
        let violations = validator.validate_license_file_exists(&crate_info);
        assert!(violations.iter().any(|v| v.id == "license-dual-incomplete"));
    }

    #[test]
    fn test_mit_license_detection() {
        let validator = LicenseValidator::new();
        
        let mit_content = r#"
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction...
"#;
        
        assert!(validator.is_mit_license(mit_content));
        assert!(!validator.is_apache_license(mit_content));
    }

    #[test]
    fn test_apache_license_detection() {
        let validator = LicenseValidator::new();
        
        let apache_content = r#"
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
"#;
        
        assert!(validator.is_apache_license(apache_content));
        assert!(!validator.is_mit_license(apache_content));
    }

    #[test]
    fn test_cargo_toml_license_workspace() {
        let validator = LicenseValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"
license.workspace = true
"#,
            vec!["Cargo.toml", "LICENSE"],
        );
        
        let violations = validator.validate_cargo_toml_license(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_cargo_toml_license_mismatch() {
        let validator = LicenseValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"
license = "GPL-3.0"
"#,
            vec!["Cargo.toml", "LICENSE"],
        );
        
        let violations = validator.validate_cargo_toml_license(&crate_info);
        assert!(violations.iter().any(|v| v.id == "license-cargo-mismatch"));
    }

    #[test]
    fn test_cargo_toml_license_correct() {
        let validator = LicenseValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"
license = "MIT OR Apache-2.0"
"#,
            vec!["Cargo.toml", "LICENSE"],
        );
        
        let violations = validator.validate_cargo_toml_license(&crate_info);
        assert!(violations.is_empty());
    }
}
