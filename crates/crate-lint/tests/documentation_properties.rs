//! Property-based tests for documentation validation
//!
//! **Feature: crates-professionalization-v2, Property 3: Required Files Existence**
//! For any crate, README.md and CHANGELOG.md exist
//! **Validates: Requirements 3.1, 3.3**
//!
//! **Feature: crates-professionalization-v2, Property 4: README Structure Completeness**
//! For any README, all required sections are detected
//! **Validates: Requirements 3.2, 3.5, 3.6**

use dx_crate_lint::models::{CargoToml, CrateInfo, CrateType};
use dx_crate_lint::validators::{DocumentationValidator, ReadmeSection, BadgeType};
use proptest::prelude::*;
use std::path::PathBuf;

/// Strategy for generating package names
fn package_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{2,20}".prop_map(|s| s.to_string())
}

/// Strategy for generating file lists with various combinations
fn file_list_strategy() -> impl Strategy<Value = Vec<String>> {
    let base_files = vec!["Cargo.toml".to_string(), "src/lib.rs".to_string()];
    
    (
        proptest::bool::ANY, // has README
        proptest::bool::ANY, // has CHANGELOG
        proptest::bool::ANY, // has LICENSE
    ).prop_map(move |(has_readme, has_changelog, has_license)| {
        let mut files = base_files.clone();
        if has_readme {
            files.push("README.md".to_string());
        }
        if has_changelog {
            files.push("CHANGELOG.md".to_string());
        }
        if has_license {
            files.push("LICENSE".to_string());
        }
        files
    })
}

/// Strategy for generating README content with various sections
fn readme_content_strategy() -> impl Strategy<Value = String> {
    (
        proptest::bool::ANY, // has title
        proptest::bool::ANY, // has badges
        proptest::bool::ANY, // has overview
        proptest::bool::ANY, // has features
        proptest::bool::ANY, // has installation
        proptest::bool::ANY, // has usage
        proptest::bool::ANY, // has license
    ).prop_map(|(has_title, has_badges, has_overview, has_features, has_installation, has_usage, has_license)| {
        let mut content = String::new();
        
        if has_title {
            content.push_str("# Test Crate\n\n");
        }
        
        if has_badges {
            content.push_str("[![crates.io](https://crates.io/crates/test)]\n");
            content.push_str("[![docs.rs](https://docs.rs/test)]\n");
            content.push_str("[![license](MIT)]\n\n");
        }
        
        if has_overview {
            content.push_str("## Overview\n\nThis is a test crate.\n\n");
        }
        
        if has_features {
            content.push_str("## Features\n\n- Feature 1\n- Feature 2\n\n");
        }
        
        if has_installation {
            content.push_str("## Installation\n\nAdd to Cargo.toml\n\n");
        }
        
        if has_usage {
            content.push_str("## Usage\n\n```rust\nuse test::*;\n```\n\n");
        }
        
        if has_license {
            content.push_str("## License\n\nMIT\n");
        }
        
        content
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
    /// Property 3: Required files existence
    /// For any crate, README.md and CHANGELOG.md must exist
    #[test]
    fn required_files_detected(
        name in package_name_strategy(),
        files in file_list_strategy(),
    ) {
        let has_readme = files.iter().any(|f| f == "README.md");
        let has_changelog = files.iter().any(|f| f == "CHANGELOG.md");
        
        let crate_info = create_crate_info(&name, files);
        let validator = DocumentationValidator::new();
        let violations = validator.validate_required_files(&crate_info);
        
        // Check README violation
        let has_readme_violation = violations.iter().any(|v| v.id == "doc-readme-missing");
        prop_assert_eq!(
            !has_readme,
            has_readme_violation,
            "README.md presence ({}) should match violation absence ({})",
            has_readme,
            !has_readme_violation
        );
        
        // Check CHANGELOG violation
        let has_changelog_violation = violations.iter().any(|v| v.id == "doc-changelog-missing");
        prop_assert_eq!(
            !has_changelog,
            has_changelog_violation,
            "CHANGELOG.md presence ({}) should match violation absence ({})",
            has_changelog,
            !has_changelog_violation
        );
    }

    /// Property 3b: Auto-fix is provided for missing files
    /// For any missing required file, an auto-fixable fix should be provided
    #[test]
    fn missing_files_have_autofix(
        name in package_name_strategy(),
    ) {
        // Create crate with no documentation files
        let files = vec!["Cargo.toml".to_string(), "src/lib.rs".to_string()];
        let crate_info = create_crate_info(&name, files);
        let validator = DocumentationValidator::new();
        let violations = validator.validate_required_files(&crate_info);
        
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

    /// Property 4: README structure completeness
    /// For any README with all sections, no section violations should be reported
    #[test]
    fn readme_sections_detected(
        name in package_name_strategy(),
        content in readme_content_strategy(),
    ) {
        let crate_info = create_crate_info(&name, vec!["README.md".to_string()]);
        let validator = DocumentationValidator::new();
        let violations = validator.validate_readme_structure(&crate_info, &content);
        
        let content_lower = content.to_lowercase();
        
        // Check each section
        for section in ReadmeSection::all() {
            let section_present = section.detection_patterns()
                .iter()
                .any(|p| content_lower.contains(p));
            
            let section_name = section.display_name().to_lowercase();
            let has_violation = violations.iter().any(|v| 
                v.id.contains(&section_name)
            );
            
            prop_assert_eq!(
                !section_present,
                has_violation,
                "Section {:?} presence ({}) should match violation absence ({})",
                section,
                section_present,
                !has_violation
            );
        }
    }

    /// Property 4b: README badges detected
    /// For any README with badges, badge violations should not be reported
    #[test]
    fn readme_badges_detected(
        name in package_name_strategy(),
        has_crates_badge in proptest::bool::ANY,
        has_docs_badge in proptest::bool::ANY,
        has_license_badge in proptest::bool::ANY,
    ) {
        let mut content = String::from("# Test\n\n");
        
        if has_crates_badge {
            content.push_str("[![crates.io](https://crates.io/crates/test)]\n");
        }
        if has_docs_badge {
            content.push_str("[![docs.rs](https://docs.rs/test)]\n");
        }
        if has_license_badge {
            content.push_str("[![license](MIT)]\n");
        }
        
        let crate_info = create_crate_info(&name, vec!["README.md".to_string()]);
        let validator = DocumentationValidator::new();
        let violations = validator.validate_readme_badges(&crate_info, &content);
        
        // Check crates.io badge
        let has_crates_violation = violations.iter().any(|v| v.id.contains("crates-io"));
        prop_assert_eq!(
            !has_crates_badge,
            has_crates_violation,
            "crates.io badge presence should match violation absence"
        );
        
        // Check docs.rs badge
        let has_docs_violation = violations.iter().any(|v| v.id.contains("docs-rs"));
        prop_assert_eq!(
            !has_docs_badge,
            has_docs_violation,
            "docs.rs badge presence should match violation absence"
        );
        
        // Check license badge
        let has_license_violation = violations.iter().any(|v| v.id.contains("license"));
        prop_assert_eq!(
            !has_license_badge,
            has_license_violation,
            "license badge presence should match violation absence"
        );
    }
}

/// Test complete README validation
#[test]
fn test_complete_readme_no_violations() {
    let complete_readme = r#"# Test Crate

[![crates.io](https://img.shields.io/crates/v/test.svg)](https://crates.io/crates/test)
[![docs.rs](https://docs.rs/test/badge.svg)](https://docs.rs/test)
[![license](https://img.shields.io/crates/l/test.svg)](LICENSE)

## Overview

This is a test crate for validation.

## Features

- Feature 1
- Feature 2

## Installation

Add to Cargo.toml:

```toml
[dependencies]
test = "0.1"
```

## Usage

```rust
use test::*;
```

## License

MIT OR Apache-2.0
"#;
    
    let crate_info = create_crate_info("test-crate", vec!["README.md".to_string()]);
    let validator = DocumentationValidator::new();
    
    let section_violations = validator.validate_readme_structure(&crate_info, complete_readme);
    let badge_violations = validator.validate_readme_badges(&crate_info, complete_readme);
    
    assert!(
        section_violations.is_empty(),
        "Expected no section violations, got: {:?}",
        section_violations
    );
    assert!(
        badge_violations.is_empty(),
        "Expected no badge violations, got: {:?}",
        badge_violations
    );
}

/// Test that missing sections are detected
#[test]
fn test_missing_sections_detected() {
    let minimal_readme = "# Test\n\nSome content.";
    
    let crate_info = create_crate_info("test-crate", vec!["README.md".to_string()]);
    let validator = DocumentationValidator::new();
    let violations = validator.validate_readme_structure(&crate_info, minimal_readme);
    
    // Should have violations for missing sections
    assert!(!violations.is_empty(), "Expected violations for minimal README");
    
    // Should detect missing overview, features, installation, usage, license
    let violation_ids: Vec<_> = violations.iter().map(|v| v.id.as_str()).collect();
    assert!(violation_ids.iter().any(|id| id.contains("overview")));
    assert!(violation_ids.iter().any(|id| id.contains("features")));
    assert!(violation_ids.iter().any(|id| id.contains("installation")));
    assert!(violation_ids.iter().any(|id| id.contains("usage")));
}

/// Test section detection patterns
#[test]
fn test_section_detection_patterns() {
    // Test various ways to write section headers
    let readme_variants = [
        ("## Overview", ReadmeSection::Overview),
        ("## About", ReadmeSection::Overview),
        ("## Description", ReadmeSection::Overview),
        ("## Features", ReadmeSection::Features),
        ("## Highlights", ReadmeSection::Features),
        ("## Installation", ReadmeSection::Installation),
        ("## Getting Started", ReadmeSection::Installation),
        ("## Usage", ReadmeSection::Usage),
        ("## Examples", ReadmeSection::Usage),
        ("## License", ReadmeSection::License),
        ("Licensed under MIT", ReadmeSection::License),
    ];
    
    for (content, expected_section) in readme_variants {
        let full_content = format!("# Test\n\n{}\n\nContent here.", content);
        let content_lower = full_content.to_lowercase();
        
        let detected = expected_section.detection_patterns()
            .iter()
            .any(|p| content_lower.contains(p));
        
        assert!(
            detected,
            "Section {:?} should be detected in content: {}",
            expected_section,
            content
        );
    }
}

/// Test badge detection patterns
#[test]
fn test_badge_detection_patterns() {
    let badge_variants = [
        ("[![crates.io](url)]", BadgeType::CratesIo),
        ("![crates-io](url)", BadgeType::CratesIo),
        ("[![docs.rs](url)]", BadgeType::DocsRs),
        ("![docsrs](url)", BadgeType::DocsRs),
        ("[![license](url)]", BadgeType::License),
        ("![License](url)", BadgeType::License),
    ];
    
    for (content, expected_badge) in badge_variants {
        let content_lower = content.to_lowercase();
        
        let detected = expected_badge.detection_patterns()
            .iter()
            .any(|p| content_lower.contains(p));
        
        assert!(
            detected,
            "Badge {:?} should be detected in content: {}",
            expected_badge,
            content
        );
    }
}
