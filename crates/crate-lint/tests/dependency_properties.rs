//! Property-based tests for dependency validation
//!
//! **Feature: crates-professionalization-v2, Property 7: Dependency Workspace Consistency**
//! For any internal dep, workspace syntax is used; no version conflicts exist
//! **Validates: Requirements 6.1, 6.3, 6.4**

use dx_crate_lint::models::{CargoToml, CrateInfo, CrateType};
use dx_crate_lint::validators::DependencyValidator;
use proptest::prelude::*;
use std::collections::HashSet;
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

/// Strategy for generating dependency configurations
fn dep_config_strategy() -> impl Strategy<Value = (bool, bool, Option<String>)> {
    prop_oneof![
        // workspace = true
        Just((true, false, None)),
        // path dependency
        Just((false, true, Some("../other".to_string()))),
        // version dependency
        version_strategy().prop_map(|v| (false, false, Some(v))),
    ]
}

/// Helper to create a CrateInfo from name and TOML content
fn create_crate_info(name: &str, toml_content: &str) -> CrateInfo {
    let cargo_toml = CargoToml::parse(toml_content).unwrap();
    CrateInfo::new(
        name.to_string(),
        PathBuf::from(format!("crates/{}", name)),
        cargo_toml,
        CrateType::Library,
    )
}

/// Helper to generate TOML with a specific dependency configuration
fn generate_toml_with_dep(name: &str, dep_name: &str, is_workspace: bool, is_path: bool, version: Option<&str>) -> String {
    let dep_spec = if is_workspace {
        "{ workspace = true }".to_string()
    } else if is_path {
        format!("{{ path = \"../{}\" }}", dep_name)
    } else if let Some(v) = version {
        format!("\"{}\"", v)
    } else {
        "\"1.0\"".to_string()
    };
    
    format!(r#"
[package]
name = "{}"
version = "0.1.0"

[dependencies]
{} = {}
"#, name, dep_name, dep_spec)
}

proptest! {
    /// Property 7a: Internal dependencies should use workspace syntax
    /// For any internal dependency, using workspace = true should not generate violations
    #[test]
    fn internal_dep_workspace_no_violation(
        crate_name in package_name_strategy(),
        dep_name in package_name_strategy(),
    ) {
        let mut internal_crates = HashSet::new();
        internal_crates.insert(dep_name.clone());
        
        let toml = generate_toml_with_dep(&crate_name, &dep_name, true, false, None);
        let crate_info = create_crate_info(&crate_name, &toml);
        
        let validator = DependencyValidator::with_internal_crates(internal_crates);
        let violations = validator.validate_internal_deps(&crate_info);
        
        prop_assert!(
            violations.is_empty(),
            "Internal dep with workspace = true should not generate violations"
        );
    }

    /// Property 7b: Internal dependencies without workspace syntax generate violations
    /// For any internal dependency not using workspace = true, a violation should be generated
    #[test]
    fn internal_dep_no_workspace_violation(
        crate_name in package_name_strategy(),
        dep_name in package_name_strategy(),
        version in version_strategy(),
    ) {
        let mut internal_crates = HashSet::new();
        internal_crates.insert(dep_name.clone());
        
        let toml = generate_toml_with_dep(&crate_name, &dep_name, false, false, Some(&version));
        let crate_info = create_crate_info(&crate_name, &toml);
        
        let validator = DependencyValidator::with_internal_crates(internal_crates);
        let violations = validator.validate_internal_deps(&crate_info);
        
        prop_assert!(
            violations.iter().any(|v| v.id == "dep-internal-not-workspace"),
            "Internal dep without workspace = true should generate violation"
        );
    }

    /// Property 7c: External dependencies don't generate internal dep violations
    /// For any external dependency, no internal dep violations should be generated
    #[test]
    fn external_dep_no_internal_violation(
        crate_name in package_name_strategy(),
        dep_name in package_name_strategy(),
        version in version_strategy(),
    ) {
        // Empty internal crates set - all deps are external
        let internal_crates = HashSet::new();
        
        let toml = generate_toml_with_dep(&crate_name, &dep_name, false, false, Some(&version));
        let crate_info = create_crate_info(&crate_name, &toml);
        
        let validator = DependencyValidator::with_internal_crates(internal_crates);
        let violations = validator.validate_internal_deps(&crate_info);
        
        prop_assert!(
            violations.is_empty(),
            "External dep should not generate internal dep violations"
        );
    }

    /// Property 7d: Version conflicts are detected
    /// For any two crates with the same dependency at different versions, a conflict is detected
    #[test]
    fn version_conflicts_detected(
        dep_name in package_name_strategy(),
        version1 in version_strategy(),
        version2 in version_strategy(),
    ) {
        // Skip if versions are the same
        prop_assume!(version1 != version2);
        
        let toml1 = generate_toml_with_dep("crate1", &dep_name, false, false, Some(&version1));
        let toml2 = generate_toml_with_dep("crate2", &dep_name, false, false, Some(&version2));
        
        let crate1 = create_crate_info("crate1", &toml1);
        let crate2 = create_crate_info("crate2", &toml2);
        
        let validator = DependencyValidator::new();
        let violations = validator.find_duplicates(&[crate1, crate2]);
        
        prop_assert!(
            violations.iter().any(|v| v.id == "dep-version-conflict"),
            "Different versions of same dep should generate conflict: {} vs {}",
            version1, version2
        );
    }

    /// Property 7e: Same versions don't generate conflicts
    /// For any two crates with the same dependency at the same version, no conflict is detected
    #[test]
    fn same_versions_no_conflict(
        dep_name in package_name_strategy(),
        version in version_strategy(),
    ) {
        let toml1 = generate_toml_with_dep("crate1", &dep_name, false, false, Some(&version));
        let toml2 = generate_toml_with_dep("crate2", &dep_name, false, false, Some(&version));
        
        let crate1 = create_crate_info("crate1", &toml1);
        let crate2 = create_crate_info("crate2", &toml2);
        
        let validator = DependencyValidator::new();
        let violations = validator.find_duplicates(&[crate1, crate2]);
        
        prop_assert!(
            violations.is_empty(),
            "Same versions should not generate conflict"
        );
    }
}

/// Test that path dependencies to internal crates generate warnings
#[test]
fn test_path_dep_warning() {
    let toml = r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
other = { path = "../crates/other" }
"#;
    
    let crate_info = create_crate_info("test-crate", toml);
    let validator = DependencyValidator::new();
    let violations = validator.validate_path_deps(&crate_info);
    
    assert!(
        violations.iter().any(|v| v.id == "dep-path-should-use-workspace"),
        "Path dep to crates/ should generate warning"
    );
}

/// Test that external path dependencies don't generate warnings
#[test]
fn test_external_path_no_warning() {
    let toml = r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
other = { path = "../external/other" }
"#;
    
    let crate_info = create_crate_info("test-crate", toml);
    let validator = DependencyValidator::new();
    let violations = validator.validate_path_deps(&crate_info);
    
    assert!(
        violations.is_empty(),
        "External path dep should not generate warning"
    );
}

/// Test common deps detection
#[test]
fn test_common_deps_detection() {
    let validator = DependencyValidator::new();
    
    // Create 4 crates all using "serde"
    let crates: Vec<_> = (1..=4)
        .map(|i| {
            let toml = format!(r#"
[package]
name = "crate{}"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#, i);
            create_crate_info(&format!("crate{}", i), &toml)
        })
        .collect();
    
    let violations = validator.find_common_deps(&crates);
    
    assert!(
        violations.iter().any(|v| v.id == "dep-should-be-workspace"),
        "Common dep used by 4 crates should be suggested for workspace.dependencies"
    );
}

/// Test that violations have auto-fixable fixes
#[test]
fn test_violations_have_fixes() {
    let mut internal_crates = HashSet::new();
    internal_crates.insert("dx-serializer".to_string());
    
    let toml = r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
dx-serializer = { path = "../serializer" }
"#;
    
    let crate_info = create_crate_info("test-crate", toml);
    let validator = DependencyValidator::with_internal_crates(internal_crates);
    let violations = validator.validate(&crate_info);
    
    for violation in &violations {
        assert!(
            violation.fix.is_some(),
            "Violation {} should have a fix",
            violation.id
        );
        if let Some(fix) = &violation.fix {
            assert!(
                fix.auto_fixable,
                "Fix for {} should be auto-fixable",
                violation.id
            );
        }
    }
}
