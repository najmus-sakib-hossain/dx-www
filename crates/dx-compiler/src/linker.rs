//! # Linker Module - The Auto-Import Engine
//!
//! Scans the project structure to build a Symbol Table, enabling implicit imports.
//!
//! ## Capabilities
//! - Scans `units/` directory
//! - Maps component names (e.g., `UI.Button`) to file paths
//! - Resolves dependency graphs automatically

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// strict symbol table mapping
/// e.g. "UI.Button" -> "units/ui/button.dx"
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub components: HashMap<String, PathBuf>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve a component name to a path
    pub fn resolve(&self, name: &str) -> Option<&PathBuf> {
        self.components.get(name)
    }
}

/// Scan the project to build the Symbol Table
pub fn scan_project(root: &Path, verbose: bool) -> Result<SymbolTable> {
    if verbose {
        println!("  🔗 Linker: Scanning project...");
    }

    let mut table = SymbolTable::new();

    // 1. Scan units/
    let units_dir = root.join("units");
    if units_dir.exists() {
        scan_units(&units_dir, &mut table, verbose)?;
    }

    if verbose {
        println!("  🔗 Linker: Indexed {} symbols", table.components.len());
    }

    Ok(table)
}

/// Scan units directory for components
fn scan_units(units_dir: &Path, table: &mut SymbolTable, verbose: bool) -> Result<()> {
    for entry in WalkDir::new(units_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "dx" || ext == "tsx") {
            // Check if it's a valid component file
            // Expected structure: units/{category}/{component}.dx
            // e.g. units/ui/button.dx -> UI.Button
            
            if let Some(symbol) = derive_symbol_name(units_dir, path) {
                if verbose {
                    println!("     + {} -> {}", symbol, path.display());
                }
                table.components.insert(symbol, path.to_path_buf());
            }
        }
    }
    
    Ok(())
}

/// Derive symbol name from file path
/// units/ui/button.dx -> UI.Button
/// units/auth/guard.dx -> Auth.Guard
fn derive_symbol_name(base: &Path, path: &Path) -> Option<String> {
    let relative = pathdiff::diff_paths(path, base)?;
    let parts: Vec<_> = relative.iter().map(|s| s.to_string_lossy()).collect();

    if parts.len() < 2 {
        return None; 
    }

    // Category is the directory (ui, auth, etc.)
    let category = capitalize(&parts[0]);
    
    // Component is the filename without extension
    let filename = path.file_stem()?.to_string_lossy();
    let component = capitalize(&filename);

    Some(format!("{}.{}", category, component))
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_symbol_name() {
        let base = Path::new("/app/units");
        
        // Simple case
        let path = Path::new("/app/units/ui/button.dx");
        assert_eq!(derive_symbol_name(base, path), Some("Ui.Button".to_string()));
        
        // Capitalization check (though folder conventions are usually lowercase)
        let path = Path::new("/app/units/auth/userProfile.dx");
        assert_eq!(derive_symbol_name(base, path), Some("Auth.UserProfile".to_string()));
    }
    
    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("ui"), "Ui");
        assert_eq!(capitalize("button"), "Button");
        assert_eq!(capitalize(""), "");
    }
}
