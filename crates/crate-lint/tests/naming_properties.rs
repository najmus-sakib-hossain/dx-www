//! Property-based tests for naming validation
//!
//! **Feature: crates-professionalization-v2, Property 2: Naming Convention Compliance**
//! For any crate, naming follows location-based patterns
//! **Validates: Requirements 2.1, 2.2, 2.3, 2.4, 2.5**

use dx_crate_lint::models::{CargoToml, CrateInfo, CrateType};
use dx_crate_lint::validators::NamingValidator;
use proptest::prelude::*;
use std::path::PathBuf;

/// Strategy for generating valid crate name suffixes
/// Excludes names that already start with dx- to avoid double-prefixing
fn name_suffix_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{1,15}"
        .prop_filter("suffix should not start with dx-", |s| !s.starts_with("dx-"))
        .prop_map(|s| s.to_string())
}

/// Helper to create a CrateInfo
fn create_crate_info(name: &str, crate_type: CrateType) -> CrateInfo {
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

proptest! {
    /// Property: dx-{name} pattern is valid for top-level tools
    /// For any valid suffix, dx-{suffix} should be a valid tool name
    #[test]
    fn dx_prefix_valid_for_tools(suffix in name_suffix_strategy()) {
        let validator = NamingValidator::new();
        let name = format!("dx-{}", suffix);
        
        prop_assert!(
            validator.is_valid_name(&name, CrateType::TopLevelTool),
            "Expected '{}' to be valid for TopLevelTool",
            name
        );
    }

    /// Property: dx-www-{name} pattern is valid for www modules
    /// For any valid suffix, dx-www-{suffix} should be a valid www module name
    #[test]
    fn dx_www_prefix_valid_for_www_modules(suffix in name_suffix_strategy()) {
        let validator = NamingValidator::new();
        let name = format!("dx-www-{}", suffix);
        
        prop_assert!(
            validator.is_valid_name(&name, CrateType::WwwModule),
            "Expected '{}' to be valid for WwwModule",
            name
        );
    }

    /// Property: dx-js-{name} pattern is valid for javascript modules
    /// For any valid suffix, dx-js-{suffix} should be a valid js module name
    #[test]
    fn dx_js_prefix_valid_for_js_modules(suffix in name_suffix_strategy()) {
        let validator = NamingValidator::new();
        let name = format!("dx-js-{}", suffix);
        
        prop_assert!(
            validator.is_valid_name(&name, CrateType::JavaScriptModule),
            "Expected '{}' to be valid for JavaScriptModule",
            name
        );
    }

    /// Property: Names without dx- prefix are invalid for tools
    /// For any name without dx- prefix, it should be invalid for TopLevelTool
    #[test]
    fn no_prefix_invalid_for_tools(suffix in name_suffix_strategy()) {
        let validator = NamingValidator::new();
        
        // Name without prefix
        prop_assert!(
            !validator.is_valid_name(&suffix, CrateType::TopLevelTool),
            "Expected '{}' to be invalid for TopLevelTool (no dx- prefix)",
            suffix
        );
    }

    /// Property: Names without dx-www- prefix are invalid for www modules
    /// For any name without dx-www- prefix, it should be invalid for WwwModule
    #[test]
    fn no_www_prefix_invalid_for_www_modules(suffix in name_suffix_strategy()) {
        let validator = NamingValidator::new();
        
        // Name with only dx- prefix (not dx-www-)
        let name = format!("dx-{}", suffix);
        prop_assert!(
            !validator.is_valid_name(&name, CrateType::WwwModule),
            "Expected '{}' to be invalid for WwwModule (needs dx-www- prefix)",
            name
        );
    }

    /// Property: Validation produces violations for non-compliant names
    /// For any crate with non-compliant name, validation should produce violations
    #[test]
    fn non_compliant_names_produce_violations(suffix in name_suffix_strategy()) {
        let validator = NamingValidator::new();
        
        // Create a crate with non-compliant name (no dx- prefix)
        let crate_info = create_crate_info(&suffix, CrateType::Library);
        let violations = validator.validate_package_name(&crate_info);
        
        prop_assert!(
            !violations.is_empty(),
            "Expected violations for non-compliant name '{}'",
            suffix
        );
    }

    /// Property: Validation produces no violations for compliant names
    /// For any crate with compliant name, validation should produce no violations
    #[test]
    fn compliant_names_produce_no_violations(suffix in name_suffix_strategy()) {
        let validator = NamingValidator::new();
        
        // Create a crate with compliant name
        let name = format!("dx-{}", suffix);
        let crate_info = create_crate_info(&name, CrateType::Library);
        let violations = validator.validate_package_name(&crate_info);
        
        prop_assert!(
            violations.is_empty(),
            "Expected no violations for compliant name '{}', got: {:?}",
            name,
            violations
        );
    }

    /// Property: Suggested names are always compliant
    /// For any crate, the suggested name should be valid for its type
    #[test]
    fn suggested_names_are_compliant(
        suffix in name_suffix_strategy(),
        crate_type in prop_oneof![
            Just(CrateType::TopLevelTool),
            Just(CrateType::Library),
        ]
    ) {
        let validator = NamingValidator::new();
        
        // Create a crate with non-compliant name
        let crate_info = create_crate_info(&suffix, crate_type);
        let suggested = validator.suggest_name(&crate_info);
        
        prop_assert!(
            validator.is_valid_name(&suggested, crate_type),
            "Suggested name '{}' should be valid for {:?}",
            suggested,
            crate_type
        );
    }
}

#[test]
fn test_forbidden_generic_names() {
    let validator = NamingValidator::new();
    
    let forbidden = vec![
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
    ];
    
    for name in forbidden {
        let crate_info = create_crate_info(name, CrateType::Library);
        let violations = validator.validate_forbidden_names(&crate_info);
        
        assert!(
            !violations.is_empty(),
            "Expected violation for forbidden name '{}'",
            name
        );
        assert!(
            violations.iter().any(|v| v.id == "naming-generic-forbidden"),
            "Expected naming-generic-forbidden violation for '{}'",
            name
        );
    }
}

#[test]
fn test_lib_name_underscore_convention() {
    let validator = NamingValidator::new();
    
    // Test that lib names with hyphens are flagged
    let toml_content = r#"
[package]
name = "dx-test-crate"
version = "0.1.0"

[lib]
name = "dx-test-crate"
"#;
    let cargo_toml = CargoToml::parse(toml_content).unwrap();
    let crate_info = CrateInfo::new(
        "dx-test-crate".to_string(),
        PathBuf::from("crates/test-crate"),
        cargo_toml,
        CrateType::Library,
    );
    
    let violations = validator.validate_lib_name(&crate_info);
    assert!(
        violations.iter().any(|v| v.id == "naming-lib-underscore"),
        "Expected lib name underscore violation"
    );
}

#[test]
fn test_compliant_lib_name() {
    let validator = NamingValidator::new();
    
    // Test that lib names with underscores and dx_ prefix are valid
    let toml_content = r#"
[package]
name = "dx-test-crate"
version = "0.1.0"

[lib]
name = "dx_test_crate"
"#;
    let cargo_toml = CargoToml::parse(toml_content).unwrap();
    let crate_info = CrateInfo::new(
        "dx-test-crate".to_string(),
        PathBuf::from("crates/test-crate"),
        cargo_toml,
        CrateType::Library,
    );
    
    let violations = validator.validate_lib_name(&crate_info);
    assert!(
        !violations.iter().any(|v| v.id == "naming-lib-underscore"),
        "Expected no lib name underscore violation"
    );
}
