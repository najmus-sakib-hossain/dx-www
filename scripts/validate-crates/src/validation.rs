//! Validation module
//!
//! Performs compliance checks against professionalization requirements.

use crate::naming::{validate_filename, validate_naming};
use crate::readme::analyze_readme;
use crate::scanner::{is_library_crate, CrateInfo};
use std::path::Path;

/// Result of validating a single crate
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub crate_path: std::path::PathBuf,
    pub issues: Vec<ValidationIssue>,
    pub passed: bool,
}

/// A single validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub category: IssueCategory,
    pub message: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,   // Must fix
    Warning, // Should fix
    Info,    // Nice to have
}

#[derive(Debug, Clone, PartialEq)]
pub enum IssueCategory {
    MissingFile,
    IncompleteReadme,
    NamingConvention,
    LicenseCompliance,
    DevelopmentArtifact,
    CargoMetadata,
    InappropriateContent,
}

/// Validate a single crate against all requirements
pub fn validate_crate(crate_info: &CrateInfo) -> ValidationResult {
    let mut issues = Vec::new();
    let crate_path = Path::new("crates").join(&crate_info.path);

    // 1. Check required files (Requirements 1.1, 2.1)
    validate_required_files(crate_info, &mut issues);

    // 2. Validate README content (Requirements 1.2, 1.3, 1.4)
    validate_readme_content(&crate_path, crate_info, &mut issues);

    // 3. Validate naming conventions (Requirements 3.1, 3.3)
    validate_naming_conventions(&crate_path, crate_info, &mut issues);

    // 4. Validate Cargo.toml metadata (Requirements 7.1, 7.2, 7.3)
    validate_cargo_metadata(crate_info, &mut issues);

    // 5. Check for forbidden files (Requirements 2.2, 4.4, 5.1, 5.3)
    validate_forbidden_files(crate_info, &mut issues);

    // 6. Check for file typos (Requirement 3.5)
    validate_file_typos(crate_info, &mut issues);

    let passed = !issues.iter().any(|i| matches!(i.severity, Severity::Error));

    ValidationResult {
        crate_path: crate_info.path.clone(),
        issues,
        passed,
    }
}

/// Validate all crates and return aggregated results
pub fn validate_all(crates: &[CrateInfo]) -> Vec<ValidationResult> {
    crates.iter().map(validate_crate).collect()
}

/// Check for required files in a crate
fn validate_required_files(crate_info: &CrateInfo, issues: &mut Vec<ValidationIssue>) {
    // Check for README.md
    if !crate_info.files.iter().any(|f| f.eq_ignore_ascii_case("readme.md")) {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::MissingFile,
            message: "Missing README.md file".to_string(),
            fix_suggestion: Some("Create a README.md using the template".to_string()),
        });
    }

    // Check for src/ directory
    if !crate_info.files.contains(&"src".to_string()) {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::MissingFile,
            message: "Missing src/ directory".to_string(),
            fix_suggestion: None,
        });
    }
}

/// Validate README content
fn validate_readme_content(
    crate_path: &Path,
    crate_info: &CrateInfo,
    issues: &mut Vec<ValidationIssue>,
) {
    let analysis = analyze_readme(crate_path);

    if !analysis.exists {
        return; // Already reported as missing file
    }

    if analysis.is_empty {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::IncompleteReadme,
            message: "README.md is empty or nearly empty".to_string(),
            fix_suggestion: Some("Add meaningful content to README.md".to_string()),
        });
        return;
    }

    // Check for title
    if !analysis.has_title {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::IncompleteReadme,
            message: "README.md missing title (H1 heading)".to_string(),
            fix_suggestion: Some(format!("Add '# {}' at the top", crate_info.name)),
        });
    }

    // Check for description
    if !analysis.has_description {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::IncompleteReadme,
            message: "README.md missing description paragraph".to_string(),
            fix_suggestion: Some("Add a description after the title".to_string()),
        });
    }

    // Check for installation section
    if !analysis.has_installation {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::IncompleteReadme,
            message: "README.md missing installation section".to_string(),
            fix_suggestion: Some("Add an '## Installation' section".to_string()),
        });
    }

    // Check for usage section
    if !analysis.has_usage {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::IncompleteReadme,
            message: "README.md missing usage/examples section".to_string(),
            fix_suggestion: Some("Add a '## Usage' section with examples".to_string()),
        });
    }

    // Check for license section
    if !analysis.has_license {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::IncompleteReadme,
            message: "README.md missing license information".to_string(),
            fix_suggestion: Some("Add a '## License' section".to_string()),
        });
    }

    // Check for inappropriate content
    if analysis.contains_task_instructions {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::InappropriateContent,
            message: "README.md contains task instructions".to_string(),
            fix_suggestion: Some("Remove task instructions and replace with documentation".to_string()),
        });
    }

    if analysis.contains_raw_prompts {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::InappropriateContent,
            message: "README.md contains raw AI prompts".to_string(),
            fix_suggestion: Some("Remove AI prompts and replace with documentation".to_string()),
        });
    }

    if analysis.contains_dev_notes {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::InappropriateContent,
            message: "README.md contains development notes".to_string(),
            fix_suggestion: Some("Move development notes to docs/development/".to_string()),
        });
    }

    // Check for subcrate documentation if this crate has subcrates
    if crate_info.has_subcrates && !analysis.has_subcrate_table {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::IncompleteReadme,
            message: "Crate has subcrates but README lacks subcrate documentation".to_string(),
            fix_suggestion: Some("Add a table documenting all subcrates".to_string()),
        });
    }
}

/// Validate naming conventions
fn validate_naming_conventions(
    crate_path: &Path,
    crate_info: &CrateInfo,
    issues: &mut Vec<ValidationIssue>,
) {
    let validation = validate_naming(crate_path, &crate_info.name);

    if !validation.is_valid_kebab_case {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::NamingConvention,
            message: format!(
                "Directory name '{}' is not valid kebab-case",
                validation.dir_name
            ),
            fix_suggestion: validation.suggested_name.clone(),
        });
    }

    if !validation.names_match {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::NamingConvention,
            message: format!(
                "Directory name '{}' doesn't match package name '{}'",
                validation.dir_name, validation.package_name
            ),
            fix_suggestion: validation.suggested_name,
        });
    }

    if validation.has_typo {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::NamingConvention,
            message: validation
                .typo_description
                .unwrap_or_else(|| "Name contains typo".to_string()),
            fix_suggestion: None,
        });
    }
}

/// Validate Cargo.toml metadata
fn validate_cargo_metadata(crate_info: &CrateInfo, issues: &mut Vec<ValidationIssue>) {
    let package = match &crate_info.manifest.package {
        Some(p) => p,
        None => {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                category: IssueCategory::CargoMetadata,
                message: "Cargo.toml missing [package] section".to_string(),
                fix_suggestion: None,
            });
            return;
        }
    };

    // Check required fields
    if package.name.is_none() {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::CargoMetadata,
            message: "Cargo.toml missing 'name' field".to_string(),
            fix_suggestion: None,
        });
    }

    if package.version.is_none() {
        issues.push(ValidationIssue {
            severity: Severity::Error,
            category: IssueCategory::CargoMetadata,
            message: "Cargo.toml missing 'version' field".to_string(),
            fix_suggestion: None,
        });
    }

    if package.edition.is_none() {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::CargoMetadata,
            message: "Cargo.toml missing 'edition' field".to_string(),
            fix_suggestion: Some("Add edition = \"2021\"".to_string()),
        });
    }

    if package.description.is_none() {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::CargoMetadata,
            message: "Cargo.toml missing 'description' field".to_string(),
            fix_suggestion: Some("Add a description of the crate".to_string()),
        });
    }

    if package.license.is_none() {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::LicenseCompliance,
            message: "Cargo.toml missing 'license' field".to_string(),
            fix_suggestion: Some("Add license = \"MIT OR Apache-2.0\"".to_string()),
        });
    }

    // Check for repository link
    if package.repository.is_none() {
        issues.push(ValidationIssue {
            severity: Severity::Info,
            category: IssueCategory::CargoMetadata,
            message: "Cargo.toml missing 'repository' field".to_string(),
            fix_suggestion: Some("Add repository URL".to_string()),
        });
    }
}

/// Check for forbidden files
fn validate_forbidden_files(crate_info: &CrateInfo, issues: &mut Vec<ValidationIssue>) {
    // Check for Cargo.lock in library crates
    if is_library_crate(crate_info) && crate_info.files.contains(&"Cargo.lock".to_string()) {
        issues.push(ValidationIssue {
            severity: Severity::Warning,
            category: IssueCategory::DevelopmentArtifact,
            message: "Library crate should not have Cargo.lock".to_string(),
            fix_suggestion: Some("Remove Cargo.lock and add to .gitignore".to_string()),
        });
    }

    // Check for .env files (excluding .env.example)
    for file in &crate_info.files {
        if file.starts_with(".env") && file != ".env.example" {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                category: IssueCategory::DevelopmentArtifact,
                message: format!("Sensitive file '{}' should not be committed", file),
                fix_suggestion: Some("Remove and add to .gitignore".to_string()),
            });
        }
    }

    // Check for progress tracking files
    let progress_patterns = [
        "PHASE",
        "TASKLIST",
        "PROGRESS",
        "_COMPLETE",
        "_STATUS",
    ];

    for file in &crate_info.files {
        let upper = file.to_uppercase();
        if progress_patterns.iter().any(|p| upper.contains(p)) && file.ends_with(".md") {
            issues.push(ValidationIssue {
                severity: Severity::Warning,
                category: IssueCategory::DevelopmentArtifact,
                message: format!("Progress tracking file '{}' should be moved to docs/archive/", file),
                fix_suggestion: Some("Move to docs/archive/".to_string()),
            });
        }
    }
}

/// Check for typos in file names
fn validate_file_typos(crate_info: &CrateInfo, issues: &mut Vec<ValidationIssue>) {
    for file in &crate_info.files {
        if let Some(corrected) = validate_filename(file) {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                category: IssueCategory::NamingConvention,
                message: format!("File '{}' contains typo", file),
                fix_suggestion: Some(format!("Rename to '{}'", corrected)),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{CargoManifest, CrateType, PackageInfo};

    fn create_test_crate(name: &str, files: Vec<&str>) -> CrateInfo {
        CrateInfo {
            path: std::path::PathBuf::from(name),
            name: name.to_string(),
            crate_type: CrateType::Library,
            has_subcrates: false,
            files: files.into_iter().map(String::from).collect(),
            manifest: CargoManifest {
                package: Some(PackageInfo {
                    name: Some(name.to_string()),
                    version: Some("0.1.0".to_string()),
                    edition: Some("2021".to_string()),
                    description: Some("Test crate".to_string()),
                    license: Some("MIT".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_validate_missing_readme() {
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src"]);
        let result = validate_crate(&crate_info);
        
        assert!(result.issues.iter().any(|i| 
            i.message.contains("Missing README.md")
        ));
    }

    #[test]
    fn test_validate_cargo_lock_in_library() {
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "Cargo.lock", "src", "README.md"]);
        let result = validate_crate(&crate_info);
        
        assert!(result.issues.iter().any(|i| 
            i.message.contains("Cargo.lock")
        ));
    }

    #[test]
    fn test_validate_env_file() {
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", ".env", "src", "README.md"]);
        let result = validate_crate(&crate_info);
        
        assert!(result.issues.iter().any(|i| 
            i.message.contains(".env")
        ));
    }

    #[test]
    fn test_validate_progress_file() {
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "PHASE3_COMPLETE.md", "src", "README.md"]);
        let result = validate_crate(&crate_info);
        
        assert!(result.issues.iter().any(|i| 
            i.message.contains("Progress tracking file")
        ));
    }
}
