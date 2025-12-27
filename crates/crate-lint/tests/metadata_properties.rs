//! Property-based tests for metadata validation
//!
//! **Feature: crates-professionalization-v2, Property 1: Workspace Inheritance Consistency**
//! For any Cargo.toml, workspace fields are correctly detected
//! **Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5**
//!
//! **Feature: crates-professionalization-v2, Property 9: Keywords and Categories Validity**
//! For any crate, keywords count is 1-5 and categories are valid
//! **Validates: Requirements 1.6, 1.7, 1.8**

use dx_crate_lint::models::{CargoToml, CrateInfo, CrateType};
use dx_crate_lint::validators::MetadataValidator;
use proptest::prelude::*;
use std::path::PathBuf;

/// Strategy for generating package names
fn package_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{2,20}".prop_map(|s| s.to_string())
}

/// Strategy for generating version strings
fn version_strategy() -> impl Strategy<Value = String> {
    (0u8..10, 0u8..20, 0u8..100).prop_map(|(major, minor, patch)| {
        format!("{}.{}.{}", major, minor, patch)
    })
}

/// Strategy for generating keyword lists
fn keywords_strategy() -> impl Strategy<Value = Vec<String>> {
    proptest::collection::vec("[a-z]{3,10}", 0..8)
}

/// Valid crates.io categories for testing
const VALID_CATEGORIES: &[&str] = &[
    "development-tools",
    "command-line-utilities",
    "web-programming",
    "encoding",
    "parser-implementations",
];

/// Strategy for generating category lists
fn categories_strategy() -> impl Strategy<Value = Vec<String>> {
    proptest::collection::vec(
        proptest::sample::select(VALID_CATEGORIES).prop_map(|s| s.to_string()),
        0..3,
    )
}

/// Helper to create a CrateInfo from TOML content
fn create_crate_info(toml_content: &str) -> Option<CrateInfo> {
    let cargo_toml = CargoToml::parse(toml_content).ok()?;
    let name = cargo_toml.package.as_ref()?.name.clone();
    Some(CrateInfo::new(
        name,
        PathBuf::from("crates/test"),
        cargo_toml,
        CrateType::Library,
    ))
}

proptest! {
    /// Property 1: Workspace inheritance is correctly detected
    /// For any Cargo.toml with workspace inheritance, the validator detects it
    #[test]
    fn workspace_inheritance_detected(
        name in package_name_strategy(),
        use_workspace in proptest::bool::ANY,
    ) {
        let toml_content = if use_workspace {
            format!(r#"
[package]
name = "{}"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Test crate"
keywords = ["test"]
categories = ["development-tools"]
"#, name)
        } else {
            format!(r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "Test crate"
keywords = ["test"]
categories = ["development-tools"]
"#, name)
        };
        
        if let Some(crate_info) = create_crate_info(&toml_content) {
            let validator = MetadataValidator::new(None);
            let violations = validator.validate_workspace_inheritance(&crate_info);
            
            if use_workspace {
                // Should have no workspace inheritance violations
                prop_assert!(
                    violations.is_empty(),
                    "Expected no violations for workspace inheritance, got: {:?}",
                    violations
                );
            } else {
                // Should have workspace inheritance violations
                prop_assert!(
                    !violations.is_empty(),
                    "Expected violations for non-workspace inheritance"
                );
            }
        }
    }

    /// Property 9a: Keywords count validation
    /// For any crate, keywords must be 1-5 items
    #[test]
    fn keywords_count_validation(
        name in package_name_strategy(),
        keywords in keywords_strategy(),
    ) {
        let keywords_toml = if keywords.is_empty() {
            String::new()
        } else {
            format!("keywords = {:?}", keywords)
        };
        
        let toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
{}
"#, name, keywords_toml);
        
        if let Some(crate_info) = create_crate_info(&toml_content) {
            let validator = MetadataValidator::new(None);
            let violations = validator.validate_keywords(&crate_info);
            
            let keyword_count = keywords.len();
            
            if keyword_count == 0 {
                // Should have missing keywords violation
                prop_assert!(
                    violations.iter().any(|v| v.id.contains("keywords")),
                    "Expected keywords violation for empty keywords"
                );
            } else if keyword_count > 5 {
                // Should have too many keywords violation
                prop_assert!(
                    violations.iter().any(|v| v.id == "metadata-keywords-too-many"),
                    "Expected too-many-keywords violation for {} keywords",
                    keyword_count
                );
            } else {
                // Should have no keywords violations
                prop_assert!(
                    violations.is_empty(),
                    "Expected no violations for {} keywords, got: {:?}",
                    keyword_count,
                    violations
                );
            }
        }
    }

    /// Property 9b: Categories validation
    /// For any crate, categories must be valid crates.io categories
    #[test]
    fn categories_validation(
        name in package_name_strategy(),
        categories in categories_strategy(),
    ) {
        let categories_toml = if categories.is_empty() {
            String::new()
        } else {
            format!("categories = {:?}", categories)
        };
        
        let toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
{}
"#, name, categories_toml);
        
        if let Some(crate_info) = create_crate_info(&toml_content) {
            let validator = MetadataValidator::new(None);
            let violations = validator.validate_categories(&crate_info);
            
            if categories.is_empty() {
                // Should have missing categories violation
                prop_assert!(
                    violations.iter().any(|v| v.id == "metadata-categories-missing"),
                    "Expected categories-missing violation"
                );
            } else {
                // All categories are valid (from our strategy), so no invalid category violations
                prop_assert!(
                    !violations.iter().any(|v| v.id == "metadata-category-invalid"),
                    "Expected no invalid-category violations for valid categories"
                );
            }
        }
    }

    /// Property: Invalid categories are detected
    #[test]
    fn invalid_categories_detected(
        name in package_name_strategy(),
        invalid_category in "[a-z]{5,15}".prop_filter(
            "Not a valid category",
            |c| !VALID_CATEGORIES.contains(&c.as_str())
        ),
    ) {
        let toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
categories = ["{}"]
"#, name, invalid_category);
        
        if let Some(crate_info) = create_crate_info(&toml_content) {
            let validator = MetadataValidator::new(None);
            let violations = validator.validate_categories(&crate_info);
            
            prop_assert!(
                violations.iter().any(|v| v.id == "metadata-category-invalid"),
                "Expected invalid-category violation for '{}'",
                invalid_category
            );
        }
    }
}

#[test]
fn test_full_metadata_validation() {
    // Test a fully compliant crate
    let toml_content = r#"
[package]
name = "test-crate"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "A test crate for validation"
documentation = "https://docs.rs/test-crate"
homepage = "https://example.com"
keywords = ["test", "validation"]
categories = ["development-tools"]
"#;
    
    let crate_info = create_crate_info(toml_content).unwrap();
    let validator = MetadataValidator::new(None);
    let violations = validator.validate(&crate_info);
    
    assert!(violations.is_empty(), "Expected no violations, got: {:?}", violations);
}

#[test]
fn test_partial_workspace_inheritance() {
    // Test a crate with partial workspace inheritance
    let toml_content = r#"
[package]
name = "test-crate"
version.workspace = true
edition = "2021"
license.workspace = true
description = "A test crate"
keywords = ["test"]
categories = ["development-tools"]
"#;
    
    let crate_info = create_crate_info(toml_content).unwrap();
    let validator = MetadataValidator::new(None);
    let violations = validator.validate_workspace_inheritance(&crate_info);
    
    // Should have violations for edition, authors, repository
    assert!(violations.iter().any(|v| v.id == "metadata-edition-workspace"));
    assert!(violations.iter().any(|v| v.id == "metadata-authors-workspace"));
    assert!(violations.iter().any(|v| v.id == "metadata-repository-workspace"));
    
    // Should NOT have violations for version, license
    assert!(!violations.iter().any(|v| v.id == "metadata-version-workspace"));
    assert!(!violations.iter().any(|v| v.id == "metadata-license-workspace"));
}
