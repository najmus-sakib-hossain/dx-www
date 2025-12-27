//! Unified auto-fixer that combines all fix capabilities
//!
//! Provides a single entry point for generating and applying all fixes.

use std::io;
use crate::{CrateInfo, Fix, FileChange, ChangeOperation, Violation};
use super::{CargoFixer, DocFixer};

/// Unified auto-fixer for all violation types
pub struct AutoFixer {
    cargo_fixer: CargoFixer,
    doc_fixer: DocFixer,
}

impl AutoFixer {
    /// Create a new AutoFixer
    pub fn new() -> Self {
        Self {
            cargo_fixer: CargoFixer::new(),
            doc_fixer: DocFixer::new(),
        }
    }

    /// Generate all fixes for a crate
    pub fn generate_fixes(&self, crate_info: &CrateInfo) -> Vec<Fix> {
        let mut fixes = Vec::new();
        
        // Cargo.toml fixes
        fixes.extend(self.cargo_fixer.generate_all_fixes(crate_info));
        
        // Documentation fixes
        fixes.extend(self.doc_fixer.generate_all_fixes(crate_info));
        
        fixes
    }

    /// Generate fixes from violations
    pub fn fixes_from_violations(&self, violations: &[Violation]) -> Vec<Fix> {
        violations
            .iter()
            .filter_map(|v| v.fix.clone())
            .filter(|f| f.auto_fixable)
            .collect()
    }

    /// Apply a single fix
    ///
    /// Returns Ok(true) if the fix was applied, Ok(false) if skipped, Err on failure
    pub fn apply_fix(&self, fix: &Fix) -> io::Result<bool> {
        if !fix.auto_fixable {
            return Ok(false);
        }

        for change in &fix.changes {
            self.apply_change(change)?;
        }

        Ok(true)
    }

    /// Apply a file change
    fn apply_change(&self, change: &FileChange) -> io::Result<()> {
        match &change.operation {
            ChangeOperation::Create { content } => {
                // Create parent directories if needed
                if let Some(parent) = change.file.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&change.file, content)?;
            }
            ChangeOperation::Modify { old, new } => {
                if change.file.exists() {
                    let content = std::fs::read_to_string(&change.file)?;
                    let new_content = content.replace(old, new);
                    std::fs::write(&change.file, new_content)?;
                }
            }
            ChangeOperation::Delete => {
                if change.file.exists() {
                    std::fs::remove_file(&change.file)?;
                }
            }
        }
        Ok(())
    }

    /// Apply all fixes, returning the count of applied fixes
    pub fn apply_all_fixes(&self, fixes: &[Fix]) -> io::Result<usize> {
        let mut applied = 0;
        
        for fix in fixes {
            if self.apply_fix(fix)? {
                applied += 1;
            }
        }
        
        Ok(applied)
    }

    /// Preview fixes without applying them
    pub fn preview_fixes(&self, fixes: &[Fix]) -> Vec<FixPreview> {
        fixes
            .iter()
            .map(|fix| FixPreview {
                description: fix.description.clone(),
                auto_fixable: fix.auto_fixable,
                files_affected: fix.changes.iter().map(|c| c.file.clone()).collect(),
            })
            .collect()
    }
}

impl Default for AutoFixer {
    fn default() -> Self {
        Self::new()
    }
}

/// Preview of a fix without applying it
#[derive(Debug, Clone)]
pub struct FixPreview {
    /// Description of what the fix does
    pub description: String,
    /// Whether this fix can be applied automatically
    pub auto_fixable: bool,
    /// Files that would be affected
    pub files_affected: Vec<std::path::PathBuf>,
}

/// Result of applying fixes
#[derive(Debug, Clone)]
pub struct FixResult {
    /// Number of fixes applied
    pub applied: usize,
    /// Number of fixes skipped
    pub skipped: usize,
    /// Errors encountered
    pub errors: Vec<String>,
}

impl FixResult {
    /// Create a new FixResult
    pub fn new() -> Self {
        Self {
            applied: 0,
            skipped: 0,
            errors: Vec::new(),
        }
    }

    /// Check if all fixes were successful
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Default for FixResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CargoToml, CrateType};
    use std::path::PathBuf;
    use tempfile::TempDir;

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
    fn test_generate_fixes() {
        let fixer = AutoFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-test",
            r#"[package]
name = "dx-test"
version = "0.1.0"
"#,
            CrateType::Library,
            vec!["Cargo.toml", "src/lib.rs"],
        );
        
        let fixes = fixer.generate_fixes(&crate_info);
        // Should have multiple fixes for missing fields and docs
        assert!(!fixes.is_empty());
    }

    #[test]
    fn test_preview_fixes() {
        let fixer = AutoFixer::new();
        
        let fixes = vec![
            Fix::new("Test fix 1", true)
                .with_change(FileChange::new(
                    PathBuf::from("test.txt"),
                    ChangeOperation::Create { content: "test".to_string() },
                )),
            Fix::new("Test fix 2", false),
        ];
        
        let previews = fixer.preview_fixes(&fixes);
        assert_eq!(previews.len(), 2);
        assert!(previews[0].auto_fixable);
        assert!(!previews[1].auto_fixable);
    }

    #[test]
    fn test_apply_create_fix() {
        let fixer = AutoFixer::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        let fix = Fix::new("Create test file", true)
            .with_change(FileChange::new(
                file_path.clone(),
                ChangeOperation::Create { content: "Hello, World!".to_string() },
            ));
        
        let result = fixer.apply_fix(&fix).unwrap();
        assert!(result);
        assert!(file_path.exists());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "Hello, World!");
    }

    #[test]
    fn test_apply_modify_fix() {
        let fixer = AutoFixer::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Create initial file
        std::fs::write(&file_path, "Hello, World!").unwrap();
        
        let fix = Fix::new("Modify test file", true)
            .with_change(FileChange::new(
                file_path.clone(),
                ChangeOperation::Modify {
                    old: "World".to_string(),
                    new: "Rust".to_string(),
                },
            ));
        
        let result = fixer.apply_fix(&fix).unwrap();
        assert!(result);
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "Hello, Rust!");
    }

    #[test]
    fn test_skip_non_auto_fixable() {
        let fixer = AutoFixer::new();
        
        let fix = Fix::new("Manual fix required", false);
        
        let result = fixer.apply_fix(&fix).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_fix_result() {
        let result = FixResult::new();
        assert!(result.is_success());
        assert_eq!(result.applied, 0);
        assert_eq!(result.skipped, 0);
    }
}
