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

/// Find clang-format in common installation locations
fn find_clang_format() -> Option<String> {
    // First check PATH using 'which' command (works in Git Bash)
    debug!("Checking for clang-format using 'which' command...");
    if let Ok(output) = Command::new("which").arg("clang-format").output() {
        if output.status.success() && !output.stdout.is_empty() {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let original_path = path_str.trim().to_string();
                // Convert Git Bash Unix-style path to Windows path
                let windows_path = if original_path.starts_with("/c/") {
                    original_path.replace("/c/", "C:\\").replace("/", "\\")
                } else {
                    original_path.clone()
                };
                info!("Found clang-format in PATH via 'which': {} -> {}", original_path, windows_path);
                return Some(windows_path);
            }
        }
    }
    
    // Fallback: check PATH directly
    if is_command_available("clang-format") {
        return Some("clang-format".to_string());
    }
    
    // Check common installation locations
    let mut common_paths: Vec<String> = Vec::new();
    
    if cfg!(windows) {
        common_paths.extend(vec![
            "C:\\Program Files\\LLVM\\bin\\clang-format.exe".to_string(),
            "C:\\Program Files (x86)\\LLVM\\bin\\clang-format.exe".to_string(),
            "C:\\LLVM\\bin\\clang-format.exe".to_string(),
        ]);
        
        // Add user-specific paths
        if let Ok(home) = std::env::var("USERPROFILE") {
            common_paths.push(format!("{}\\.local\\bin\\clang-format.exe", home));
            common_paths.push(format!("{}\\.local\\bin\\clang-format", home));
        }
    } else if cfg!(target_os = "macos") {
        common_paths.extend(vec![
            "/usr/local/bin/clang-format".to_string(),
            "/opt/homebrew/bin/clang-format".to_string(),
            "/usr/bin/clang-format".to_string(),
        ]);
        
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.local/bin/clang-format", home));
        }
    } else {
        common_paths.extend(vec![
            "/usr/bin/clang-format".to_string(),
            "/usr/local/bin/clang-format".to_string(),
        ]);
        
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.local/bin/clang-format", home));
        }
    }
    
    for path in common_paths {
        if std::path::Path::new(&path).exists() {
            // Verify it works
            if Command::new(&path).arg("--version").output().is_ok() {
                info!("Found clang-format at: {}", path);
                return Some(path);
            }
        }
    }
    
    None
}

/// Find clang-tidy in common installation locations
fn find_clang_tidy() -> Option<String> {
    // First check PATH using 'which' command (works in Git Bash)
    debug!("Checking for clang-tidy using 'which' command...");
    if let Ok(output) = Command::new("which").arg("clang-tidy").output() {
        if output.status.success() && !output.stdout.is_empty() {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let original_path = path_str.trim().to_string();
                // Convert Git Bash Unix-style path to Windows path
                let windows_path = if original_path.starts_with("/c/") {
                    original_path.replace("/c/", "C:\\").replace("/", "\\")
                } else {
                    original_path.clone()
                };
                info!("Found clang-tidy in PATH via 'which': {} -> {}", original_path, windows_path);
                return Some(windows_path);
            }
        }
    }
    
    // Fallback: check PATH directly
    if is_command_available("clang-tidy") {
        return Some("clang-tidy".to_string());
    }
    
    // Check common installation locations
    let mut common_paths: Vec<String> = Vec::new();
    
    if cfg!(windows) {
        common_paths.extend(vec![
            "C:\\Program Files\\LLVM\\bin\\clang-tidy.exe".to_string(),
            "C:\\Program Files (x86)\\LLVM\\bin\\clang-tidy.exe".to_string(),
            "C:\\LLVM\\bin\\clang-tidy.exe".to_string(),
        ]);
        
        // Add user-specific paths
        if let Ok(home) = std::env::var("USERPROFILE") {
            common_paths.push(format!("{}\\.local\\bin\\clang-tidy.exe", home));
            common_paths.push(format!("{}\\.local\\bin\\clang-tidy", home));
        }
    } else if cfg!(target_os = "macos") {
        common_paths.extend(vec![
            "/usr/local/bin/clang-tidy".to_string(),
            "/opt/homebrew/bin/clang-tidy".to_string(),
            "/usr/bin/clang-tidy".to_string(),
        ]);
        
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.local/bin/clang-tidy", home));
        }
    } else {
        common_paths.extend(vec![
            "/usr/bin/clang-tidy".to_string(),
            "/usr/local/bin/clang-tidy".to_string(),
        ]);
        
        if let Ok(home) = std::env::var("HOME") {
            common_paths.push(format!("{}/.local/bin/clang-tidy", home));
        }
    }
    
    for path in common_paths {
        if std::path::Path::new(&path).exists() {
            // Verify it works
            if Command::new(&path).arg("--version").output().is_ok() {
                info!("Found clang-tidy at: {}", path);
                return Some(path);
            }
        }
    }
    
    None
}

/// Attempt to install clang-format using system package manager or direct download
fn install_clang_format_auto() -> bool {
    info!("Attempting automatic installation of clang-format...");
    
    let result = if cfg!(target_os = "windows") {
        // Try direct download via curl (download static binary from community repo)
        if let Ok(home) = std::env::var("USERPROFILE") {
            let local_bin = format!("{}/.local/bin", home.replace("\\", "/"));
            let _ = std::fs::create_dir_all(&local_bin);
            
            // Download clang-format from muttleyxd's static binaries repo
            let install_path = format!("{}/clang-format.exe", local_bin);
            
            let download = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "curl -sSL https://github.com/muttleyxd/clang-tools-static-binaries/releases/download/master-796e77c/clang-format-18_windows-amd64.exe -o {}",
                    install_path
                ))
                .status();
            
            if download.is_ok() && download.unwrap().success() {
                // Verify the download worked (file should be > 1MB)
                if let Ok(metadata) = std::fs::metadata(&install_path) {
                    if metadata.len() > 1_000_000 {
                        info!("Successfully installed clang-format via curl to {}", install_path);
                        return true;
                    }
                }
            }
        }
        
        // Fallback: Try Chocolatey
        let choco = Command::new("choco")
            .args(&["install", "llvm", "-y"])
            .status();
        
        if choco.is_ok() && choco.unwrap().success() {
            info!("Successfully installed LLVM (includes clang-format) via Chocolatey");
            return true;
        }
        
        // Try Scoop
        let scoop = Command::new("scoop")
            .args(&["install", "llvm"])
            .status();
        
        if scoop.is_ok() && scoop.unwrap().success() {
            info!("Successfully installed LLVM (includes clang-format) via Scoop");
            return true;
        }
        
        false
    } else if cfg!(target_os = "macos") {
        // Try Homebrew first
        let brew = Command::new("brew")
            .args(&["install", "clang-format"])
            .status();
        
        if brew.is_ok() && brew.unwrap().success() {
            info!("Successfully installed clang-format via Homebrew");
            return true;
        }
        
        false
    } else {
        // Linux - try common package managers
        
        // Try apt-get (Debian/Ubuntu)
        let apt = Command::new("sudo")
            .args(&["apt-get", "install", "-y", "clang-format"])
            .status();
        
        if apt.is_ok() && apt.unwrap().success() {
            info!("Successfully installed clang-format via apt-get");
            return true;
        }
        
        // Try dnf (Fedora/RHEL)
        let dnf = Command::new("sudo")
            .args(&["dnf", "install", "-y", "clang-tools-extra"])
            .status();
        
        if dnf.is_ok() && dnf.unwrap().success() {
            info!("Successfully installed clang-format via dnf");
            return true;
        }
        
        // Try pacman (Arch)
        let pacman = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "clang"])
            .status();
        
        if pacman.is_ok() && pacman.unwrap().success() {
            info!("Successfully installed clang-format via pacman");
            return true;
        }
        
        false
    };
    
    result
}

/// Attempt to install clang-tidy using system package manager or direct download
fn install_clang_tidy_auto() -> bool {
    info!("Attempting automatic installation of clang-tidy...");
    
    let result = if cfg!(target_os = "windows") {
        // Try direct download via curl (download static binary from community repo)
        if let Ok(home) = std::env::var("USERPROFILE") {
            let local_bin = format!("{}/.local/bin", home.replace("\\", "/"));
            let _ = std::fs::create_dir_all(&local_bin);
            
            // Download clang-tidy from muttleyxd's static binaries repo
            let install_path = format!("{}/clang-tidy.exe", local_bin);
            
            let download = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "curl -sSL https://github.com/muttleyxd/clang-tools-static-binaries/releases/download/master-796e77c/clang-tidy-18_windows-amd64.exe -o {}",
                    install_path
                ))
                .status();
            
            if download.is_ok() && download.unwrap().success() {
                // Verify the download worked (file should be > 1MB)
                if let Ok(metadata) = std::fs::metadata(&install_path) {
                    if metadata.len() > 1_000_000 {
                        info!("Successfully installed clang-tidy via curl to {}", install_path);
                        return true;
                    }
                }
            }
        }
        
        // Fallback: Try Chocolatey
        let choco = Command::new("choco")
            .args(&["install", "llvm", "-y"])
            .status();
        
        if choco.is_ok() && choco.unwrap().success() {
            info!("Successfully installed LLVM (includes clang-tidy) via Chocolatey");
            return true;
        }
        
        // Try Scoop
        let scoop = Command::new("scoop")
            .args(&["install", "llvm"])
            .status();
        
        if scoop.is_ok() && scoop.unwrap().success() {
            info!("Successfully installed LLVM (includes clang-tidy) via Scoop");
            return true;
        }
        
        false
    } else if cfg!(target_os = "macos") {
        // Try Homebrew first
        let brew = Command::new("brew")
            .args(&["install", "llvm"])
            .status();
        
        if brew.is_ok() && brew.unwrap().success() {
            info!("Successfully installed LLVM (includes clang-tidy) via Homebrew");
            return true;
        }
        
        false
    } else {
        // Linux - try common package managers
        
        // Try apt-get (Debian/Ubuntu)
        let apt = Command::new("sudo")
            .args(&["apt-get", "install", "-y", "clang-tidy"])
            .status();
        
        if apt.is_ok() && apt.unwrap().success() {
            info!("Successfully installed clang-tidy via apt-get");
            return true;
        }
        
        // Try dnf (Fedora/RHEL)
        let dnf = Command::new("sudo")
            .args(&["dnf", "install", "-y", "clang-tools-extra"])
            .status();
        
        if dnf.is_ok() && dnf.unwrap().success() {
            info!("Successfully installed clang-tidy via dnf");
            return true;
        }
        
        // Try pacman (Arch)
        let pacman = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "clang"])
            .status();
        
        if pacman.is_ok() && pacman.unwrap().success() {
            info!("Successfully installed clang-tidy via pacman");
            return true;
        }
        
        false
    };
    
    result
}

/// Helper to run clang commands with proper handling on Windows
fn run_clang_command(cmd: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    // Call clang tools directly
    Command::new(cmd).args(args).output()
}

// NOTE: previously there was a helper to run clang-format via stdin/stdout
// (run_clang_format_stdin). It was unused and produced a dead-code warning.
// The function was removed to keep the codebase minimal. If in the future
// piping content via stdin is needed, reintroduce a dedicated helper.

// Search upwards from the file directory for a .clang-format or _clang-format file.
fn has_clang_format_file(start: &std::path::Path) -> bool {
    let mut cur = Some(start);
    while let Some(path) = cur {
        let dot = path.join(".clang-format");
        let unders = path.join("_clang-format");
        if dot.exists() || unders.exists() {
            return true;
        }
        cur = path.parent();
    }
    false
}

/// Print installation instructions for clang-format
fn print_clang_format_instructions() {
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("⚠️  clang-format installation failed!");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("\nAutomatic installation failed. Please install clang-format manually:");
    eprintln!();
    
    if cfg!(target_os = "windows") {
        eprintln!("  Windows (using Chocolatey):");
        eprintln!("    choco install llvm");
        eprintln!();
        eprintln!("  Windows (using Scoop):");
        eprintln!("    scoop install llvm");
        eprintln!();
        eprintln!("  Windows (manual):");
        eprintln!("    Download from: https://github.com/llvm/llvm-project/releases");
    } else if cfg!(target_os = "macos") {
        eprintln!("  macOS (using Homebrew):");
        eprintln!("    brew install clang-format");
        eprintln!();
        eprintln!("  macOS (using MacPorts):");
        eprintln!("    sudo port install clang-17");
    } else {
        eprintln!("  Ubuntu/Debian:");
        eprintln!("    sudo apt-get update && sudo apt-get install clang-format");
        eprintln!();
        eprintln!("  Fedora/RHEL:");
        eprintln!("    sudo dnf install clang-tools-extra");
        eprintln!();
        eprintln!("  Arch Linux:");
        eprintln!("    sudo pacman -S clang");
    }
    
    eprintln!();
    eprintln!("After installation, run this command again.");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Print installation instructions for clang-tidy
fn print_clang_tidy_instructions() {
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("⚠️  clang-tidy installation failed!");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("\nAutomatic installation failed. Please install clang-tidy manually:");
    eprintln!();
    
    if cfg!(target_os = "windows") {
        eprintln!("  Windows (using Chocolatey):");
        eprintln!("    choco install llvm");
        eprintln!();
        eprintln!("  Windows (using Scoop):");
        eprintln!("    scoop install llvm");
        eprintln!();
        eprintln!("  Windows (manual):");
        eprintln!("    Download from: https://github.com/llvm/llvm-project/releases");
    } else if cfg!(target_os = "macos") {
        eprintln!("  macOS (using Homebrew):");
        eprintln!("    brew install llvm");
        eprintln!();
        eprintln!("  macOS (using MacPorts):");
        eprintln!("    sudo port install clang-17");
    } else {
        eprintln!("  Ubuntu/Debian:");
        eprintln!("    sudo apt-get update && sudo apt-get install clang-tidy");
        eprintln!();
        eprintln!("  Fedora/RHEL:");
        eprintln!("    sudo dnf install clang-tools-extra");
        eprintln!();
        eprintln!("  Arch Linux:");
        eprintln!("    sudo pacman -S clang");
    }
    
    eprintln!();
    eprintln!("After installation, run this command again.");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Ensure clang-format is available, attempting automatic installation if needed
fn ensure_clang_format() -> Option<String> {
    // First check if it's already available (in PATH or common locations)
    if let Some(cmd) = find_clang_format() {
        debug!("clang-format found: {}", cmd);
        return Some(cmd);
    }
    
    // Not found - attempt automatic installation
    eprintln!("\n🔧 clang-format not found. Attempting automatic installation...");
    
    if install_clang_format_auto() {
        // Check again after installation
        if let Some(cmd) = find_clang_format() {
            eprintln!("✅ clang-format successfully installed!\n");
            return Some(cmd);
        }
    }
    
    // Installation failed - show manual instructions
    print_clang_format_instructions();
    None
}

/// Ensure clang-tidy is available, attempting automatic installation if needed
fn ensure_clang_tidy() -> Option<String> {
    // First check if it's already available (in PATH or common locations)
    if let Some(cmd) = find_clang_tidy() {
        debug!("clang-tidy found: {}", cmd);
        return Some(cmd);
    }
    
    // Not found - attempt automatic installation
    eprintln!("\n🔧 clang-tidy not found. Attempting automatic installation...");
    
    if install_clang_tidy_auto() {
        // Check again after installation
        if let Some(cmd) = find_clang_tidy() {
            eprintln!("✅ clang-tidy successfully installed!\n");
            return Some(cmd);
        }
    }
    
    // Installation failed - show manual instructions
    print_clang_tidy_instructions();
    None
}

/// Format a C/C++ file using clang-format
#[instrument(name = "cli_format_cpp", level = "debug", skip(ctx, path))]
pub(super) fn format_cpp<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting C/C++ file: {}", path_str);

    // Ensure clang-format is available, guiding installation if needed
    let clang_format_cmd = match ensure_clang_format() {
        Some(cmd) => cmd,
        None => {
            info!("clang-format not available, skipping C/C++ file: {}", path_str);
            return Ok(FileStatus::Unchanged);
        }
    };

    // Read file content - convert to absolute path
    let file_path = std::path::PathBuf::from(path.as_path().as_str());
    
    // Use the original path for reading, canonicalize can cause issues on Windows
    let abs_path = file_path.clone();
    let _abs_path_str = abs_path.to_string_lossy().to_string();
    
    let content = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read C/C++ file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/cpp")),
            ));
        }
    };

    let original_content = content.clone();

    // Run clang-format with -i flag to format in-place. Prefer project .clang-format when present
    let style_flag = if let Some(parent) = file_path.parent() {
        if has_clang_format_file(parent) {
            "-style=file"
        } else {
            "-style=Google"
        }
    } else {
        "-style=Google"
    };

    let file_path_str = match file_path.canonicalize() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => file_path.to_string_lossy().to_string(),
    };

    let output = match run_clang_command(&clang_format_cmd, &["-i", style_flag, &file_path_str]) {
        Ok(output) => output,
        Err(e) => {
            error!("Failed to run clang-format on {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/cpp")),
            ));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("clang-format failed on {}: {}", path_str, stderr);
        return Err(Message::from(
            biome_diagnostics::IoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("clang-format error: {}", stderr),
            ))
            .with_file_path(path_str)
            .with_category(category!("format/cpp")),
        ));
    }

    // Read the formatted content
    let formatted = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read formatted C/C++ file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/cpp")),
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
            error!("Failed to restore original C/C++ file {}: {}", path_str, e);
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

/// Lint a C/C++ file using clang-tidy
#[instrument(name = "cli_lint_cpp", level = "debug", skip(ctx, path))]
pub(super) fn lint_cpp<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting C/C++ file: {}", path_str);

    // Ensure clang-tidy is available, guiding installation if needed
    let clang_tidy_cmd = match ensure_clang_tidy() {
        Some(cmd) => cmd,
        None => {
            info!("clang-tidy not available, skipping C/C++ lint: {}", path_str);
            return Ok(FileStatus::Unchanged);
        }
    };

    // Determine language standard based on file extension
    let extension = path.extension().unwrap_or("");
    let std_flag = if extension == "cpp" || extension == "cc" || extension == "cxx" || extension == "hpp" || extension == "hxx" {
        "-std=c++17"
    } else {
        "-std=c11"
    };

    // Convert to absolute path
    let file_path = std::path::PathBuf::from(path.as_path().as_str());
    let abs_path = match file_path.canonicalize() {
        Ok(p) => p,
        Err(_) => file_path.clone(),
    };
    let abs_path_str = abs_path.to_string_lossy().to_string();

    // Run clang-tidy with appropriate standard
    let output = match run_clang_command(&clang_tidy_cmd, &[&abs_path_str, "--", std_flag]) {
        Ok(output) => output,
        Err(e) => {
            error!("Failed to run clang-tidy on {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/cpp")),
            ));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse output for warnings/errors
    let combined_output = format!("{}\n{}", stdout, stderr);
    let has_issues = combined_output.contains("warning:") || combined_output.contains("error:");

    if has_issues {
        // Extract and report issues
        for line in combined_output.lines() {
            if line.contains("warning:") || line.contains("error:") {
                ctx.push_message(Message::from(
                    biome_diagnostics::IoError::from(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("C/C++ lint: {}", line.trim()),
                    ))
                    .with_file_path(path_str.clone()),
                ));
            }
        }
        Err(Message::Failure)
    } else {
        info!("C/C++ file {} passed linting", path_str);
        Ok(FileStatus::Unchanged)
    }
}

/// Check (lint and format) a C/C++ file
#[instrument(name = "cli_check_cpp", level = "debug", skip(ctx, path))]
pub(super) fn check_cpp<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking C/C++ file: {}", path_str);

    // First lint
    let lint_result = lint_cpp(ctx, path.clone());

    // Continue with formatting even if linting found issues
    let format_result = format_cpp(ctx, path);

    // Return the more severe result
    match (lint_result, format_result) {
        (Err(e), _) | (_, Err(e)) => Err(e),
        (Ok(FileStatus::Changed), _) | (_, Ok(FileStatus::Changed)) => {
            Ok(FileStatus::Changed)
        }
        _ => Ok(FileStatus::Unchanged),
    }
}
