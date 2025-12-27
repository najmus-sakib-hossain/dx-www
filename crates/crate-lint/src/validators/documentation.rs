//! Documentation validator for crate documentation files

use crate::models::{
    CrateInfo, CrateType, Violation, ViolationCategory, Severity, Fix, FileChange, ChangeOperation,
};

/// Required sections in a README file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadmeSection {
    Title,
    Badges,
    Overview,
    Features,
    Installation,
    Usage,
    License,
}

impl ReadmeSection {
    /// Get all required sections
    pub fn all() -> &'static [ReadmeSection] {
        &[
            ReadmeSection::Title,
            ReadmeSection::Badges,
            ReadmeSection::Overview,
            ReadmeSection::Features,
            ReadmeSection::Installation,
            ReadmeSection::Usage,
            ReadmeSection::License,
        ]
    }
    
    /// Get display name for the section
    pub fn display_name(&self) -> &'static str {
        match self {
            ReadmeSection::Title => "Title",
            ReadmeSection::Badges => "Badges",
            ReadmeSection::Overview => "Overview",
            ReadmeSection::Features => "Features",
            ReadmeSection::Installation => "Installation",
            ReadmeSection::Usage => "Usage",
            ReadmeSection::License => "License",
        }
    }
    
    /// Get patterns to detect this section
    pub fn detection_patterns(&self) -> &'static [&'static str] {
        match self {
            ReadmeSection::Title => &["# "],
            ReadmeSection::Badges => &["[![", "[!", "![crates.io]", "![docs.rs]", "![license]"],
            ReadmeSection::Overview => &["## overview", "## about", "## description", "## introduction"],
            ReadmeSection::Features => &["## features", "## highlights", "## capabilities"],
            ReadmeSection::Installation => &["## installation", "## getting started", "## setup", "## install"],
            ReadmeSection::Usage => &["## usage", "## examples", "## quick start", "## example"],
            ReadmeSection::License => &["## license", "licensed under", "mit license", "apache license"],
        }
    }
}

/// Types of badges expected in README
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadgeType {
    CratesIo,
    DocsRs,
    License,
}

impl BadgeType {
    /// Get all badge types
    pub fn all() -> &'static [BadgeType] {
        &[BadgeType::CratesIo, BadgeType::DocsRs, BadgeType::License]
    }
    
    /// Get display name for the badge
    pub fn display_name(&self) -> &'static str {
        match self {
            BadgeType::CratesIo => "crates.io",
            BadgeType::DocsRs => "docs.rs",
            BadgeType::License => "License",
        }
    }
    
    /// Get patterns to detect this badge
    pub fn detection_patterns(&self) -> &'static [&'static str] {
        match self {
            BadgeType::CratesIo => &["crates.io", "crates-io", "img.shields.io/crates"],
            BadgeType::DocsRs => &["docs.rs", "docsrs"],
            BadgeType::License => &["license", "img.shields.io/badge/license"],
        }
    }
}

/// Validator for documentation files
pub struct DocumentationValidator {
    required_sections: Vec<ReadmeSection>,
    required_badges: Vec<BadgeType>,
}

impl DocumentationValidator {
    /// Create a new documentation validator
    pub fn new() -> Self {
        Self {
            required_sections: ReadmeSection::all().to_vec(),
            required_badges: BadgeType::all().to_vec(),
        }
    }
    
    /// Validate a crate's documentation
    pub fn validate(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check required files exist
        violations.extend(self.validate_required_files(crate_info));
        
        // Check README structure if it exists
        if crate_info.has_file("README.md") {
            let readme_path = crate_info.path.join("README.md");
            if let Ok(content) = std::fs::read_to_string(&readme_path) {
                violations.extend(self.validate_readme_structure(crate_info, &content));
                violations.extend(self.validate_readme_badges(crate_info, &content));
            }
        }
        
        // Check CHANGELOG format if it exists
        if crate_info.has_file("CHANGELOG.md") {
            let changelog_path = crate_info.path.join("CHANGELOG.md");
            if let Ok(content) = std::fs::read_to_string(&changelog_path) {
                violations.extend(self.validate_changelog(crate_info, &content));
            }
        }
        
        // Check CONTRIBUTING.md for top-level tools
        if crate_info.crate_type == CrateType::TopLevelTool {
            violations.extend(self.validate_contributing(crate_info));
        }
        
        violations
    }
    
    /// Validate required documentation files exist
    pub fn validate_required_files(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check README.md
        if !crate_info.has_file("README.md") {
            let fix = Fix::new("Create README.md with standard template", true)
                .with_change(FileChange::new(
                    crate_info.path.join("README.md"),
                    ChangeOperation::Create {
                        content: self.generate_readme_template(crate_info),
                    },
                ));
            
            violations.push(
                Violation::new(
                    "doc-readme-missing",
                    &crate_info.name,
                    ViolationCategory::Documentation,
                    Severity::Error,
                    "Crate must have a README.md file",
                )
                .with_file(crate_info.path.join("README.md"))
                .with_fix(fix),
            );
        }
        
        // Check CHANGELOG.md
        if !crate_info.has_file("CHANGELOG.md") {
            let fix = Fix::new("Create CHANGELOG.md following Keep a Changelog format", true)
                .with_change(FileChange::new(
                    crate_info.path.join("CHANGELOG.md"),
                    ChangeOperation::Create {
                        content: self.generate_changelog_template(crate_info),
                    },
                ));
            
            violations.push(
                Violation::new(
                    "doc-changelog-missing",
                    &crate_info.name,
                    ViolationCategory::Documentation,
                    Severity::Error,
                    "Crate must have a CHANGELOG.md file following Keep a Changelog format",
                )
                .with_file(crate_info.path.join("CHANGELOG.md"))
                .with_fix(fix),
            );
        }
        
        violations
    }
    
    /// Validate README structure has all required sections
    pub fn validate_readme_structure(&self, crate_info: &CrateInfo, content: &str) -> Vec<Violation> {
        let mut violations = Vec::new();
        let content_lower = content.to_lowercase();
        let readme_path = crate_info.path.join("README.md");
        
        for section in &self.required_sections {
            if !self.detect_section(&content_lower, section) {
                violations.push(
                    Violation::new(
                        format!("doc-readme-section-{}", section.display_name().to_lowercase()),
                        &crate_info.name,
                        ViolationCategory::Documentation,
                        Severity::Warning,
                        format!("README.md should have a {} section", section.display_name()),
                    )
                    .with_file(&readme_path),
                );
            }
        }
        
        violations
    }
    
    /// Validate README has required badges
    pub fn validate_readme_badges(&self, crate_info: &CrateInfo, content: &str) -> Vec<Violation> {
        let mut violations = Vec::new();
        let content_lower = content.to_lowercase();
        let readme_path = crate_info.path.join("README.md");
        
        for badge in &self.required_badges {
            if !self.detect_badge(&content_lower, badge) {
                violations.push(
                    Violation::new(
                        format!("doc-readme-badge-{}", badge.display_name().to_lowercase().replace('.', "-")),
                        &crate_info.name,
                        ViolationCategory::Documentation,
                        Severity::Warning,
                        format!("README.md should have a {} badge", badge.display_name()),
                    )
                    .with_file(&readme_path),
                );
            }
        }
        
        violations
    }
    
    /// Validate CHANGELOG follows Keep a Changelog format
    pub fn validate_changelog(&self, crate_info: &CrateInfo, content: &str) -> Vec<Violation> {
        let mut violations = Vec::new();
        let content_lower = content.to_lowercase();
        let changelog_path = crate_info.path.join("CHANGELOG.md");
        
        // Check for Keep a Changelog format indicators
        let has_keepachangelog = content_lower.contains("keep a changelog") 
            || content_lower.contains("keepachangelog");
        let _has_semver = content_lower.contains("semantic versioning") 
            || content_lower.contains("semver");
        let has_unreleased = content_lower.contains("## [unreleased]") 
            || content_lower.contains("## unreleased");
        let has_version_section = content.contains("## [") 
            || content.contains("## v");
        
        if !has_keepachangelog && !has_unreleased && !has_version_section {
            violations.push(
                Violation::new(
                    "doc-changelog-format",
                    &crate_info.name,
                    ViolationCategory::Documentation,
                    Severity::Warning,
                    "CHANGELOG.md should follow Keep a Changelog format",
                )
                .with_file(&changelog_path),
            );
        }
        
        // Check for standard sections
        let standard_sections = ["added", "changed", "deprecated", "removed", "fixed", "security"];
        let has_any_section = standard_sections.iter().any(|s| content_lower.contains(&format!("### {}", s)));
        
        if !has_any_section && content.len() > 100 {
            violations.push(
                Violation::new(
                    "doc-changelog-sections",
                    &crate_info.name,
                    ViolationCategory::Documentation,
                    Severity::Info,
                    "CHANGELOG.md should use standard sections: Added, Changed, Deprecated, Removed, Fixed, Security",
                )
                .with_file(&changelog_path),
            );
        }
        
        violations
    }
    
    /// Validate CONTRIBUTING.md exists for top-level tools
    pub fn validate_contributing(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        if !crate_info.has_file("CONTRIBUTING.md") {
            violations.push(
                Violation::new(
                    "doc-contributing-missing",
                    &crate_info.name,
                    ViolationCategory::Documentation,
                    Severity::Info,
                    "Top-level tool crates should have a CONTRIBUTING.md file",
                )
                .with_file(crate_info.path.join("CONTRIBUTING.md")),
            );
        }
        
        violations
    }
    
    /// Detect if a section exists in the content
    fn detect_section(&self, content: &str, section: &ReadmeSection) -> bool {
        section.detection_patterns().iter().any(|pattern| content.contains(pattern))
    }
    
    /// Detect if a badge exists in the content
    fn detect_badge(&self, content: &str, badge: &BadgeType) -> bool {
        badge.detection_patterns().iter().any(|pattern| content.contains(pattern))
    }
    
    /// Generate a README template for a crate
    fn generate_readme_template(&self, crate_info: &CrateInfo) -> String {
        let name = &crate_info.name;
        let description = crate_info.package().description.as_deref().unwrap_or("A DX ecosystem crate");
        
        format!(r#"# {name}

[![crates.io](https://img.shields.io/crates/v/{name}.svg)](https://crates.io/crates/{name})
[![docs.rs](https://docs.rs/{name}/badge.svg)](https://docs.rs/{name})
[![license](https://img.shields.io/crates/l/{name}.svg)](LICENSE)

## Overview

{description}

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
{name} = "0.1"
```

## Usage

```rust
use {lib_name}::prelude::*;

// Example code here
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
"#, name = name, description = description, lib_name = name.replace('-', "_"))
    }
    
    /// Generate a CHANGELOG template
    fn generate_changelog_template(&self, crate_info: &CrateInfo) -> String {
        format!(r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release of {}

### Changed

### Deprecated

### Removed

### Fixed

### Security
"#, crate_info.name)
    }
}

impl Default for DocumentationValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CargoToml, CrateType};
    use std::path::PathBuf;

    fn create_test_crate(name: &str, files: Vec<&str>) -> CrateInfo {
        let cargo_toml = CargoToml::parse(&format!(r#"
[package]
name = "{}"
version = "0.1.0"
description = "Test crate"
"#, name)).unwrap();
        
        CrateInfo::new(
            name.to_string(),
            PathBuf::from(format!("crates/{}", name)),
            cargo_toml,
            CrateType::Library,
        ).with_files(files.into_iter().map(PathBuf::from).collect())
    }

    #[test]
    fn test_missing_readme() {
        let validator = DocumentationValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs"]);
        
        let violations = validator.validate_required_files(&crate_info);
        assert!(violations.iter().any(|v| v.id == "doc-readme-missing"));
    }

    #[test]
    fn test_missing_changelog() {
        let validator = DocumentationValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "README.md", "src/lib.rs"]);
        
        let violations = validator.validate_required_files(&crate_info);
        assert!(violations.iter().any(|v| v.id == "doc-changelog-missing"));
    }

    #[test]
    fn test_all_files_present() {
        let validator = DocumentationValidator::new();
        let crate_info = create_test_crate("test-crate", vec![
            "Cargo.toml", "README.md", "CHANGELOG.md", "src/lib.rs"
        ]);
        
        let violations = validator.validate_required_files(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_readme_section_detection() {
        let validator = DocumentationValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["README.md"]);
        
        let complete_readme = r#"# Test Crate

[![crates.io](https://crates.io/crates/test)]
[![docs.rs](https://docs.rs/test)]
[![license](MIT)]

## Overview

This is a test crate.

## Features

- Feature 1

## Installation

Add to Cargo.toml

## Usage

```rust
use test_crate::*;
```

## License

MIT
"#;
        
        let violations = validator.validate_readme_structure(&crate_info, complete_readme);
        assert!(violations.is_empty(), "Expected no violations for complete README, got: {:?}", violations);
    }

    #[test]
    fn test_readme_missing_sections() {
        let validator = DocumentationValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["README.md"]);
        
        let incomplete_readme = "# Test Crate\n\nSome content.";
        
        let violations = validator.validate_readme_structure(&crate_info, incomplete_readme);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_badge_detection() {
        let validator = DocumentationValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["README.md"]);
        
        let readme_with_badges = r#"# Test
[![crates.io](https://img.shields.io/crates/v/test.svg)]
[![docs.rs](https://docs.rs/test/badge.svg)]
[![license](https://img.shields.io/badge/license-MIT-blue.svg)]
"#;
        
        let violations = validator.validate_readme_badges(&crate_info, readme_with_badges);
        assert!(violations.is_empty(), "Expected no badge violations, got: {:?}", violations);
    }

    #[test]
    fn test_changelog_format_validation() {
        let validator = DocumentationValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["CHANGELOG.md"]);
        
        let good_changelog = r#"# Changelog

Based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- New feature
"#;
        
        let violations = validator.validate_changelog(&crate_info, good_changelog);
        assert!(violations.is_empty(), "Expected no violations for good changelog, got: {:?}", violations);
    }

    #[test]
    fn test_section_enum() {
        assert_eq!(ReadmeSection::Title.display_name(), "Title");
        assert_eq!(ReadmeSection::all().len(), 7);
    }

    #[test]
    fn test_badge_enum() {
        assert_eq!(BadgeType::CratesIo.display_name(), "crates.io");
        assert_eq!(BadgeType::all().len(), 3);
    }
}
