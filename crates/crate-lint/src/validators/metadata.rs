//! Metadata validator for Cargo.toml compliance

use crate::models::{
    CrateInfo, Violation, ViolationCategory, Severity, Fix, FileChange, ChangeOperation,
    WorkspacePackage,
};
use std::collections::HashSet;

/// Valid crates.io categories
const VALID_CATEGORIES: &[&str] = &[
    "accessibility",
    "aerospace",
    "algorithms",
    "api-bindings",
    "asynchronous",
    "authentication",
    "caching",
    "command-line-interface",
    "command-line-utilities",
    "compilers",
    "compression",
    "computer-vision",
    "concurrency",
    "config",
    "cryptography",
    "data-structures",
    "database",
    "database-implementations",
    "date-and-time",
    "development-tools",
    "email",
    "embedded",
    "emulators",
    "encoding",
    "external-ffi-bindings",
    "filesystem",
    "finance",
    "game-development",
    "game-engines",
    "games",
    "graphics",
    "gui",
    "hardware-support",
    "internationalization",
    "localization",
    "mathematics",
    "memory-management",
    "multimedia",
    "network-programming",
    "no-std",
    "os",
    "parser-implementations",
    "parsing",
    "rendering",
    "rust-patterns",
    "science",
    "simulation",
    "template-engine",
    "text-editors",
    "text-processing",
    "value-formatting",
    "virtualization",
    "visualization",
    "wasm",
    "web-programming",
    "code-generators",
];

/// Validator for Cargo.toml metadata
pub struct MetadataValidator {
    workspace_package: Option<WorkspacePackage>,
    valid_categories: HashSet<&'static str>,
}

impl MetadataValidator {
    /// Create a new metadata validator
    pub fn new(workspace_package: Option<&WorkspacePackage>) -> Self {
        Self {
            workspace_package: workspace_package.cloned(),
            valid_categories: VALID_CATEGORIES.iter().copied().collect(),
        }
    }
    
    /// Validate a crate's metadata
    pub fn validate(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check workspace inheritance
        violations.extend(self.validate_workspace_inheritance(crate_info));
        
        // Check required fields
        violations.extend(self.validate_required_fields(crate_info));
        
        // Check keywords
        violations.extend(self.validate_keywords(crate_info));
        
        // Check categories
        violations.extend(self.validate_categories(crate_info));
        
        violations
    }
    
    /// Validate workspace inheritance fields
    pub fn validate_workspace_inheritance(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let package = crate_info.package();
        let cargo_path = crate_info.cargo_toml_path();
        
        // Check version.workspace
        if !package.version.is_workspace() {
            let fix = Fix::new("Use version.workspace = true", true)
                .with_change(FileChange::new(
                    &cargo_path,
                    ChangeOperation::Modify {
                        old: format!("version = {:?}", package.version.as_literal().unwrap_or("0.1.0")),
                        new: "version.workspace = true".to_string(),
                    },
                ));
            
            violations.push(
                Violation::new(
                    "metadata-version-workspace",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Error,
                    "Package should use version.workspace = true for workspace inheritance",
                )
                .with_file(&cargo_path)
                .with_fix(fix),
            );
        }
        
        // Check edition.workspace
        if !package.edition.is_workspace() {
            violations.push(
                Violation::new(
                    "metadata-edition-workspace",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Error,
                    "Package should use edition.workspace = true for workspace inheritance",
                )
                .with_file(&cargo_path)
                .with_fix(Fix::new("Use edition.workspace = true", true)),
            );
        }
        
        // Check authors.workspace
        if !package.authors.is_workspace() {
            violations.push(
                Violation::new(
                    "metadata-authors-workspace",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Error,
                    "Package should use authors.workspace = true for workspace inheritance",
                )
                .with_file(&cargo_path)
                .with_fix(Fix::new("Use authors.workspace = true", true)),
            );
        }
        
        // Check license.workspace
        if !package.license.is_workspace() {
            violations.push(
                Violation::new(
                    "metadata-license-workspace",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Error,
                    "Package should use license.workspace = true for workspace inheritance",
                )
                .with_file(&cargo_path)
                .with_fix(Fix::new("Use license.workspace = true", true)),
            );
        }
        
        // Check repository.workspace
        if !package.repository.is_workspace() {
            violations.push(
                Violation::new(
                    "metadata-repository-workspace",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Error,
                    "Package should use repository.workspace = true for workspace inheritance",
                )
                .with_file(&cargo_path)
                .with_fix(Fix::new("Use repository.workspace = true", true)),
            );
        }
        
        violations
    }
    
    /// Validate required fields
    pub fn validate_required_fields(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let package = crate_info.package();
        let cargo_path = crate_info.cargo_toml_path();
        
        // Check description
        if package.description.is_none() {
            violations.push(
                Violation::new(
                    "metadata-description-missing",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Error,
                    "Package must have a description field",
                )
                .with_file(&cargo_path)
                .with_fix(Fix::new("Add description field", true)),
            );
        } else if let Some(desc) = &package.description {
            if desc.len() < 10 {
                violations.push(
                    Violation::new(
                        "metadata-description-short",
                        &crate_info.name,
                        ViolationCategory::Metadata,
                        Severity::Warning,
                        "Package description should be at least 10 characters",
                    )
                    .with_file(&cargo_path),
                );
            }
        }
        
        // Check documentation
        if package.documentation.is_none() {
            violations.push(
                Violation::new(
                    "metadata-documentation-missing",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Warning,
                    "Package should have a documentation field pointing to docs.rs",
                )
                .with_file(&cargo_path)
                .with_fix(Fix::new("Add documentation = \"https://docs.rs/crate-name\"", true)),
            );
        }
        
        // Check homepage
        if package.homepage.is_none() {
            violations.push(
                Violation::new(
                    "metadata-homepage-missing",
                    &crate_info.name,
                    ViolationCategory::Metadata,
                    Severity::Warning,
                    "Package should have a homepage field",
                )
                .with_file(&cargo_path)
                .with_fix(Fix::new("Add homepage field", true)),
            );
        }
        
        violations
    }
    
    /// Validate keywords
    pub fn validate_keywords(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let package = crate_info.package();
        let cargo_path = crate_info.cargo_toml_path();
        
        match &package.keywords {
            None => {
                violations.push(
                    Violation::new(
                        "metadata-keywords-missing",
                        &crate_info.name,
                        ViolationCategory::Metadata,
                        Severity::Error,
                        "Package must have keywords field with 1-5 terms",
                    )
                    .with_file(&cargo_path)
                    .with_fix(Fix::new("Add keywords = [\"keyword1\", \"keyword2\"]", true)),
                );
            }
            Some(keywords) => {
                if keywords.is_empty() {
                    violations.push(
                        Violation::new(
                            "metadata-keywords-empty",
                            &crate_info.name,
                            ViolationCategory::Metadata,
                            Severity::Error,
                            "Package keywords must have at least 1 term",
                        )
                        .with_file(&cargo_path),
                    );
                } else if keywords.len() > 5 {
                    violations.push(
                        Violation::new(
                            "metadata-keywords-too-many",
                            &crate_info.name,
                            ViolationCategory::Metadata,
                            Severity::Error,
                            format!("Package keywords must have at most 5 terms (found {})", keywords.len()),
                        )
                        .with_file(&cargo_path),
                    );
                }
            }
        }
        
        violations
    }
    
    /// Validate categories
    pub fn validate_categories(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let package = crate_info.package();
        let cargo_path = crate_info.cargo_toml_path();
        
        match &package.categories {
            None => {
                violations.push(
                    Violation::new(
                        "metadata-categories-missing",
                        &crate_info.name,
                        ViolationCategory::Metadata,
                        Severity::Error,
                        "Package must have categories field",
                    )
                    .with_file(&cargo_path)
                    .with_fix(Fix::new("Add categories = [\"development-tools\"]", true)),
                );
            }
            Some(categories) => {
                for category in categories {
                    if !self.valid_categories.contains(category.as_str()) {
                        violations.push(
                            Violation::new(
                                "metadata-category-invalid",
                                &crate_info.name,
                                ViolationCategory::Metadata,
                                Severity::Error,
                                format!("Invalid category '{}'. Must be a valid crates.io category", category),
                            )
                            .with_file(&cargo_path),
                        );
                    }
                }
            }
        }
        
        violations
    }
    
    /// Check if a category is valid
    pub fn is_valid_category(&self, category: &str) -> bool {
        self.valid_categories.contains(category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CargoToml, CrateType};
    use std::path::PathBuf;

    fn create_test_crate(toml_content: &str) -> CrateInfo {
        let cargo_toml = CargoToml::parse(toml_content).unwrap();
        let name = cargo_toml.package.as_ref().unwrap().name.clone();
        CrateInfo::new(
            name,
            PathBuf::from("crates/test"),
            cargo_toml,
            CrateType::Library,
        )
    }

    #[test]
    fn test_workspace_inheritance_validation() {
        let validator = MetadataValidator::new(None);
        
        // Test crate without workspace inheritance
        let crate_info = create_test_crate(r#"
[package]
name = "test-crate"
version = "0.1.0"
edition = "2021"
"#);
        
        let violations = validator.validate_workspace_inheritance(&crate_info);
        assert!(!violations.is_empty());
        assert!(violations.iter().any(|v| v.id == "metadata-version-workspace"));
        assert!(violations.iter().any(|v| v.id == "metadata-edition-workspace"));
    }

    #[test]
    fn test_workspace_inheritance_compliant() {
        let validator = MetadataValidator::new(None);
        
        // Test crate with workspace inheritance
        let crate_info = create_test_crate(r#"
[package]
name = "test-crate"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
"#);
        
        let violations = validator.validate_workspace_inheritance(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_keywords_validation() {
        let validator = MetadataValidator::new(None);
        
        // Test missing keywords
        let crate_info = create_test_crate(r#"
[package]
name = "test-crate"
version = "0.1.0"
"#);
        
        let violations = validator.validate_keywords(&crate_info);
        assert!(violations.iter().any(|v| v.id == "metadata-keywords-missing"));
        
        // Test too many keywords
        let crate_info = create_test_crate(r#"
[package]
name = "test-crate"
version = "0.1.0"
keywords = ["a", "b", "c", "d", "e", "f"]
"#);
        
        let violations = validator.validate_keywords(&crate_info);
        assert!(violations.iter().any(|v| v.id == "metadata-keywords-too-many"));
    }

    #[test]
    fn test_categories_validation() {
        let validator = MetadataValidator::new(None);
        
        // Test invalid category
        let crate_info = create_test_crate(r#"
[package]
name = "test-crate"
version = "0.1.0"
categories = ["invalid-category"]
"#);
        
        let violations = validator.validate_categories(&crate_info);
        assert!(violations.iter().any(|v| v.id == "metadata-category-invalid"));
        
        // Test valid category
        let crate_info = create_test_crate(r#"
[package]
name = "test-crate"
version = "0.1.0"
categories = ["development-tools"]
"#);
        
        let violations = validator.validate_categories(&crate_info);
        assert!(violations.is_empty());
    }
}
