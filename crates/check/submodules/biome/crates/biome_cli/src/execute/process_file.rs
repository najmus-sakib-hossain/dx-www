mod check;
mod cpp;
mod format;
mod go;
mod kotlin;
mod php;
mod lint_and_assist;
mod markdown;
mod python;
mod rust;
mod search;
mod toml;
pub(crate) mod workspace_file;

use crate::execute::TraversalMode;
use crate::execute::diagnostics::{ResultExt, UnhandledDiagnostic};
use crate::execute::traverse::TraversalOptions;
use biome_analyze::RuleCategoriesBuilder;
use biome_diagnostics::{DiagnosticExt, DiagnosticTags, Error, category};
use biome_fs::BiomePath;
use biome_service::workspace::{
    DocumentFileSource, FeatureKind, FileFeaturesResult, FeaturesSupported, SupportKind, SupportsFeatureParams,
};
use check::check_file;
use format::format;
use lint_and_assist::lint_and_assist;
use search::search;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug)]
pub(crate) enum FileStatus {
    /// File changed and it was a success
    Changed,
    /// File unchanged, and it was a success
    Unchanged,
    /// While handling the file, something happened
    Message(Message),
    /// A match was found while searching a file
    SearchResult(usize, Message),
    /// File ignored, it should not be count as "handled"
    Ignored,
    /// Files that belong to other tools and shouldn't be touched
    Protected(String),
}

impl FileStatus {
    pub const fn is_changed(&self) -> bool {
        matches!(self, Self::Changed)
    }
}

/// Wrapper type for messages that can be printed during the traversal process
#[derive(Debug)]
pub(crate) enum Message {
    SkippedFixes {
        /// Suggested fixes skipped during the lint traversal
        skipped_suggested_fixes: u32,
    },
    Failure,
    Error(Error),
    Diagnostics {
        file_path: String,
        content: String,
        diagnostics: Vec<Error>,
        skipped_diagnostics: u32,
    },
    Diff {
        file_name: String,
        old: String,
        new: String,
        diff_kind: DiffKind,
    },
}

impl Message {
    pub(crate) const fn is_failure(&self) -> bool {
        matches!(self, Self::Failure)
    }
}

#[derive(Debug)]
pub(crate) enum DiffKind {
    Format,
}

impl<D> From<D> for Message
where
    Error: From<D>,
    D: std::fmt::Debug,
{
    fn from(err: D) -> Self {
        Self::Error(Error::from(err))
    }
}

/// The return type for [process_file], with the following semantics:
/// - `Ok(Success)` means the operation was successful (the file is added to
///   the `processed` counter)
/// - `Ok(Message(_))` means the operation was successful but a message still
///   needs to be printed (eg. the diff when not in CI or write mode)
/// - `Ok(Ignored)` means the file was ignored (the file is not added to the
///   `processed` or `skipped` counters)
/// - `Err(_)` means the operation failed and the file should be added to the
///   `skipped` counter
pub(crate) type FileResult = Result<FileStatus, Message>;

/// Data structure that allows to pass [TraversalOptions] to multiple consumers, bypassing the
/// compiler constraints set by the lifetimes of the [TraversalOptions]
pub(crate) struct SharedTraversalOptions<'ctx, 'app> {
    inner: &'app TraversalOptions<'ctx, 'app>,
    _p: PhantomData<&'app ()>,
}

impl<'ctx, 'app> SharedTraversalOptions<'ctx, 'app> {
    fn new(t: &'app TraversalOptions<'ctx, 'app>) -> Self {
        Self {
            _p: PhantomData,
            inner: t,
        }
    }
}

impl<'ctx, 'app> Deref for SharedTraversalOptions<'ctx, 'app> {
    type Target = TraversalOptions<'ctx, 'app>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/// Create a FeaturesSupported for TOML files that supports all features
fn toml_features_supported() -> FeaturesSupported {
    // Create a FeaturesSupported where Format, Lint, and Assist are Supported
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::Supported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    // SAFETY: FeaturesSupported is a newtype around an array, we're constructing it directly
    unsafe { std::mem::transmute(features) }
}

/// Create a FeaturesSupported for Markdown files that supports all features
fn markdown_features_supported() -> FeaturesSupported {
    // Create a FeaturesSupported where Format, Lint, and Assist are Supported
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::Supported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    // SAFETY: FeaturesSupported is a newtype around an array, we're constructing it directly
    unsafe { std::mem::transmute(features) }
}

/// Create a FeaturesSupported for Python files that supports all features
fn python_features_supported() -> FeaturesSupported {
    // Create a FeaturesSupported where Format, Lint, and Assist are Supported
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::Supported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    // SAFETY: FeaturesSupported is a newtype around an array, we're constructing it directly
    unsafe { std::mem::transmute(features) }
}

/// Create a FeaturesSupported for PHP files that supports all features
fn php_features_supported() -> FeaturesSupported {
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::Supported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    unsafe { std::mem::transmute(features) }
}

/// Create a FeaturesSupported for C/C++ files that supports all features
fn cpp_features_supported() -> FeaturesSupported {
    // Create a FeaturesSupported where Format, Lint, and Assist are Supported
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::Supported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    // SAFETY: FeaturesSupported is a newtype around an array, we're constructing it directly
    unsafe { std::mem::transmute(features) }
}

/// Returns which features Kotlin files support
fn kotlin_features_supported() -> FeaturesSupported {
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::FileNotSupported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    // SAFETY: FeaturesSupported is a newtype around an array, we're constructing it directly
    unsafe { std::mem::transmute(features) }
}

/// Returns which features Go files support
fn go_features_supported() -> FeaturesSupported {
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::FileNotSupported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    // SAFETY: FeaturesSupported is a newtype around an array, we're constructing it directly
    unsafe { std::mem::transmute(features) }
}

/// Returns which features Rust files support
fn rust_features_supported() -> FeaturesSupported {
    // The indices correspond to: [Format, Lint, Assist, Search, Debug, HtmlFullSupport]
    let features = [
        SupportKind::Supported,  // Format
        SupportKind::Supported,  // Lint
        SupportKind::FileNotSupported,  // Assist
        SupportKind::FileNotSupported,  // Search
        SupportKind::FileNotSupported,  // Debug
        SupportKind::FileNotSupported,  // HtmlFullSupport
    ];
    // SAFETY: FeaturesSupported is a newtype around an array, we're constructing it directly
    unsafe { std::mem::transmute(features) }
}

/// This function performs the actual processing: it reads the file from disk
/// and parse it; analyze and / or format it; then it either fails if error
/// diagnostics were emitted, or compare the formatted code with the original
/// content of the file and emit a diff or write the new content to the disk if
/// write mode is enabled
pub(crate) fn process_file(ctx: &TraversalOptions, biome_path: &BiomePath) -> FileResult {
    let _ = tracing::trace_span!("process_file", path = ?biome_path).entered();
    
    // Handle TOML files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| ext == "toml") {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let toml_features = toml_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &toml_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &toml_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &toml_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    // Handle Markdown files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| ext == "md" || ext == "markdown") {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let markdown_features = markdown_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &markdown_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &markdown_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &markdown_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    // Handle Python files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| ext == "py" || ext == "pyi") {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let python_features = python_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &python_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &python_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &python_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    // Handle PHP files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| ext == "php") {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let php_features = php_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &php_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &php_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &php_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    // Handle C/C++ files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| {
        ext == "c" || ext == "cpp" || ext == "cc" || ext == "cxx" || ext == "h" || ext == "hpp" || ext == "hxx"
    }) {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let cpp_features = cpp_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &cpp_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &cpp_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &cpp_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    // Handle Kotlin files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| {
        ext == "kt" || ext == "kts"
    }) {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let kotlin_features = kotlin_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &kotlin_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &kotlin_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &kotlin_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    // Handle Go files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| ext == "go") {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let go_features = go_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &go_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &go_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &go_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    // Handle Rust files directly at the CLI level (bypassing service layer)
    if biome_path.extension().map_or(false, |ext| ext == "rs") {
        let shared_context = &SharedTraversalOptions::new(ctx);
        let rust_features = rust_features_supported();
        return match ctx.execution.traversal_mode {
            TraversalMode::Format { .. } => {
                format::format(shared_context, biome_path.clone(), &rust_features)
            }
            TraversalMode::Lint { .. } => {
                lint_and_assist::lint_and_assist(
                    shared_context,
                    biome_path.clone(),
                    false,
                    None,
                    RuleCategoriesBuilder::default().with_lint().with_syntax().build(),
                    &rust_features,
                )
            }
            TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
                check::check_file(shared_context, biome_path.clone(), &rust_features)
            }
            _ => Ok(FileStatus::Ignored),
        };
    }

    tracing::trace_span!("process_file_workspace", path = ?biome_path).in_scope(|| {
        let FileFeaturesResult {
            features_supported: file_features,
        } = ctx
            .workspace
            .file_features(SupportsFeatureParams {
                project_key: ctx.project_key,
                path: biome_path.clone(),
                features: ctx.execution.to_feature(),
            })
            .with_file_path_and_code_and_tags(
                biome_path.to_string(),
                category!("files/missingHandler"),
                DiagnosticTags::VERBOSE,
            )?;

        // first we stop if there are some files that don't have ALL features enabled, e.g. images, fonts, etc.
        if file_features.is_ignored() || file_features.is_not_enabled() {
            return Ok(FileStatus::Ignored);
        } else if file_features.is_not_supported() || !DocumentFileSource::can_read(biome_path) {
            return Err(Message::from(
                UnhandledDiagnostic.with_file_path(biome_path.to_string()),
            ));
        }

        // then we pick the specific features for this file
        let unsupported_reason = match ctx.execution.traversal_mode() {
        TraversalMode::Check { .. } | TraversalMode::CI { .. } => file_features
            .support_kind_if_not_enabled(FeatureKind::Lint)
            .and(file_features.support_kind_if_not_enabled(FeatureKind::Format))
            .and(file_features.support_kind_if_not_enabled(FeatureKind::Assist)),
        TraversalMode::Format { .. } => Some(file_features.support_kind_for(FeatureKind::Format)),
        TraversalMode::Lint { .. } => Some(file_features.support_kind_for(FeatureKind::Lint)),
            TraversalMode::Migrate { .. } => None,
            TraversalMode::Search { .. } => Some(file_features.support_kind_for(FeatureKind::Search)),
        };

        if let Some(reason) = unsupported_reason {
            match reason {
                SupportKind::FileNotSupported => {
                    return Err(Message::from(
                        UnhandledDiagnostic.with_file_path(biome_path.to_string()),
                    ));
                }
                SupportKind::FeatureNotEnabled | SupportKind::Ignored => {
                    return Ok(FileStatus::Ignored);
                }
                SupportKind::Protected => {
                    return Ok(FileStatus::Protected(biome_path.to_string()));
                }
                SupportKind::Supported => {}
            };
        }

        let shared_context = &SharedTraversalOptions::new(ctx);

        match ctx.execution.traversal_mode {
        TraversalMode::Lint {
            ref suppression_reason,
            suppress,
            ..
        } => {
            let categories = RuleCategoriesBuilder::default().with_lint().with_syntax();
            // the unsupported case should be handled already at this point
            lint_and_assist(
                shared_context,
                biome_path.clone(),
                suppress,
                suppression_reason.as_deref(),
                categories.build(),
                &file_features,
            )
        }
        TraversalMode::Format { .. } => {
            // the unsupported case should be handled already at this point
            format(shared_context, biome_path.clone(), &file_features)
        }
        TraversalMode::Check { .. } | TraversalMode::CI { .. } => {
            check_file(shared_context, biome_path.clone(), &file_features)
        }
        TraversalMode::Migrate { .. } => {
            unreachable!("The migration should not be called for this file")
        }
            TraversalMode::Search { ref pattern, .. } => {
                // the unsupported case should be handled already at this point
                search(shared_context, biome_path.clone(), pattern)
            }
        }
    })
}
