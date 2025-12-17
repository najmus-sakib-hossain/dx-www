//! DX Bundle Resolve - Parallel Import Resolution
//! Implements Node.js module resolution algorithm

use std::fs;
use std::path::{Path, PathBuf};

/// Resolve import specifier using Node.js algorithm
pub fn resolve_import(specifier: &str, from: &Path, project_root: &Path) -> Option<PathBuf> {
    // Relative imports
    if specifier.starts_with('.') {
        resolve_relative(specifier, from)
    }
    // Absolute or package imports
    else {
        resolve_package(specifier, from, project_root)
    }
}

/// Resolve relative import (./foo, ../bar)
fn resolve_relative(specifier: &str, from: &Path) -> Option<PathBuf> {
    let base = from.parent()?;
    let target = base.join(specifier);

    // Try exact match
    if target.exists() && target.is_file() {
        return Some(target);
    }

    // Try with extensions
    for ext in &["", ".js", ".jsx", ".ts", ".tsx", ".mjs", ".cjs"] {
        let with_ext = if ext.is_empty() {
            target.clone()
        } else {
            target.with_extension(&ext[1..])
        };

        if with_ext.exists() && with_ext.is_file() {
            return Some(with_ext);
        }
    }

    // Try as directory with index
    if target.is_dir() {
        for ext in &["js", "jsx", "ts", "tsx", "mjs", "cjs"] {
            let index = target.join(format!("index.{}", ext));
            if index.exists() {
                return Some(index);
            }
        }
    }

    None
}

/// Resolve package import (foo, foo/bar)
fn resolve_package(specifier: &str, from: &Path, project_root: &Path) -> Option<PathBuf> {
    let mut current = from.parent()?;

    loop {
        let node_modules = current.join("node_modules");
        if node_modules.exists() {
            let package_path = node_modules.join(specifier);

            // Try direct file
            if package_path.exists() && package_path.is_file() {
                return Some(package_path);
            }

            // Try with extensions
            for ext in &["js", "jsx", "ts", "tsx", "mjs", "cjs"] {
                let with_ext = package_path.with_extension(ext);
                if with_ext.exists() {
                    return Some(with_ext);
                }
            }

            // Try as directory
            if package_path.is_dir() {
                // Check package.json
                let pkg_json = package_path.join("package.json");
                if pkg_json.exists()
                    && let Some(main) = read_package_main(&pkg_json)
                {
                    let main_path = package_path.join(main);
                    if main_path.exists() {
                        return Some(main_path);
                    }
                }

                // Try index files
                for ext in &["js", "jsx", "ts", "tsx", "mjs", "cjs"] {
                    let index = package_path.join(format!("index.{}", ext));
                    if index.exists() {
                        return Some(index);
                    }
                }
            }
        }

        // Move up directory tree
        if current == project_root || current.parent().is_none() {
            break;
        }
        current = current.parent()?;
    }

    None
}

/// Read main field from package.json
fn read_package_main(pkg_json: &Path) -> Option<String> {
    let content = fs::read_to_string(pkg_json).ok()?;
    // Simple JSON parsing for "main" field
    let main_start = content.find(r#""main""#)?;
    let value_start = content[main_start..].find(':')?;
    let quote_start = content[main_start + value_start..].find('"')?;
    let remaining = &content[main_start + value_start + quote_start + 1..];
    let quote_end = remaining.find('"')?;
    Some(remaining[..quote_end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_resolution() {
        // Tests would go here
    }
}
