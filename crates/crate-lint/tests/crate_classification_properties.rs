//! Property-based tests for crate classification
//!
//! **Feature: crates-professionalization-v2, Property 2: Naming Convention Compliance**
//! For any crate path, classification determines correct naming pattern
//! **Validates: Requirements 2.1, 2.2**

use dx_crate_lint::models::CrateType;
use proptest::prelude::*;
use std::path::Path;

/// Strategy for generating valid crate names
fn crate_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{2,20}".prop_map(|s| s.to_string())
}

proptest! {
    /// Property: Top-level tool crates are correctly classified
    /// For any crate in crates/{dx|cli|crate-lint}, it should be classified as TopLevelTool
    #[test]
    fn top_level_tools_classified_correctly(
        tool in prop_oneof![
            Just("dx"),
            Just("cli"),
            Just("crate-lint"),
        ]
    ) {
        let path = format!("crates/{}", tool);
        let crate_type = CrateType::from_path(Path::new(&path));
        prop_assert_eq!(crate_type, CrateType::TopLevelTool);
    }

    /// Property: WWW modules are correctly classified
    /// For any crate in crates/www/{name}, it should be classified as WwwModule
    #[test]
    fn www_modules_classified_correctly(name in crate_name_strategy()) {
        let path = format!("crates/www/{}", name);
        let crate_type = CrateType::from_path(Path::new(&path));
        prop_assert_eq!(crate_type, CrateType::WwwModule);
    }

    /// Property: JavaScript modules are correctly classified
    /// For any crate in crates/javascript/{name}, it should be classified as JavaScriptModule
    #[test]
    fn javascript_modules_classified_correctly(name in crate_name_strategy()) {
        let path = format!("crates/javascript/{}", name);
        let crate_type = CrateType::from_path(Path::new(&path));
        prop_assert_eq!(crate_type, CrateType::JavaScriptModule);
    }

    /// Property: Library crates are correctly classified
    /// For any crate in crates/{name} that is not a tool, it should be classified as Library
    #[test]
    fn library_crates_classified_correctly(
        name in crate_name_strategy().prop_filter(
            "Not a tool name",
            |n| !["dx", "cli", "crate-lint"].contains(&n.as_str())
        )
    ) {
        let path = format!("crates/{}", name);
        let crate_type = CrateType::from_path(Path::new(&path));
        prop_assert_eq!(crate_type, CrateType::Library);
    }

    /// Property: Classification determines correct naming pattern
    /// For any crate type, the expected naming pattern is consistent
    #[test]
    fn naming_pattern_consistency(
        crate_type in prop_oneof![
            Just(CrateType::TopLevelTool),
            Just(CrateType::Library),
            Just(CrateType::WwwModule),
            Just(CrateType::JavaScriptModule),
        ]
    ) {
        // Each crate type should have a consistent display name
        let display = crate_type.display_name();
        prop_assert!(!display.is_empty());
        
        // Verify the expected patterns
        match crate_type {
            CrateType::TopLevelTool => {
                prop_assert_eq!(display, "Top-level Tool");
            }
            CrateType::Library => {
                prop_assert_eq!(display, "Library");
            }
            CrateType::WwwModule => {
                prop_assert_eq!(display, "WWW Module");
            }
            CrateType::JavaScriptModule => {
                prop_assert_eq!(display, "JavaScript Module");
            }
            CrateType::Nested => {
                prop_assert_eq!(display, "Nested Crate");
            }
        }
    }
}

#[test]
fn test_specific_crate_classifications() {
    // Test known crate paths
    let test_cases = vec![
        ("crates/dx", CrateType::TopLevelTool),
        ("crates/cli", CrateType::TopLevelTool),
        ("crates/crate-lint", CrateType::TopLevelTool),
        ("crates/serializer", CrateType::Library),
        ("crates/forge", CrateType::Library),
        ("crates/generator", CrateType::Library),
        ("crates/www/core", CrateType::WwwModule),
        ("crates/www/dom", CrateType::WwwModule),
        ("crates/www/server", CrateType::WwwModule),
        ("crates/javascript/runtime", CrateType::JavaScriptModule),
        ("crates/javascript/bundler", CrateType::JavaScriptModule),
    ];
    
    for (path, expected) in test_cases {
        let actual = CrateType::from_path(Path::new(path));
        assert_eq!(actual, expected, "Path '{}' should be {:?}, got {:?}", path, expected, actual);
    }
}

#[test]
fn test_windows_path_classification() {
    // Test Windows-style paths
    let www_path = Path::new("crates\\www\\core");
    assert_eq!(CrateType::from_path(www_path), CrateType::WwwModule);
    
    let js_path = Path::new("crates\\javascript\\runtime");
    assert_eq!(CrateType::from_path(js_path), CrateType::JavaScriptModule);
}
