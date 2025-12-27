//! Dependency validator for workspace consistency

use crate::models::{
    CrateInfo, Violation, ViolationCategory, Severity, Fix,
};
use std::collections::{HashMap, HashSet};

/// Validator for dependency consistency
pub struct DependencyValidator {
    /// Known internal crate names in the workspace
    internal_crates: HashSet<String>,
    /// Workspace-level dependencies
    workspace_deps: HashMap<String, String>,
}

impl DependencyValidator {
    /// Create a new dependency validator
    pub fn new() -> Self {
        Self {
            internal_crates: HashSet::new(),
            workspace_deps: HashMap::new(),
        }
    }
    
    /// Create a validator with known internal crates
    pub fn with_internal_crates(internal_crates: HashSet<String>) -> Self {
        Self {
            internal_crates,
            workspace_deps: HashMap::new(),
        }
    }
    
    /// Create a validator with workspace dependencies
    pub fn with_workspace_deps(workspace_deps: HashMap<String, String>) -> Self {
        Self {
            internal_crates: HashSet::new(),
            workspace_deps,
        }
    }
    
    /// Create a fully configured validator
    pub fn with_config(internal_crates: HashSet<String>, workspace_deps: HashMap<String, String>) -> Self {
        Self {
            internal_crates,
            workspace_deps,
        }
    }
    
    /// Validate a crate's dependencies
    pub fn validate(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        // Check internal dependencies use workspace syntax
        violations.extend(self.validate_internal_deps(crate_info));
        
        // Check for path dependencies that should use workspace
        violations.extend(self.validate_path_deps(crate_info));
        
        violations
    }
    
    /// Validate that internal dependencies use workspace = true
    pub fn validate_internal_deps(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let cargo_path = crate_info.cargo_toml_path();
        
        for (name, dep) in &crate_info.cargo_toml.dependencies {
            // Check if this is an internal crate
            if self.internal_crates.contains(name) {
                if !dep.is_workspace() {
                    let fix = Fix::new(
                        format!("Change {} to use workspace = true", name),
                        true,
                    );
                    
                    violations.push(
                        Violation::new(
                            "dep-internal-not-workspace",
                            &crate_info.name,
                            ViolationCategory::Dependency,
                            Severity::Error,
                            format!(
                                "Internal dependency '{}' should use {{ workspace = true }}",
                                name
                            ),
                        )
                        .with_file(&cargo_path)
                        .with_fix(fix),
                    );
                }
            }
        }
        
        // Also check dev-dependencies
        for (name, dep) in &crate_info.cargo_toml.dev_dependencies {
            if self.internal_crates.contains(name) && !dep.is_workspace() {
                violations.push(
                    Violation::new(
                        "dep-internal-not-workspace",
                        &crate_info.name,
                        ViolationCategory::Dependency,
                        Severity::Error,
                        format!(
                            "Internal dev-dependency '{}' should use {{ workspace = true }}",
                            name
                        ),
                    )
                    .with_file(&cargo_path)
                    .with_fix(Fix::new(
                        format!("Change {} to use workspace = true", name),
                        true,
                    )),
                );
            }
        }
        
        violations
    }
    
    /// Validate path dependencies that might need workspace syntax
    pub fn validate_path_deps(&self, crate_info: &CrateInfo) -> Vec<Violation> {
        let mut violations = Vec::new();
        let cargo_path = crate_info.cargo_toml_path();
        
        for (name, dep) in &crate_info.cargo_toml.dependencies {
            // If it's a path dependency but not using workspace
            if dep.is_path() && !dep.is_workspace() {
                // Check if this path points to an internal crate
                if let Some(path) = dep.path() {
                    if path.contains("crates/") || path.contains("crates\\") {
                        violations.push(
                            Violation::new(
                                "dep-path-should-use-workspace",
                                &crate_info.name,
                                ViolationCategory::Dependency,
                                Severity::Warning,
                                format!(
                                    "Path dependency '{}' should use workspace = true instead of path",
                                    name
                                ),
                            )
                            .with_file(&cargo_path)
                            .with_fix(Fix::new(
                                format!("Change {} to use workspace = true", name),
                                true,
                            )),
                        );
                    }
                }
            }
        }
        
        violations
    }
    
    /// Find duplicate dependency versions across multiple crates
    pub fn find_duplicates(&self, crates: &[CrateInfo]) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut dep_versions: HashMap<String, Vec<(String, String)>> = HashMap::new();
        
        // Collect all dependency versions
        for crate_info in crates {
            for (name, dep) in &crate_info.cargo_toml.dependencies {
                if let Some(version) = dep.version() {
                    dep_versions
                        .entry(name.clone())
                        .or_default()
                        .push((crate_info.name.clone(), version.to_string()));
                }
            }
        }
        
        // Find duplicates with different versions
        for (dep_name, versions) in dep_versions {
            let unique_versions: HashSet<_> = versions.iter().map(|(_, v)| v.as_str()).collect();
            
            if unique_versions.len() > 1 {
                let version_list: Vec<_> = versions
                    .iter()
                    .map(|(crate_name, version)| format!("{}: {}", crate_name, version))
                    .collect();
                
                violations.push(
                    Violation::new(
                        "dep-version-conflict",
                        "workspace",
                        ViolationCategory::Dependency,
                        Severity::Warning,
                        format!(
                            "Dependency '{}' has conflicting versions: {}",
                            dep_name,
                            version_list.join(", ")
                        ),
                    ),
                );
            }
        }
        
        violations
    }
    
    /// Check if a dependency should be in workspace.dependencies
    pub fn should_be_workspace_dep(&self, dep_name: &str, crates: &[CrateInfo]) -> bool {
        let usage_count = crates
            .iter()
            .filter(|c| c.cargo_toml.dependencies.contains_key(dep_name))
            .count();
        
        // If used by 3+ crates, should be in workspace.dependencies
        usage_count >= 3
    }
    
    /// Find common dependencies that should be in workspace.dependencies
    pub fn find_common_deps(&self, crates: &[CrateInfo]) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut dep_usage: HashMap<String, usize> = HashMap::new();
        
        // Count dependency usage
        for crate_info in crates {
            for name in crate_info.cargo_toml.dependencies.keys() {
                *dep_usage.entry(name.clone()).or_default() += 1;
            }
        }
        
        // Find deps used by 3+ crates that aren't in workspace.dependencies
        for (dep_name, count) in dep_usage {
            if count >= 3 && !self.workspace_deps.contains_key(&dep_name) {
                violations.push(
                    Violation::new(
                        "dep-should-be-workspace",
                        "workspace",
                        ViolationCategory::Dependency,
                        Severity::Info,
                        format!(
                            "Dependency '{}' is used by {} crates and should be in workspace.dependencies",
                            dep_name, count
                        ),
                    ),
                );
            }
        }
        
        violations
    }
}

impl Default for DependencyValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CargoToml, CrateType};
    use std::path::PathBuf;

    fn create_test_crate(name: &str, toml_content: &str) -> CrateInfo {
        let cargo_toml = CargoToml::parse(toml_content).unwrap();
        CrateInfo::new(
            name.to_string(),
            PathBuf::from(format!("crates/{}", name)),
            cargo_toml,
            CrateType::Library,
        )
    }

    #[test]
    fn test_internal_dep_not_workspace() {
        let mut internal_crates = HashSet::new();
        internal_crates.insert("dx-serializer".to_string());
        
        let validator = DependencyValidator::with_internal_crates(internal_crates);
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
dx-serializer = { path = "../serializer" }
"#,
        );
        
        let violations = validator.validate_internal_deps(&crate_info);
        assert!(violations.iter().any(|v| v.id == "dep-internal-not-workspace"));
    }

    #[test]
    fn test_internal_dep_workspace() {
        let mut internal_crates = HashSet::new();
        internal_crates.insert("dx-serializer".to_string());
        
        let validator = DependencyValidator::with_internal_crates(internal_crates);
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
dx-serializer = { workspace = true }
"#,
        );
        
        let violations = validator.validate_internal_deps(&crate_info);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_path_dep_should_use_workspace() {
        let validator = DependencyValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
other = { path = "../crates/other" }
"#,
        );
        
        let violations = validator.validate_path_deps(&crate_info);
        assert!(violations.iter().any(|v| v.id == "dep-path-should-use-workspace"));
    }

    #[test]
    fn test_find_duplicates() {
        let validator = DependencyValidator::new();
        
        let crate1 = create_test_crate(
            "crate1",
            r#"
[package]
name = "crate1"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
        );
        
        let crate2 = create_test_crate(
            "crate2",
            r#"
[package]
name = "crate2"
version = "0.1.0"

[dependencies]
serde = "1.1"
"#,
        );
        
        let violations = validator.find_duplicates(&[crate1, crate2]);
        assert!(violations.iter().any(|v| v.id == "dep-version-conflict"));
    }

    #[test]
    fn test_no_duplicates() {
        let validator = DependencyValidator::new();
        
        let crate1 = create_test_crate(
            "crate1",
            r#"
[package]
name = "crate1"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
        );
        
        let crate2 = create_test_crate(
            "crate2",
            r#"
[package]
name = "crate2"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
        );
        
        let violations = validator.find_duplicates(&[crate1, crate2]);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_find_common_deps() {
        let validator = DependencyValidator::new();
        
        let crates: Vec<_> = (1..=4)
            .map(|i| {
                create_test_crate(
                    &format!("crate{}", i),
                    &format!(r#"
[package]
name = "crate{}"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#, i),
                )
            })
            .collect();
        
        let violations = validator.find_common_deps(&crates);
        assert!(violations.iter().any(|v| v.id == "dep-should-be-workspace"));
    }

    #[test]
    fn test_external_dep_no_violation() {
        let validator = DependencyValidator::new();
        let crate_info = create_test_crate(
            "test-crate",
            r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
"#,
        );
        
        let violations = validator.validate(&crate_info);
        assert!(violations.is_empty());
    }
}
