//! Property-based tests for crates.io publication readiness
//!
//! **Feature: crates-professionalization-v2, Property 8: Crates.io Publication Readiness**
//! For any publishable crate, all required fields exist
//! **Validates: Requirements 8.1, 8.2, 8.3**

use dx_crate_lint::models::{CargoToml, CrateInfo, CrateType};
use dx_crate_lint::validators::MetadataValidator;
use proptest::prelude::*;
use std::path::PathBuf;

/// Strategy for generating package names
fn package_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{2,20}".prop_map(|s| s.to_string())
}

/// Strategy for generating descriptions
fn description_strategy() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        Just(None),
        Just(Some("A short description".to_string())),
        Just(Some("A longer description that provides more context about what this crate does".to_string())),
    ]
}

/// Strategy for generating documentation URLs
fn documentation_strategy() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        Just(None),
        Just(Some("https://docs.rs/test-crate".to_string())),
    ]
}

/// Strategy for generating homepage URLs
fn homepage_strategy() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        Just(None),
        Just(Some("https://example.com".to_string())),
    ]
}

/// Helper to create a CrateInfo with specific fields
fn create_crate_info(
    name: &str,
    description: Option<&str>,
    documentation: Option<&str>,
    homepage: Option<&str>,
    has_keywords: bool,
    has_categories: bool,
) -> CrateInfo {
    let desc_line = description.map(|d| format!("description = \"{}\"", d)).unwrap_or_default();
    let doc_line = documentation.map(|d| format!("documentation = \"{}\"", d)).unwrap_or_default();
    let home_line = homepage.map(|h| format!("homepage = \"{}\"", h)).unwrap_or_default();
    let keywords_line = if has_keywords { "keywords = [\"test\", \"validation\"]" } else { "" };
    let categories_line = if has_categories { "categories = [\"development-tools\"]" } else { "" };
    
    let toml_content = format!(r#"
[package]
name = "{}"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
{}
{}
{}
{}
{}
"#, name, desc_line, doc_line, home_line, keywords_line, categories_line);
    
    let cargo_toml = CargoToml::parse(&toml_content).unwrap();
    CrateInfo::new(
        name.to_string(),
        PathBuf::from(format!("crates/{}", name)),
        cargo_toml,
        CrateType::Library,
    )
}

proptest! {
    /// Property 8a: Crates with all required fields have no metadata violations
    /// For any crate with description, keywords, and categories, no required field violations
    #[test]
    fn complete_metadata_no_violations(
        name in package_name_strategy(),
    ) {
        let crate_info = create_crate_info(
            &name,
            Some("A test crate for validation"),
            Some("https://docs.rs/test"),
            Some("https://example.com"),
            true,
            true,
        );
        
        let validator = MetadataValidator::new(None);
        let violations = validator.validate_required_fields(&crate_info);
        
        // Should have no violations for required fields
        prop_assert!(
            !violations.iter().any(|v| v.id == "metadata-description-missing"),
            "Should not have description-missing violation"
        );
    }

    /// Property 8b: Missing description generates violation
    /// For any crate without description, a violation is generated
    #[test]
    fn missing_description_violation(
        name in package_name_strategy(),
    ) {
        let crate_info = create_crate_info(
            &name,
            None, // No description
            Some("https://docs.rs/test"),
            Some("https://example.com"),
            true,
            true,
        );
        
        let validator = MetadataValidator::new(None);
        let violations = validator.validate_required_fields(&crate_info);
        
        prop_assert!(
            violations.iter().any(|v| v.id == "metadata-description-missing"),
            "Should have description-missing violation"
        );
    }

    /// Property 8c: Missing keywords generates violation
    /// For any crate without keywords, a violation is generated
    #[test]
    fn missing_keywords_violation(
        name in package_name_strategy(),
    ) {
        let crate_info = create_crate_info(
            &name,
            Some("A test crate"),
            Some("https://docs.rs/test"),
            Some("https://example.com"),
            false, // No keywords
            true,
        );
        
        let validator = MetadataValidator::new(None);
        let violations = validator.validate_keywords(&crate_info);
        
        prop_assert!(
            violations.iter().any(|v| v.id.contains("keywords")),
            "Should have keywords violation"
        );
    }

    /// Property 8d: Missing categories generates violation
    /// For any crate without categories, a violation is generated
    #[test]
    fn missing_categories_violation(
        name in package_name_strategy(),
    ) {
        let crate_info = create_crate_info(
            &name,
            Some("A test crate"),
            Some("https://docs.rs/test"),
            Some("https://example.com"),
            true,
            false, // No categories
        );
        
        let validator = MetadataValidator::new(None);
        let violations = validator.validate_categories(&crate_info);
        
        prop_assert!(
            violations.iter().any(|v| v.id == "metadata-categories-missing"),
            "Should have categories-missing violation"
        );
    }

    /// Property 8e: Missing documentation generates warning
    /// For any crate without documentation URL, a warning is generated
    #[test]
    fn missing_documentation_warning(
        name in package_name_strategy(),
    ) {
        let crate_info = create_crate_info(
            &name,
            Some("A test crate"),
            None, // No documentation
            Some("https://example.com"),
            true,
            true,
        );
        
        let validator = MetadataValidator::new(None);
        let violations = validator.validate_required_fields(&crate_info);
        
        prop_assert!(
            violations.iter().any(|v| v.id == "metadata-documentation-missing"),
            "Should have documentation-missing warning"
        );
    }
}

/// Test complete crates.io readiness
#[test]
fn test_cratesio_ready_crate() {
    let crate_info = create_crate_info(
        "test-crate",
        Some("A comprehensive test crate for validation"),
        Some("https://docs.rs/test-crate"),
        Some("https://example.com/test-crate"),
        true,
        true,
    );
    
    let validator = MetadataValidator::new(None);
    
    // Check all validations
    let required_violations = validator.validate_required_fields(&crate_info);
    let keyword_violations = validator.validate_keywords(&crate_info);
    let category_violations = validator.validate_categories(&crate_info);
    let workspace_violations = validator.validate_workspace_inheritance(&crate_info);
    
    // Should have no critical violations
    assert!(
        !required_violations.iter().any(|v| v.id == "metadata-description-missing"),
        "Should not have description-missing"
    );
    assert!(
        keyword_violations.is_empty(),
        "Should have no keyword violations"
    );
    assert!(
        category_violations.is_empty(),
        "Should have no category violations"
    );
    assert!(
        workspace_violations.is_empty(),
        "Should have no workspace inheritance violations"
    );
}

/// Test that all required crates.io fields are checked
#[test]
fn test_all_cratesio_fields_checked() {
    // Create a minimal crate missing everything
    let toml_content = r#"
[package]
name = "minimal-crate"
version = "0.1.0"
"#;
    
    let cargo_toml = CargoToml::parse(toml_content).unwrap();
    let crate_info = CrateInfo::new(
        "minimal-crate".to_string(),
        PathBuf::from("crates/minimal-crate"),
        cargo_toml,
        CrateType::Library,
    );
    
    let validator = MetadataValidator::new(None);
    let all_violations = validator.validate(&crate_info);
    
    // Should have violations for:
    // - version.workspace
    // - edition.workspace
    // - authors.workspace
    // - license.workspace
    // - repository.workspace
    // - description
    // - keywords
    // - categories
    // - documentation (warning)
    // - homepage (warning)
    
    let violation_ids: Vec<_> = all_violations.iter().map(|v| v.id.as_str()).collect();
    
    assert!(violation_ids.iter().any(|id| id.contains("version")));
    assert!(violation_ids.iter().any(|id| id.contains("edition")));
    assert!(violation_ids.iter().any(|id| id.contains("description")));
    assert!(violation_ids.iter().any(|id| id.contains("keywords")));
    assert!(violation_ids.iter().any(|id| id.contains("categories")));
}

/// Test that workspace inheritance is properly validated
#[test]
fn test_workspace_inheritance_for_publication() {
    let toml_content = r#"
[package]
name = "test-crate"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "A test crate"
keywords = ["test"]
categories = ["development-tools"]
documentation = "https://docs.rs/test-crate"
homepage = "https://example.com"
"#;
    
    let cargo_toml = CargoToml::parse(toml_content).unwrap();
    let crate_info = CrateInfo::new(
        "test-crate".to_string(),
        PathBuf::from("crates/test-crate"),
        cargo_toml,
        CrateType::Library,
    );
    
    let validator = MetadataValidator::new(None);
    let violations = validator.validate(&crate_info);
    
    // Should have no violations
    assert!(
        violations.is_empty(),
        "Fully compliant crate should have no violations, got: {:?}",
        violations
    );
}
