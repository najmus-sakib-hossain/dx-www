//! # dx-compiler Library API
//!
//! Public API for integrating the dx compiler into other tools.
//! This allows dx-cli and other build tools to use the compiler as a library.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::time::Instant;

// Re-export modules
pub mod analyzer;
pub mod codegen;
pub mod codegen_macro;
pub mod codegen_micro;
pub mod config;
pub mod dev_server;
pub mod linker;
pub mod loader;
pub mod packer;
pub mod pwa;
pub mod rpc;
pub mod splitter;
pub mod swc_parser;

// Ecosystem integrations
pub mod cmd;
pub mod ecosystem;

pub use analyzer::*;
pub use linker::*;

/// Compilation result containing output paths and metadata
#[derive(Debug, Clone)]
pub struct CompileResult {
    /// The runtime variant that was selected (micro or macro)
    pub runtime_variant: analyzer::RuntimeVariant,
    /// Complexity metrics that drove the decision
    pub metrics: analyzer::ComplexityMetrics,
    /// Path to the generated HTIP binary file
    pub htip_path: PathBuf,
    /// Path to the generated templates JSON file
    pub templates_path: PathBuf,
    /// Path to the generated Rust code (if any)
    pub rust_path: Option<PathBuf>,
    /// Total compilation time
    pub compile_time_ms: u128,
    /// Total size of output artifacts in bytes
    pub total_size: u64,
}

/// Compile a TSX entry file to optimized binary artifacts
///
/// # Arguments
/// * `entry` - Path to the entry .tsx file
/// * `output` - Directory where artifacts will be written
/// * `verbose` - Enable verbose logging
///
/// # Returns
/// * `CompileResult` with paths to generated artifacts and metadata
pub fn compile_tsx(entry: &Path, output: &Path, verbose: bool) -> Result<CompileResult> {
    let start_time = Instant::now();

    // Ensure output directory exists
    std::fs::create_dir_all(output).context("Failed to create output directory")?;

    if verbose {
        println!("🏭 Compiling {} → {}", entry.display(), output.display());
    }

    // Step 0: Linker Scan
    let search_root = entry.parent().unwrap_or_else(|| Path::new("."));
    let symbol_table = linker::scan_project(search_root, verbose)?;

    // Step 1: Parse
    let parsed_ast =
        parser::parse_entry(entry, &symbol_table, verbose).context("Failed to parse entry file")?;

    // Step 2: Analyze & Decide
    let (metrics, runtime_variant) = analyzer::analyze_and_decide(&parsed_ast, verbose)?;

    if verbose {
        println!(
            "  🧠 {} runtime selected",
            match runtime_variant {
                analyzer::RuntimeVariant::Micro => "Micro (338B)",
                analyzer::RuntimeVariant::Macro => "Macro (7.5KB)",
            }
        );
    }

    // Step 3: Tree Shake
    let shaken = parser::tree_shake(parsed_ast, verbose)?;

    // Step 4: Split
    let (templates, bindings, state_schema) = splitter::split_components(shaken, verbose)?;

    // Step 5: Generate HTIP Binary
    let (htip_stream, _string_table) =
        codegen::generate_htip(&templates, &bindings, &state_schema, verbose)?;

    // Step 6: Write HTIP to disk
    let htip_path = output.join("app.htip");
    std::fs::write(&htip_path, &htip_stream)?;

    // Step 7: Generate templates.json
    let templates_json = serde_json::to_string_pretty(&templates)?;
    let templates_path = output.join("templates.json");
    std::fs::write(&templates_path, &templates_json)?;

    // Step 8: Generate Rust code based on runtime variant
    let rust_path = if runtime_variant == analyzer::RuntimeVariant::Micro {
        let rust_code =
            codegen_micro::generate_micro(&templates, &bindings, &state_schema, verbose)?;
        let path = output.join("generated.rs");
        std::fs::write(&path, &rust_code)?;
        Some(path)
    } else {
        // Macro mode
        codegen_macro::serialize_layout(&templates, output)?;
        let rust_code =
            codegen_macro::generate_macro(&templates, &bindings, &state_schema, verbose)?;
        let path = output.join("generated.rs");
        std::fs::write(&path, &rust_code)?;
        Some(path)
    };

    // Step 9: Pack into .dxb (using pack_dxb_htip for compatibility)
    packer::pack_dxb_htip(output, &templates, &htip_stream, verbose)?;

    // Calculate total size
    let mut total_size = 0u64;
    let dxb_path = output.join("app.dxb");
    if htip_path.exists() {
        total_size += std::fs::metadata(&htip_path)?.len();
    }
    if dxb_path.exists() {
        total_size += std::fs::metadata(&dxb_path)?.len();
    }

    let compile_time_ms = start_time.elapsed().as_millis();

    if verbose {
        println!("✓ Compilation complete in {}ms", compile_time_ms);
        println!("  Total size: {} bytes", total_size);
    }

    Ok(CompileResult {
        runtime_variant,
        metrics,
        htip_path,
        templates_path,
        rust_path,
        compile_time_ms,
        total_size,
    })
}

/// Analyze a TSX file and return complexity metrics without compiling
///
/// This is useful for build tools that want to understand the application
/// without performing a full compilation.
/// Analyze a TSX file and return complexity metrics without compiling
pub fn analyze_tsx(
    entry: &Path,
    verbose: bool,
) -> Result<(analyzer::ComplexityMetrics, analyzer::RuntimeVariant)> {
    let search_root = entry.parent().unwrap_or_else(|| Path::new("."));
    let symbol_table = linker::scan_project(search_root, verbose)?;
    let parsed_ast = parser::parse_entry(entry, &symbol_table, verbose)?;
    analyzer::analyze_and_decide(&parsed_ast, verbose)
}

/// Quick compilation check - returns true if entry file can be compiled
pub fn can_compile(entry: &Path) -> bool {
    let symbol_table = linker::SymbolTable::new();
    parser::parse_entry(entry, &symbol_table, false).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_compile_tsx_basic() {
        let temp = TempDir::new().unwrap();
        let entry = temp.path().join("App.tsx");
        let output = temp.path().join("dist");

        // Create a simple TSX file
        fs::write(
            &entry,
            r#"
export default function App() {
    return <div>Hello World</div>;
}
        "#,
        )
        .unwrap();

        let result = compile_tsx(&entry, &output, false);
        assert!(result.is_ok(), "Compilation should succeed");

        let compile_result = result.unwrap();
        assert!(compile_result.htip_path.exists());
        assert!(compile_result.templates_path.exists());
    }

    #[test]
    fn test_analyze_tsx() {
        let temp = TempDir::new().unwrap();
        let entry = temp.path().join("App.tsx");

        fs::write(
            &entry,
            r#"
import { useState } from 'dx';
export default function App() {
    const [count, setCount] = useState(0);
    return <div>{count}</div>;
}
        "#,
        )
        .unwrap();

        let result = analyze_tsx(&entry, false);
        assert!(result.is_ok(), "Analysis should succeed");
    }
}
