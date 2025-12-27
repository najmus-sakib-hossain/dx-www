//! Crate scanning and discovery module
//!
//! Discovers all crates in the repository by finding Cargo.toml files
//! and parsing their metadata.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a discovered crate in the monorepo
#[derive(Debug, Clone)]
pub struct CrateInfo {
    /// Path relative to crates directory
    pub path: PathBuf,
    /// Crate name from Cargo.toml
    pub name: String,
    /// Whether this is a library or binary crate
    pub crate_type: CrateType,
    /// Whether this crate has subcrates
    pub has_subcrates: bool,
    /// List of files present in the crate root
    pub files: Vec<String>,
    /// Parsed Cargo.toml metadata
    pub manifest: CargoManifest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrateType {
    Library,
    Binary,
    Mixed,
}

/// Parsed Cargo.toml structure
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CargoManifest {
    pub package: Option<PackageInfo>,
    #[serde(default)]
    pub lib: Option<LibTarget>,
    #[serde(default, rename = "bin")]
    pub bins: Option<Vec<BinTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PackageInfo {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_workspace_or_string")]
    pub version: Option<String>,
    #[serde(default, deserialize_with = "deserialize_workspace_or_string")]
    pub edition: Option<String>,
    #[serde(default, deserialize_with = "deserialize_workspace_or_vec")]
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_workspace_or_string")]
    pub license: Option<String>,
    #[serde(default, deserialize_with = "deserialize_workspace_or_string")]
    pub repository: Option<String>,
    #[serde(default, deserialize_with = "deserialize_workspace_or_string")]
    pub documentation: Option<String>,
    pub publish: Option<bool>,
}

/// Deserialize a field that can be either a string or { workspace = true }
fn deserialize_workspace_or_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    
    struct StringOrWorkspace;
    
    impl<'de> Visitor<'de> for StringOrWorkspace {
        type Value = Option<String>;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or { workspace = true }")
        }
        
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }
        
        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            // Handle { workspace = true } - just return None to indicate workspace inheritance
            while let Some((key, _value)) = map.next_entry::<String, toml::Value>()? {
                if key == "workspace" {
                    return Ok(Some("[workspace]".to_string()));
                }
            }
            Ok(None)
        }
        
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }
    
    deserializer.deserialize_any(StringOrWorkspace)
}

/// Deserialize a field that can be either a vec or { workspace = true }
fn deserialize_workspace_or_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    
    struct VecOrWorkspace;
    
    impl<'de> Visitor<'de> for VecOrWorkspace {
        type Value = Option<Vec<String>>;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a vec or { workspace = true }")
        }
        
        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<String>()? {
                vec.push(value);
            }
            Ok(Some(vec))
        }
        
        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            // Handle { workspace = true }
            while let Some((key, _value)) = map.next_entry::<String, toml::Value>()? {
                if key == "workspace" {
                    return Ok(Some(vec!["[workspace]".to_string()]));
                }
            }
            Ok(None)
        }
        
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }
    
    deserializer.deserialize_any(VecOrWorkspace)
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibTarget {
    pub name: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BinTarget {
    pub name: Option<String>,
    pub path: Option<String>,
}

/// Scans the crates directory and returns all crate information
pub fn scan_crates(crates_dir: &Path) -> Result<Vec<CrateInfo>> {
    let mut crates = Vec::new();
    let mut seen_paths: HashSet<PathBuf> = HashSet::new();

    // Walk through all directories looking for Cargo.toml files
    for entry in WalkDir::new(crates_dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e.path()))
    {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a Cargo.toml file
        if !path.is_file() || path.file_name() != Some("Cargo.toml".as_ref()) {
            continue;
        }

        let crate_dir = path.parent().unwrap();
        
        // Skip if we've already processed this crate
        if seen_paths.contains(crate_dir) {
            continue;
        }
        seen_paths.insert(crate_dir.to_path_buf());

        // Parse the crate
        match parse_crate(crate_dir, crates_dir) {
            Ok(crate_info) => crates.push(crate_info),
            Err(e) => {
                eprintln!("Warning: Failed to parse {}: {}", crate_dir.display(), e);
            }
        }
    }

    // Sort by path for consistent output
    crates.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(crates)
}

/// Check if a directory should be excluded from scanning
fn is_excluded_dir(path: &Path) -> bool {
    let excluded = [
        "target",
        "node_modules",
        ".git",
        ".dx",
        ".kiro",
        ".vscode",
        ".github",
        "proptest-regressions",
    ];

    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| excluded.contains(&name))
        .unwrap_or(false)
}

/// Parse a single crate directory
fn parse_crate(crate_dir: &Path, crates_root: &Path) -> Result<CrateInfo> {
    let cargo_toml_path = crate_dir.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;

    let manifest: CargoManifest = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", cargo_toml_path.display()))?;

    let name = manifest
        .package
        .as_ref()
        .and_then(|p| p.name.clone())
        .unwrap_or_else(|| {
            crate_dir
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        });

    let crate_type = determine_crate_type(&manifest, crate_dir);
    let has_subcrates = check_for_subcrates(crate_dir);
    let files = list_root_files(crate_dir)?;
    let relative_path = crate_dir
        .strip_prefix(crates_root)
        .unwrap_or(crate_dir)
        .to_path_buf();

    Ok(CrateInfo {
        path: relative_path,
        name,
        crate_type,
        has_subcrates,
        files,
        manifest,
    })
}

/// Determine if a crate is a library, binary, or mixed
pub fn determine_crate_type(manifest: &CargoManifest, crate_dir: &Path) -> CrateType {
    let has_lib = manifest.lib.is_some() || crate_dir.join("src/lib.rs").exists();
    let has_bin = manifest.bins.as_ref().map(|b| !b.is_empty()).unwrap_or(false)
        || crate_dir.join("src/main.rs").exists();

    match (has_lib, has_bin) {
        (true, true) => CrateType::Mixed,
        (true, false) => CrateType::Library,
        (false, true) => CrateType::Binary,
        (false, false) => CrateType::Library, // Default to library
    }
}

/// Check if a crate directory contains subcrates
fn check_for_subcrates(crate_dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(crate_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("Cargo.toml").exists() {
                return true;
            }
        }
    }
    false
}

/// List files in the crate root directory (not recursive)
fn list_root_files(crate_dir: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    
    for entry in fs::read_dir(crate_dir)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        files.push(file_name);
    }
    
    files.sort();
    Ok(files)
}

/// Check if a crate is a library (no bin targets, has lib target)
pub fn is_library_crate(crate_info: &CrateInfo) -> bool {
    matches!(crate_info.crate_type, CrateType::Library)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_excluded_dir() {
        assert!(is_excluded_dir(Path::new("/some/path/target")));
        assert!(is_excluded_dir(Path::new("/some/path/node_modules")));
        assert!(is_excluded_dir(Path::new("/some/path/.git")));
        assert!(!is_excluded_dir(Path::new("/some/path/src")));
        assert!(!is_excluded_dir(Path::new("/some/path/crates")));
    }

    #[test]
    fn test_determine_crate_type_library() {
        let manifest = CargoManifest {
            package: Some(PackageInfo::default()),
            lib: Some(LibTarget::default()),
            bins: None,
        };
        let temp_dir = std::env::temp_dir();
        assert_eq!(determine_crate_type(&manifest, &temp_dir), CrateType::Library);
    }
}
