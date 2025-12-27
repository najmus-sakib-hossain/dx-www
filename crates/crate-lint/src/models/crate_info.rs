//! Crate information model

use std::path::PathBuf;
use super::CargoToml;

/// Information about a discovered crate
#[derive(Debug, Clone)]
pub struct CrateInfo {
    /// Package name from Cargo.toml
    pub name: String,
    /// Path to the crate directory
    pub path: PathBuf,
    /// Parsed Cargo.toml
    pub cargo_toml: CargoToml,
    /// Type of crate based on location
    pub crate_type: CrateType,
    /// List of files in the crate directory
    pub files: Vec<PathBuf>,
}

impl CrateInfo {
    /// Create a new CrateInfo
    pub fn new(
        name: String,
        path: PathBuf,
        cargo_toml: CargoToml,
        crate_type: CrateType,
    ) -> Self {
        Self {
            name,
            path,
            cargo_toml,
            crate_type,
            files: Vec::new(),
        }
    }
    
    /// Add files to the crate info
    pub fn with_files(mut self, files: Vec<PathBuf>) -> Self {
        self.files = files;
        self
    }
    
    /// Get the path to Cargo.toml
    pub fn cargo_toml_path(&self) -> PathBuf {
        self.path.join("Cargo.toml")
    }
    
    /// Check if a file exists in the crate
    pub fn has_file(&self, name: &str) -> bool {
        self.files.iter().any(|f| {
            f.file_name()
                .map(|n| n.to_string_lossy() == name)
                .unwrap_or(false)
        })
    }
    
    /// Get the package, panics if not a package crate
    pub fn package(&self) -> &super::Package {
        self.cargo_toml.package.as_ref().expect("CrateInfo should only be created for package crates")
    }
    
    /// Get the expected package name based on crate type
    pub fn expected_package_name(&self) -> String {
        match self.crate_type {
            CrateType::TopLevelTool => {
                let base = self.path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| self.name.clone());
                if base.starts_with("dx-") {
                    base
                } else {
                    format!("dx-{}", base)
                }
            }
            CrateType::WwwModule => {
                let base = self.path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| self.name.clone());
                if base.starts_with("dx-www-") {
                    base
                } else {
                    format!("dx-www-{}", base)
                }
            }
            CrateType::JavaScriptModule => {
                let base = self.path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| self.name.clone());
                format!("dx-js-{}", base)
            }
            CrateType::Library | CrateType::Nested => {
                // Libraries keep their name but should have dx- prefix
                let base = self.path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| self.name.clone());
                if base.starts_with("dx-") {
                    base
                } else {
                    format!("dx-{}", base)
                }
            }
        }
    }
    
    /// Get the expected library name (with underscores)
    pub fn expected_lib_name(&self) -> String {
        self.expected_package_name().replace('-', "_")
    }
}

/// Type of crate based on its location in the workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrateType {
    /// Top-level tool crate (crates/dx, crates/cli, crates/forge)
    TopLevelTool,
    /// Library crate (crates/serializer, crates/generator)
    Library,
    /// WWW module (crates/www/*)
    WwwModule,
    /// JavaScript module (crates/javascript/*)
    JavaScriptModule,
    /// Nested crate within another crate
    Nested,
}

impl CrateType {
    /// Determine crate type from path relative to workspace root
    pub fn from_path(path: &std::path::Path) -> Self {
        let path_str = path.to_string_lossy();
        
        // Check for www modules
        if path_str.contains("crates/www/") || path_str.contains("crates\\www\\") {
            return CrateType::WwwModule;
        }
        
        // Check for javascript modules
        if path_str.contains("crates/javascript/") || path_str.contains("crates\\javascript\\") {
            return CrateType::JavaScriptModule;
        }
        
        // Check for nested crates (more than 2 levels deep in crates/)
        let components: Vec<_> = path.components().collect();
        let crates_idx = components.iter().position(|c| {
            c.as_os_str().to_string_lossy() == "crates"
        });
        
        if let Some(idx) = crates_idx {
            let depth = components.len() - idx - 1;
            if depth > 1 {
                // Check if it's a known nested structure
                let parent = components.get(idx + 1)
                    .map(|c| c.as_os_str().to_string_lossy().to_string());
                
                match parent.as_deref() {
                    Some("www") | Some("javascript") => {
                        // These are expected nested structures
                    }
                    _ => return CrateType::Nested,
                }
            }
        }
        
        // Top-level tools are specific crates
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        
        match name.as_str() {
            "dx" | "cli" | "crate-lint" => CrateType::TopLevelTool,
            _ => CrateType::Library,
        }
    }
    
    /// Get display name for the crate type
    pub fn display_name(&self) -> &'static str {
        match self {
            CrateType::TopLevelTool => "Top-level Tool",
            CrateType::Library => "Library",
            CrateType::WwwModule => "WWW Module",
            CrateType::JavaScriptModule => "JavaScript Module",
            CrateType::Nested => "Nested Crate",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_crate_type_from_path() {
        assert_eq!(
            CrateType::from_path(Path::new("crates/dx")),
            CrateType::TopLevelTool
        );
        assert_eq!(
            CrateType::from_path(Path::new("crates/cli")),
            CrateType::TopLevelTool
        );
        assert_eq!(
            CrateType::from_path(Path::new("crates/serializer")),
            CrateType::Library
        );
        assert_eq!(
            CrateType::from_path(Path::new("crates/www/core")),
            CrateType::WwwModule
        );
        assert_eq!(
            CrateType::from_path(Path::new("crates/javascript/runtime")),
            CrateType::JavaScriptModule
        );
    }

    #[test]
    fn test_expected_package_name() {
        use super::super::CargoToml;
        
        let cargo_toml = CargoToml::parse(r#"
[package]
name = "test"
version = "0.1.0"
"#).unwrap();
        
        let crate_info = CrateInfo::new(
            "serializer".to_string(),
            PathBuf::from("crates/serializer"),
            cargo_toml,
            CrateType::Library,
        );
        
        assert_eq!(crate_info.expected_package_name(), "dx-serializer");
        assert_eq!(crate_info.expected_lib_name(), "dx_serializer");
    }
}
