//! Documentation file auto-fixer
//!
//! Provides functionality to automatically create missing documentation files
//! using the templates module.

use crate::{CrateInfo, CrateType, Fix, FileChange, ChangeOperation};
use super::Templates;

/// Auto-fixer for documentation files
pub struct DocFixer;

impl DocFixer {
    /// Create a new DocFixer
    pub fn new() -> Self {
        Self
    }

    /// Generate a fix for missing README.md
    pub fn fix_missing_readme(&self, crate_info: &CrateInfo) -> Option<Fix> {
        let readme_path = crate_info.path.join("README.md");
        
        if crate_info.has_file("README.md") {
            return None;
        }

        let description = crate_info.cargo_toml.package
            .as_ref()
            .and_then(|p| p.description.as_deref());
        
        let content = Templates::readme_minimal(&crate_info.name, description);

        Some(Fix::new(
            format!("Create README.md for {}", crate_info.name),
            true,
        ).with_change(FileChange::new(
            readme_path,
            ChangeOperation::Create { content },
        )))
    }

    /// Generate a fix for missing CHANGELOG.md
    pub fn fix_missing_changelog(&self, crate_info: &CrateInfo) -> Option<Fix> {
        let changelog_path = crate_info.path.join("CHANGELOG.md");
        
        if crate_info.has_file("CHANGELOG.md") {
            return None;
        }

        let content = Templates::changelog(&crate_info.name);

        Some(Fix::new(
            format!("Create CHANGELOG.md for {}", crate_info.name),
            true,
        ).with_change(FileChange::new(
            changelog_path,
            ChangeOperation::Create { content },
        )))
    }

    /// Generate a fix for missing LICENSE file
    pub fn fix_missing_license(&self, crate_info: &CrateInfo) -> Option<Fix> {
        // Check if any license file exists
        let has_license = crate_info.has_file("LICENSE") 
            || crate_info.has_file("LICENSE.md")
            || (crate_info.has_file("LICENSE-MIT") && crate_info.has_file("LICENSE-APACHE"));
        
        if has_license {
            return None;
        }

        // Create a dual license file
        let license_path = crate_info.path.join("LICENSE");
        let content = Templates::license_dual();

        let mut fix = Fix::new(
            format!("Create LICENSE for {}", crate_info.name),
            true,
        ).with_change(FileChange::new(
            license_path,
            ChangeOperation::Create { content },
        ));

        // Also create LICENSE-MIT and LICENSE-APACHE
        fix = fix.with_change(FileChange::new(
            crate_info.path.join("LICENSE-MIT"),
            ChangeOperation::Create { content: Templates::license_mit() },
        ));
        
        fix = fix.with_change(FileChange::new(
            crate_info.path.join("LICENSE-APACHE"),
            ChangeOperation::Create { content: Templates::license_apache() },
        ));

        Some(fix)
    }

    /// Generate a fix for missing CONTRIBUTING.md (for top-level tools)
    pub fn fix_missing_contributing(&self, crate_info: &CrateInfo) -> Option<Fix> {
        // Only top-level tools need CONTRIBUTING.md
        if crate_info.crate_type != CrateType::TopLevelTool {
            return None;
        }

        if crate_info.has_file("CONTRIBUTING.md") {
            return None;
        }

        let contributing_path = crate_info.path.join("CONTRIBUTING.md");
        let content = Templates::contributing(&crate_info.name);

        Some(Fix::new(
            format!("Create CONTRIBUTING.md for {}", crate_info.name),
            true,
        ).with_change(FileChange::new(
            contributing_path,
            ChangeOperation::Create { content },
        )))
    }

    /// Generate a fix for missing .gitignore
    pub fn fix_missing_gitignore(&self, crate_info: &CrateInfo) -> Option<Fix> {
        // Check if .gitignore is needed (has target/ or build artifacts)
        let has_target = crate_info.files.iter().any(|f| {
            f.to_string_lossy().contains("target")
        });
        
        // Also check if it's a binary crate (likely to have build artifacts)
        let is_binary = crate_info.cargo_toml.bins.len() > 0;
        
        if !has_target && !is_binary {
            return None;
        }

        if crate_info.has_file(".gitignore") {
            return None;
        }

        let gitignore_path = crate_info.path.join(".gitignore");
        let content = Templates::gitignore().to_string();

        Some(Fix::new(
            format!("Create .gitignore for {}", crate_info.name),
            true,
        ).with_change(FileChange::new(
            gitignore_path,
            ChangeOperation::Create { content },
        )))
    }

    /// Apply all documentation fixes for a crate
    pub fn generate_all_fixes(&self, crate_info: &CrateInfo) -> Vec<Fix> {
        let mut fixes = Vec::new();
        
        if let Some(fix) = self.fix_missing_readme(crate_info) {
            fixes.push(fix);
        }
        
        if let Some(fix) = self.fix_missing_changelog(crate_info) {
            fixes.push(fix);
        }
        
        if let Some(fix) = self.fix_missing_license(crate_info) {
            fixes.push(fix);
        }
        
        if let Some(fix) = self.fix_missing_contributing(crate_info) {
            fixes.push(fix);
        }
        
        if let Some(fix) = self.fix_missing_gitignore(crate_info) {
            fixes.push(fix);
        }

        fixes
    }
}

impl Default for DocFixer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CargoToml;
    use std::path::PathBuf;

    fn create_test_crate_info(name: &str, cargo_content: &str, crate_type: CrateType, files: Vec<&str>) -> CrateInfo {
        let cargo_toml = CargoToml::parse(cargo_content).unwrap();
        CrateInfo::new(
            name.to_string(),
            PathBuf::from(format!("crates/{}", name)),
            cargo_toml,
            crate_type,
        ).with_files(files.iter().map(|f| PathBuf::from(*f)).collect())
    }

    #[test]
    fn test_fix_missing_readme() {
        let fixer = DocFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-test",
            r#"[package]
name = "dx-test"
version = "0.1.0"
description = "A test crate"
"#,
            CrateType::Library,
            vec!["Cargo.toml", "src/lib.rs"],
        );
        
        let fix = fixer.fix_missing_readme(&crate_info);
        assert!(fix.is_some());
        
        let fix = fix.unwrap();
        assert!(fix.description.contains("README.md"));
        assert!(fix.auto_fixable);
        assert_eq!(fix.changes.len(), 1);
    }

    #[test]
    fn test_no_fix_when_readme_exists() {
        let fixer = DocFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-test",
            r#"[package]
name = "dx-test"
version = "0.1.0"
"#,
            CrateType::Library,
            vec!["Cargo.toml", "src/lib.rs", "README.md"],
        );
        
        let fix = fixer.fix_missing_readme(&crate_info);
        assert!(fix.is_none());
    }

    #[test]
    fn test_fix_missing_changelog() {
        let fixer = DocFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-test",
            r#"[package]
name = "dx-test"
version = "0.1.0"
"#,
            CrateType::Library,
            vec!["Cargo.toml", "src/lib.rs"],
        );
        
        let fix = fixer.fix_missing_changelog(&crate_info);
        assert!(fix.is_some());
        
        let fix = fix.unwrap();
        assert!(fix.description.contains("CHANGELOG.md"));
    }

    #[test]
    fn test_fix_missing_license() {
        let fixer = DocFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-test",
            r#"[package]
name = "dx-test"
version = "0.1.0"
"#,
            CrateType::Library,
            vec!["Cargo.toml", "src/lib.rs"],
        );
        
        let fix = fixer.fix_missing_license(&crate_info);
        assert!(fix.is_some());
        
        let fix = fix.unwrap();
        assert!(fix.description.contains("LICENSE"));
        // Should create LICENSE, LICENSE-MIT, and LICENSE-APACHE
        assert_eq!(fix.changes.len(), 3);
    }

    #[test]
    fn test_contributing_only_for_tools() {
        let fixer = DocFixer::new();
        
        // Library should not get CONTRIBUTING.md
        let lib_crate = create_test_crate_info(
            "dx-test",
            r#"[package]
name = "dx-test"
version = "0.1.0"
"#,
            CrateType::Library,
            vec!["Cargo.toml", "src/lib.rs"],
        );
        
        assert!(fixer.fix_missing_contributing(&lib_crate).is_none());
        
        // Tool should get CONTRIBUTING.md
        let tool_crate = create_test_crate_info(
            "dx-cli",
            r#"[package]
name = "dx-cli"
version = "0.1.0"
"#,
            CrateType::TopLevelTool,
            vec!["Cargo.toml", "src/main.rs"],
        );
        
        assert!(fixer.fix_missing_contributing(&tool_crate).is_some());
    }

    #[test]
    fn test_generate_all_fixes() {
        let fixer = DocFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-test",
            r#"[package]
name = "dx-test"
version = "0.1.0"
"#,
            CrateType::Library,
            vec!["Cargo.toml", "src/lib.rs"],
        );
        
        let fixes = fixer.generate_all_fixes(&crate_info);
        // Should have README, CHANGELOG, and LICENSE fixes
        assert!(fixes.len() >= 3);
    }
}
