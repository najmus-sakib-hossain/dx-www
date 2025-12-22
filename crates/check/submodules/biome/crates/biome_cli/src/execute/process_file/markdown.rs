use super::{DiffKind, FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_diagnostics::{category, DiagnosticExt};
use biome_fs::BiomePath;
use rumdl_lib::config::{Config as RumdlConfig, GlobalConfig, MarkdownFlavor};
use rumdl_lib::fix_coordinator::FixCoordinator;
use rumdl_lib::lint_context::LintContext;
use rumdl_lib::rule::Rule;
use rumdl_lib::rules;
use std::collections::BTreeMap;
use tracing::{debug, error, info, instrument};

/// Format a Markdown file using rumdl
#[instrument(name = "cli_format_markdown", level = "debug", skip(ctx, path))]
pub(super) fn format_markdown<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Formatting Markdown file: {}", path_str);

    // Read the file content
    let mut content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Markdown file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("format/markdown")),
            ));
        }
    };

    let original_content = content.clone();
    
    // Detect original line ending
    let original_line_ending = rumdl_lib::utils::detect_line_ending_enum(&content);
    
    // Normalize to LF for processing
    content = rumdl_lib::utils::normalize_line_ending(&content, rumdl_lib::utils::LineEnding::Lf);

    // Create a default rumdl config
    let config = RumdlConfig {
        global: GlobalConfig::default(),
        per_file_ignores: std::collections::HashMap::new(),
        rules: BTreeMap::new(),
    };

    // Get all rules
    let all_rules: Vec<Box<dyn Rule>> = rules::all_rules(&config);
    
    // Create lint context to get warnings
    let mut lint_ctx = LintContext::new(&content, MarkdownFlavor::Standard);

    // Collect all warnings (flatten the nested vectors and handle Results)
    let mut all_warnings = Vec::new();
    for rule in &all_rules {
        match rule.check(&mut lint_ctx) {
            Ok(warnings) => {
                all_warnings.extend(warnings);
            }
            Err(e) => {
                error!("Error checking rule {}: {}", rule.name(), e);
            }
        }
    }
    
    // Apply fixes using FixCoordinator
    let coordinator = FixCoordinator::new();
    let result = coordinator.apply_fixes_iterative(
        &all_rules,
        &all_warnings,
        &mut content,
        &config,
        100,  // max iterations
    );
    
    match result {
        Ok((fixes_applied, _, _, _, _)) => {
            debug!("Applied {} fixes to {}", fixes_applied, path_str);
        }
        Err(e) => {
            error!("Error applying fixes to {}: {}", path_str, e);
        }
    }

    // Restore original line ending
    let formatted = rumdl_lib::utils::normalize_line_ending(&content, original_line_ending);

    // Check if content changed
    if original_content == formatted {
        return Ok(FileStatus::Unchanged);
    }

    // If we're in check mode, report that the file needs formatting
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

    // Write the formatted content back
    if let Err(e) = std::fs::write(path.as_path(), &formatted) {
        error!("Failed to write formatted Markdown file {}: {}", path_str, e);
        return Err(Message::from(
            biome_diagnostics::IoError::from(e)
                .with_file_path(path_str)
                .with_category(category!("format/markdown")),
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

/// Lint a Markdown file using rumdl
#[instrument(name = "cli_lint_markdown", level = "debug", skip(ctx, path))]
pub(super) fn lint_markdown<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Linting Markdown file: {}", path_str);

    // Read the file content
    let content = match std::fs::read_to_string(path.as_path()) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read Markdown file {}: {}", path_str, e);
            return Err(Message::from(
                biome_diagnostics::IoError::from(e)
                    .with_file_path(path_str)
                    .with_category(category!("lint/markdown")),
            ));
        }
    };

    // Normalize to LF for processing
    let normalized_content = rumdl_lib::utils::normalize_line_ending(&content, rumdl_lib::utils::LineEnding::Lf);

    // Create a default rumdl config
    let config = RumdlConfig {
        global: GlobalConfig::default(),
        per_file_ignores: std::collections::HashMap::new(),
        rules: BTreeMap::new(),
    };

    // Get all rules
    let all_rules: Vec<Box<dyn Rule>> = rules::all_rules(&config);

    // Create lint context and check
    let mut lint_ctx = LintContext::new(&normalized_content, MarkdownFlavor::Standard);
    
    let mut has_errors = false;
    for rule in &all_rules {
        match rule.check(&mut lint_ctx) {
            Ok(warnings) => {
                for warning in warnings {
                    has_errors = true;
                    let msg = format!(
                        "Markdown lint error [{}]: {} at line {}",
                        warning.rule_name.as_deref().unwrap_or("unknown"),
                        warning.message,
                        warning.line
                    );
                    ctx.push_message(Message::from(
                        biome_diagnostics::IoError::from(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            msg.clone(),
                        ))
                        .with_file_path(path_str.clone()),
                    ));
                }
            }
            Err(e) => {
                error!("Error linting Markdown file {}: {}", path_str, e);
                return Err(Message::Failure);
            }
        }
    }

    if has_errors {
        Err(Message::Failure)
    } else {
        info!("Markdown file {} is valid", path_str);
        Ok(FileStatus::Unchanged)
    }
}

/// Check (lint and format) a Markdown file using rumdl
#[instrument(name = "cli_check_markdown", level = "debug", skip(ctx, path))]
pub(super) fn check_markdown<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
) -> FileResult {
    let path_str = path.to_string();
    debug!("Checking Markdown file: {}", path_str);

    // First lint the file
    let lint_result = lint_markdown(ctx, path.clone())?;

    // If linting found errors and we're not in fix mode, return
    if matches!(lint_result, FileStatus::Message(_)) && !ctx.execution.should_write() {
        return Ok(lint_result);
    }

    // Then format the file
    format_markdown(ctx, path)
}
