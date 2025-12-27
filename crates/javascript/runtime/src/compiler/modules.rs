//! Module System Implementation
//!
//! Supports:
//! - ES6 modules (import/export)
//! - CommonJS (require/module.exports)
//! - Dynamic imports
//! - Module resolution
//! - Package.json parsing

use crate::error::{DxError, DxResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Module types
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    ESModule,
    CommonJS,
    JSON,
}

/// A compiled module
#[derive(Debug, Clone)]
pub struct Module {
    /// Module path
    pub path: PathBuf,
    /// Module type
    pub module_type: ModuleType,
    /// Exported values
    pub exports: HashMap<String, usize>, // Export name -> value pointer
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// Module resolver
pub struct ModuleResolver {
    /// Resolved modules cache
    modules: HashMap<PathBuf, Module>,
    /// Module search paths
    search_paths: Vec<PathBuf>,
}

impl ModuleResolver {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            search_paths: vec![PathBuf::from("node_modules"), PathBuf::from(".")],
        }
    }

    /// Resolve a module specifier to a file path
    pub fn resolve(&self, specifier: &str, from: &Path) -> DxResult<PathBuf> {
        // Relative imports
        if specifier.starts_with("./") || specifier.starts_with("../") {
            let base = from.parent().unwrap_or(Path::new("."));
            let path = base.join(specifier);
            return self.resolve_file(&path);
        }

        // Absolute imports
        if specifier.starts_with('/') {
            return self.resolve_file(Path::new(specifier));
        }

        // Package imports
        self.resolve_package(specifier)
    }

    /// Resolve a file path
    fn resolve_file(&self, path: &Path) -> DxResult<PathBuf> {
        // Try exact match
        if path.exists() {
            return Ok(path.to_path_buf());
        }

        // Try with extensions
        for ext in &["", ".js", ".ts", ".tsx", ".jsx", ".mjs", ".cjs"] {
            let path_with_ext = path.with_extension(ext.trim_start_matches('.'));
            if path_with_ext.exists() {
                return Ok(path_with_ext);
            }
        }

        // Try index files
        if path.is_dir() {
            for index in &["index.js", "index.ts", "index.tsx", "index.jsx"] {
                let index_path = path.join(index);
                if index_path.exists() {
                    return Ok(index_path);
                }
            }
        }

        Err(DxError::ModuleNotFound(path.display().to_string()))
    }

    /// Resolve a package
    fn resolve_package(&self, name: &str) -> DxResult<PathBuf> {
        for search_path in &self.search_paths {
            let package_path = search_path.join(name);

            // Try package.json
            let package_json = package_path.join("package.json");
            if package_json.exists() {
                if let Ok(main) = self.read_package_main(&package_json) {
                    let main_path = package_path.join(main);
                    if let Ok(resolved) = self.resolve_file(&main_path) {
                        return Ok(resolved);
                    }
                }
            }

            // Try index files
            if let Ok(resolved) = self.resolve_file(&package_path) {
                return Ok(resolved);
            }
        }

        Err(DxError::ModuleNotFound(name.to_string()))
    }

    /// Read package.json main field
    fn read_package_main(&self, path: &Path) -> DxResult<String> {
        let content = std::fs::read_to_string(path).map_err(|e| DxError::IoError(e.to_string()))?;

        // Simple JSON parsing for main field
        // TODO: Use proper JSON parser
        if let Some(start) = content.find("\"main\"") {
            if let Some(colon) = content[start..].find(':') {
                let after_colon = &content[start + colon + 1..];
                if let Some(quote_start) = after_colon.find('"') {
                    let after_quote = &after_colon[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        return Ok(after_quote[..quote_end].to_string());
                    }
                }
            }
        }

        Ok("index.js".to_string())
    }

    /// Load a module
    pub fn load(&mut self, path: &PathBuf) -> DxResult<&Module> {
        if !self.modules.contains_key(path) {
            let module = self.compile_module(path)?;
            self.modules.insert(path.clone(), module);
        }
        Ok(self.modules.get(path).unwrap())
    }

    /// Compile a module
    fn compile_module(&self, path: &PathBuf) -> DxResult<Module> {
        let content = std::fs::read_to_string(path).map_err(|e| DxError::IoError(e.to_string()))?;

        // Detect module type
        let module_type = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            ModuleType::JSON
        } else if content.contains("import ") || content.contains("export ") {
            ModuleType::ESModule
        } else {
            ModuleType::CommonJS
        };

        // Parse and compile
        // TODO: Actual compilation
        let module = Module {
            path: path.clone(),
            module_type,
            exports: HashMap::new(),
            dependencies: Vec::new(),
        };

        Ok(module)
    }
}

/// ES Module parser
pub struct ESModuleParser;

impl ESModuleParser {
    /// Extract imports from source
    pub fn extract_imports(source: &str) -> Vec<ImportStatement> {
        let mut imports = Vec::new();

        // Simple regex-like parsing
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") {
                // Parse import statement
                // TODO: Proper AST-based parsing
                if let Some(from_pos) = trimmed.find(" from ") {
                    let specifier = &trimmed[from_pos + 6..];
                    let specifier = specifier.trim().trim_matches('"').trim_matches('\'');
                    imports.push(ImportStatement {
                        specifier: specifier.to_string(),
                        imports: Vec::new(),
                    });
                }
            }
        }

        imports
    }

    /// Extract exports from source
    pub fn extract_exports(source: &str) -> Vec<ExportStatement> {
        let mut exports = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("export ") {
                exports.push(ExportStatement {
                    name: "default".to_string(),
                    is_default: trimmed.contains("export default"),
                });
            }
        }

        exports
    }
}

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub specifier: String,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ExportStatement {
    pub name: String,
    pub is_default: bool,
}

/// CommonJS parser
pub struct CommonJSParser;

impl CommonJSParser {
    /// Extract requires from source
    pub fn extract_requires(source: &str) -> Vec<String> {
        let mut requires = Vec::new();

        for line in source.lines() {
            if let Some(pos) = line.find("require(") {
                let after = &line[pos + 8..];
                if let Some(quote_start) = after.find(|c| c == '"' || c == '\'') {
                    let quote_char = after.chars().nth(quote_start).unwrap();
                    let after_quote = &after[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find(quote_char) {
                        requires.push(after_quote[..quote_end].to_string());
                    }
                }
            }
        }

        requires
    }
}
