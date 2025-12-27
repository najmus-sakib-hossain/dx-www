//! Property-based tests for license validation
//!
//! **Feature: crates-professionalization-v2, Property 5: License Content Validity**
//! For any LICENSE file, content matches expected and Cargo.toml is consistent
//! **Validates: Requirements 4.2, 4.3**

use dx_crate_lint::models::{CargoToml, CrateInfo, CrateType};
use dx_crate_lint::validators::LicenseValidator;
use proptest::prelude::*;
use std::path::PathBuf;

/// Strategy for generating package names
fn package_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{2,20}".prop_map(|s| s.to_string())
}

/// Strategy for generating license file configurations
fn license_files_strategy() -> impl Strategy<Value = Vec<String>> {
    prop_oneof![
        // No license files
        Just(vec!["Cargo.toml".to_string(), "src/lib.rs".to_string()]),
        // Single LICENSE file
        Just(vec!["Cargo.toml".to_string(), "src/lib.rs".to_string(), "LICENSE".to_string()]),
        // Dual license files
        Just(vec![
            "Cargo.toml".to_string(),
            "src/lib.rs".to_string(),
            "LICENSE-MIT".to_string(),
            "LICENSE-APACHE".to_string(),
        ]),
        // Only MIT
        Just(vec!["Cargo.toml".to_string(), "src/lib.rs".to_string(), "LICENSE-MIT".to_string()]),
        // Only Apache
        Just(vec!["Cargo.toml".to_string(), "src/lib.rs".to_string(), "LICENSE-APACHE".to_string()]),
        // All three
        Just(vec![
            "Cargo.toml".to_string(),
            "src/lib.rs".to_string(),
            "LICENSE".to_string(),
            "LICENSE-MIT".to_string(),
            "LICENSE-APACHE".to_string(),
        ]),
    ]
}

/// Strategy for generating Cargo.toml license field values
fn cargo_license_strategy() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        Just(None),
        Just(Some("MIT OR Apache-2.0".to_string())),
        Just(Some("MIT".to_string())),
        Just(Some("Apache-2.0".to_string())),
        Just(Some("GPL-3.0".to_string())),
        Just(Some("UNLICENSED".to_string())),
    ]
}

/// Helper to create a CrateInfo from name, license field, and files
fn create_crate_info(name: &str, license: Option<&str>, files: Vec<String>) -> CrateInfo {
    let license_line = match license {
        Some(l) => format!("license = \"{}\"", l),
        None => String::new(),
    };
    
    let toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
{}
"#, name, license_line);
    
    let cargo_toml = CargoToml::parse(&toml_content).unwrap();
    CrateInfo::new(
        name.to_string(),
        PathBuf::from(format!("crates/{}", name)),
        cargo_toml,
        CrateType::Library,
    ).with_files(files.into_iter().map(PathBuf::from).collect())
}

/// Helper to create a CrateInfo with workspace license
fn create_crate_info_workspace_license(name: &str, files: Vec<String>) -> CrateInfo {
    let toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
license.workspace = true
"#, name);
    
    let cargo_toml = CargoToml::parse(&toml_content).unwrap();
    CrateInfo::new(
        name.to_string(),
        PathBuf::from(format!("crates/{}", name)),
        cargo_toml,
        CrateType::Library,
    ).with_files(files.into_iter().map(PathBuf::from).collect())
}

proptest! {
    /// Property 5a: License file existence detection
    /// For any crate, missing license files are detected
    #[test]
    fn license_file_existence_detected(
        name in package_name_strategy(),
        files in license_files_strategy(),
    ) {
        let has_license = files.iter().any(|f| f == "LICENSE");
        let has_mit = files.iter().any(|f| f == "LICENSE-MIT");
        let has_apache = files.iter().any(|f| f == "LICENSE-APACHE");
        
        let crate_info = create_crate_info(&name, None, files);
        let validator = LicenseValidator::new();
        let violations = validator.validate_license_file_exists(&crate_info);
        
        let has_missing_violation = violations.iter().any(|v| v.id == "license-file-missing");
        let has_incomplete_violation = violations.iter().any(|v| v.id == "license-dual-incomplete");
        
        // Should have missing violation if no LICENSE and not both MIT+Apache
        let should_have_missing = !has_license && !(has_mit && has_apache);
        prop_assert_eq!(
            should_have_missing,
            has_missing_violation,
            "Missing license detection mismatch: has_license={}, has_mit={}, has_apache={}",
            has_license, has_mit, has_apache
        );
        
        // Should have incomplete violation if only one of MIT/Apache
        let should_have_incomplete = (has_mit && !has_apache) || (!has_mit && has_apache);
        prop_assert_eq!(
            should_have_incomplete,
            has_incomplete_violation,
            "Incomplete dual license detection mismatch"
        );
    }

    /// Property 5b: Cargo.toml license consistency
    /// For any crate with non-standard license, a warning is generated
    #[test]
    fn cargo_license_consistency(
        name in package_name_strategy(),
        license in cargo_license_strategy(),
    ) {
        let files = vec!["Cargo.toml".to_string(), "LICENSE".to_string()];
        let crate_info = create_crate_info(&name, license.as_deref(), files);
        let validator = LicenseValidator::new();
        let violations = validator.validate_cargo_toml_license(&crate_info);
        
        let has_mismatch_violation = violations.iter().any(|v| v.id == "license-cargo-mismatch");
        
        // Should have mismatch if license is not MIT, Apache-2.0, or MIT OR Apache-2.0
        let is_acceptable = match license.as_deref() {
            None => true, // No license field is handled elsewhere
            Some("MIT OR Apache-2.0") => true,
            Some("MIT") => true,
            Some("Apache-2.0") => true,
            _ => false,
        };
        
        if license.is_some() {
            prop_assert_eq!(
                !is_acceptable,
                has_mismatch_violation,
                "License mismatch detection for {:?}",
                license
            );
        }
    }

    /// Property 5c: Workspace license inheritance is accepted
    /// For any crate using workspace license, no license violations are generated
    #[test]
    fn workspace_license_accepted(
        name in package_name_strategy(),
    ) {
        let files = vec!["Cargo.toml".to_string(), "LICENSE".to_string()];
        let crate_info = create_crate_info_workspace_license(&name, files);
        let validator = LicenseValidator::new();
        let violations = validator.validate_cargo_toml_license(&crate_info);
        
        prop_assert!(
            violations.is_empty(),
            "Workspace license should not generate violations, got: {:?}",
            violations
        );
    }
}

/// Test MIT license content detection
#[test]
fn test_mit_license_content_patterns() {
    let validator = LicenseValidator::new();
    
    let mit_variants = [
        "MIT License\n\nCopyright (c) 2024",
        "The MIT License (MIT)\n\nCopyright",
        "Permission is hereby granted, free of charge, to any person",
    ];
    
    for content in mit_variants {
        assert!(
            validator.is_mit_license(content),
            "Should detect MIT license in: {}",
            content
        );
    }
}

/// Test Apache license content detection
#[test]
fn test_apache_license_content_patterns() {
    let validator = LicenseValidator::new();
    
    let apache_variants = [
        "Apache License\nVersion 2.0, January 2004",
        "Licensed under the Apache License, Version 2.0",
        "Apache-2.0 License",
    ];
    
    for content in apache_variants {
        assert!(
            validator.is_apache_license(content),
            "Should detect Apache license in: {}",
            content
        );
    }
}

/// Test that non-license content is not detected
#[test]
fn test_non_license_content() {
    let validator = LicenseValidator::new();
    
    let non_license = [
        "This is just some random text",
        "Copyright 2024 - All rights reserved",
        "Proprietary software",
    ];
    
    for content in non_license {
        assert!(
            !validator.is_mit_license(content),
            "Should not detect MIT license in: {}",
            content
        );
        assert!(
            !validator.is_apache_license(content),
            "Should not detect Apache license in: {}",
            content
        );
    }
}

/// Test complete license validation
#[test]
fn test_complete_license_validation() {
    let files = vec![
        "Cargo.toml".to_string(),
        "src/lib.rs".to_string(),
        "LICENSE".to_string(),
    ];
    
    let crate_info = create_crate_info("test-crate", Some("MIT OR Apache-2.0"), files);
    let validator = LicenseValidator::new();
    
    // File existence should pass
    let file_violations = validator.validate_license_file_exists(&crate_info);
    assert!(file_violations.is_empty());
    
    // Cargo.toml license should pass
    let cargo_violations = validator.validate_cargo_toml_license(&crate_info);
    assert!(cargo_violations.is_empty());
}

/// Test auto-fix is provided for missing license
#[test]
fn test_missing_license_has_autofix() {
    let files = vec!["Cargo.toml".to_string(), "src/lib.rs".to_string()];
    let crate_info = create_crate_info("test-crate", None, files);
    let validator = LicenseValidator::new();
    
    let violations = validator.validate_license_file_exists(&crate_info);
    
    let missing_violation = violations.iter().find(|v| v.id == "license-file-missing");
    assert!(missing_violation.is_some());
    
    let fix = missing_violation.unwrap().fix.as_ref();
    assert!(fix.is_some());
    assert!(fix.unwrap().auto_fixable);
}
