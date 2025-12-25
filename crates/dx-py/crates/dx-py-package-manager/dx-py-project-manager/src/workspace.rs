//! Workspace/monorepo support
//!
//! Provides Cargo-style workspace management for Python projects.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Workspace configuration
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WorkspaceConfig {
    /// Glob patterns for workspace members
    #[serde(default)]
    pub members: Vec<String>,
    /// Glob patterns for excluded paths
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Shared dependencies across workspace
    #[serde(default)]
    pub shared_dependencies: HashMap<String, String>,
}

/// A workspace member (individual project)
#[derive(Debug, Clone)]
pub struct WorkspaceMember {
    /// Path to the member project
    pub path: PathBuf,
    /// Project name
    pub name: String,
    /// Project version
    pub version: String,
    /// Dependencies
    pub dependencies: HashMap<String, String>,
    /// Dev dependencies
    pub dev_dependencies: HashMap<String, String>,
}

impl WorkspaceMember {
    /// Load a workspace member from a directory
    pub fn load(path: &Path) -> Result<Self> {
        let pyproject_path = path.join("pyproject.toml");
        let pyproject_dx_path = path.join("pyproject.dx");

        if pyproject_dx_path.exists() {
            // Binary format - not implemented yet
            return Err(Error::Cache("Binary pyproject.dx not yet supported".to_string()));
        }

        if !pyproject_path.exists() {
            return Err(Error::Cache(format!(
                "No pyproject.toml found in {}",
                path.display()
            )));
        }

        let content = std::fs::read_to_string(&pyproject_path)?;
        let toml: toml::Value = toml::from_str(&content)
            .map_err(|e| Error::Cache(format!("Failed to parse pyproject.toml: {}", e)))?;

        let project = toml.get("project").ok_or_else(|| {
            Error::Cache("Missing [project] section in pyproject.toml".to_string())
        })?;

        let name = project
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Cache("Missing project.name".to_string()))?
            .to_string();

        let version = project
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0")
            .to_string();

        let dependencies = Self::parse_dependencies(project.get("dependencies"));
        let dev_dependencies = Self::parse_optional_dependencies(
            project.get("optional-dependencies"),
            "dev",
        );

        Ok(Self {
            path: path.to_path_buf(),
            name,
            version,
            dependencies,
            dev_dependencies,
        })
    }

    /// Parse dependencies from TOML array
    fn parse_dependencies(deps: Option<&toml::Value>) -> HashMap<String, String> {
        let mut result = HashMap::new();

        if let Some(toml::Value::Array(arr)) = deps {
            for dep in arr {
                if let Some(dep_str) = dep.as_str() {
                    // Parse "package>=1.0" format
                    let (name, version) = Self::parse_dep_string(dep_str);
                    result.insert(name, version);
                }
            }
        }

        result
    }

    /// Parse optional dependencies group
    fn parse_optional_dependencies(
        optional: Option<&toml::Value>,
        group: &str,
    ) -> HashMap<String, String> {
        let mut result = HashMap::new();

        if let Some(toml::Value::Table(table)) = optional {
            if let Some(toml::Value::Array(arr)) = table.get(group) {
                for dep in arr {
                    if let Some(dep_str) = dep.as_str() {
                        let (name, version) = Self::parse_dep_string(dep_str);
                        result.insert(name, version);
                    }
                }
            }
        }

        result
    }

    /// Parse a dependency string like "requests>=2.0"
    fn parse_dep_string(s: &str) -> (String, String) {
        // Find version constraint operator
        let operators = [">=", "<=", "==", "!=", "~=", ">", "<"];
        for op in operators {
            if let Some(idx) = s.find(op) {
                return (s[..idx].trim().to_string(), s[idx..].trim().to_string());
            }
        }
        (s.trim().to_string(), "*".to_string())
    }
}

/// Workspace manager
///
/// Manages Cargo-style workspaces for Python projects.
pub struct WorkspaceManager {
    /// Workspace root directory
    root: PathBuf,
    /// Workspace configuration
    config: WorkspaceConfig,
    /// Cached workspace members
    members_cache: Option<Vec<WorkspaceMember>>,
}

impl WorkspaceManager {
    /// Load a workspace from a directory
    pub fn load(root: &Path) -> Result<Self> {
        let config = Self::load_config(root)?;

        Ok(Self {
            root: root.to_path_buf(),
            config,
            members_cache: None,
        })
    }

    /// Load workspace configuration
    fn load_config(root: &Path) -> Result<WorkspaceConfig> {
        let pyproject_dx = root.join("pyproject.dx");
        let pyproject_toml = root.join("pyproject.toml");

        if pyproject_dx.exists() {
            // Binary format - not implemented yet
            return Err(Error::Cache("Binary pyproject.dx not yet supported".to_string()));
        }

        if !pyproject_toml.exists() {
            // No workspace config, return empty
            return Ok(WorkspaceConfig::default());
        }

        let content = std::fs::read_to_string(&pyproject_toml)?;
        let toml: toml::Value = toml::from_str(&content)
            .map_err(|e| Error::Cache(format!("Failed to parse pyproject.toml: {}", e)))?;

        // Check for [tool.dx-py.workspace] section
        let workspace_config = toml
            .get("tool")
            .and_then(|t| t.get("dx-py"))
            .and_then(|d| d.get("workspace"));

        if let Some(ws) = workspace_config {
            let config: WorkspaceConfig = ws
                .clone()
                .try_into()
                .map_err(|e| Error::Cache(format!("Failed to parse workspace config: {}", e)))?;
            return Ok(config);
        }

        Ok(WorkspaceConfig::default())
    }

    /// Get the workspace root
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get the workspace configuration
    pub fn config(&self) -> &WorkspaceConfig {
        &self.config
    }

    /// Get all workspace members
    pub fn members(&mut self) -> Result<&[WorkspaceMember]> {
        if self.members_cache.is_none() {
            self.members_cache = Some(self.discover_members()?);
        }
        Ok(self.members_cache.as_ref().unwrap())
    }

    /// Discover workspace members based on glob patterns
    fn discover_members(&self) -> Result<Vec<WorkspaceMember>> {
        let mut members = Vec::new();

        if self.config.members.is_empty() {
            // No workspace members defined, check if root is a project
            if self.root.join("pyproject.toml").exists() {
                if let Ok(member) = WorkspaceMember::load(&self.root) {
                    members.push(member);
                }
            }
            return Ok(members);
        }

        for pattern in &self.config.members {
            let full_pattern = self.root.join(pattern);
            let pattern_str = full_pattern.to_string_lossy();

            for entry in glob::glob(&pattern_str)
                .map_err(|e| Error::Cache(format!("Invalid glob pattern: {}", e)))?
            {
                let path = entry.map_err(|e| Error::Cache(format!("Glob error: {}", e)))?;

                // Check if excluded
                if self.is_excluded(&path) {
                    continue;
                }

                // Check if it's a project directory
                if path.join("pyproject.toml").exists() || path.join("pyproject.dx").exists() {
                    if let Ok(member) = WorkspaceMember::load(&path) {
                        members.push(member);
                    }
                }
            }
        }

        Ok(members)
    }

    /// Check if a path is excluded
    fn is_excluded(&self, path: &Path) -> bool {
        for pattern in &self.config.exclude {
            let full_pattern = self.root.join(pattern);
            if let Ok(glob_pattern) = glob::Pattern::new(&full_pattern.to_string_lossy()) {
                if glob_pattern.matches_path(path) {
                    return true;
                }
            }
        }
        false
    }

    /// Get all dependencies across the workspace
    pub fn all_dependencies(&mut self) -> Result<HashMap<String, String>> {
        let mut all_deps = HashMap::new();

        // Add shared dependencies first
        for (name, version) in &self.config.shared_dependencies {
            all_deps.insert(name.clone(), version.clone());
        }

        // Collect member dependencies
        let member_deps: Vec<(String, String)> = self
            .members()?
            .iter()
            .flat_map(|m| m.dependencies.iter())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Add member dependencies (shared takes precedence)
        for (name, version) in member_deps {
            if !self.config.shared_dependencies.contains_key(&name) {
                all_deps.insert(name, version);
            }
        }

        Ok(all_deps)
    }

    /// Check if this is a workspace (has members defined)
    pub fn is_workspace(&self) -> bool {
        !self.config.members.is_empty()
    }

    /// Find a member by name
    pub fn find_member(&mut self, name: &str) -> Result<Option<&WorkspaceMember>> {
        let members = self.members()?;
        Ok(members.iter().find(|m| m.name == name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_workspace_config_default() {
        let config = WorkspaceConfig::default();
        assert!(config.members.is_empty());
        assert!(config.exclude.is_empty());
        assert!(config.shared_dependencies.is_empty());
    }

    #[test]
    fn test_workspace_member_parse_dep_string() {
        let (name, version) = WorkspaceMember::parse_dep_string("requests>=2.0");
        assert_eq!(name, "requests");
        assert_eq!(version, ">=2.0");

        let (name, version) = WorkspaceMember::parse_dep_string("flask");
        assert_eq!(name, "flask");
        assert_eq!(version, "*");
    }

    #[test]
    fn test_workspace_manager_empty() {
        let temp_dir = TempDir::new().unwrap();
        let manager = WorkspaceManager::load(temp_dir.path()).unwrap();
        assert!(!manager.is_workspace());
    }

    #[test]
    fn test_workspace_manager_with_pyproject() {
        let temp_dir = TempDir::new().unwrap();

        // Create a simple pyproject.toml
        let pyproject = r#"
[project]
name = "test-project"
version = "1.0.0"
dependencies = ["requests>=2.0", "flask"]
"#;
        std::fs::write(temp_dir.path().join("pyproject.toml"), pyproject).unwrap();

        let mut manager = WorkspaceManager::load(temp_dir.path()).unwrap();
        let members = manager.members().unwrap();

        assert_eq!(members.len(), 1);
        assert_eq!(members[0].name, "test-project");
        assert_eq!(members[0].version, "1.0.0");
        assert!(members[0].dependencies.contains_key("requests"));
        assert!(members[0].dependencies.contains_key("flask"));
    }

    #[test]
    fn test_workspace_manager_with_workspace_config() {
        let temp_dir = TempDir::new().unwrap();

        // Create workspace pyproject.toml
        let pyproject = r#"
[project]
name = "workspace-root"
version = "1.0.0"

[tool.dx-py.workspace]
members = ["packages/*"]
shared_dependencies = { requests = ">=2.28" }
"#;
        std::fs::write(temp_dir.path().join("pyproject.toml"), pyproject).unwrap();

        // Create a package
        let pkg_dir = temp_dir.path().join("packages").join("pkg-a");
        std::fs::create_dir_all(&pkg_dir).unwrap();
        let pkg_pyproject = r#"
[project]
name = "pkg-a"
version = "0.1.0"
dependencies = ["flask"]
"#;
        std::fs::write(pkg_dir.join("pyproject.toml"), pkg_pyproject).unwrap();

        let mut manager = WorkspaceManager::load(temp_dir.path()).unwrap();
        assert!(manager.is_workspace());

        let members = manager.members().unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].name, "pkg-a");

        let all_deps = manager.all_dependencies().unwrap();
        assert!(all_deps.contains_key("requests"));
        assert!(all_deps.contains_key("flask"));
        // Shared dependency should take precedence
        assert_eq!(all_deps.get("requests"), Some(&">=2.28".to_string()));
    }
}
