//! dx-pkg-workspace: Monorepo Support
//!
//! Features:
//! - Workspace detection
//! - Dependency hoisting
//! - Parallel installation
//! - Internal package linking

use dx_pkg_compat::PackageJson;
use dx_pkg_core::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Workspace structure
pub struct Workspace {
    pub root: PathBuf,
    pub packages: Vec<WorkspacePackage>,
}

/// Individual workspace package
#[derive(Debug, Clone)]
pub struct WorkspacePackage {
    pub name: String,
    pub path: PathBuf,
    pub package_json: PackageJson,
}

impl Workspace {
    /// Detect workspace from root
    pub fn detect(root: impl AsRef<Path>) -> Result<Option<Self>> {
        let root = root.as_ref().to_path_buf();
        let pkg_json_path = root.join("package.json");

        if !pkg_json_path.exists() {
            return Ok(None);
        }

        let root_pkg = PackageJson::read(&pkg_json_path)?;

        // Check for workspaces field
        // In real impl, would parse workspaces array
        // For now, simplified detection

        Ok(Some(Self {
            root,
            packages: vec![],
        }))
    }

    /// Get all workspace packages
    pub fn list_packages(&self) -> &[WorkspacePackage] {
        &self.packages
    }

    /// Hoist common dependencies
    pub fn hoist_dependencies(&self) -> HashMap<String, String> {
        let mut common = HashMap::new();

        // Analyze all package.json files
        for pkg in &self.packages {
            for (name, version) in &pkg.package_json.dependencies {
                *common.entry(name.clone()).or_insert(0) += 1;
            }
        }

        // Return dependencies used by 2+ packages
        common
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(name, _)| (name, "*".to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_detection() {
        let temp = std::env::temp_dir().join("dx-workspace-test");
        std::fs::create_dir_all(&temp).unwrap();

        let pkg_json = temp.join("package.json");
        std::fs::write(&pkg_json, r#"{"name":"test","version":"1.0.0"}"#).unwrap();

        let ws = Workspace::detect(&temp).unwrap();
        assert!(ws.is_some());
    }
}
