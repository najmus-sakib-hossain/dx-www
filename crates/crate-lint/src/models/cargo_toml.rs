//! Cargo.toml parsing structures with workspace inheritance support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a complete Cargo.toml file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CargoToml {
    #[serde(default)]
    pub package: Option<Package>,
    #[serde(default)]
    pub lib: Option<LibConfig>,
    #[serde(default, rename = "bin")]
    pub bins: Vec<BinConfig>,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: HashMap<String, Dependency>,
    #[serde(default, rename = "build-dependencies")]
    pub build_dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub workspace: Option<WorkspaceConfig>,
}

/// Package metadata section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Package {
    pub name: String,
    #[serde(default)]
    pub version: VersionSpec,
    #[serde(default)]
    pub edition: EditionSpec,
    #[serde(default)]
    pub authors: AuthorsSpec,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: LicenseSpec,
    #[serde(default)]
    pub repository: RepositorySpec,
    #[serde(default)]
    pub documentation: DocumentationSpec,
    #[serde(default)]
    pub homepage: HomepageSpec,
    #[serde(default)]
    pub keywords: Option<Vec<String>>,
    #[serde(default)]
    pub categories: Option<Vec<String>>,
    #[serde(default, rename = "rust-version")]
    pub rust_version: Option<String>,
    #[serde(default)]
    pub include: Option<Vec<String>>,
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    #[serde(default)]
    pub readme: Option<String>,
    #[serde(default)]
    pub publish: Option<bool>,
}

/// Version specification - either literal or workspace inheritance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum VersionSpec {
    Literal(String),
    Workspace(WorkspaceInherit),
    #[default]
    Missing,
}

impl VersionSpec {
    pub fn is_workspace(&self) -> bool {
        matches!(self, VersionSpec::Workspace(w) if w.workspace)
    }
    
    pub fn as_literal(&self) -> Option<&str> {
        match self {
            VersionSpec::Literal(s) => Some(s),
            _ => None,
        }
    }
}

/// Edition specification - either literal or workspace inheritance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum EditionSpec {
    Literal(String),
    Workspace(WorkspaceInherit),
    #[default]
    Missing,
}

impl EditionSpec {
    pub fn is_workspace(&self) -> bool {
        matches!(self, EditionSpec::Workspace(w) if w.workspace)
    }
}

/// Authors specification - either literal or workspace inheritance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum AuthorsSpec {
    Literal(Vec<String>),
    Workspace(WorkspaceInherit),
    #[default]
    Missing,
}

impl AuthorsSpec {
    pub fn is_workspace(&self) -> bool {
        matches!(self, AuthorsSpec::Workspace(w) if w.workspace)
    }
}

/// License specification - either literal or workspace inheritance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum LicenseSpec {
    Literal(String),
    Workspace(WorkspaceInherit),
    #[default]
    Missing,
}

impl LicenseSpec {
    pub fn is_workspace(&self) -> bool {
        matches!(self, LicenseSpec::Workspace(w) if w.workspace)
    }
    
    pub fn as_literal(&self) -> Option<&str> {
        match self {
            LicenseSpec::Literal(s) => Some(s),
            _ => None,
        }
    }
}

/// Repository specification - either literal or workspace inheritance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum RepositorySpec {
    Literal(String),
    Workspace(WorkspaceInherit),
    #[default]
    Missing,
}

impl RepositorySpec {
    pub fn is_workspace(&self) -> bool {
        matches!(self, RepositorySpec::Workspace(w) if w.workspace)
    }
}

/// Homepage specification - either literal or workspace inheritance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum HomepageSpec {
    Literal(String),
    Workspace(WorkspaceInherit),
    #[default]
    Missing,
}

impl HomepageSpec {
    pub fn is_workspace(&self) -> bool {
        matches!(self, HomepageSpec::Workspace(w) if w.workspace)
    }
    
    pub fn as_literal(&self) -> Option<&str> {
        match self {
            HomepageSpec::Literal(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn is_set(&self) -> bool {
        !matches!(self, HomepageSpec::Missing)
    }
}

/// Documentation specification - either literal or workspace inheritance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum DocumentationSpec {
    Literal(String),
    Workspace(WorkspaceInherit),
    #[default]
    Missing,
}

impl DocumentationSpec {
    pub fn is_workspace(&self) -> bool {
        matches!(self, DocumentationSpec::Workspace(w) if w.workspace)
    }
    
    pub fn as_literal(&self) -> Option<&str> {
        match self {
            DocumentationSpec::Literal(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn is_set(&self) -> bool {
        !matches!(self, DocumentationSpec::Missing)
    }
}

/// Workspace inheritance marker
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceInherit {
    pub workspace: bool,
}

/// Library configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibConfig {
    pub name: Option<String>,
    pub path: Option<String>,
    #[serde(default, rename = "crate-type")]
    pub crate_type: Option<Vec<String>>,
}

/// Binary configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinConfig {
    pub name: String,
    pub path: Option<String>,
    #[serde(default, rename = "required-features")]
    pub required_features: Option<Vec<String>>,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DetailedDependency),
}

impl Dependency {
    pub fn is_workspace(&self) -> bool {
        match self {
            Dependency::Detailed(d) => d.workspace.unwrap_or(false),
            Dependency::Simple(_) => false,
        }
    }
    
    pub fn is_path(&self) -> bool {
        match self {
            Dependency::Detailed(d) => d.path.is_some(),
            Dependency::Simple(_) => false,
        }
    }
    
    pub fn version(&self) -> Option<&str> {
        match self {
            Dependency::Simple(v) => Some(v),
            Dependency::Detailed(d) => d.version.as_deref(),
        }
    }
    
    pub fn path(&self) -> Option<&str> {
        match self {
            Dependency::Detailed(d) => d.path.as_deref(),
            Dependency::Simple(_) => None,
        }
    }
}

/// Detailed dependency specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DetailedDependency {
    pub version: Option<String>,
    pub path: Option<String>,
    pub workspace: Option<bool>,
    pub features: Option<Vec<String>>,
    pub optional: Option<bool>,
    #[serde(default, rename = "default-features")]
    pub default_features: Option<bool>,
    pub package: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
}

/// Workspace configuration (for root Cargo.toml)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub package: Option<WorkspacePackage>,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

/// Workspace-level package defaults
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspacePackage {
    pub version: Option<String>,
    pub edition: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
}

impl CargoToml {
    /// Parse a Cargo.toml from a string
    pub fn parse(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
    
    /// Serialize to TOML string
    pub fn to_toml_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
    
    /// Check if this is a workspace root (has [workspace] section)
    pub fn is_workspace_root(&self) -> bool {
        self.workspace.is_some()
    }
    
    /// Check if this is a package (has [package] section)
    pub fn is_package(&self) -> bool {
        self.package.is_some()
    }
    
    /// Get the package name, if this is a package
    pub fn package_name(&self) -> Option<&str> {
        self.package.as_ref().map(|p| p.name.as_str())
    }
    
    /// Get all internal dependencies (those with path or workspace = true)
    pub fn internal_dependencies(&self) -> Vec<(&String, &Dependency)> {
        self.dependencies
            .iter()
            .filter(|(_, dep)| dep.is_path() || dep.is_workspace())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_cargo_toml() {
        let content = r#"
[package]
name = "test-crate"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
"#;
        let cargo = CargoToml::parse(content).unwrap();
        assert!(cargo.package.is_some());
        let package = cargo.package.unwrap();
        assert_eq!(package.name, "test-crate");
        assert!(matches!(package.version, VersionSpec::Literal(ref v) if v == "0.1.0"));
    }

    #[test]
    fn test_parse_workspace_inheritance() {
        let content = r#"
[package]
name = "test-crate"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
internal = { workspace = true }
"#;
        let cargo = CargoToml::parse(content).unwrap();
        let package = cargo.package.unwrap();
        assert!(package.version.is_workspace());
        assert!(package.edition.is_workspace());
        assert!(package.authors.is_workspace());
        assert!(package.license.is_workspace());
        assert!(package.repository.is_workspace());
        assert!(cargo.dependencies.get("internal").unwrap().is_workspace());
    }

    #[test]
    fn test_parse_detailed_dependency() {
        let content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
local = { path = "../local" }
"#;
        let cargo = CargoToml::parse(content).unwrap();
        let serde = cargo.dependencies.get("serde").unwrap();
        assert_eq!(serde.version(), Some("1.0"));
        
        let local = cargo.dependencies.get("local").unwrap();
        assert!(local.is_path());
        assert_eq!(local.path(), Some("../local"));
    }

    #[test]
    fn test_parse_workspace_only() {
        let content = r#"
[workspace]
resolver = "2"
members = ["crates/test"]

[workspace.package]
version = "0.1.0"
edition = "2024"
"#;
        let cargo = CargoToml::parse(content).unwrap();
        assert!(cargo.is_workspace_root());
        assert!(!cargo.is_package());
        assert!(cargo.package.is_none());
    }
}
