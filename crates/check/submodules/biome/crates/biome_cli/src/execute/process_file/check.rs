use crate::execute::process_file::format::format_with_guard;
use crate::execute::process_file::lint_and_assist::analyze_with_guard;
use crate::execute::process_file::workspace_file::WorkspaceFile;
use crate::execute::process_file::{FileResult, FileStatus, Message, SharedTraversalOptions};
use biome_analyze::RuleCategoriesBuilder;
use biome_diagnostics::DiagnosticExt;
use biome_fs::{BiomePath, TraversalContext};
use biome_service::diagnostics::FileTooLarge;
use biome_service::workspace::FeaturesSupported;

pub(crate) fn check_file<'ctx>(
    ctx: &'ctx SharedTraversalOptions<'ctx, '_>,
    path: BiomePath,
    file_features: &FeaturesSupported,
) -> FileResult {
    // Check if this is a TOML file
    if path.extension().map_or(false, |ext| ext == "toml") {
        return super::toml::check_toml(ctx, path);
    }

    // Check if this is a Markdown file
    if path.extension().map_or(false, |ext| ext == "md" || ext == "markdown") {
        return super::markdown::check_markdown(ctx, path);
    }

    // Check if this is a Python file
    if path.extension().map_or(false, |ext| ext == "py" || ext == "pyi") {
        return super::python::check_python(ctx, path);
    }

    // Check if this is a PHP file
    if path.extension().map_or(false, |ext| ext == "php") {
        return super::php::check_php(ctx, path);
    }

    // Check if this is a C/C++ file
    if path.extension().map_or(false, |ext| {
        ext == "c" || ext == "cpp" || ext == "cc" || ext == "cxx" || ext == "h" || ext == "hpp" || ext == "hxx"
    }) {
        return super::cpp::check_cpp(ctx, path);
    }

    // Check if this is a Kotlin file
    if path.extension().map_or(false, |ext| ext == "kt" || ext == "kts") {
        return super::kotlin::check_kotlin(ctx, path);
    }

    // Check if this is a Go file
    if path.extension().map_or(false, |ext| ext == "go") {
        return super::go::check_go(ctx, path);
    }

    // Check if this is a Rust file
    if path.extension().map_or(false, |ext| ext == "rs") {
        return super::rust::check_rust(ctx, path);
    }

    let mut has_failures = false;
    let mut workspace_file = WorkspaceFile::new(ctx, path)?;
    let result = workspace_file.guard().check_file_size()?;
    if result.is_too_large() {
        ctx.push_diagnostic(
            FileTooLarge::from(result)
                .with_file_path(workspace_file.path.to_string())
                .with_category(ctx.execution.as_diagnostic_category()),
        );
        return Ok(FileStatus::Ignored);
    }
    let _ = tracing::info_span!("Check ", path =? workspace_file.path).entered();

    let mut categories = RuleCategoriesBuilder::default().with_syntax();
    if file_features.supports_lint() {
        categories = categories.with_lint();
    }
    if file_features.supports_assist() {
        categories = categories.with_assist();
    }

    let analyzer_result = analyze_with_guard(
        ctx,
        &mut workspace_file,
        false,
        None,
        categories.build(),
        file_features,
    );

    let mut changed = false;
    // To reduce duplication of the same error on format and lint_and_assist
    let mut skipped_parse_error = false;

    match analyzer_result {
        Ok(status) => {
            if matches!(status, FileStatus::Ignored) && ctx.execution.should_skip_parse_errors() {
                skipped_parse_error = true;
            }

            if status.is_changed() {
                changed = true
            }
            if let FileStatus::Message(msg) = status {
                if msg.is_failure() {
                    has_failures = true;
                }
                ctx.push_message(msg);
            }
        }
        Err(err) => {
            ctx.push_message(err);
            has_failures = true;
        }
    }

    if file_features.supports_format() {
        if ctx.execution.should_skip_parse_errors() && skipped_parse_error {
            // Parse errors are already skipped during the analyze phase, so no need to do it here.
        } else {
            let format_result = format_with_guard(ctx, &mut workspace_file, file_features);
            match format_result {
                Ok(status) => {
                    if status.is_changed() {
                        changed = true
                    }
                    if let FileStatus::Message(msg) = status {
                        if msg.is_failure() {
                            has_failures = true;
                        }
                        ctx.push_message(msg);
                    }
                }
                Err(err) => {
                    ctx.push_message(err);
                    has_failures = true;
                }
            }
        }
    }

    if has_failures {
        Ok(FileStatus::Message(Message::Failure))
    } else if changed {
        Ok(FileStatus::Changed)
    } else {
        Ok(FileStatus::Unchanged)
    }
}
