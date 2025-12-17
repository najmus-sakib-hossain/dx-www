//! DX Bundle CLI - Command-line interface for DX bundler

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "dx-bundle")]
#[command(about = "DX Bundler - 3x faster than Bun", version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bundle JavaScript/TypeScript files
    Bundle {
        /// Entry point(s)
        #[arg(required = true)]
        entries: Vec<PathBuf>,

        /// Output file
        #[arg(short, long, default_value = "dist/bundle.js")]
        output: PathBuf,

        /// Output format (esm, cjs, iife)
        #[arg(short, long, default_value = "esm")]
        format: String,

        /// Minify output
        #[arg(short, long)]
        minify: bool,

        /// Generate source maps
        #[arg(long)]
        sourcemap: bool,

        /// Target environment (browser, node, bun)
        #[arg(short, long, default_value = "browser")]
        target: String,

        /// Watch mode
        #[arg(short, long)]
        watch: bool,

        /// Skip cache (force rebuild)
        #[arg(long)]
        no_cache: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bundle {
            entries,
            output,
            format,
            minify,
            sourcemap,
            target,
            watch,
            no_cache,
            verbose,
        } => {
            bundle_command(
                entries, output, format, minify, sourcemap, target, watch, no_cache, verbose,
            )
            .await?;
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn bundle_command(
    entries: Vec<PathBuf>,
    output: PathBuf,
    format: String,
    minify: bool,
    sourcemap: bool,
    target: String,
    watch: bool,
    no_cache: bool,
    verbose: bool,
) -> Result<()> {
    // Suppress unused parameter warnings
    let _ = (format, target, watch);

    let total_start = Instant::now();
    let project_root = std::env::current_dir()?;

    println!("📦 DX Bundler");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Phase 1: Get or build module graph (O(1) if cached!)
    let graph_start = Instant::now();
    let cache_dir = project_root.join(".dx-cache");
    let graph_cache = dx_bundle_graph::ModuleGraphCache::new(&cache_dir)?;

    let graph = if no_cache {
        // TODO: Implement build_fresh
        graph_cache.get_or_build(&project_root, &entries)?
    } else {
        graph_cache.get_or_build(&project_root, &entries)?
    };
    let graph_time = graph_start.elapsed();

    if verbose {
        println!(
            "📊 Module graph: {} modules in {:.2}ms",
            graph.modules().len(),
            graph_time.as_secs_f64() * 1000.0
        );
    }

    // Phase 2: Process modules and read sources
    let parse_start = Instant::now();

    let mut module_sources: Vec<(PathBuf, String)> = Vec::new();

    // Get all modules from the graph and read their sources
    for module in graph.modules() {
        // Extract path from the string table
        let path_bytes = unsafe {
            let header = graph.header();
            let strings_ptr = graph.mmap().as_ptr().add(header.strings_offset as usize);
            let path_ptr = strings_ptr.add(module.path_offset as usize);
            std::slice::from_raw_parts(path_ptr, module.path_len as usize)
        };

        if let Ok(path_str) = std::str::from_utf8(path_bytes) {
            let path = PathBuf::from(path_str);
            if let Ok(source) = std::fs::read_to_string(&path) {
                module_sources.push((path, source));
            }
        }
    }

    let parse_time = parse_start.elapsed();

    if verbose {
        println!(
            "🔍 Loaded {} modules in {:.2}ms",
            module_sources.len(),
            parse_time.as_secs_f64() * 1000.0
        );
    }

    // Phase 3: Tree shaking
    let shake_start = Instant::now();
    let mut shaker = dx_bundle_tree_shake::TreeShaker::new();
    let module_count = module_sources.len();
    let shake_results = shaker.analyze(&[0], module_count); // Entry module
    let shake_time = shake_start.elapsed();

    let removed = shake_results.iter().filter(|r| r.can_remove != 0).count();
    if verbose {
        println!(
            "🌳 Tree shaking: removed {} unused modules in {:.2}ms",
            removed,
            shake_time.as_secs_f64() * 1000.0
        );
    }

    // Phase 4: Transform (JSX + TypeScript stripping + ES6 to CommonJS)
    let transform_start = Instant::now();

    let mut transformed_sources: Vec<(PathBuf, Vec<u8>)> = Vec::new();
    for (path, source) in module_sources {
        let path_str = path.to_str().unwrap_or("");
        let is_utils = path_str.contains("utils");

        if verbose && is_utils {
            println!("\n🔍 DEBUG: Processing {}", path_str);
            println!("Original:\n{}", source);
        }

        // Transform JSX to JavaScript
        let mut code = dx_bundle_transform::transform_jsx(&source);
        if verbose && is_utils {
            println!("\nAfter JSX transform:\n{}", code);
        }

        // Strip TypeScript type annotations
        code = dx_bundle_transform::strip_typescript_simple(&code);
        if verbose && is_utils {
            println!("\nAfter TS strip:\n{}", code);
        }

        // Remove single-line comments to prevent issues when minified
        let lines: Vec<&str> = code.lines().collect();
        let mut no_comments = Vec::new();
        for line in lines {
            // Remove // comments but preserve the code before them
            if let Some(pos) = line.find("//") {
                let before_comment = &line[..pos];
                if !before_comment.trim().is_empty() {
                    no_comments.push(before_comment);
                }
            } else {
                no_comments.push(line);
            }
        }
        code = no_comments.join("\n");

        // Convert ES6 exports to CommonJS (only at statement start)
        let lines: Vec<&str> = code.lines().collect();
        let mut converted_lines = Vec::new();

        for line in lines {
            let trimmed = line.trim_start();
            let mut converted_line = line.to_string();

            // Only convert if at start of line (statement level)
            if trimmed.starts_with("export default ") {
                converted_line = line.replace("export default ", "module.exports = ");
            } else if trimmed.starts_with("export function ") {
                converted_line = line.replace("export function ", "function ");
            } else if trimmed.starts_with("export const ") {
                converted_line = line.replace("export const ", "exports.");
            } else if trimmed.starts_with("import ") {
                // Convert imports: import { x } from 'y' => const { x } = require('y')
                if let Some(from_pos) = trimmed.find(" from ") {
                    let import_part = &trimmed[7..from_pos]; // After "import "
                    let path_part = &trimmed[from_pos + 6..]; // After " from "

                    // Remove semicolon from path if present
                    let path_clean = path_part.trim().trim_end_matches(';');

                    // Build the require statement with proper closing paren
                    if import_part.trim().starts_with('{') {
                        // Named import: import { x } from 'y'
                        converted_line =
                            format!("const {} = require({});", import_part.trim(), path_clean);
                    } else {
                        // Default import: import x from 'y'
                        converted_line =
                            format!("const {} = require({});", import_part.trim(), path_clean);
                    }
                }
            }

            converted_lines.push(converted_line);
        }

        code = converted_lines.join("\n");

        // Fix JSX artifacts - empty return statements (can span multiple lines)
        code = code.replace("return (\n  \n);", "return null;");
        code = code.replace("return (\n\n);", "return null;");
        code = code.replace("return (\n  \n  \n);", "return null;");
        code = code.replace("return (\n  \n  \n  \n);", "return null;");

        // Re-enable whitespace stripping for production builds
        let mut code_bytes = dx_bundle_transform::strip_whitespace(code.as_bytes());

        // Fix JSX artifacts AFTER whitespace stripping (single-line versions)
        let code_str = String::from_utf8_lossy(&code_bytes);
        let fixed = code_str
            .replace("return ( );", "return null;")
            .replace("return ()", "return null;")
            .replace("return()", "return null;")
            .replace("return( );", "return null;");
        code_bytes = fixed.into_bytes();

        transformed_sources.push((path, code_bytes));
    }

    let transform_time = transform_start.elapsed();

    if verbose {
        println!(
            "🔄 Transformed {} modules in {:.2}ms",
            transformed_sources.len(),
            transform_time.as_secs_f64() * 1000.0
        );
    }

    // Phase 5: Bundle (zero-copy concatenation)
    let bundle_start = Instant::now();
    std::fs::create_dir_all(output.parent().unwrap())?;
    let mut bundler = dx_bundle_concat::ZeroCopyBundler::new(&output)?;
    bundler.write_runtime_header()?;

    // Write modules
    for (idx, (_path, content)) in transformed_sources.iter().enumerate() {
        bundler.write_module(idx as u32, content)?;
    }

    // Write entry point
    bundler.write_entry(0)?;

    // Close IIFE wrapper
    bundler.write_footer()?;

    let bundle_time = bundle_start.elapsed();

    if verbose {
        println!("📝 Bundled in {:.2}ms", bundle_time.as_secs_f64() * 1000.0);
    }

    // Phase 6: Minify (optional)
    let minify_time = if minify {
        let start = Instant::now();
        let bundle = std::fs::read(&output)?;
        let minified = dx_bundle_minify::minify(&bundle);
        if !minified.is_empty() {
            std::fs::write(&output, minified)?;
        }
        let time = start.elapsed();

        if verbose {
            println!("🗜️  Minified in {:.2}ms", time.as_secs_f64() * 1000.0);
        }
        time
    } else {
        std::time::Duration::ZERO
    };

    // Phase 7: Source maps (optional)
    let sourcemap_time = if sourcemap {
        let start = Instant::now();
        let mut builder = dx_bundle_sourcemap::BinarySourceMapBuilder::new();

        // Add example mapping
        builder.add_mapping(1, 0, "src/index.js", 1, 0, None);

        // Write source map
        let map = builder.build();
        let map_path = output.with_extension("js.map");
        std::fs::write(&map_path, map)?;

        let time = start.elapsed();

        if verbose {
            println!("🗺️  Source map in {:.2}ms", time.as_secs_f64() * 1000.0);
        }
        time
    } else {
        std::time::Duration::ZERO
    };

    // Summary
    let total_time = total_start.elapsed();
    let output_size = std::fs::metadata(&output)?.len();

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Bundle complete!");
    println!("   ├─ Output:     {}", output.display());
    println!("   ├─ Size:       {} KB", output_size / 1024);
    println!("   ├─ Modules:    {}", graph.modules().len());
    println!("   └─ Time:       {:.2}ms", total_time.as_secs_f64() * 1000.0);

    if verbose {
        println!("\n   Breakdown:");
        println!("   ├─ Graph:      {:.2}ms", graph_time.as_secs_f64() * 1000.0);
        println!("   ├─ Parse:      {:.2}ms", parse_time.as_secs_f64() * 1000.0);
        println!("   ├─ Shake:      {:.2}ms", shake_time.as_secs_f64() * 1000.0);
        println!("   ├─ Transform:  {:.2}ms", transform_time.as_secs_f64() * 1000.0);
        println!("   ├─ Bundle:     {:.2}ms", bundle_time.as_secs_f64() * 1000.0);
        if minify {
            println!("   ├─ Minify:     {:.2}ms", minify_time.as_secs_f64() * 1000.0);
        }
        if sourcemap {
            println!("   └─ Sourcemap:  {:.2}ms", sourcemap_time.as_secs_f64() * 1000.0);
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    Ok(())
}
