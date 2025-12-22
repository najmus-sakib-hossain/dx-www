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

/// Find ktlint in common installation locations
fn find_ktlint() -> Option<String> {
    // First check PATH using 'which' command (works in Git Bash)
    debug!("Checking for ktlint using 'which' command...");
    if let Ok(output) = Command::new("which").arg("ktlint").output() {
        if output.status.success() && !output.stdout.is_empty() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                let path = path.trim().to_string();
                info!("Found ktlint in PATH via 'which': {}", path);
                return Some(path);
            }
        }
    }
    
    // Fallback: check PATH directly
    debug!("Checking for ktlint in PATH...");
    if is_command_available("ktlint") {
        info!("Found ktlint in PATH");
        return Some("ktlint".to_string());
    }
    
    // Check common installation locations
    let mut common_paths: Vec<String> = Vec::new();
    
    if cfg!(windows) {
        common_paths.extend(vec![
            "C:\\Program Files\\ktlint\\ktlint.exe".to_string(),
            "C:\\Program Files\\ktlint\\ktlint".to_string(),
            "C:\\ktlint\\ktlint.exe".to_string(),
            "C:\\ktlint\\ktlint".to_string(),
            "C:\\tools\\ktlint\\ktlint.exe".to_string(),
            "C:\\tools\\ktlint\\ktlint".to_string(),
        ]);
        
        // Add user-specific paths
        if let Ok(home) = std::env::var("USERPROFILE") {
            common_paths.push(format!("{}\\.local\\bin\\ktlint.exe", home));
            common_paths.push(format!("{}\\.local\\bin\\ktlint", home));
            common_paths.push(format!("{}\\scoop\\shims\\ktlint.exe", home));
            common_paths.push(format!("{}\\scoop\\shims\\ktlint", home));
        }
    } else if cfg!(target_os = "macos") {
        common_paths.extend(vec![
            "/usr/local/bin/ktlint".to_string(),
            "/opt/homebrew/bin/ktlint".to_string(),
            "/usr/bin/ktlint".to_string(),
        ]);
        
        // Add user-specific paths
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.local/bin/ktlint", home));
        }
    } else {
        common_paths.extend(vec![
            "/usr/local/bin/ktlint".to_string(),
            "/usr/bin/ktlint".to_string(),
        ]);
        
        // Add user-specific paths
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.local/bin/ktlint", home));
        }
    }
    
    debug!("Checking common installation locations: {:?}", common_paths);
    
    for path in common_paths {
        if std::path::Path::new(&path).exists() {
            // Verify it works
            if Command::new(&path).arg("--version").output().is_ok() {
                info!("Found ktlint at: {}", path);
                return Some(path);
            }
        }
    }
    
    None
}

/// Attempt to install ktlint using system package manager or direct download
fn install_ktlint_auto() -> bool {
    info!("Attempting automatic installation of ktlint...");
    
    let result = if cfg!(target_os = "windows") {
        // Try direct download via curl (works on Git Bash/Windows)
        if let Ok(home) = std::env::var("USERPROFILE") {
            let local_bin = format!("{}/.local/bin", home.replace("\\", "/"));
            
            // Create directory if it doesn't exist
            let _ = std::fs::create_dir_all(&local_bin);
            
            let install_path = format!("{}/ktlint", local_bin);
            
            // Download using bash with curl
            let download = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "curl -sSLO https://github.com/pinterest/ktlint/releases/download/1.5.0/ktlint && chmod a+x ktlint && mv ktlint {}",
                    install_path
                ))
                .status();
            
            if download.is_ok() && download.unwrap().success() {
                info!("Successfully installed ktlint via curl to {}", install_path);
                return true;
            }
        }
        
        // Fallback: Try Chocolatey
        let choco = Command::new("choco")
            .args(&["install", "ktlint", "-y"])
            .status();
        
        if choco.is_ok() && choco.unwrap().success() {
            info!("Successfully installed ktlint via Chocolatey");
            return true;
        }
        
        // Try Scoop
        let scoop = Command::new("scoop")
            .args(&["install", "ktlint"])
            .status();
        
        if scoop.is_ok() && scoop.unwrap().success() {
            info!("Successfully installed ktlint via Scoop");
            return true;
        }
        
        false
    } else if cfg!(target_os = "macos") {
        // Try Homebrew first
        let brew = Command::new("brew")
            .args(&["install", "ktlint"])
            .status();
        
        if brew.is_ok() && brew.unwrap().success() {
            info!("Successfully installed ktlint via Homebrew");
            return true;
        }
        
        // Fallback: Direct download
        if let Ok(home) = std::env::var("HOME") {
            let local_bin = format!("{}/.local/bin", home);
            let _ = std::fs::create_dir_all(&local_bin);
            let install_path = format!("{}/ktlint", local_bin);
            
            let download = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "curl -sSLO https://github.com/pinterest/ktlint/releases/download/1.5.0/ktlint && chmod a+x ktlint && mv ktlint {}",
                    install_path
                ))
                .status();
            
            if download.is_ok() && download.unwrap().success() {
                info!("Successfully installed ktlint via curl to {}", install_path);
                return true;
            }
        }
        
        false
    } else {
        // Linux - try direct download to user's local bin first
        if let Ok(home) = std::env::var("HOME") {
            let local_bin = format!("{}/.local/bin", home);
            let _ = std::fs::create_dir_all(&local_bin);
            let install_path = format!("{}/ktlint", local_bin);
            
            let download = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "curl -sSLO https://github.com/pinterest/ktlint/releases/download/1.5.0/ktlint && chmod a+x ktlint && mv ktlint {}",
                    install_path
                ))
                .status();
            
            if download.is_ok() && download.unwrap().success() {
                info!("Successfully installed ktlint via curl to {}", install_path);
                return true;
            }
        }
        
        // Fallback: Try system-wide installation with sudo
        let wget = Command::new("wget")
            .args(&[
                "-O", "/tmp/ktlint",
                "https://github.com/pinterest/ktlint/releases/download/1.5.0/ktlint"
            ])
            .status();
        
        if wget.is_ok() && wget.unwrap().success() {
            let chmod = Command::new("chmod")
                .args(&["+x", "/tmp/ktlint"])
                .status();
            
            if chmod.is_ok() && chmod.unwrap().success() {
                let mv = Command::new("sudo")
                    .args(&["mv", "/tmp/ktlint", "/usr/local/bin/ktlint"])
                    .status();
                
                if mv.is_ok() && mv.unwrap().success() {
                    info!("Successfully installed ktlint via wget");
                    return true;
                }
            }
        }
        
        false
    };
    
    result
}

/// Print installation instructions for ktlint
fn print_ktlint_instructions() {
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("⚠️  ktlint installation failed!");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("\nAutomatic installation failed. Please install ktlint manually:");
    eprintln!();
    
    if cfg!(target_os = "windows") {
        eprintln!("  Windows (using Chocolatey):");
        eprintln!("    choco install ktlint");
        eprintln!();
        eprintln!("  Windows (using Scoop):");
        eprintln!("    scoop install ktlint");
        eprintln!();
        eprintln!("  Windows (manual):");
        eprintln!("    Download from: https://github.com/pinterest/ktlint/releases");
        eprintln!("    Place ktlint.exe in your PATH");
    } else if cfg!(target_os = "macos") {
        eprintln!("  macOS (using Homebrew):");
        eprintln!("    brew install ktlint");
        eprintln!();
        eprintln!("  macOS (manual):");
        eprintln!("    curl -sSLO https://github.com/pinterest/ktlint/releases/latest/download/ktlint");
        eprintln!("    chmod a+x ktlint");
        eprintln!("    sudo mv ktlint /usr/local/bin/");
    } else {
        eprintln!("  Linux (manual download):");
        eprintln!("    wget -O ktlint https://github.com/pinterest/ktlint/releases/latest/download/ktlint");
        eprintln!("    chmod a+x ktlint");
        eprintln!("    sudo mv ktlint /usr/local/bin/");
    }
    
    eprintln!();
    eprintln!("After installation, run this command again.");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Helper to run ktlint command with proper handling for shell scripts on Windows  
fn run_ktlint_command(ktlint_cmd: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    // On Windows, ktlint is a shell script that needs to be executed via bash
    if cfg!(windows) {
        // If ktlint_cmd looks like a path (contains slashes or backslashes), pass it
        // directly to bash so it executes the script file. Otherwise use `bash -lc` so
        // PATH resolution works (ktlint may be a shim in PATH).
        if ktlint_cmd.contains('/') || ktlint_cmd.contains('\\') {
            let mut command = Command::new("bash");
            command.arg(ktlint_cmd); // bash will execute the script file
            for arg in args {
                command.arg(arg);
            }
            command.stdin(std::process::Stdio::null()).output()
        } else {
            // Build a single command string so bash resolves ktlint via PATH
            let mut parts: Vec<String> = Vec::with_capacity(1 + args.len());
            parts.push(ktlint_cmd.to_string());
            for &a in args {
                // Simple escaping: wrap in single-quotes if contains whitespace or special chars
                if a.contains(' ') || a.contains('"') || a.contains('\'') {
                    let escaped = a.replace("'", "'\\''");
                    parts.push(format!("'{}'", escaped));
                } else {
                    parts.push(a.to_string());
                }
            }
            let cmdline = parts.join(" ");
            Command::new("bash").arg("-lc").arg(cmdline).stdin(std::process::Stdio::null()).output()
        }
    } else {
        // On Unix, execute directly
        Command::new(ktlint_cmd).args(args).output()
    }
}

/// Ensure ktlint is available, attempting automatic installation if needed
fn ensure_ktlint() -> Option<String> {
    // First check if it's already available (in PATH or common locations)
    if let Some(cmd) = find_ktlint() {
        debug!("ktlint found: {}", cmd);
        return Some(cmd);
    }
    
    // Not found - attempt automatic installation
    eprintln!("\n🔧 ktlint not found. Attempting automatic installation...");
    
    if install_ktlint_auto() {
        // Check again after installation
        if let Some(cmd) = find_ktlint() {
            eprintln!("✅ ktlint successfully installed!\n");
            return Some(cmd);
        }
    }
    
    // Installation failed - show manual instructions
    print_ktlint_instructions();
    None
}

/// Format a Kotlin file using ktlint
#[instrument(name = "cli_format_kotlin", level = "debug", skip(ctx, path))]
pub(super) fn format_kotlin<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting Kotlin file: {}", path_str);

    // Ensure ktlint is available, guiding installation if needed
    let ktlint_cmd = match ensure_ktlint() {
        Some(cmd) => cmd,
        None => {
            info!("ktlint not available, skipping Kotlin file: {}", path_str);
            return Ok(FileStatus::Unchanged);
        }
    };

    // Read file content - convert BiomePath to std PathBuf for proper handling
    let file_path = std::path::PathBuf::from(path.as_path().as_str());
    
    let content = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Kotlin file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/kotlin")),
            ));
        }
    };

    let original_content = content.clone();

    // Run ktlint format - use the file path directly
    let file_path_str = file_path.to_string_lossy().to_string();
    let output = match run_ktlint_command(&ktlint_cmd, &["--format", &file_path_str]) {
        Ok(output) => output,
        Err(e) => {
            error!("Failed to run ktlint on {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/kotlin")),
            ));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // ktlint returns exit code 1 when it finds and fixes issues, but stderr should be empty
        // Only treat as error if there's actual stderr content
        if !stderr.trim().is_empty() {
            error!("ktlint failed on {}: {}", path_str, stderr);
            return Err(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("ktlint error: {}", stderr),
                ))
                .with_file_path(path_str)
                .with_category(category!("format/kotlin")),
            ));
        }
        // If stderr is empty but exit code is 1, it means ktlint found and fixed issues
        info!("ktlint found and fixed issues in {}", path_str);
    }

    // Read the formatted content (ktlint modifies the file in place)
    let formatted = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read formatted Kotlin file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/kotlin")),
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
            error!("Failed to restore original Kotlin file {}: {}", path_str, e);
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

/// Lint a Kotlin file using ktlint
#[instrument(name = "cli_lint_kotlin", level = "debug", skip(ctx, path))]
pub(super) fn lint_kotlin<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting Kotlin file: {}", path_str);

    // Ensure ktlint is available, guiding installation if needed
    let ktlint_cmd = match ensure_ktlint() {
        Some(cmd) => cmd,
        None => {
            info!("ktlint not available, skipping Kotlin lint: {}", path_str);
            return Ok(FileStatus::Unchanged);
        }
    };

    // Run ktlint lint (without --format flag)
    let file_path = std::path::PathBuf::from(path.as_path().as_str());
    let abs_path = match file_path.canonicalize() {
        Ok(p) => p,
        Err(_) => file_path.clone(),
    };
    let abs_path_str = abs_path.to_string_lossy().to_string();
    
    let output = match run_ktlint_command(&ktlint_cmd, &[&abs_path_str]) {
        Ok(output) => output,
        Err(e) => {
            error!("Failed to run ktlint on {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/kotlin")),
            ));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse output for warnings/errors
    let combined_output = format!("{}\n{}", stdout, stderr);
    let has_issues = !output.status.success() || combined_output.contains("✖");

    if has_issues {
        // Extract and report issues
        for line in combined_output.lines() {
            if !line.trim().is_empty() && (line.contains(":") || line.contains("✖")) {
                ctx.push_message(Message::from(
                    biome_diagnostics::IoError::from(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Kotlin lint: {}", line.trim()),
                    ))
                    .with_file_path(path_str.clone()),
                ));
            }
        }
        Err(Message::Failure)
    } else {
        info!("Kotlin file {} passed linting", path_str);
        Ok(FileStatus::Unchanged)
    }
}

/// Check (lint and format) a Kotlin file
#[instrument(name = "cli_check_kotlin", level = "debug", skip(ctx, path))]
pub(super) fn check_kotlin<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking Kotlin file: {}", path_str);

    // First lint
    let lint_result = lint_kotlin(ctx, path.clone());

    // Continue with formatting even if linting found issues
    let format_result = format_kotlin(ctx, path);

    // Return the more severe result
    match (lint_result, format_result) {
        (Err(e), _) | (_, Err(e)) => Err(e),
        (Ok(FileStatus::Changed), _) | (_, Ok(FileStatus::Changed)) => {
            Ok(FileStatus::Changed)
        }
        _ => Ok(FileStatus::Unchanged),
    }
}
