//! # Parser Module - The Reader
//!
//! Reads `.tsx` files and builds a custom Dependency Graph.
//!
//! NOTE: This is a simplified regex-based parser for MVP.
//! Production version will use SWC (fastest TS/JS parser in Rust) once
//! serde compatibility issues are resolved.
//!
//! Current capabilities:
//! - Traverse files
//! - Identify components
//! - Extract state declarations
//! - Validate against banned keywords

use anyhow::{Context, Result, anyhow};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Banned keywords that will fail the build immediately
const BANNED_KEYWORDS: &[&str] = &[
    "eval",
    "innerHTML",
    "outerHTML",
    "document.write",
    "Function",
    "dangerouslySetInnerHTML",
];

/// Parsed module with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedModule {
    pub path: PathBuf,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub components: Vec<Component>,
    pub hash: String, // Blake3 hash for cache invalidation
}

/// Component definition extracted from the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub props: Vec<PropDef>,
    pub state: Vec<StateDef>,
    pub jsx_body: String, // Serialized JSX for splitter
    pub hooks: Vec<HookCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropDef {
    pub name: String,
    pub type_annotation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDef {
    pub name: String,
    pub initial_value: String,
    pub type_annotation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookCall {
    pub hook_name: String,
    pub args: Vec<String>,
}

/// Parse the entry file and all dependencies
pub fn parse_entry(entry: &Path, verbose: bool) -> Result<Vec<ParsedModule>> {
    if verbose {
        println!("  Parsing entry: {}", entry.display());
    }

    let mut visited = HashSet::new();
    let mut modules = Vec::new();

    parse_module_recursive(entry, &mut visited, &mut modules, verbose)?;

    if verbose {
        println!("  Parsed {} modules", modules.len());
    }

    Ok(modules)
}

/// Recursively parse a module and its dependencies
fn parse_module_recursive(
    path: &Path,
    visited: &mut HashSet<PathBuf>,
    modules: &mut Vec<ParsedModule>,
    verbose: bool,
) -> Result<()> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;

    if visited.contains(&canonical) {
        return Ok(());
    }
    visited.insert(canonical.clone());

    let module = parse_single_module(path, verbose)?;

    // Queue dependencies
    for _import in &module.imports {
        // TODO: Resolve import paths and recurse
        // This is simplified - production would need proper module resolution
    }

    modules.push(module);
    Ok(())
}

/// Parse a single module file using SWC
fn parse_single_module(path: &Path, verbose: bool) -> Result<ParsedModule> {
    // Read source for security validation
    let source = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    // Validate against banned keywords (SECURITY CHECK)
    for banned in BANNED_KEYWORDS {
        if source.contains(banned) {
            return Err(anyhow!(
                "SECURITY VIOLATION: File {} contains banned keyword: {}",
                path.display(),
                banned
            ));
        }
    }

    // Compute hash for cache invalidation
    let hash = blake3::hash(source.as_bytes()).to_hex().to_string();

    // Extract imports (simplified - looks for import statements)
    let import_regex = Regex::new(r#"import\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap();
    let imports: Vec<String> =
        import_regex.captures_iter(&source).map(|cap| cap[1].to_string()).collect();

    // Extract exports (simplified)
    let export_regex = Regex::new(r"export\s+(default\s+)?(function|const|class)\s+(\w+)").unwrap();
    let exports: Vec<String> =
        export_regex.captures_iter(&source).map(|cap| cap[3].to_string()).collect();

    // Extract components (functions starting with uppercase)
    let component_regex = Regex::new(r"(?:function|const)\s+([A-Z]\w*)\s*(?:\(|=)").unwrap();
    let mut components = Vec::new();

    for cap in component_regex.captures_iter(&source) {
        let name = cap[1].to_string();

        // Extract JSX body (simplified - find return statement)
        let jsx_body = extract_jsx_body(&source, &name);

        // Extract state calls
        let state = extract_state(&source, &name);

        components.push(Component {
            name,
            props: Vec::new(), // TODO: Extract props
            state,
            jsx_body,
            hooks: Vec::new(), // TODO: Extract hooks
        });
    }

    if verbose && !components.is_empty() {
        println!("    Found {} components in {}", components.len(), path.display());
    }

    Ok(ParsedModule {
        path: path.to_path_buf(),
        imports,
        exports,
        components,
        hash,
    })
}

/// Extract JSX body from component (simplified)
fn extract_jsx_body(source: &str, component_name: &str) -> String {
    // Look for return statement with JSX
    let pattern = format!(
        r"(?s)(?:function|const)\s+{}\s*.*?return\s*\((.*?)\);",
        regex::escape(component_name)
    );
    if let Ok(regex) = Regex::new(&pattern)
        && let Some(cap) = regex.captures(source) {
            return cap[1].trim().to_string();
        }

    // Alternative: return without parentheses
    let pattern = format!(
        r"(?s)(?:function|const)\s+{}\s*.*?return\s+(<.*?>)",
        regex::escape(component_name)
    );
    if let Ok(regex) = Regex::new(&pattern)
        && let Some(cap) = regex.captures(source) {
            return cap[1].trim().to_string();
        }

    String::new()
}

/// Extract state declarations from component
fn extract_state(source: &str, _component_name: &str) -> Vec<StateDef> {
    let mut states = Vec::new();

    // Look for useState calls
    let state_regex = Regex::new(r"const\s+\[(\w+),\s*set\w+\]\s*=\s*useState\(([^)]+)\)").unwrap();

    for cap in state_regex.captures_iter(source) {
        let name = cap[1].to_string();
        let initial_value = cap[2].trim().to_string();

        // Infer type from initial value
        let type_annotation = if initial_value.starts_with('"') || initial_value.starts_with('\'') {
            "string".to_string()
        } else if initial_value == "true" || initial_value == "false" {
            "boolean".to_string()
        } else {
            "number".to_string()
        };

        states.push(StateDef {
            name,
            initial_value,
            type_annotation,
        });
    }

    states
}

/// Tree shake unused imports
pub fn tree_shake(modules: Vec<ParsedModule>, verbose: bool) -> Result<Vec<ParsedModule>> {
    if verbose {
        println!("  Tree shaking unused imports...");
    }

    // TODO: Implement proper tree shaking
    // For now, just return as-is
    // Production would:
    // 1. Build import graph
    // 2. Mark used symbols
    // 3. Remove unused imports
    // 4. Dead code elimination

    Ok(modules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banned_keywords() {
        let source = r#"
            function App() {
                eval("dangerous code");
                return <div>Hello</div>;
            }
        "#;

        let result = std::panic::catch_unwind(|| {
            for banned in BANNED_KEYWORDS {
                assert!(source.contains(banned));
            }
        });

        assert!(result.is_ok());
    }
}
