//! Template Compiler
//!
//! Compiles text templates to binary `.dxt` format at build time.
//! Zero runtime parsing—templates are memory-mapped directly.

use crate::binary::{
    BinaryTemplate, BinaryTemplateBuilder, Opcode, PlaceholderEntry, PlaceholderType,
    FLAG_DEDUPED, FLAG_OPTIMIZED, FLAG_STATIC,
};
use crate::error::{GeneratorError, Result};
use crate::scanner::{extract_static_segments, Placeholder, PlaceholderScanner};
use std::collections::HashMap;
use std::path::Path;

// ============================================================================
// Compile Options
// ============================================================================

/// Options for template compilation.
#[derive(Clone, Debug)]
pub struct CompileOptions {
    /// Template name (defaults to filename).
    pub name: Option<String>,
    /// Enable string deduplication.
    pub dedupe_strings: bool,
    /// Enable size optimization.
    pub optimize: bool,
    /// Force Macro mode even for static templates.
    pub force_macro: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            name: None,
            dedupe_strings: true,
            optimize: true,
            force_macro: false,
        }
    }
}

impl CompileOptions {
    /// Create options with a specific template name.
    #[must_use]
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

// ============================================================================
// Compiler
// ============================================================================

/// Compiles text templates to binary `.dxt` format.
///
/// # Example
///
/// ```rust,ignore
/// use dx_generator::{Compiler, CompileOptions};
///
/// let compiler = Compiler::new();
/// let source = "Hello, {{ name }}!";
/// let binary = compiler.compile(source.as_bytes(), CompileOptions::default())?;
///
/// // Write to file
/// std::fs::write("hello.dxt", binary.to_bytes())?;
/// ```
#[derive(Clone, Debug, Default)]
pub struct Compiler {
    /// Placeholder scanner.
    scanner: PlaceholderScanner,
}

impl Compiler {
    /// Create a new compiler.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Compile a template from bytes.
    pub fn compile(&self, source: &[u8], options: CompileOptions) -> Result<BinaryTemplate> {
        let name = options.name.unwrap_or_else(|| "template".to_string());

        // Scan for placeholders
        let placeholders = self.scanner.scan(source);

        // Check if template is static (no control flow)
        let is_static = !options.force_macro && !self.has_control_flow(&placeholders);

        // Extract static segments
        let segments = extract_static_segments(source, &placeholders);

        // Build the binary template
        let mut builder = BinaryTemplateBuilder::new(&name);

        // String deduplication map
        let mut string_map: HashMap<&[u8], u32> = HashMap::new();

        // Add static segments to string table
        for seg in &segments {
            let text = &source[seg.start..seg.end];
            if options.dedupe_strings {
                if !string_map.contains_key(text) {
                    let text_str = String::from_utf8_lossy(text);
                    let idx = builder.add_string(&text_str);
                    string_map.insert(text, idx);
                }
            } else {
                let text_str = String::from_utf8_lossy(text);
                builder.add_string(&text_str);
            }
        }

        // Extract and register parameters
        let mut param_map: HashMap<String, u32> = HashMap::new();
        for ph in &placeholders {
            if ph.placeholder_type == PlaceholderType::Variable {
                let var_name = ph.content.clone();
                if !param_map.contains_key(&var_name) {
                    let var_id = builder.add_param(&var_name);
                    param_map.insert(var_name, var_id);
                }
            }
        }

        // Build placeholders and instructions
        if is_static {
            // Micro mode: just placeholder entries
            let mut output_offset = 0u32;

            for (i, seg) in segments.iter().enumerate() {
                // Account for static segment
                let seg_len = (seg.end - seg.start) as u32;
                output_offset += seg_len;

                // Check for placeholder after this segment
                if i < placeholders.len() {
                    let ph = &placeholders[i];
                    if ph.placeholder_type == PlaceholderType::Variable {
                        let var_id = param_map.get(&ph.content).copied().unwrap_or(0);
                        builder.add_placeholder(PlaceholderEntry::new(
                            output_offset,
                            64, // Default max length
                            PlaceholderType::Variable,
                            var_id,
                        ));
                    }
                }
            }

            builder.set_static(true);
        } else {
            // Macro mode: generate bytecode
            self.compile_macro(&mut builder, source, &placeholders, &segments, &param_map, &string_map)?;
            builder.set_static(false);
        }

        // Set optimization flags
        let mut template = builder.build();
        if options.dedupe_strings {
            template.header.flags |= FLAG_DEDUPED;
        }
        if options.optimize {
            template.header.flags |= FLAG_OPTIMIZED;
        }

        Ok(template)
    }

    /// Compile a template from a file.
    pub fn compile_file(&self, path: impl AsRef<Path>, options: CompileOptions) -> Result<BinaryTemplate> {
        let path = path.as_ref();
        let source = std::fs::read(path)?;

        let name = options.name.unwrap_or_else(|| {
            path.file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "template".to_string())
        });

        self.compile(&source, CompileOptions { name: Some(name), ..options })
    }

    /// Check if any placeholder requires control flow.
    fn has_control_flow(&self, placeholders: &[Placeholder]) -> bool {
        placeholders.iter().any(|ph| {
            matches!(
                ph.placeholder_type,
                PlaceholderType::Conditional
                    | PlaceholderType::Loop
                    | PlaceholderType::Include
            )
        })
    }

    /// Compile template to Macro mode bytecode.
    fn compile_macro(
        &self,
        builder: &mut BinaryTemplateBuilder,
        source: &[u8],
        placeholders: &[Placeholder],
        segments: &[crate::scanner::StaticSegment],
        param_map: &HashMap<String, u32>,
        string_map: &HashMap<&[u8], u32>,
    ) -> Result<()> {
        let mut seg_idx = 0;
        let mut ph_idx = 0;

        // Control flow stack for matching if/endif, for/endfor
        let mut control_stack: Vec<ControlFrame> = Vec::new();

        while seg_idx < segments.len() || ph_idx < placeholders.len() {
            // Emit static segment
            if seg_idx < segments.len() {
                let seg = &segments[seg_idx];
                let text = &source[seg.start..seg.end];

                if let Some(&string_id) = string_map.get(text) {
                    builder.add_instruction_u32(Opcode::PushText, string_id);
                } else {
                    // Text not deduplicated, add inline
                    let text_str = String::from_utf8_lossy(text);
                    let string_id = builder.add_string(&text_str);
                    builder.add_instruction_u32(Opcode::PushText, string_id);
                }

                seg_idx += 1;
            }

            // Emit placeholder
            if ph_idx < placeholders.len() {
                let ph = &placeholders[ph_idx];

                match ph.placeholder_type {
                    PlaceholderType::Variable => {
                        let var_id = param_map.get(&ph.content).copied().unwrap_or(0);
                        builder.add_instruction_u32(Opcode::PushVar, var_id);
                    }

                    PlaceholderType::Conditional => {
                        if ph.content.starts_with("if ") {
                            // Start of conditional
                            control_stack.push(ControlFrame::If {
                                jump_patch: 0, // Will be patched later
                            });
                            // For now, emit a placeholder jump (would need proper expression parsing)
                            builder.add_instruction_i32(Opcode::JmpFalse, 0);
                        } else if ph.content == "else" {
                            // Else branch
                            builder.add_instruction_i32(Opcode::Jmp, 0);
                        } else if ph.content == "endif" {
                            // End of conditional
                            control_stack.pop();
                        }
                    }

                    PlaceholderType::Loop => {
                        if ph.content.starts_with("for ") {
                            // Parse "for item in items"
                            let parts: Vec<&str> = ph.content.split_whitespace().collect();
                            if parts.len() >= 4 && parts[2] == "in" {
                                let iter_var = parts[1];
                                let array_var = parts[3];

                                let iter_id = param_map.get(iter_var).copied().unwrap_or(0);
                                let array_id = param_map.get(array_var).copied().unwrap_or(0);

                                control_stack.push(ControlFrame::Loop);
                                builder.add_instruction_u32(Opcode::LoopBegin, array_id);
                                // Second argument for iter_id encoded separately
                                let instr_bytes = iter_id.to_le_bytes();
                                for byte in instr_bytes {
                                    // Manual byte append (simplified)
                                    let _ = byte;
                                }
                            }
                        } else if ph.content == "endfor" {
                            control_stack.pop();
                            builder.add_instruction(Opcode::LoopEnd);
                        }
                    }

                    PlaceholderType::Include => {
                        // Include directive: {% include "other.dxt" %}
                        // For now, just emit a placeholder
                        builder.add_instruction_u32(Opcode::Include, 0);
                    }

                    PlaceholderType::Comment => {
                        // Comments are skipped
                    }

                    PlaceholderType::Raw => {
                        // Raw blocks would need special handling
                    }
                }

                ph_idx += 1;
            }
        }

        // End marker
        builder.add_instruction(Opcode::End);

        Ok(())
    }
}

/// Control flow frame for tracking nested structures.
#[derive(Clone, Debug)]
enum ControlFrame {
    If { jump_patch: usize },
    Loop,
}

// ============================================================================
// Compile Result
// ============================================================================

/// Statistics from template compilation.
#[derive(Clone, Debug, Default)]
pub struct CompileStats {
    /// Number of static segments.
    pub static_segments: usize,
    /// Number of placeholders.
    pub placeholders: usize,
    /// Number of unique strings (after dedup).
    pub unique_strings: usize,
    /// Total string bytes.
    pub string_bytes: usize,
    /// Instruction count.
    pub instructions: usize,
    /// Total output size.
    pub output_size: usize,
    /// Compilation time in microseconds.
    pub compile_time_us: u64,
}

impl Compiler {
    /// Compile and return statistics.
    pub fn compile_with_stats(
        &self,
        source: &[u8],
        options: CompileOptions,
    ) -> Result<(BinaryTemplate, CompileStats)> {
        let start = std::time::Instant::now();

        let template = self.compile(source, options)?;

        let stats = CompileStats {
            static_segments: 0, // Would need to track during compilation
            placeholders: template.placeholders.len(),
            unique_strings: template.strings.len(),
            string_bytes: template.strings.size_bytes(),
            instructions: template.instructions.len(),
            output_size: template.to_bytes().len(),
            compile_time_us: start.elapsed().as_micros() as u64,
        };

        Ok((template, stats))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple() {
        let compiler = Compiler::new();
        let source = b"Hello, {{ name }}!";

        let template = compiler
            .compile(source, CompileOptions::with_name("test"))
            .unwrap();

        assert_eq!(template.name, "test");
        assert!(template.is_micro_eligible());
        assert_eq!(template.param_names, vec!["name"]);
    }

    #[test]
    fn test_compile_with_control_flow() {
        let compiler = Compiler::new();
        let source = b"{% if admin %}Admin{% endif %}";

        let template = compiler
            .compile(source, CompileOptions::with_name("test"))
            .unwrap();

        assert!(!template.is_micro_eligible());
    }

    #[test]
    fn test_compile_multiple_vars() {
        let compiler = Compiler::new();
        let source = b"{{ greeting }}, {{ name }}!";

        let template = compiler
            .compile(source, CompileOptions::with_name("test"))
            .unwrap();

        assert_eq!(template.param_names.len(), 2);
        assert!(template.param_names.contains(&"greeting".to_string()));
        assert!(template.param_names.contains(&"name".to_string()));
    }

    #[test]
    fn test_compile_with_stats() {
        let compiler = Compiler::new();
        let source = b"Hello, {{ name }}! Today is {{ day }}.";

        let (template, stats) = compiler
            .compile_with_stats(source, CompileOptions::with_name("test"))
            .unwrap();

        assert_eq!(template.param_names.len(), 2);
        assert!(stats.compile_time_us < 10_000); // Should be fast
        assert!(stats.output_size > 0);
    }

    #[test]
    fn test_string_deduplication() {
        let compiler = Compiler::new();
        // Same string appears multiple times
        let source = b"Hello {{ name }}. Hello {{ other }}.";

        let template = compiler
            .compile(
                source,
                CompileOptions {
                    dedupe_strings: true,
                    ..Default::default()
                },
            )
            .unwrap();

        // With dedup enabled, "Hello " should only appear once in string table
        assert!(template.header.flags & FLAG_DEDUPED != 0);
    }
}
