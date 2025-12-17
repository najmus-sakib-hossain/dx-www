//! Unified single-pass transformation pipeline
//!
//! JSX + TypeScript + ES6 + Minify in ONE pass - 4x faster than multi-pass

pub mod unified;
pub mod jsx;
pub mod typescript;
pub mod es6;

pub use unified::UnifiedPipeline;

use dx_bundle_core::{BundleError, BundleResult, ImportMap, ModuleId};

/// Transform source in a single pass
pub fn transform(
    source: &[u8],
    module_id: ModuleId,
    imports: &ImportMap,
    options: &TransformOptions,
) -> BundleResult<Vec<u8>> {
    dx_bundle_core::with_arena(|arena| {
        let mut output = dx_bundle_core::ArenaOutput::new(arena);
        UnifiedPipeline::transform(source, &mut output, imports, module_id, options)?;
        Ok(output.to_vec())
    })
}

/// Transform options
#[derive(Clone, Debug)]
pub struct TransformOptions {
    /// Strip TypeScript types
    pub strip_typescript: bool,
    /// Transform JSX to createElement calls
    pub transform_jsx: bool,
    /// JSX factory function
    pub jsx_factory: String,
    /// JSX fragment
    pub jsx_fragment: String,
    /// Transform ES6 to CommonJS
    pub transform_es6: bool,
    /// Minify output
    pub minify: bool,
    /// Preserve comments
    pub preserve_comments: bool,
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self {
            strip_typescript: true,
            transform_jsx: true,
            jsx_factory: "React.createElement".into(),
            jsx_fragment: "React.Fragment".into(),
            transform_es6: true,
            minify: false,
            preserve_comments: false,
        }
    }
}
