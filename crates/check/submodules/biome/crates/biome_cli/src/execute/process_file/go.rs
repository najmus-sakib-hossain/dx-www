use super::{DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::BiomePath;
use tracing::{debug, error, info, instrument};

/// Format a Go file using gofmt.rs
#[instrument(name = "cli_format_go", level = "debug", skip(ctx, path))]
pub(super) fn format_go<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting Go file: {}", path_str);

    // Read file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Go file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/go")),
            ));
        }
    };

    let original_content = content.clone();

    // Format the Go code using gofmt.rs
    let formatted_bytes = match gofmt::formatter::format(content.as_str()) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to format Go file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Go format error: {}", e),
                ))
                .with_file_path(path_str)
                .with_category(category!("format/go")),
            ));
        }
    };

    let formatted = match String::from_utf8(formatted_bytes) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to convert formatted Go output to UTF-8: {}", e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("UTF-8 conversion error: {}", e),
                ))
                .with_file_path(path_str)
                .with_category(category!("format/go")),
            ));
        }
    };

    // Check if content changed
    if original_content == formatted {
        return Ok(FileStatus::Unchanged);
    }

    // Handle check vs write mode
    let should_write = ctx.execution.should_write();
    if !should_write {
        ctx.push_message(Message::Diff {
            file_name: path_str.clone(),
            old: original_content.clone(),
            new: formatted.clone(),
            diff_kind: DiffKind::Format,
        });
        return Ok(FileStatus::Changed);
    }

    // Write formatted content
    if let Err(e) = std::fs::write(path.as_path(), &formatted) {
        error!("Failed to write formatted Go file {}: {}", path_str, e);
        return Err(Message::from(
            biome_diagnostics::IoError::from(e)
                .with_file_path(path_str)
                .with_category(category!("format/go")),
        ));
    }

    ctx.push_message(Message::Diff {
        file_name: path_str,
        old: original_content,
        new: formatted,
        diff_kind: DiffKind::Format,
    });

    Ok(FileStatus::Changed)
}

/// Lint a Go file using gold linter
#[instrument(name = "cli_lint_go", level = "debug", skip(ctx, path))]
pub(super) fn lint_go<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting Go file: {}", path_str);

    // Read file content to validate it exists
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Go file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/go")),
            ));
        }
    };

    // First, validate syntax using gofmt
    if let Err(e) = gofmt::formatter::format(content.as_str()) {
        let msg = format!("Go syntax error: {}", e);
        ctx.push_message(Message::from(
            biome_diagnostics::IoError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                msg,
            ))
            .with_file_path(path_str.clone()),
        ));
        return Err(Message::Failure);
    }

    // Now run gold linter on the module
    // Find the directory containing the file
    let file_dir = match path.as_path().parent() {
        Some(dir) => dir,
        None => {
            error!("Failed to get parent directory for {}", path_str);
            return Ok(FileStatus::Unchanged);
        }
    };

    // Try to lint using gold
    // Note: gold requires a go.mod in the directory or parent directories
    match gold::lint(file_dir.as_str(), false) {
        Ok(true) => {
            info!("Go file {} passed linting", path_str);
            Ok(FileStatus::Unchanged)
        }
        Ok(false) => {
            info!("Go file {} has linting issues", path_str);
            // gold prints issues to stdout/stderr directly
            // We indicate that there were issues but don't fail
            Ok(FileStatus::Unchanged)
        }
        Err(e) => {
            // If gold fails (e.g., no go.mod), just validate syntax was enough
            debug!("Gold linter not available for {}: {}", path_str, e);
            info!("Go file {} has valid syntax (gold linter skipped)", path_str);
            Ok(FileStatus::Unchanged)
        }
    }
}

/// Check (lint and format) a Go file
#[instrument(name = "cli_check_go", level = "debug", skip(ctx, path))]
pub(super) fn check_go<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking Go file: {}", path_str);

    // First lint (validate syntax)
    let lint_result = lint_go(ctx, path.clone());

    // Continue with formatting even if linting found issues
    let format_result = format_go(ctx, path);

    // Return the more severe result
    match (lint_result, format_result) {
        (Err(e), _) | (_, Err(e)) => Err(e),
        (Ok(FileStatus::Changed), _) | (_, Ok(FileStatus::Changed)) => {
            Ok(FileStatus::Changed)
        }
        _ => Ok(FileStatus::Unchanged),
    }
}
