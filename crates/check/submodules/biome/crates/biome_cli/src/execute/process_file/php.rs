use super::{DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::BiomePath;
use bumpalo::Bump;
use std::borrow::Cow;
use std::io;
use tracing::{debug, error, info, instrument};

/// Format a PHP file using Mago's formatter
#[instrument(name = "cli_format_php", level = "debug", skip(ctx, path))]
pub(super) fn format_php<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting PHP file: {}", path_str);

    // Read file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read PHP file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/php")),
            ));
        }
    };

    let original_content = content.clone();

    // Format using mago-formatter
    let arena = Bump::new();
    let settings = mago_formatter::settings::FormatSettings::default();
    let php_version = mago_php_version::PHPVersion::default();
    let formatter = mago_formatter::Formatter::new(&arena, php_version, settings);

    let formatted = match formatter.format_code(Cow::Owned(path_str.clone()), Cow::Owned(content.clone())) {
        Ok(s) => s.to_string(),
        Err(e) => {
            error!("Failed to format PHP file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(std::io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("PHP format error: {}", e),
                ))
                .with_file_path(path_str)
                .with_category(category!("format/php")),
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
        error!("Failed to write formatted PHP file {}: {}", path_str, e);
        return Err(Message::from(
            biome_diagnostics::IoError::from(e)
                .with_file_path(path_str)
                .with_category(category!("format/php")),
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

/// Lint a PHP file using Mago's linter + semantics
#[instrument(name = "cli_lint_php", level = "debug", skip(ctx, path))]
pub(super) fn lint_php<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting PHP file: {}", path_str);

    // Read file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read PHP file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/php")),
            ));
        }
    };

    // Create ephemeral mago file and parse
    let arena = Bump::new();
    let mago_file = mago_database::file::File::ephemeral(Cow::Owned(path_str.clone()), Cow::Owned(content.clone()));

    let (program, parse_error) = mago_syntax::parser::parse_file(&arena, &mago_file);
    let mut has_errors = false;

    if let Some(err) = parse_error {
        has_errors = true;
        let msg = format!("PHP parse error: {}", err);
        ctx.push_message(Message::from(
            biome_diagnostics::IoError::from(std::io::Error::new(io::ErrorKind::InvalidData, msg)).with_file_path(path_str.clone()).with_category(category!("lint/php")),
        ));
    }

    // Resolve names, run semantics and linter
    let resolved_names = mago_names::resolver::NameResolver::new(&arena).resolve(program);
    let semantics_checker = mago_semantics::SemanticsChecker::new(mago_php_version::PHPVersion::default());
    let mut issues = semantics_checker.check(&mago_file, program, &resolved_names);

    let linter_settings = mago_linter::settings::Settings::default();
    let linter = mago_linter::Linter::new(&arena, linter_settings, None, false);
    let linter_issues = linter.lint(&mago_file, program, &resolved_names);

    issues.extend(linter_issues);

    for issue in issues.iter() {
        let mut msg = format!("{}: {}", issue.level, issue.message);
        if let Some(code) = &issue.code {
            msg.push_str(&format!(" ({})", code));
        }
        ctx.push_message(Message::from(biome_diagnostics::IoError::from(std::io::Error::new(io::ErrorKind::InvalidData, msg)).with_file_path(path_str.clone()).with_category(category!("lint/php"))));
    }

    if has_errors || !issues.is_empty() {
        Err(Message::Failure)
    } else {
        info!("PHP file {} is valid", path_str);
        Ok(FileStatus::Unchanged)
    }
}

/// Check (lint and format) a PHP file
#[instrument(name = "cli_check_php", level = "debug", skip(ctx, path))]
pub(super) fn check_php<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking PHP file: {}", path_str);

    // First lint
    let lint_result = lint_php(ctx, path.clone())?;

    // If linting found errors and we're not in fix mode, return
    if matches!(lint_result, FileStatus::Message(_)) && !ctx.execution.should_write() {
        return Ok(lint_result);
    }

    // Then format
    format_php(ctx, path)
}
