//! # Dx Compiler - The Factory
//!
//! The Transpiler-to-Binary Pipeline that converts `.tsx` into machine-executable `.dxb` and `.wasm`.
//!
//! ## Philosophy
//! "Separate Structure from Logic."
//! - Input: Developer writes `App.tsx`
//! - Parse: Use `swc` to parse the AST
//! - Split: Separate Static HTML from Dynamic Expressions
//! - Gen A: Serialize Static HTML to `layout.bin` (bincode)
//! - Gen B: Transpile Dynamic Logic to Rust (`generated.rs`)
//! - Compile: Invoke `rustc` to compile `generated.rs` into `logic.wasm`
//!
//! This gives you the performance of Rust WASM with the syntax of TypeScript.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::Instant;

mod analyzer;
mod codegen;
mod codegen_macro;
mod codegen_micro;
mod config;
mod dev_server;
mod linker;
mod packer;
mod parser;
mod pwa;
mod rpc;
mod splitter;

#[derive(Parser)]
#[command(name = "dx")]
#[command(about = "Dx Compiler - Transpiler-to-Binary Pipeline", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project into optimized .dxb artifacts
    Build {
        /// Entry point directory (default: pages)
        #[arg(short, long, default_value = "pages")]
        entry: PathBuf,

        /// Output directory (default: dist)
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,

        /// Skip WASM optimization (faster builds)
        #[arg(long)]
        skip_optimize: bool,
    },

    /// Start development mode with hot-swap
    Dev {
        /// Entry point directory (default: pages)
        #[arg(short, long, default_value = "pages")]
        entry: PathBuf,

        /// Port for dev server (default: 3000)
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },

    /// Create a new Dx project
    New {
        /// Project name
        name: String,

        /// Use template (minimal, counter, todomvc)
        #[arg(short, long, default_value = "minimal")]
        template: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            entry,
            output,
            verbose,
            skip_optimize,
        } => {
            build_project(entry, output, verbose, skip_optimize).await?;
        }
        Commands::Dev {
            entry,
            port,
            verbose,
        } => {
            run_dev_server(entry, port, verbose).await?;
        }
        Commands::New { name, template } => {
            create_new_project(name, template)?;
        }
    }

    Ok(())
}

/// Build the project into optimized artifacts
async fn build_project(
    entry: PathBuf,
    output: PathBuf,
    verbose: bool,
    skip_optimize: bool,
) -> Result<()> {
    let start_time = Instant::now();

    // Ensure output directory exists
    std::fs::create_dir_all(&output).context("Failed to create output directory")?;

    println!("{}", style("🏭 Dx Compiler - Building...").bold().cyan());
    println!();

    // Create progress bar (7 steps now with analysis)
    let pb = ProgressBar::new(7);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>2}/{len:2} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    // Silence unused warning for now
    let _ = skip_optimize;

    // Step 0: Linker Scan (The Omni-Linker)
    pb.set_message("Linking project symbols...");
    let search_root = if entry.file_name().map_or(false, |n| n == "pages") {
        PathBuf::from(".")
    } else {
        entry.parent().map(|p| p.to_path_buf()).unwrap_or(PathBuf::from("."))
    };
    let symbol_table = linker::scan_project(&search_root, verbose)?;

    // Step 1: Parse
    pb.set_message("Parsing .dx files...");
    let parsed_ast = parser::parse_entry(&entry, &symbol_table, verbose).context("Failed to parse entry file")?;
    pb.inc(1);

    // Step 2: Analyze & Decide (THE INTELLIGENCE)
    pb.set_message("Analyzing complexity...");
    let (metrics, runtime_variant) = analyzer::analyze_and_decide(&parsed_ast, verbose)?;

    println!(
        "  🧠 {} runtime selected",
        if runtime_variant == analyzer::RuntimeVariant::Micro {
            style("Micro (338B)").green().bold()
        } else {
            style("Macro (7.5KB)").cyan().bold()
        }
    );
    pb.inc(1);

    // Step 3: Tree Shake
    pb.set_message("Tree shaking imports...");
    let shaken = parser::tree_shake(parsed_ast, verbose)?;
    pb.inc(1);

    // Step 4: Split
    pb.set_message("Splitting structure from logic...");
    let (templates, bindings, state_schema) = splitter::split_components(shaken, verbose)?;
    pb.inc(1);

    // Step 5: Generate HTIP Binary (for Macro mode) OR Rust FFI (for Micro mode)
    pb.set_message("Generating code...");

    // Generate HTIP binary (used by both modes for templates)
    let (htip_stream, _string_table) =
        codegen::generate_htip(&templates, &bindings, &state_schema, verbose)?;

    // For Micro mode: generate raw Rust FFI code
    if runtime_variant == analyzer::RuntimeVariant::Micro {
        pb.set_message("Generating Micro Rust FFI code...");
        let rust_code =
            codegen_micro::generate_micro(&templates, &bindings, &state_schema, verbose)?;
        let rust_path = output.join("generated.rs");
        std::fs::write(&rust_path, &rust_code)?;

        if verbose {
            println!("  ✓ Generated Micro Rust code: {}", rust_path.display());
        }
    }

    // For Macro mode: generate layout.bin + Rust glue code
    if runtime_variant == analyzer::RuntimeVariant::Macro {
        pb.set_message("Generating Macro layout + glue code...");

        // Serialize templates to layout.bin
        codegen_macro::serialize_layout(&templates, &output)?;

        // Generate Rust glue code
        let rust_code =
            codegen_macro::generate_macro(&templates, &bindings, &state_schema, verbose)?;
        let rust_path = output.join("generated.rs");
        std::fs::write(&rust_path, &rust_code)?;

        if verbose {
            println!("  ✓ Generated Macro layout.bin + Rust code: {}", output.display());
        }
    }
    pb.inc(1);

    // Step 6: Pack .dxb (templates + HTIP stream + runtime metadata)
    pb.set_message("Packing .dxb artifact...");
    packer::pack_dxb_htip(&output, &templates, &htip_stream, verbose)?;

    // Write runtime selection metadata
    let metadata_path = output.join("runtime.json");
    let metadata = serde_json::json!({
        "runtime": runtime_variant.as_str(),
        "metrics": metrics,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    std::fs::write(metadata_path, serde_json::to_string_pretty(&metadata)?)?;
    pb.inc(1);

    // Step 7: Copy correct runtime WASM
    pb.set_message(format!("Copying {} runtime...", runtime_variant.as_str()));
    copy_runtime_wasm(&output, runtime_variant, verbose)?;
    pb.inc(1);

    pb.finish_with_message("Build complete!");

    let elapsed = start_time.elapsed();
    println!();
    println!("{} Built in {:.2}s", style("✓").green().bold(), elapsed.as_secs_f32());
    println!("  {} {}", style("Output:").dim(), output.display());
    println!(
        "  {} {} (auto-selected)",
        style("Runtime:").dim(),
        runtime_variant.description()
    );
    println!();

    Ok(())
}

/// Copy the appropriate runtime WASM based on variant selection
fn copy_runtime_wasm(
    output: &Path,
    variant: analyzer::RuntimeVariant,
    verbose: bool,
) -> Result<()> {
    use std::fs;

    // Determine source path based on variant
    let runtime_src = match variant {
        analyzer::RuntimeVariant::Micro => {
            // Look for dx-client-tiny in target/release or pkg/
            let candidates = [
                PathBuf::from("target/pkg_minimal/dx_client_bg.wasm"),
                PathBuf::from("target/release/dx-client-tiny.wasm"),
                PathBuf::from("../target/pkg_minimal/dx_client_bg.wasm"),
            ];

            candidates.into_iter().find(|p| p.exists()).context(
                "dx-client-tiny.wasm not found. Run: cargo build --release --bin dx-client-tiny",
            )?
        }
        analyzer::RuntimeVariant::Macro => {
            // Look for dx-client in target/release or pkg/
            let candidates = [
                PathBuf::from("target/pkg/dx_client_bg.wasm"),
                PathBuf::from("target/release/dx-client.wasm"),
                PathBuf::from("../target/pkg/dx_client_bg.wasm"),
            ];

            candidates
                .into_iter()
                .find(|p| p.exists())
                .context("dx-client.wasm not found. Run: cargo build --release --bin dx-client")?
        }
    };

    let runtime_dest = output.join("runtime.wasm");

    if verbose {
        println!("  Copying {} -> {}", runtime_src.display(), runtime_dest.display());
    }

    fs::copy(&runtime_src, &runtime_dest)
        .with_context(|| format!("Failed to copy runtime from {}", runtime_src.display()))?;

    Ok(())
}

/// Run development server with hot-swap
async fn run_dev_server(entry: PathBuf, port: u16, verbose: bool) -> Result<()> {
    println!("{}", style("🔥 Dx Dev Server - Hot Module Replacement").bold().cyan());
    println!();
    println!("  {} http://localhost:{}", style("Local:").dim(), port);
    println!();

    dev_server::start(entry, port, verbose).await?;

    Ok(())
}

/// Create a new Dx project from template
fn create_new_project(name: String, template: String) -> Result<()> {
    println!("{}", style(format!("📦 Creating new project: {}", name)).bold().cyan());
    println!("  Template: {}", template);
    println!();

    // TODO: Implement project scaffolding
    println!("{}", style("⚠️  Project creation not yet implemented").yellow());
    println!("For now, copy the examples/hello-world template manually.");

    Ok(())
}
