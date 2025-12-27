//! Cargo.toml auto-fixer
//!
//! Provides functionality to automatically fix Cargo.toml violations
//! including workspace inheritance, required fields, and naming conventions.

use std::path::Path;
use crate::{CrateInfo, CrateType, Fix, FileChange, ChangeOperation};

/// Auto-fixer for Cargo.toml files
pub struct CargoFixer;

impl CargoFixer {
    /// Create a new CargoFixer
    pub fn new() -> Self {
        Self
    }

    /// Generate fixes for workspace inheritance violations
    ///
    /// Adds `field.workspace = true` for version, edition, authors, license, repository
    pub fn fix_workspace_inheritance(&self, crate_info: &CrateInfo) -> Vec<Fix> {
        let mut fixes = Vec::new();
        let cargo_path = crate_info.cargo_toml_path();
        
        // Read current Cargo.toml content
        let content = match std::fs::read_to_string(&cargo_path) {
            Ok(c) => c,
            Err(_) => return fixes,
        };

        let package = match &crate_info.cargo_toml.package {
            Some(p) => p,
            None => return fixes,
        };

        // Check each field that should use workspace inheritance
        let fields_to_check = [
            ("version", !package.version.is_workspace()),
            ("edition", !package.edition.is_workspace()),
            ("authors", !package.authors.is_workspace()),
            ("license", !package.license.is_workspace()),
            ("repository", !package.repository.is_workspace()),
        ];

        for (field, needs_fix) in fields_to_check {
            if needs_fix {
                if let Some(fix) = self.create_workspace_field_fix(&content, field, &cargo_path) {
                    fixes.push(fix);
                }
            }
        }

        fixes
    }

    /// Create a fix for a single workspace field
    fn create_workspace_field_fix(&self, content: &str, field: &str, cargo_path: &Path) -> Option<Fix> {
        // Find the field in the content and create a replacement
        let patterns = [
            format!("{} = \"", field),
            format!("{} = [", field),
        ];

        for pattern in &patterns {
            if let Some(start) = content.find(pattern) {
                // Find the end of the value
                let after_eq = start + pattern.len() - 1;
                let rest = &content[after_eq..];
                
                let end = if pattern.ends_with('"') {
                    // String value - find closing quote
                    rest.find('"').map(|i| after_eq + i + 1)
                } else {
                    // Array value - find closing bracket
                    rest.find(']').map(|i| after_eq + i + 1)
                };

                if let Some(end_pos) = end {
                    let old_value = &content[start..end_pos];
                    let new_value = format!("{}.workspace = true", field);
                    
                    return Some(Fix::new(
                        format!("Replace {} with workspace inheritance", field),
                        true,
                    ).with_change(FileChange::new(
                        cargo_path.to_path_buf(),
                        ChangeOperation::Modify {
                            old: old_value.to_string(),
                            new: new_value,
                        },
                    )));
                }
            }
        }

        None
    }

    /// Generate fixes for missing required fields
    ///
    /// Adds description, keywords, categories with sensible defaults
    pub fn fix_missing_fields(&self, crate_info: &CrateInfo) -> Vec<Fix> {
        let mut fixes = Vec::new();
        let cargo_path = crate_info.cargo_toml_path();
        
        let content = match std::fs::read_to_string(&cargo_path) {
            Ok(c) => c,
            Err(_) => return fixes,
        };

        let package = match &crate_info.cargo_toml.package {
            Some(p) => p,
            None => return fixes,
        };

        // Check for missing description
        if package.description.is_none() {
            let description = self.generate_description(crate_info);
            if let Some(fix) = self.create_add_field_fix(&content, "description", &format!("\"{}\"", description), &cargo_path) {
                fixes.push(fix);
            }
        }

        // Check for missing keywords
        if package.keywords.is_none() {
            let keywords = self.generate_keywords(crate_info);
            let keywords_str = format!("[{}]", keywords.iter().map(|k| format!("\"{}\"", k)).collect::<Vec<_>>().join(", "));
            if let Some(fix) = self.create_add_field_fix(&content, "keywords", &keywords_str, &cargo_path) {
                fixes.push(fix);
            }
        }

        // Check for missing categories
        if package.categories.is_none() {
            let categories = self.generate_categories(crate_info);
            let categories_str = format!("[{}]", categories.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", "));
            if let Some(fix) = self.create_add_field_fix(&content, "categories", &categories_str, &cargo_path) {
                fixes.push(fix);
            }
        }

        // Check for missing homepage
        if !package.homepage.is_set() {
            if let Some(fix) = self.create_add_field_fix(&content, "homepage.workspace", "true", &cargo_path) {
                fixes.push(fix);
            }
        }

        // Check for missing documentation
        if !package.documentation.is_set() {
            if let Some(fix) = self.create_add_field_fix(&content, "documentation.workspace", "true", &cargo_path) {
                fixes.push(fix);
            }
        }

        fixes
    }

    /// Create a fix to add a new field to Cargo.toml
    fn create_add_field_fix(&self, content: &str, field: &str, value: &str, cargo_path: &Path) -> Option<Fix> {
        // Find the [package] section and add the field after the name
        if let Some(package_start) = content.find("[package]") {
            // Find the next line after [package]
            let after_package = &content[package_start..];
            if let Some(newline_pos) = after_package.find('\n') {
                // Find the name = line
                let rest = &after_package[newline_pos..];
                if let Some(name_pos) = rest.find("name = ") {
                    // Find the end of the name line
                    let name_start = package_start + newline_pos + name_pos;
                    let name_rest = &content[name_start..];
                    if let Some(name_end) = name_rest.find('\n') {
                        let insert_pos = name_start + name_end;
                        let old_content = &content[insert_pos..insert_pos + 1];
                        let new_content = format!("\n{} = {}", field, value);
                        
                        return Some(Fix::new(
                            format!("Add {} field to Cargo.toml", field),
                            true,
                        ).with_change(FileChange::new(
                            cargo_path.to_path_buf(),
                            ChangeOperation::Modify {
                                old: old_content.to_string(),
                                new: new_content,
                            },
                        )));
                    }
                }
            }
        }
        None
    }

    /// Generate a description based on crate info
    fn generate_description(&self, crate_info: &CrateInfo) -> String {
        let name = &crate_info.name;
        match crate_info.crate_type {
            CrateType::TopLevelTool => format!("DX ecosystem tool: {}", name),
            CrateType::Library => format!("DX ecosystem library for {}", name.replace("dx-", "").replace('-', " ")),
            CrateType::WwwModule => format!("DX web framework module: {}", name.replace("dx-www-", "")),
            CrateType::JavaScriptModule => format!("DX JavaScript integration: {}", name.replace("dx-js-", "")),
            CrateType::Nested => format!("DX internal component: {}", name),
        }
    }

    /// Generate keywords based on crate info
    fn generate_keywords(&self, crate_info: &CrateInfo) -> Vec<String> {
        let mut keywords = vec!["dx".to_string()];
        
        match crate_info.crate_type {
            CrateType::TopLevelTool => {
                keywords.push("cli".to_string());
                keywords.push("tool".to_string());
            }
            CrateType::Library => {
                keywords.push("library".to_string());
            }
            CrateType::WwwModule => {
                keywords.push("web".to_string());
                keywords.push("framework".to_string());
            }
            CrateType::JavaScriptModule => {
                keywords.push("javascript".to_string());
                keywords.push("wasm".to_string());
            }
            CrateType::Nested => {
                keywords.push("internal".to_string());
            }
        }

        // Add name-based keyword
        let name_keyword = crate_info.name
            .replace("dx-", "")
            .replace("www-", "")
            .replace("js-", "");
        if !name_keyword.is_empty() && keywords.len() < 5 {
            keywords.push(name_keyword);
        }

        keywords.truncate(5);
        keywords
    }

    /// Generate categories based on crate info
    fn generate_categories(&self, crate_info: &CrateInfo) -> Vec<String> {
        match crate_info.crate_type {
            CrateType::TopLevelTool => vec!["development-tools".to_string(), "command-line-utilities".to_string()],
            CrateType::Library => vec!["development-tools".to_string()],
            CrateType::WwwModule => vec!["web-programming".to_string()],
            CrateType::JavaScriptModule => vec!["wasm".to_string(), "web-programming".to_string()],
            CrateType::Nested => vec!["development-tools".to_string()],
        }
    }

    /// Generate fixes for naming convention violations
    ///
    /// Suggests renaming packages to follow dx-{name} pattern
    pub fn fix_naming_convention(&self, crate_info: &CrateInfo) -> Option<Fix> {
        let expected_name = crate_info.expected_package_name();
        let current_name = &crate_info.name;
        
        if current_name == &expected_name {
            return None;
        }

        let cargo_path = crate_info.cargo_toml_path();
        let content = match std::fs::read_to_string(&cargo_path) {
            Ok(c) => c,
            Err(_) => return None,
        };

        // Find and replace the name field
        let old_pattern = format!("name = \"{}\"", current_name);
        let new_pattern = format!("name = \"{}\"", expected_name);

        if content.contains(&old_pattern) {
            Some(Fix::new(
                format!("Rename package from '{}' to '{}'", current_name, expected_name),
                true,
            ).with_change(FileChange::new(
                cargo_path,
                ChangeOperation::Modify {
                    old: old_pattern,
                    new: new_pattern,
                },
            )))
        } else {
            None
        }
    }

    /// Generate fixes for library name convention violations
    ///
    /// Ensures lib name uses underscores instead of hyphens
    pub fn fix_lib_name(&self, crate_info: &CrateInfo) -> Option<Fix> {
        let cargo_path = crate_info.cargo_toml_path();
        let content = match std::fs::read_to_string(&cargo_path) {
            Ok(c) => c,
            Err(_) => return None,
        };

        // Check if there's a [lib] section with a name
        if let Some(lib) = &crate_info.cargo_toml.lib {
            if let Some(lib_name) = &lib.name {
                if lib_name.contains('-') {
                    let expected_lib_name = lib_name.replace('-', "_");
                    let old_pattern = format!("name = \"{}\"", lib_name);
                    let new_pattern = format!("name = \"{}\"", expected_lib_name);

                    // Only fix if this is in the [lib] section
                    if let Some(lib_section) = content.find("[lib]") {
                        let lib_content = &content[lib_section..];
                        if lib_content.contains(&old_pattern) {
                            return Some(Fix::new(
                                format!("Rename lib from '{}' to '{}' (use underscores)", lib_name, expected_lib_name),
                                true,
                            ).with_change(FileChange::new(
                                cargo_path,
                                ChangeOperation::Modify {
                                    old: old_pattern,
                                    new: new_pattern,
                                },
                            )));
                        }
                    }
                }
            }
        }

        None
    }

    /// Apply all auto-fixes for a crate
    pub fn generate_all_fixes(&self, crate_info: &CrateInfo) -> Vec<Fix> {
        let mut fixes = Vec::new();
        
        fixes.extend(self.fix_workspace_inheritance(crate_info));
        fixes.extend(self.fix_missing_fields(crate_info));
        
        if let Some(fix) = self.fix_naming_convention(crate_info) {
            fixes.push(fix);
        }
        
        if let Some(fix) = self.fix_lib_name(crate_info) {
            fixes.push(fix);
        }

        fixes
    }
}

impl Default for CargoFixer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CargoToml;
    use std::path::PathBuf;

    fn create_test_crate_info(name: &str, cargo_content: &str, crate_type: CrateType) -> CrateInfo {
        let cargo_toml = CargoToml::parse(cargo_content).unwrap();
        CrateInfo::new(
            name.to_string(),
            PathBuf::from(format!("crates/{}", name)),
            cargo_toml,
            crate_type,
        )
    }

    #[test]
    fn test_generate_description() {
        let fixer = CargoFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-serializer",
            r#"[package]
name = "dx-serializer"
version = "0.1.0"
"#,
            CrateType::Library,
        );
        
        let desc = fixer.generate_description(&crate_info);
        assert!(desc.contains("serializer"));
    }

    #[test]
    fn test_generate_keywords() {
        let fixer = CargoFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-cli",
            r#"[package]
name = "dx-cli"
version = "0.1.0"
"#,
            CrateType::TopLevelTool,
        );
        
        let keywords = fixer.generate_keywords(&crate_info);
        assert!(keywords.contains(&"dx".to_string()));
        assert!(keywords.contains(&"cli".to_string()));
        assert!(keywords.len() <= 5);
    }

    #[test]
    fn test_generate_categories() {
        let fixer = CargoFixer::new();
        
        let crate_info = create_test_crate_info(
            "dx-www-core",
            r#"[package]
name = "dx-www-core"
version = "0.1.0"
"#,
            CrateType::WwwModule,
        );
        
        let categories = fixer.generate_categories(&crate_info);
        assert!(categories.contains(&"web-programming".to_string()));
    }
}
