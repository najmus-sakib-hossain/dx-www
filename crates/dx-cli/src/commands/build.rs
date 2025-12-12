/**
 * `dx build` - Production build command
 *
 * Compiles the application with optimizations:
 * 1. dx-compiler → generates optimized binary
 * 2. wasm-opt → optimizes WASM (if not skipped)
 * 3. Output → dist/ directory
 */
use anyhow::{Context, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Instant;

use crate::config::ProjectConfig;

pub async fn execute(release: bool, output: PathBuf, skip_optimize: bool) -> Result<()> {
    println!("{}", style("🏭 Building for production...").bold());
    println!();

    // Load configuration
    let config = ProjectConfig::load(".").with_context(|| "Failed to load dx config")?;

    let start = Instant::now();

    // Create progress bar
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}%")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Step 1: Compile TypeScript to Binary (includes WASM generation)
    pb.set_message("📝 Compiling TSX to binary...");
    pb.set_position(0);
    let _compile_result = compile_typescript(&config, &output).await?;
    pb.set_position(70); // Compilation includes WASM generation

    // Step 3: Optimize (if not skipped)
    if !skip_optimize && release {
        pb.set_message("🔧 Optimizing WASM...");
        optimize_wasm(&output).await?;
        pb.set_position(90);
    } else {
        pb.set_position(90);
    }

    // Step 4: Copy artifacts to output
    pb.set_message("📦 Copying artifacts...");
    copy_artifacts(&output).await?;
    pb.set_position(100);
    pb.finish_with_message("✓ Build complete");

    let duration = start.elapsed();

    // Show results
    println!();
    println!("{}", style("✨ Build successful!").green().bold());
    println!();
    println!("  {} Output: {}", style("→").cyan(), output.display());
    println!("  {} Duration: {:.2}s", style("→").cyan(), duration.as_secs_f64());
    println!();

    // Show file sizes
    show_bundle_sizes(&output).await?;

    // Show next steps
    println!("\n{}", style("Next steps:").bold());
    println!("  {} Deploy the dist/ directory to your hosting provider", style("1.").cyan());
    println!(
        "  {} Or run a local server: {}",
        style("2.").cyan(),
        style("python -m http.server 8000").bold()
    );

    Ok(())
}

/// Compile TypeScript using dx-compiler
async fn compile_typescript(
    _config: &ProjectConfig,
    output: &PathBuf,
) -> Result<dx_compiler::CompileResult> {
    use std::path::Path;

    // Determine entry point
    let entry = Path::new("src/App.tsx");
    if !entry.exists() {
        anyhow::bail!("Entry file not found: {}", entry.display());
    }

    // Compile using dx-compiler
    let compile_result = dx_compiler::compile_tsx(entry, output, false)?;

    tracing::info!(
        "Compiled with {} runtime ({} bytes)",
        match compile_result.runtime_variant {
            dx_compiler::analyzer::RuntimeVariant::Micro => "micro",
            dx_compiler::analyzer::RuntimeVariant::Macro => "macro",
        },
        compile_result.total_size
    );

    Ok(compile_result)
}

/// Generate WASM binary
async fn generate_wasm(_config: &ProjectConfig, _release: bool) -> Result<()> {
    // TODO: Call wasm-pack or cargo build --target wasm32-unknown-unknown
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    Ok(())
}

/// Optimize WASM with wasm-opt
async fn optimize_wasm(output: &PathBuf) -> Result<()> {
    use std::process::Command;

    let wasm_path = output.join("runtime.wasm");

    // Check if wasm-opt is available
    let status = Command::new("wasm-opt").arg("--version").output();

    if status.is_err() {
        tracing::warn!("wasm-opt not found, skipping optimization");
        tracing::warn!("Install: https://github.com/WebAssembly/binaryen");
        return Ok(());
    }

    // Run optimization
    let output = Command::new("wasm-opt")
        .arg("-Oz") // Maximum size optimization
        .arg("--enable-bulk-memory")
        .arg("--enable-simd")
        .arg(&wasm_path)
        .arg("-o")
        .arg(&wasm_path)
        .output()
        .with_context(|| "Failed to run wasm-opt")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("wasm-opt failed: {}", stderr);
    }

    Ok(())
}

/// Copy build artifacts to output directory
async fn copy_artifacts(output: &PathBuf) -> Result<()> {
    use std::fs;

    // Create output directory
    fs::create_dir_all(output).with_context(|| "Failed to create output directory")?;

    // Copy files
    let files = vec![
        ("app.dxb", "app.dxb"),
        ("runtime.wasm", "runtime.wasm"),
        ("runtime.json", "runtime.json"),
    ];

    for (src, dst) in files {
        let src_path = PathBuf::from("target/dx").join(src);
        let dst_path = output.join(dst);

        if src_path.exists() {
            fs::copy(&src_path, &dst_path).with_context(|| format!("Failed to copy {}", src))?;
        }
    }

    Ok(())
}

/// Show bundle sizes
async fn show_bundle_sizes(output: &PathBuf) -> Result<()> {
    use std::fs;

    println!("{}", style("📊 Bundle Sizes:").bold());
    println!();

    let files = vec![
        ("app.dxb", "Binary Layout"),
        ("runtime.wasm", "WASM Runtime"),
        ("runtime.json", "Metadata"),
    ];

    let mut total_size = 0;

    for (file, desc) in files {
        let path = output.join(file);
        if let Ok(metadata) = fs::metadata(&path) {
            let size = metadata.len();
            total_size += size;
            println!(
                "  {} {}: {} ({} bytes)",
                style("•").dim(),
                style(desc).bold(),
                format_size(size),
                size.to_string().as_str()
            );
        }
    }

    println!();
    println!("  {} Total: {}", style("→").cyan(), style(format_size(total_size)).bold());

    // Show comparison
    if total_size > 0 && total_size < 50_000 {
        println!();
        println!(
            "  {} That's {}× smaller than React!",
            style("🎉").bold(),
            style((140_000 / total_size).to_string()).green()
        );
    }

    Ok(())
}

/// Format bytes as human-readable size
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
