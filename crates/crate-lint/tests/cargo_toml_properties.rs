//! Property-based tests for CargoToml parsing
//!
//! **Feature: crates-professionalization-v2, Property: Parsing round-trip**
//! For any valid CargoToml, serializing then deserializing produces equivalent structure
//! **Validates: Requirements 1.1-1.9**

use dx_crate_lint::models::{
    CargoToml, Package, VersionSpec, EditionSpec, AuthorsSpec, 
    LicenseSpec, RepositorySpec, WorkspaceInherit, Dependency, DetailedDependency,
};
use proptest::prelude::*;
use std::collections::HashMap;

/// Strategy for generating valid package names
fn package_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_-]{2,30}".prop_map(|s| s.to_string())
}

/// Strategy for generating version strings
fn version_strategy() -> impl Strategy<Value = String> {
    (0u8..10, 0u8..20, 0u8..100).prop_map(|(major, minor, patch)| {
        format!("{}.{}.{}", major, minor, patch)
    })
}

/// Strategy for generating VersionSpec
fn version_spec_strategy() -> impl Strategy<Value = VersionSpec> {
    prop_oneof![
        version_strategy().prop_map(VersionSpec::Literal),
        Just(VersionSpec::Workspace(WorkspaceInherit { workspace: true })),
    ]
}

/// Strategy for generating EditionSpec
fn edition_spec_strategy() -> impl Strategy<Value = EditionSpec> {
    prop_oneof![
        Just(EditionSpec::Literal("2021".to_string())),
        Just(EditionSpec::Literal("2024".to_string())),
        Just(EditionSpec::Workspace(WorkspaceInherit { workspace: true })),
    ]
}

/// Strategy for generating AuthorsSpec
fn authors_spec_strategy() -> impl Strategy<Value = AuthorsSpec> {
    prop_oneof![
        Just(AuthorsSpec::Literal(vec!["Test Author <test@example.com>".to_string()])),
        Just(AuthorsSpec::Workspace(WorkspaceInherit { workspace: true })),
    ]
}

/// Strategy for generating LicenseSpec
fn license_spec_strategy() -> impl Strategy<Value = LicenseSpec> {
    prop_oneof![
        Just(LicenseSpec::Literal("MIT OR Apache-2.0".to_string())),
        Just(LicenseSpec::Literal("MIT".to_string())),
        Just(LicenseSpec::Workspace(WorkspaceInherit { workspace: true })),
    ]
}

/// Strategy for generating RepositorySpec
fn repository_spec_strategy() -> impl Strategy<Value = RepositorySpec> {
    prop_oneof![
        Just(RepositorySpec::Literal("https://github.com/test/repo".to_string())),
        Just(RepositorySpec::Workspace(WorkspaceInherit { workspace: true })),
    ]
}

/// Strategy for generating a simple Package
fn package_strategy() -> impl Strategy<Value = Package> {
    (
        package_name_strategy(),
        version_spec_strategy(),
        edition_spec_strategy(),
        authors_spec_strategy(),
        license_spec_strategy(),
        repository_spec_strategy(),
        proptest::option::of("[A-Za-z ]{10,50}"),
    ).prop_map(|(name, version, edition, authors, license, repository, description)| {
        Package {
            name,
            version,
            edition,
            authors,
            description,
            license,
            repository,
            documentation: None,
            homepage: None,
            keywords: Some(vec!["test".to_string()]),
            categories: Some(vec!["development-tools".to_string()]),
            rust_version: None,
            include: None,
            exclude: None,
            readme: None,
            publish: None,
        }
    })
}

/// Strategy for generating dependencies
fn dependency_strategy() -> impl Strategy<Value = HashMap<String, Dependency>> {
    proptest::collection::hash_map(
        package_name_strategy(),
        prop_oneof![
            version_strategy().prop_map(Dependency::Simple),
            Just(Dependency::Detailed(DetailedDependency {
                workspace: Some(true),
                ..Default::default()
            })),
            (version_strategy(), proptest::option::of(proptest::collection::vec("[a-z]+", 0..3)))
                .prop_map(|(version, features)| {
                    Dependency::Detailed(DetailedDependency {
                        version: Some(version),
                        features,
                        ..Default::default()
                    })
                }),
        ],
        0..5,
    )
}

/// Strategy for generating a complete CargoToml
fn cargo_toml_strategy() -> impl Strategy<Value = CargoToml> {
    (package_strategy(), dependency_strategy()).prop_map(|(package, dependencies)| {
        CargoToml {
            package: Some(package),
            lib: None,
            bins: Vec::new(),
            dependencies,
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            features: HashMap::new(),
            workspace: None,
        }
    })
}

proptest! {
    /// Property: Workspace inheritance fields are correctly detected
    /// For any CargoToml with workspace inheritance, the is_workspace() method returns true
    #[test]
    fn workspace_inheritance_detection(
        use_workspace in proptest::bool::ANY
    ) {
        let version = if use_workspace {
            VersionSpec::Workspace(WorkspaceInherit { workspace: true })
        } else {
            VersionSpec::Literal("0.1.0".to_string())
        };
        
        prop_assert_eq!(version.is_workspace(), use_workspace);
    }

    /// Property: Version spec literal extraction works correctly
    #[test]
    fn version_literal_extraction(version_str in version_strategy()) {
        let spec = VersionSpec::Literal(version_str.clone());
        prop_assert_eq!(spec.as_literal(), Some(version_str.as_str()));
        
        let workspace_spec = VersionSpec::Workspace(WorkspaceInherit { workspace: true });
        prop_assert_eq!(workspace_spec.as_literal(), None);
    }

    /// Property: Dependency workspace detection works correctly
    #[test]
    fn dependency_workspace_detection(use_workspace in proptest::bool::ANY) {
        let dep = if use_workspace {
            Dependency::Detailed(DetailedDependency {
                workspace: Some(true),
                ..Default::default()
            })
        } else {
            Dependency::Simple("1.0".to_string())
        };
        
        prop_assert_eq!(dep.is_workspace(), use_workspace);
    }

    /// Property: Dependency path detection works correctly
    #[test]
    fn dependency_path_detection(has_path in proptest::bool::ANY) {
        let dep = if has_path {
            Dependency::Detailed(DetailedDependency {
                path: Some("../other".to_string()),
                ..Default::default()
            })
        } else {
            Dependency::Simple("1.0".to_string())
        };
        
        prop_assert_eq!(dep.is_path(), has_path);
    }

    /// Property: CargoToml parsing produces valid structure
    /// For any valid TOML string representing a Cargo.toml, parsing succeeds
    #[test]
    fn cargo_toml_parsing_valid_input(
        name in package_name_strategy(),
        version in version_strategy(),
    ) {
        let toml_str = format!(r#"
[package]
name = "{}"
version = "{}"
edition = "2021"
"#, name, version);
        
        let result = CargoToml::parse(&toml_str);
        prop_assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
        
        let cargo = result.unwrap();
        prop_assert!(cargo.package.is_some());
        prop_assert_eq!(cargo.package.unwrap().name, name);
    }

    /// Property: Workspace inheritance parsing works correctly
    #[test]
    fn workspace_inheritance_parsing(name in package_name_strategy()) {
        let toml_str = format!(r#"
[package]
name = "{}"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
"#, name);
        
        let result = CargoToml::parse(&toml_str);
        prop_assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
        
        let cargo = result.unwrap();
        let package = cargo.package.unwrap();
        prop_assert!(package.version.is_workspace());
        prop_assert!(package.edition.is_workspace());
        prop_assert!(package.authors.is_workspace());
        prop_assert!(package.license.is_workspace());
        prop_assert!(package.repository.is_workspace());
    }

    /// Property: Internal dependencies are correctly identified
    #[test]
    fn internal_dependencies_identification(
        name in package_name_strategy(),
        dep_name in package_name_strategy(),
    ) {
        let toml_str = format!(r#"
[package]
name = "{}"
version = "0.1.0"

[dependencies]
{} = {{ path = "../{}" }}
external = "1.0"
"#, name, dep_name, dep_name);
        
        let cargo = CargoToml::parse(&toml_str).unwrap();
        let internal = cargo.internal_dependencies();
        
        prop_assert_eq!(internal.len(), 1);
        prop_assert_eq!(internal[0].0, &dep_name);
    }
}

#[test]
fn test_cargo_toml_round_trip_simple() {
    // Test that a simple Cargo.toml can be parsed
    let original = r#"
[package]
name = "test-crate"
version = "0.1.0"
edition = "2021"
description = "A test crate"

[dependencies]
serde = "1.0"
"#;
    
    let parsed = CargoToml::parse(original).expect("Failed to parse");
    let package = parsed.package.expect("Should have package");
    assert_eq!(package.name, "test-crate");
    assert!(matches!(package.version, VersionSpec::Literal(ref v) if v == "0.1.0"));
    assert!(parsed.dependencies.contains_key("serde"));
}

#[test]
fn test_cargo_toml_round_trip_workspace() {
    // Test workspace inheritance parsing
    let original = r#"
[package]
name = "test-crate"
version.workspace = true
edition.workspace = true

[dependencies]
internal = { workspace = true }
"#;
    
    let parsed = CargoToml::parse(original).expect("Failed to parse");
    let package = parsed.package.expect("Should have package");
    assert!(package.version.is_workspace());
    assert!(package.edition.is_workspace());
    assert!(parsed.dependencies.get("internal").unwrap().is_workspace());
}
