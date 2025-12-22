use super::{DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions};
use crate::execute::diagnostics::SkippedDiagnostic;
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::BiomePath;
use taplo::{formatter, parser};
use tracing::{debug, error, info, instrument};

/// Format a TOML file using Taplo
#[instrument(name = "cli_format_toml", level = "debug", skip(ctx, path))]
pub(super) fn format_toml<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting TOML file: {}", path_str);

    // Read the file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read TOML file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/toml")),
            ));
        }
    };

    // Parse the TOML file
    let parse = parser::parse(&content);

    // Check for parse errors
    if !parse.errors.is_empty() {
        let skip_parse_errors = ctx.execution.should_skip_parse_errors();
        
        if skip_parse_errors {
            ctx.push_message(Message::from(
                SkippedDiagnostic.with_file_path(path_str.clone()),
            ));
            return Ok(FileStatus::Ignored);
        }

        // Report parse errors
        for error in &parse.errors {
            let msg = format!("TOML parse error at {:?}: {}", error.range, error);
            ctx.push_message(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    msg.clone(),
                ))
                .with_file_path(path_str.clone()),
            ));
        }

        return Ok(FileStatus::Ignored);
    }

    // Format the TOML using Taplo formatter
    // Note: align_entries feature may not work as expected in Taplo 0.14.0
    let format_opts = formatter::Options {
        align_entries: true,
        align_comments: false,
        ..formatter::Options::default()
    };
    let formatted = formatter::format(&content, format_opts);

    // Check if content changed
    if content == formatted {
        return Ok(FileStatus::Unchanged);
    }

    // If we're in check mode, report that the file needs formatting
    let should_write = ctx.execution.should_write();
    if !should_write {
        ctx.push_message(Message::Diff {
            file_name: path_str.clone(),
            old: content.clone(),
            new: formatted.clone(),
            diff_kind: DiffKind::Format,
        });
        return Ok(FileStatus::Changed);
    }

    // Write the formatted content back
    if let Err(e) = std::fs::write(path.as_path(), &formatted) {
        error!("Failed to write formatted TOML file {}: {}", path_str, e);
        return Err(Message::from(
            biome_diagnostics::IoError::from(e)
                .with_file_path(path_str)
                .with_category(category!("format/toml")),
        ));
    }

    ctx.push_message(Message::Diff {
        file_name: path_str,
        old: content,
        new: formatted,
        diff_kind: DiffKind::Format,
    });

    Ok(FileStatus::Changed)
}

/// Lint a TOML file using Taplo
#[instrument(name = "cli_lint_toml", level = "debug", skip(ctx, path))]
pub(super) fn lint_toml<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting TOML file: {}", path_str);

    // Read the file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read TOML file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/toml")),
            ));
        }
    };

    // Parse the TOML file
    let parse = parser::parse(&content);

    let mut has_errors = false;

    // Check for parse errors
    if !parse.errors.is_empty() {
        has_errors = true;
        for error in &parse.errors {
            let msg = format!("TOML syntax error: {}", error);
            ctx.push_message(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    msg.clone(),
                ))
                .with_file_path(path_str.clone()),
            ));
        }
    }

    // Validate the DOM
    let dom = parse.into_dom();
    if let Err(errors) = dom.validate() {
        has_errors = true;
        for error in errors {
            let msg = format!("TOML semantic error: {}", error);
            ctx.push_message(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    msg.clone(),
                ))
                .with_file_path(path_str.clone()),
            ));
        }
    }

    if has_errors {
        Err(Message::Failure)
    } else {
        info!("TOML file {} is valid", path_str);
        Ok(FileStatus::Unchanged)
    }
}

/// Check (lint and format) a TOML file using Taplo
#[instrument(name = "cli_check_toml", level = "debug", skip(ctx, path))]
pub(super) fn check_toml<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking TOML file: {}", path_str);

    // First lint the file
    let lint_result = lint_toml(ctx, path.clone())?;

    // If linting found errors and we're not in fix mode, return
    if matches!(lint_result, FileStatus::Message(_)) && !ctx.execution.should_write() {
        return Ok(lint_result);
    }

    // Then format the file
    format_toml(ctx, path)
}
