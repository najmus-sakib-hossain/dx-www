//! Fix Engine
//!
//! Predictive fix engine with pre-compiled fix templates.
//! Applies fixes in microseconds via pattern matching.

use crate::diagnostics::{Diagnostic, Edit, Fix};
use std::collections::HashMap;

/// Pre-compiled fix template
#[derive(Clone)]
pub struct FixTemplate {
    /// Rule ID this fix applies to
    pub rule_id: String,
    /// Pattern to match
    pub pattern: FixPattern,
    /// Replacement template
    pub replacement: ReplacementTemplate,
}

/// Pattern for matching code to fix
#[derive(Clone)]
pub enum FixPattern {
    /// Exact string match
    Exact(Vec<u8>),
    /// Simple pattern with wildcards
    Wildcard(String),
}

/// Template for replacement text
#[derive(Clone)]
pub struct ReplacementTemplate {
    /// Segments of the replacement
    pub segments: Vec<Segment>,
}

#[derive(Clone)]
pub enum Segment {
    /// Literal text
    Literal(Vec<u8>),
    /// Captured group reference (1-indexed)
    Capture(u8),
}

impl ReplacementTemplate {
    /// Create a simple literal replacement
    pub fn literal(text: &str) -> Self {
        Self {
            segments: vec![Segment::Literal(text.as_bytes().to_vec())],
        }
    }

    /// Apply template with captures
    pub fn apply(&self, captures: &[&[u8]]) -> Vec<u8> {
        let mut result = Vec::new();
        for segment in &self.segments {
            match segment {
                Segment::Literal(lit) => result.extend(lit),
                Segment::Capture(n) => {
                    if let Some(&capture) = captures.get(*n as usize) {
                        result.extend(capture);
                    }
                }
            }
        }
        result
    }
}

/// Fix engine with pre-compiled templates
pub struct FixEngine {
    /// Fix templates by rule ID
    templates: HashMap<String, Vec<FixTemplate>>,
}

impl FixEngine {
    /// Create a new fix engine with built-in templates
    pub fn new() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
        };
        engine.register_builtin_fixes();
        engine
    }

    /// Register built-in fix templates
    fn register_builtin_fixes(&mut self) {
        // eqeqeq: == to ===
        self.register(FixTemplate {
            rule_id: "eqeqeq".to_string(),
            pattern: FixPattern::Exact(b"==".to_vec()),
            replacement: ReplacementTemplate::literal("==="),
        });

        // eqeqeq: != to !==
        self.register(FixTemplate {
            rule_id: "eqeqeq".to_string(),
            pattern: FixPattern::Exact(b"!=".to_vec()),
            replacement: ReplacementTemplate::literal("!=="),
        });

        // no-var: var to let
        self.register(FixTemplate {
            rule_id: "no-var".to_string(),
            pattern: FixPattern::Exact(b"var ".to_vec()),
            replacement: ReplacementTemplate::literal("let "),
        });
    }

    /// Register a fix template
    pub fn register(&mut self, template: FixTemplate) {
        self.templates
            .entry(template.rule_id.clone())
            .or_default()
            .push(template);
    }

    /// Apply a single fix to source
    pub fn apply_fix(&self, source: &[u8], fix: &Fix) -> Vec<u8> {
        // Sort edits by position (reverse order for safe application)
        let mut edits = fix.edits.clone();
        edits.sort_by_key(|e| std::cmp::Reverse(e.span.start));

        let mut current_source = source.to_vec();
        for edit in &edits {
            let start = edit.span.start as usize;
            let end = edit.span.end as usize;

            let mut new_source = Vec::with_capacity(current_source.len());
            new_source.extend(&current_source[..start]);
            new_source.extend(edit.new_text.as_bytes());
            new_source.extend(&current_source[end..]);

            current_source = new_source;
        }

        current_source
    }

    /// Apply all fixes from diagnostics
    pub fn apply_all_fixes(&self, source: &[u8], diagnostics: &[Diagnostic]) -> Vec<u8> {
        // Collect all fixes
        let mut all_edits: Vec<Edit> = diagnostics
            .iter()
            .filter_map(|d| d.fix.as_ref())
            .flat_map(|f| f.edits.iter().cloned())
            .collect();

        // Sort by position (reverse order)
        all_edits.sort_by_key(|e| std::cmp::Reverse(e.span.start));

        // Check for overlapping edits and remove them
        let mut filtered_edits: Vec<Edit> = Vec::new();
        let mut last_start = usize::MAX;

        for edit in all_edits {
            let end = edit.span.end as usize;
            if end <= last_start {
                last_start = edit.span.start as usize;
                filtered_edits.push(edit);
            }
            // Skip overlapping edits
        }

        // Apply edits
        let mut result = source.to_vec();
        for edit in filtered_edits {
            let start = edit.span.start as usize;
            let end = edit.span.end as usize;

            let mut new_result = Vec::with_capacity(result.len());
            new_result.extend(&result[..start]);
            new_result.extend(edit.new_text.as_bytes());
            new_result.extend(&result[end..]);

            result = new_result;
        }

        result
    }

    /// Get fix template for a rule
    pub fn get_templates(&self, rule_id: &str) -> Option<&Vec<FixTemplate>> {
        self.templates.get(rule_id)
    }
}

impl Default for FixEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// XOR differential patch for efficient fix transmission
#[derive(Debug, Clone)]
pub struct XorPatch {
    /// Hash of original content
    pub base_hash: [u8; 32],
    /// XOR chunks
    pub chunks: Vec<XorChunk>,
}

#[derive(Debug, Clone)]
pub struct XorChunk {
    /// Offset in source
    pub offset: u32,
    /// XOR data
    pub xor_data: Vec<u8>,
}

impl XorPatch {
    /// Compute XOR patch between original and fixed content
    pub fn compute(original: &[u8], fixed: &[u8]) -> Self {
        let mut chunks = Vec::new();
        let max_len = original.len().max(fixed.len());

        let mut i = 0;
        while i < max_len {
            let orig = original.get(i).copied().unwrap_or(0);
            let fix = fixed.get(i).copied().unwrap_or(0);

            if orig != fix {
                // Start a new chunk
                let offset = i as u32;
                let mut xor_data = Vec::new();

                while i < max_len {
                    let orig = original.get(i).copied().unwrap_or(0);
                    let fix = fixed.get(i).copied().unwrap_or(0);

                    if orig == fix {
                        break;
                    }

                    xor_data.push(orig ^ fix);
                    i += 1;
                }

                chunks.push(XorChunk { offset, xor_data });
            } else {
                i += 1;
            }
        }

        Self {
            base_hash: *blake3::hash(original).as_bytes(),
            chunks,
        }
    }

    /// Apply XOR patch to original content
    pub fn apply(&self, original: &[u8]) -> Vec<u8> {
        let mut result = original.to_vec();

        for chunk in &self.chunks {
            for (i, &xor_byte) in chunk.xor_data.iter().enumerate() {
                let pos = chunk.offset as usize + i;
                if pos < result.len() {
                    result[pos] ^= xor_byte;
                } else {
                    // Handle length changes
                    while result.len() < pos {
                        result.push(0);
                    }
                    result.push(xor_byte);
                }
            }
        }

        result
    }

    /// Get total patch size in bytes
    pub fn size(&self) -> usize {
        32 + // base_hash
        self.chunks.iter().map(|c| 4 + c.xor_data.len()).sum::<usize>()
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.size());
        bytes.extend(&self.base_hash);
        for chunk in &self.chunks {
            bytes.extend(&chunk.offset.to_le_bytes());
            bytes.extend(&(chunk.xor_data.len() as u16).to_le_bytes());
            bytes.extend(&chunk.xor_data);
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::Span;

    #[test]
    fn test_replacement_template() {
        let template = ReplacementTemplate {
            segments: vec![
                Segment::Literal(b"const ".to_vec()),
                Segment::Capture(0),
                Segment::Literal(b" = ".to_vec()),
                Segment::Capture(1),
            ],
        };

        let result = template.apply(&[b"x", b"42"]);
        assert_eq!(result, b"const x = 42");
    }

    #[test]
    fn test_apply_fix() {
        let engine = FixEngine::new();
        let source = b"if (x == y) {}";
        
        let fix = Fix {
            description: "Use ===".to_string(),
            edits: vec![Edit {
                span: Span::new(6, 8),
                new_text: "===".to_string(),
            }],
        };

        let result = engine.apply_fix(source, &fix);
        assert_eq!(result, b"if (x === y) {}");
    }

    #[test]
    fn test_xor_patch() {
        let original = b"const x = foo();";
        let fixed = b"const x = bar();";

        let patch = XorPatch::compute(original, fixed);
        let applied = patch.apply(original);

        assert_eq!(applied, fixed);
        // XOR patch captures differences, size depends on diff spread
        assert!(patch.size() > 0);
    }
}
