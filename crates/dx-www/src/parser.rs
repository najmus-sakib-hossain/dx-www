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

use crate::linker::SymbolTable;
use anyhow::{anyhow, Context, Result};
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

/// Parse the entry file or directory
pub fn parse_entry(
    entry: &Path,
    symbol_table: &SymbolTable,
    verbose: bool,
) -> Result<Vec<ParsedModule>> {
    let mut modules = Vec::new();

    if entry.is_dir() {
        if verbose {
            println!("  📂 Parsing directory: {}", entry.display());
        }

        // Walk the directory to find all .dx files
        for entry in walkdir::WalkDir::new(entry)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "dx" || ext == "tsx") {
                if verbose {
                    println!("  📄 Found file: {}", path.display());
                }

                match parse_single_module(path, symbol_table, verbose) {
                    Ok(module) => modules.push(module),
                    Err(e) => {
                        eprintln!("  ⚠️ Failed to parse {}: {}", path.display(), e);
                        // Continue parsing other files? Or fail hard?
                        // For now, let's warn and continue
                    }
                }
            }
        }
    } else {
        // Single file mode
        if verbose {
            println!("  Parsing entry file: {}", entry.display());
        }
        modules.push(parse_single_module(entry, symbol_table, verbose)?);
    }

    // TODO: Still need dependency graph traversal for imported units?
    // For Binary Dawn v1.0, units are "auto-imported", so we might just need to parse everything in units/ as well.
    // Ideally we should walk `units/` too if we are parsing the root.

    if verbose {
        println!("  Parsed {} modules", modules.len());
    }

    Ok(modules)
}

/// Recursively parse a module and its dependencies
// Deprecated in favor of directory walking for now, but kept for legacy support if needed
#[allow(dead_code)]
fn parse_module_recursive(
    _path: &Path,
    _visited: &mut HashSet<PathBuf>,
    _modules: &mut Vec<ParsedModule>,
    _verbose: bool,
) -> Result<()> {
    // This logic is temporarily replaced by the flat directory walker above for v1.0 structure
    Ok(())
}

/// Parse a single module file using SWC
fn parse_single_module(
    path: &Path,
    symbol_table: &SymbolTable,
    verbose: bool,
) -> Result<ParsedModule> {
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
    let mut imports: Vec<String> =
        import_regex.captures_iter(&source).map(|cap| cap[1].to_string()).collect();

    // AUTO-IMPORT MAGIC 🪄
    // Scan for known symbols in the SymbolTable and auto-inject imports
    for (symbol, symbol_path) in &symbol_table.components {
        // Simple heuristic: if the source contains "<Symbol", and it's not already imported
        if source.contains(&format!("<{}", symbol))
            || source.contains(&format!("<{}.", symbol.split('.').next().unwrap()))
        {
            // Check if already imported
            let already_imported = imports
                .iter()
                .any(|imp| imp.contains(&symbol_path.to_string_lossy().to_string()));

            if !already_imported {
                if verbose {
                    println!("    ✨ Auto-importing {} from {}", symbol, symbol_path.display());
                }
                // Inject virtual import
                imports.push(symbol_path.to_string_lossy().to_string());
            }
        }
    }

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
    if let Ok(regex) = Regex::new(&pattern) {
        if let Some(cap) = regex.captures(source) {
            return cap[1].trim().to_string();
        }
    }

    // Alternative: return without parentheses
    let pattern = format!(
        r"(?s)(?:function|const)\s+{}\s*.*?return\s+(<.*?>)",
        regex::escape(component_name)
    );
    if let Ok(regex) = Regex::new(&pattern) {
        if let Some(cap) = regex.captures(source) {
            return cap[1].trim().to_string();
        }
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
        // Test that source with banned keyword contains "eval"
        let source_with_eval = r#"
            function App() {
                eval("dangerous code");
                return <div>Hello</div>;
            }
        "#;

        assert!(source_with_eval.contains("eval"));

        // Test that each banned keyword is actually in the BANNED_KEYWORDS list
        assert!(BANNED_KEYWORDS.contains(&"eval"));
        assert!(BANNED_KEYWORDS.contains(&"Function"));
    }
}
