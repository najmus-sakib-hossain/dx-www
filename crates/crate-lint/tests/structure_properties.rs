//! Property-based tests for structure validation
//!
//! **Feature: crates-professionalization-v2, Property 6: Source Directory Structure**
//! For any crate, src/ contains lib.rs or main.rs
//! **Validates: Requirements 5.1**

use dx_crate_lint::models::{CargoToml, CrateInfo, CrateType};
use dx_crate_lint::validators::StructureValidator;
use proptest::prelude::*;
use std::path::PathBuf;

/// Strategy for generating package names
fn package_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{2,20}".prop_map(|s| s.to_string())
}

/// Strategy for generating file lists with various src configurations
fn src_files_strategy() -> impl Strategy<Value = Vec<String>> {
    prop_oneof![
        // No src directory
        Just(vec!["Cargo.toml".to_string()]),
        // src with lib.rs
        Just(vec!["Cargo.toml".to_string(), "src/lib.rs".to_string()]),
        // src with main.rs
        Just(vec!["Cargo.toml".to_string(), "src/main.rs".to_string()]),
        // src with both
        Just(vec!["Cargo.toml".to_string(), "src/lib.rs".to_string(), "src/main.rs".to_string()]),
        // src with other files but no entry point
        Just(vec!["Cargo.toml".to_string(), "src/mod.rs".to_string(), "src/utils.rs".to_string()]),
        // Complete structure
        Just(vec![
            "Cargo.toml".to_string(),
            "src/lib.rs".to_string(),
            "tests/test.rs".to_string(),
            "README.md".to_string(),
        ]),
    ]
}

/// Strategy for generating file lists with build artifacts
fn build_artifact_strategy() -> impl Strategy<Value = Vec<String>> {
    (
        proptest::bool::ANY, // has target
        proptest::bool::ANY, // has gitignore
    ).prop_map(|(has_target, has_gitignore)| {
        let mut files = vec![
            "Cargo.toml".to_string(),
            "src/lib.rs".to_string(),
        ];
        if has_target {
            files.push("target/debug".to_string());
        }
        if has_gitignore {
            files.push(".gitignore".to_string());
        }
        files
    })
}

/// Helper to create a CrateInfo from name and files
fn create_crate_info(name: &str, files: Vec<String>) -> CrateInfo {
    let toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
description = "Test crate"
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
    /// Property 6: Source directory structure
    /// For any crate, src/ must exist and contain lib.rs or main.rs
    #[test]
    fn source_structure_validated(
        name in package_name_strategy(),
        files in src_files_strategy(),
    ) {
        let has_src = files.iter().any(|f| f.starts_with("src/") || f.starts_with("src\\"));
        let has_lib_rs = files.iter().any(|f| f == "src/lib.rs" || f == "src\\lib.rs");
        let has_main_rs = files.iter().any(|f| f == "src/main.rs" || f == "src\\main.rs");
        let has_entry_point = has_lib_rs || has_main_rs;
        
        let crate_info = create_crate_info(&name, files);
        let validator = StructureValidator::new();
        
        // Check src directory validation
        let src_violations = validator.validate_src_directory(&crate_info);
        let has_src_violation = src_violations.iter().any(|v| v.id == "structure-src-missing");
        
        prop_assert_eq!(
            !has_src,
            has_src_violation,
            "src directory presence ({}) should match violation absence ({})",
            has_src,
            !has_src_violation
        );
        
        // Check entry point validation
        let entry_violations = validator.validate_entry_point(&crate_info);
        let has_entry_violation = entry_violations.iter().any(|v| v.id == "structure-entry-point-missing");
        
        // Entry point violation should only occur if src exists but no entry point
        let should_have_entry_violation = has_src && !has_entry_point;
        prop_assert_eq!(
            should_have_entry_violation,
            has_entry_violation,
            "Entry point violation mismatch: has_src={}, has_entry_point={}",
            has_src,
            has_entry_point
        );
    }

    /// Property 6b: Build artifacts require .gitignore
    /// For any crate with build artifacts, .gitignore should exist
    #[test]
    fn gitignore_required_for_artifacts(
        name in package_name_strategy(),
        files in build_artifact_strategy(),
    ) {
        let has_target = files.iter().any(|f| f.starts_with("target/") || f.starts_with("target\\") || f == "target");
        let has_gitignore = files.iter().any(|f| f == ".gitignore");
        
        let crate_info = create_crate_info(&name, files);
        let validator = StructureValidator::new();
        let violations = validator.validate_gitignore(&crate_info);
        
        let has_gitignore_violation = violations.iter().any(|v| v.id == "structure-gitignore-missing");
        
        // Should have violation if target exists but no gitignore
        let should_have_violation = has_target && !has_gitignore;
        prop_assert_eq!(
            should_have_violation,
            has_gitignore_violation,
            "Gitignore violation mismatch: has_target={}, has_gitignore={}",
            has_target,
            has_gitignore
        );
    }

    /// Property 6c: Auto-fix is provided for missing structure
    /// For any missing required structure, an auto-fixable fix should be provided
    #[test]
    fn missing_structure_has_autofix(
        name in package_name_strategy(),
    ) {
        // Create crate with no src directory
        let files = vec!["Cargo.toml".to_string()];
        let crate_info = create_crate_info(&name, files);
        let validator = StructureValidator::new();
        let violations = validator.validate(&crate_info);
        
        // All violations should have auto-fixable fixes
        for violation in &violations {
            prop_assert!(
                violation.fix.is_some(),
                "Violation {} should have a fix",
                violation.id
            );
            if let Some(fix) = &violation.fix {
                prop_assert!(
                    fix.auto_fixable,
                    "Fix for {} should be auto-fixable",
                    violation.id
                );
            }
        }
    }
}

/// Test complete structure validation
#[test]
fn test_complete_structure_no_violations() {
    let files = vec![
        "Cargo.toml".to_string(),
        "src/lib.rs".to_string(),
        "tests/test.rs".to_string(),
        "README.md".to_string(),
    ];
    
    let crate_info = create_crate_info("test-crate", files);
    let validator = StructureValidator::new();
    let violations = validator.validate(&crate_info);
    
    assert!(
        violations.is_empty(),
        "Expected no violations for complete structure, got: {:?}",
        violations
    );
}

/// Test that missing src is detected
#[test]
fn test_missing_src_detected() {
    let files = vec!["Cargo.toml".to_string()];
    let crate_info = create_crate_info("test-crate", files);
    let validator = StructureValidator::new();
    let violations = validator.validate(&crate_info);
    
    assert!(
        violations.iter().any(|v| v.id == "structure-src-missing"),
        "Should detect missing src directory"
    );
}

/// Test that missing entry point is detected
#[test]
fn test_missing_entry_point_detected() {
    let files = vec![
        "Cargo.toml".to_string(),
        "src/mod.rs".to_string(),
        "src/utils.rs".to_string(),
    ];
    let crate_info = create_crate_info("test-crate", files);
    let validator = StructureValidator::new();
    let violations = validator.validate(&crate_info);
    
    assert!(
        violations.iter().any(|v| v.id == "structure-entry-point-missing"),
        "Should detect missing entry point"
    );
}

/// Test has_directory helper
#[test]
fn test_has_directory_helper() {
    let files = vec![
        "Cargo.toml".to_string(),
        "src/lib.rs".to_string(),
        "tests/test.rs".to_string(),
    ];
    let crate_info = create_crate_info("test-crate", files);
    let validator = StructureValidator::new();
    
    assert!(validator.has_directory(&crate_info, "src"));
    assert!(validator.has_directory(&crate_info, "tests"));
    assert!(!validator.has_directory(&crate_info, "benches"));
    assert!(!validator.has_directory(&crate_info, "examples"));
}
