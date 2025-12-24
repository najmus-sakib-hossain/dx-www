use super::{DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::BiomePath;
use ruff_python_ast::PySourceType;
use ruff_python_formatter::{format_module_source, PyFormatOptions};
use tracing::{debug, error, info, instrument};

/// Format a Python file using ruff_python_formatter
#[instrument(name = "cli_format_python", level = "debug", skip(ctx, path))]
pub(super) fn format_python<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting Python file: {}", path_str);

    // Read file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Python file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/python")),
            ));
        }
    };

    let original_content = content.clone();

    // Determine source type based on extension
    let source_type = if path_str.ends_with(".pyi") {
        PySourceType::Stub
    } else {
        PySourceType::Python
    };

    // Create format options with defaults
    let options = PyFormatOptions::from_source_type(source_type);

    // Format the Python code
    let formatted = match format_module_source(&content, options) {
        Ok(printed) => printed.into_code(),
        Err(e) => {
            error!("Failed to format Python file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Python format error: {}", e),
                ))
                .with_file_path(path_str)
                .with_category(category!("format/python")),
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
        error!(
            "Failed to write formatted Python file {}: {}",
            path_str, e
        );
        return Err(Message::from(
            biome_diagnostics::IoError::from(e)
                .with_file_path(path_str)
                .with_category(category!("format/python")),
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

/// Lint a Python file - Currently just validates parsing
#[instrument(name = "cli_lint_python", level = "debug", skip(ctx, path))]
pub(super) fn lint_python<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting Python file: {}", path_str);

    // Read file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Python file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/python")),
            ));
        }
    };

    // Determine source type
    let source_type = if path_str.ends_with(".pyi") {
        PySourceType::Stub
    } else {
        PySourceType::Python
    };

    // Try to format to validate syntax
    let options = PyFormatOptions::from_source_type(source_type);
    match format_module_source(&content, options) {
        Ok(_) => {
            info!("Python file {} has valid syntax", path_str);
            Ok(FileStatus::Unchanged)
        }
        Err(e) => {
            let msg = format!("Python syntax error: {}", e);
            ctx.push_message(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    msg,
                ))
                .with_file_path(path_str.clone()),
            ));
            Err(Message::Failure)
        }
    }
}

/// Check (lint and format) a Python file
#[instrument(name = "cli_check_python", level = "debug", skip(ctx, path))]
pub(super) fn check_python<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking Python file: {}", path_str);

    // First lint (validate syntax)
    let lint_result = lint_python(ctx, path.clone());

    // Continue with formatting even if linting found issues
    let format_result = format_python(ctx, path);

    // Return the more severe result
    match (lint_result, format_result) {
        (Err(e), _) | (_, Err(e)) => Err(e),
        (Ok(FileStatus::Changed), _) | (_, Ok(FileStatus::Changed)) => {
            Ok(FileStatus::Changed)
        }
        _ => Ok(FileStatus::Unchanged),
    }
}
