//! Unified transformation pipeline - JSX + TypeScript + ES6 in one pass

pub mod unified;
pub mod jsx;
pub mod typescript;
pub mod es6;

use dx_bundle_core::{ImportMap, ModuleId};
use dx_bundle_core::error::BundleResult;

pub use unified::UnifiedPipeline;

/// Public transform API
pub fn transform(
    source: &[u8],
    _module_id: ModuleId,
    imports: &ImportMap,
    options: &TransformOptions,
) -> BundleResult<Vec<u8>> {
    // Convert to string for transformations
    let source_str = std::str::from_utf8(source)
        .map_err(|e| dx_bundle_core::error::BundleError::transform_error(format!("Invalid UTF-8: {}", e)))?;
    
    let mut result = source_str.to_string();
    
    // Phase 1: Strip TypeScript if enabled
    if options.strip_typescript {
        result = strip_typescript(&result);
    }
    
    // Phase 2: Transform JSX if enabled
    if options.transform_jsx {
        result = transform_jsx_code(&result, &options.jsx_factory);
    }
    
    // Phase 3: Rewrite imports based on ImportMap (disabled for now)
    // if !imports.is_empty() {
    //     result = rewrite_imports(&result, imports);
    // }
    
    // Phase 4: Minify if enabled
    if options.minify {
        result = minify_code(&result);
    }
    
    Ok(result.into_bytes())
}

/// Strip TypeScript type annotations
fn strip_typescript(source: &str) -> String {
    let mut result = source.to_string();
    
    // Remove interface declarations
    while let Some(start) = result.find("interface ") {
        if let Some(end) = find_block_end(&result[start..]) {
            result.replace_range(start..start + end, "");
        } else {
            break;
        }
    }
    
    // Remove type aliases
    while let Some(start) = result.find("type ") {
        if let Some(end) = result[start..].find([';', '\n']) {
            result.replace_range(start..start + end + 1, "");
        } else {
            break;
        }
    }
    
    // Remove type annotations from variables: const x: Type = ... → const x = ...
    result = remove_type_annotations(result);
    
    // Remove access modifiers
    for modifier in &["private ", "public ", "protected ", "readonly "] {
        result = result.replace(modifier, "");
    }
    
    result
}

/// Transform JSX to createElement calls
fn transform_jsx_code(source: &str, factory: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '<' && i + 1 < chars.len() {
            let next = chars[i + 1];
            
            // JSX opening tag
            if next.is_alphabetic() || next.is_uppercase() {
                // Simple replacement for now - convert <Component /> to React.createElement
                let tag_start = i + 1;
                let mut tag_end = tag_start;
                while tag_end < chars.len() && (chars[tag_end].is_alphanumeric() || chars[tag_end] == '_') {
                    tag_end += 1;
                }
                let tag_name: String = chars[tag_start..tag_end].iter().collect();
                
                // Skip to end of tag
                while i < chars.len() && chars[i] != '>' {
                    i += 1;
                }
                i += 1;
                
                // For now, simple transformation
                result.push_str(factory);
                result.push('(');
                result.push_str(&format!("'{}'", tag_name));
                result.push_str(", null)");
                continue;
            }
        }
        
        result.push(chars[i]);
        i += 1;
    }
    
    result
}

/// Rewrite imports based on ImportMap
fn rewrite_imports(source: &str, _imports: &ImportMap) -> String {
    // TODO: Implement import rewriting based on resolution map
    source.to_string()
}

/// Simple minification
fn minify_code(source: &str) -> String {
    // Remove extra whitespace (simple version)
    source
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Find the end of a block (matching {})
fn find_block_end(source: &str) -> Option<usize> {
    let mut depth = 0;
    let mut in_block = false;
    
    for (i, ch) in source.char_indices() {
        match ch {
            '{' => {
                in_block = true;
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 && in_block {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// Remove type annotations from variable declarations
fn remove_type_annotations(mut source: String) -> String {
    let patterns = ["const ", "let ", "var ", "function "];
    
    for pattern in &patterns {
        let mut iteration = 0;
        while iteration < 50 {
            iteration += 1;
            let before_len = source.len();
            
            if let Some(start) = source.find(pattern) {
                let after_keyword = &source[start + pattern.len()..];
                
                // Find identifier
                let mut ident_end = 0;
                for (idx, ch) in after_keyword.char_indices() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident_end = idx + ch.len_utf8();
                    } else {
                        break;
                    }
                }
                
                // Check for type annotation
                let after_ident = &after_keyword[ident_end..];
                if after_ident.starts_with(": ") {
                    // Find delimiter (= or ; or newline)
                    let colon_pos = start + pattern.len() + ident_end;
                    let after_colon = &source[colon_pos + 2..];
                    
                    if let Some(delim_pos) = after_colon.find(['=', ';', '\n']) {
                        // Remove the type annotation
                        source.replace_range(colon_pos..colon_pos + 2 + delim_pos, "");
                        continue;
                    }
                }
            }
            
            if source.len() == before_len {
                break;
            }
        }
    }
    
    source
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
