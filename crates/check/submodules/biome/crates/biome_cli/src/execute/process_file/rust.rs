use super::{DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::BiomePath;
use std::process::Command;
use tracing::{debug, error, info, warn, instrument};

/// Check if a command is available in PATH
fn is_command_available(cmd: &str) -> bool {
    // Try using 'which' command first (works in Git Bash on Windows)
    if let Ok(output) = Command::new("which").arg(cmd).output() {
        if output.status.success() && !output.stdout.is_empty() {
            return true;
        }
    }
    
    // Fallback to trying the command directly
    Command::new(cmd).arg("--version").output().is_ok()
}

/// Find rustfmt in PATH or common installation locations
fn find_rustfmt() -> Option<String> {
    // First check PATH using 'which' command (works in Git Bash)
    debug!("Checking for rustfmt using 'which' command...");
    if let Ok(output) = Command::new("which").arg("rustfmt").output() {
        if output.status.success() && !output.stdout.is_empty() {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let original_path = path_str.trim().to_string();
                // Convert Git Bash Unix-style path to Windows path
                let windows_path = if original_path.starts_with("/c/") {
                    original_path.replace("/c/", "C:\\").replace("/", "\\")
                } else {
                    original_path.clone()
                };
                info!("Found rustfmt in PATH via 'which': {} -> {}", original_path, windows_path);
                return Some(windows_path);
            }
        }
    }
    
    // Fallback: check PATH directly
    if is_command_available("rustfmt") {
        return Some("rustfmt".to_string());
    }
    
    // Check common installation locations
    let mut common_paths: Vec<String> = Vec::new();
    
    if cfg!(windows) {
        if let Ok(home) = std::env::var("USERPROFILE") {
            common_paths.push(format!("{}\\.cargo\\bin\\rustfmt.exe", home));
            common_paths.push(format!("{}\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\bin\\rustfmt.exe", home));
        }
    } else {
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.cargo/bin/rustfmt", home));
            common_paths.push(format!("{}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustfmt", home));
        }
    }
    
    for path in common_paths {
        if std::path::Path::new(&path).exists() {
            // Verify it works
            if Command::new(&path).arg("--version").output().is_ok() {
                info!("Found rustfmt at: {}", path);
                return Some(path);
            }
        }
    }
    
    None
}

/// Find cargo in PATH or common installation locations
fn find_cargo() -> Option<String> {
    // First check PATH using 'which' command (works in Git Bash)
    debug!("Checking for cargo using 'which' command...");
    if let Ok(output) = Command::new("which").arg("cargo").output() {
        if output.status.success() && !output.stdout.is_empty() {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let original_path = path_str.trim().to_string();
                // Convert Git Bash Unix-style path to Windows path
                let windows_path = if original_path.starts_with("/c/") {
                    original_path.replace("/c/", "C:\\").replace("/", "\\")
                } else {
                    original_path.clone()
                };
                info!("Found cargo in PATH via 'which': {} -> {}", original_path, windows_path);
                return Some(windows_path);
            }
        }
    }
    
    // Fallback: check PATH directly
    if is_command_available("cargo") {
        return Some("cargo".to_string());
    }
    
    // Check common installation locations
    let mut common_paths: Vec<String> = Vec::new();
    
    if cfg!(windows) {
        if let Ok(home) = std::env::var("USERPROFILE") {
            common_paths.push(format!("{}\\.cargo\\bin\\cargo.exe", home));
        }
    } else {
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.cargo/bin/cargo", home));
        }
    }
    
    for path in common_paths {
        if std::path::Path::new(&path).exists() {
            // Verify it works
            if Command::new(&path).arg("--version").output().is_ok() {
                info!("Found cargo at: {}", path);
                return Some(path);
            }
        }
    }
    
    None
}

/// Attempt to install rustfmt using rustup
fn install_rustfmt_auto() -> bool {
    info!("Attempting automatic installation of rustfmt...");
    
    // rustfmt is typically installed via rustup
    let result = Command::new("rustup")
        .args(&["component", "add", "rustfmt"])
        .status();
    
    if result.is_ok() && result.unwrap().success() {
        info!("Successfully installed rustfmt via rustup");
        return true;
    }
    
    false
}

/// Attempt to install clippy using rustup
fn install_clippy_auto() -> bool {
    info!("Attempting automatic installation of clippy...");
    
    // clippy is typically installed via rustup
    let result = Command::new("rustup")
        .args(&["component", "add", "clippy"])
        .status();
    
    if result.is_ok() && result.unwrap().success() {
        info!("Successfully installed clippy via rustup");
        return true;
    }
    
    false
}

/// Print installation instructions for rustfmt
fn print_rustfmt_instructions() {
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("⚠️  rustfmt installation failed!");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("\nAutomatic installation failed. Please install rustfmt manually:");
    eprintln!();
    eprintln!("  Install rustfmt (requires Rust toolchain):");
    eprintln!("    rustup component add rustfmt");
    eprintln!();
    eprintln!("  If you don't have Rust installed:");
    eprintln!("    Visit https://rustup.rs/ to install Rust");
    eprintln!();
    eprintln!("After installation, run this command again.");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Print installation instructions for clippy
fn print_clippy_instructions() {
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("⚠️  clippy installation failed!");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("\nAutomatic installation failed. Please install clippy manually:");
    eprintln!();
    eprintln!("  Install clippy (requires Rust toolchain):");
    eprintln!("    rustup component add clippy");
    eprintln!();
    eprintln!("  If you don't have Rust installed:");
    eprintln!("    Visit https://rustup.rs/ to install Rust");
    eprintln!();
    eprintln!("After installation, run this command again.");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Ensure rustfmt is available, attempting automatic installation if needed
fn ensure_rustfmt() -> Option<String> {
    // First check if it's already available
    if let Some(cmd) = find_rustfmt() {
        debug!("rustfmt found: {}", cmd);
        return Some(cmd);
    }
    
    // Not found - attempt automatic installation
    eprintln!("\n🔧 rustfmt not found. Attempting automatic installation...");
    
    if install_rustfmt_auto() {
        // Check again after installation
        if let Some(cmd) = find_rustfmt() {
            eprintln!("✅ rustfmt successfully installed!\n");
            return Some(cmd);
        }
    }
    
    // Installation failed - show manual instructions
    print_rustfmt_instructions();
    None
}

/// Ensure clippy is available, attempting automatic installation if needed
fn ensure_clippy() -> Option<String> {
    // First check if cargo is available (required for clippy)
    if let Some(cmd) = find_cargo() {
        debug!("cargo found: {}", cmd);
        
        // Check if clippy component is installed
        if Command::new(&cmd).args(&["clippy", "--version"]).output().is_ok() {
            return Some(cmd);
        }
        
        // Not found - attempt automatic installation
        eprintln!("\n🔧 clippy not found. Attempting automatic installation...");
        
        if install_clippy_auto() {
            // Check again after installation
            if Command::new(&cmd).args(&["clippy", "--version"]).output().is_ok() {
                eprintln!("✅ clippy successfully installed!\n");
                return Some(cmd);
            }
        }
        
        // Installation failed - show manual instructions
        print_clippy_instructions();
        None
    } else {
        print_clippy_instructions();
        None
    }
}

/// Search upwards from the file directory for a rustfmt.toml or .rustfmt.toml file
fn has_rustfmt_config(start: &std::path::Path) -> bool {
    let mut cur = Some(start);
    while let Some(path) = cur {
        let rustfmt_toml = path.join("rustfmt.toml");
        let dot_rustfmt_toml = path.join(".rustfmt.toml");
        if rustfmt_toml.exists() || dot_rustfmt_toml.exists() {
            return true;
        }
        cur = path.parent();
    }
    false
}

/// Format a Rust file using rustfmt
#[instrument(name = "cli_format_rust", level = "debug", skip(ctx, path))]
pub(super) fn format_rust<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting Rust file: {}", path_str);

    // Ensure rustfmt is available
    let rustfmt_cmd = match ensure_rustfmt() {
        Some(cmd) => cmd,
        None => {
            info!("rustfmt not available, skipping Rust file: {}", path_str);
            return Ok(FileStatus::Unchanged);
        }
    };

    // Read file content
    let file_path = std::path::PathBuf::from(path.as_path().as_str());
    
    let content = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Rust file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/rust")),
            ));
        }
    };

    let original_content = content.clone();

    // Determine if there's a rustfmt config file
    let has_config = if let Some(parent) = file_path.parent() {
        has_rustfmt_config(parent)
    } else {
        false
    };

    let file_path_str = match file_path.canonicalize() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => file_path.to_string_lossy().to_string(),
    };

    // Run rustfmt (defaults to project config or edition 2021)
    let mut args = vec![];
    if !has_config {
        // Use edition 2021 as default if no config found
        args.push("--edition");
        args.push("2021");
    }
    args.push(&file_path_str);

    let output = match Command::new(&rustfmt_cmd).args(&args).output() {
        Ok(output) => output,
        Err(e) => {
            error!("Failed to run rustfmt on {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/rust")),
            ));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("rustfmt failed on {}: {}", path_str, stderr);
        return Err(Message::from(
            biome_diagnostics::IoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("rustfmt error: {}", stderr),
            ))
            .with_file_path(path_str)
            .with_category(category!("format/rust")),
        ));
    }

    // Read the formatted content
    let formatted = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read formatted Rust file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/rust")),
            ));
        }
    };

    // Check if content changed
    debug!("Original content length: {}, Formatted content length: {}", original_content.len(), formatted.len());
    debug!("Content changed: {}", original_content != formatted);
    if original_content == formatted {
        debug!("Content is the same, returning Unchanged");
        return Ok(FileStatus::Unchanged);
    }

    // Handle check vs write mode
    let should_write = ctx.execution.should_write();
    if !should_write {
        // Restore original content since we're in check mode
        if let Err(e) = std::fs::write(&file_path, &original_content) {
            error!("Failed to restore original Rust file {}: {}", path_str, e);
        }
        
        ctx.push_message(Message::Diff {
            file_name: path_str.clone(),
            old: original_content.clone(),
            new: formatted.clone(),
            diff_kind: DiffKind::Format,
        });
        return Ok(FileStatus::Changed);
    }

    ctx.push_message(Message::Diff {
        file_name: path_str,
        old: original_content,
        new: formatted,
        diff_kind: DiffKind::Format,
    });

    Ok(FileStatus::Changed)
}

/// Lint a Rust file using cargo clippy
#[instrument(name = "cli_lint_rust", level = "debug", skip(ctx, path))]
pub(super) fn lint_rust<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting Rust file: {}", path_str);

    // Ensure cargo (clippy) is available
    let cargo_cmd = match ensure_clippy() {
        Some(cmd) => cmd,
        None => {
            info!("clippy not available, skipping Rust lint: {}", path_str);
            return Ok(FileStatus::Unchanged);
        }
    };

    // Convert to absolute path
    let file_path = std::path::PathBuf::from(path.as_path().as_str());
    let abs_path = match file_path.canonicalize() {
        Ok(p) => p,
        Err(_) => file_path.clone(),
    };
    let abs_path_str = abs_path.to_string_lossy().to_string();

    // Find the Cargo.toml directory (workspace root)
    let mut cargo_dir = abs_path.parent();
    while let Some(dir) = cargo_dir {
        if dir.join("Cargo.toml").exists() {
            break;
        }
        cargo_dir = dir.parent();
    }

    // Run cargo clippy for the specific file
    // Note: clippy works best when run on the entire project, but we'll try to check the specific file
    let output = if let Some(dir) = cargo_dir {
        // Change to the cargo directory and run clippy
        Command::new(&cargo_cmd)
            .args(&["clippy", "--message-format=short", "--", "-D", "warnings"])
            .current_dir(dir)
            .output()
    } else {
        // No Cargo.toml found, try to run rustc --check directly
        warn!("No Cargo.toml found for {}, performing basic syntax check only", path_str);
        Command::new("rustc")
            .args(&["--crate-type", "lib", "--emit=metadata", &abs_path_str, "-o", "/dev/null"])
            .output()
    };

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            error!("Failed to run clippy on {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/rust")),
            ));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse output for warnings/errors related to this file
    let combined_output = format!("{}\n{}", stdout, stderr);
    
    // Filter for issues related to the specific file
    let file_name = abs_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    
    let mut has_issues = false;
    for line in combined_output.lines() {
        // Check if line contains warning or error and references our file
        if (line.contains("warning:") || line.contains("error:")) && 
           (line.contains(&abs_path_str) || line.contains(file_name)) {
            has_issues = true;
            ctx.push_message(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Rust lint: {}", line.trim()),
                ))
                .with_file_path(path_str.clone()),
            ));
        }
    }

    if has_issues {
        Err(Message::Failure)
    } else {
        info!("Rust file {} passed linting", path_str);
        Ok(FileStatus::Unchanged)
    }
}

/// Check (lint and format) a Rust file
#[instrument(name = "cli_check_rust", level = "debug", skip(ctx, path))]
pub(super) fn check_rust<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking Rust file: {}", path_str);

    // First format
    let format_result = format_rust(ctx, path.clone());

    // Then lint
    let lint_result = lint_rust(ctx, path);

    // Return the more severe result
    match (format_result, lint_result) {
        (Err(e), _) | (_, Err(e)) => Err(e),
        (Ok(FileStatus::Changed), _) | (_, Ok(FileStatus::Changed)) => {
            Ok(FileStatus::Changed)
        }
        _ => Ok(FileStatus::Unchanged),
    }
}
