//! Structure validator for crate directory structure

use crate::models::{
    CrateInfo, Violation, ViolationCategory, Severity, Fix, FileChange, ChangeOperation,
};

/// Required directories for a crate
#[allow(dead_code)]
const REQUIRED_DIRS: &[&str] = &["src"];

/// Optional standard directories
#[allow(dead_code)]
const OPTIONAL_DIRS: &[&str] = &["tests", "benches", "examples", "docs"];

/// Directories that should have .gitignore if they contain build artifacts
const BUILD_ARTIFACT_DIRS: &[&str] = &["target", "pkg", "dist", "build"];

/// Validator for directory structure
pub struct StructureValidator;

impl StructureValidator {
    /// Create a new structure validator
    pub fn new() -> Self {
        Self
    }
    
    /// Validate a crate's directory structure
    pub fn validate(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check src directory exists
        violations.extend(self.validate_src_directory(crate_info));
        
        // Check for entry point (lib.rs or main.rs)
        violations.extend(self.validate_entry_point(crate_info));
        
        // Check for .gitignore in crates with build artifacts
        violations.extend(self.validate_gitignore(crate_info));
        
        violations
    }
    
    /// Validate that src/ directory exists
    pub fn validate_src_directory(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        let has_src = crate_info.files.iter().any(|f| {
            let path_str = f.to_string_lossy();
            path_str.starts_with("src/") || path_str.starts_with("src\\") || path_str == "src"
        });
        
        if !has_src {
            let fix = Fix::new("Create src/ directory with lib.rs", true)
                .with_change(FileChange::new(
                    crate_info.path.join("src/lib.rs"),
                    ChangeOperation::Create {
                        content: self.generate_lib_rs_template(crate_info),
                    },
                ));
            
            violations.push(
                Violation::new(
                    "structure-src-missing",
                    &crate_info.name,
                    ViolationCategory::Structure,
                    Severity::Error,
                    "Crate must have a src/ directory",
                )
                .with_file(crate_info.path.join("src"))
                .with_fix(fix),
            );
        }
        
        violations
    }
    
    /// Validate that lib.rs or main.rs exists in src/
    pub fn validate_entry_point(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        let has_lib_rs = crate_info.files.iter().any(|f| {
            let path_str = f.to_string_lossy();
            path_str == "src/lib.rs" || path_str == "src\\lib.rs"
        });
        
        let has_main_rs = crate_info.files.iter().any(|f| {
            let path_str = f.to_string_lossy();
            path_str == "src/main.rs" || path_str == "src\\main.rs"
        });
        
        if !has_lib_rs && !has_main_rs {
            // Check if src directory exists first
            let has_src = crate_info.files.iter().any(|f| {
                let path_str = f.to_string_lossy();
                path_str.starts_with("src/") || path_str.starts_with("src\\")
            });
            
            if has_src {
                let fix = Fix::new("Create src/lib.rs entry point", true)
                    .with_change(FileChange::new(
                        crate_info.path.join("src/lib.rs"),
                        ChangeOperation::Create {
                            content: self.generate_lib_rs_template(crate_info),
                        },
                    ));
                
                violations.push(
                    Violation::new(
                        "structure-entry-point-missing",
                        &crate_info.name,
                        ViolationCategory::Structure,
                        Severity::Error,
                        "Crate must have src/lib.rs or src/main.rs",
                    )
                    .with_file(crate_info.path.join("src/lib.rs"))
                    .with_fix(fix),
                );
            }
        }
        
        violations
    }
    
    /// Validate .gitignore exists in crates with build artifacts
    pub fn validate_gitignore(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check if crate has any build artifact directories
        let has_build_artifacts = crate_info.files.iter().any(|f| {
            let path_str = f.to_string_lossy();
            BUILD_ARTIFACT_DIRS.iter().any(|dir| {
                path_str.starts_with(&format!("{}/", dir)) 
                    || path_str.starts_with(&format!("{}\\", dir))
                    || path_str == *dir
            })
        });
        
        if has_build_artifacts {
            let has_gitignore = crate_info.has_file(".gitignore");
            
            if !has_gitignore {
                let fix = Fix::new("Create .gitignore for build artifacts", true)
                    .with_change(FileChange::new(
                        crate_info.path.join(".gitignore"),
                        ChangeOperation::Create {
                            content: self.generate_gitignore_template(),
                        },
                    ));
                
                violations.push(
                    Violation::new(
                        "structure-gitignore-missing",
                        &crate_info.name,
                        ViolationCategory::Structure,
                        Severity::Warning,
                        "Crate with build artifacts should have a .gitignore file",
                    )
                    .with_file(crate_info.path.join(".gitignore"))
                    .with_fix(fix),
                );
            }
        }
        
        violations
    }
    
    /// Check if a crate has a specific directory
    pub fn has_directory(&self, crate_info: &CrateInfo, dir_name: &str) -> bool {
        crate_info.files.iter().any(|f| {
            let path_str = f.to_string_lossy();
            path_str.starts_with(&format!("{}/", dir_name))
                || path_str.starts_with(&format!("{}\\", dir_name))
                || path_str == dir_name
        })
    }
    
    /// Generate a lib.rs template
    fn generate_lib_rs_template(&self, crate_info: &CrateInfo) -> String {
        let description = crate_info.package().description.as_deref().unwrap_or("A DX ecosystem crate");
        format!(r#"//! {}
//!
//! This crate is part of the DX ecosystem.

/// Re-export commonly used items
pub mod prelude {{
    // Add commonly used items here
}}
"#, description)
    }
    
    /// Generate a .gitignore template
    fn generate_gitignore_template(&self) -> String {
        r#"# Build artifacts
/target/
/pkg/
/dist/
/build/

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Cargo
Cargo.lock
"#.to_string()
    }
}

impl Default for StructureValidator {
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
    fn test_missing_src_directory() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml"]);
        
        let violations = validator.validate_src_directory(&crate_info);
        assert!(violations.iter().any(|v| v.id == "structure-src-missing"));
    }

    #[test]
    fn test_src_directory_present() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs"]);
        
        let violations = validator.validate_src_directory(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_missing_entry_point() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/mod.rs"]);
        
        let violations = validator.validate_entry_point(&crate_info);
        assert!(violations.iter().any(|v| v.id == "structure-entry-point-missing"));
    }

    #[test]
    fn test_lib_rs_present() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs"]);
        
        let violations = validator.validate_entry_point(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_main_rs_present() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/main.rs"]);
        
        let violations = validator.validate_entry_point(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_both_entry_points_present() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs", "src/main.rs"]);
        
        let violations = validator.validate_entry_point(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_gitignore_needed_with_target() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs", "target/debug"]);
        
        let violations = validator.validate_gitignore(&crate_info);
        assert!(violations.iter().any(|v| v.id == "structure-gitignore-missing"));
    }

    #[test]
    fn test_gitignore_present() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs", "target/debug", ".gitignore"]);
        
        let violations = validator.validate_gitignore(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_no_gitignore_needed_without_artifacts() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs"]);
        
        let violations = validator.validate_gitignore(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_has_directory() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec!["Cargo.toml", "src/lib.rs", "tests/test.rs"]);
        
        assert!(validator.has_directory(&crate_info, "src"));
        assert!(validator.has_directory(&crate_info, "tests"));
        assert!(!validator.has_directory(&crate_info, "benches"));
    }

    #[test]
    fn test_complete_structure() {
        let validator = StructureValidator::new();
        let crate_info = create_test_crate("test-crate", vec![
            "Cargo.toml",
            "src/lib.rs",
            "tests/integration.rs",
            "README.md",
        ]);
        
        let violations = validator.validate(&crate_info);
        assert!(violations.is_empty(), "Expected no violations, got: {:?}", violations);
    }
}
