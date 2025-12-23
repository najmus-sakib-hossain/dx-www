# DX Serializer VS Code Extension - Professional Implementation Plan

## 📋 Project Overview

### Goal
Create a bulletproof VS Code extension that provides seamless editing of `.dx` files with:
- Human-readable view in editor
- Dense/token-efficient format on disk (for LLMs and Git)
- Perfect auto-save compatibility
- Zero data corruption or value truncation

### Core Requirements
| Requirement | Solution |
|-------------|----------|
| Auto-save compatibility | Validation-gated saves with debouncing |
| Value preservation | Lossless round-trip serialization |
| Special character handling | Smart quoting (`don't` → `"don't"`) |
| LLM compatibility | Dense format on disk, same file path |
| Performance | Rust/WASM core for sub-millisecond transforms |

---

## 📁 Project Structure

```
vscode-dx-serializer/
├── .vscode/
│   └── launch.json                 # Debug configuration
├── media/
│   ├── logo.png                    # Extension icon (128x128)
│   └── file-extension-dark.png     # File icon for .dx files
├── wasm/
│   ├── dx_serializer.js            # Generated WASM bindings
│   ├── dx_serializer.d.ts          # TypeScript definitions
│   ├── dx_serializer_bg.wasm       # WASM binary
│   └── package.json                # WASM package manifest
├── src/
│   ├── extension.ts                # Extension entry point
│   ├── dxLensFileSystem.ts         # Virtual file system provider
│   ├── dxDocumentManager.ts        # Document state management
│   ├── dxCore.ts                   # WASM core wrapper
│   ├── dxValidator.ts              # Syntax validation
│   ├── dxFormatter.ts              # Human formatting logic
│   └── utils.ts                    # Utility functions
├── syntaxes/
│   └── dx.tmLanguage.json          # TextMate grammar (minimal)
├── test/
│   └── suite/
│       ├── extension.test.ts
│       ├── roundtrip.test.ts
│       └── autosave.test.ts
├── package.json
├── tsconfig.json
├── language-configuration.json
├── README.md
└── CHANGELOG.md
```

---

## 🦀 Rust WASM Core (Using Existing Serializer)

### `crates/serializer/src/wasm.rs`

```rust
//! WASM bindings for the DX Serializer VS Code extension
//! 
//! This module exposes the serializer functionality to JavaScript/TypeScript
//! through wasm-bindgen, enabling sub-millisecond transformations in VS Code.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import from your existing serializer crate
use crate::{DxValue, DxParser, DxFormatter, DxError};

/// Validation result returned to TypeScript
#[derive(Serialize, Deserialize, Clone, Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct ValidationResult {
    pub success: bool,
    pub error: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub hint: Option<String>,
}

#[wasm_bindgen]
impl ValidationResult {
    #[wasm_bindgen(constructor)]
    pub fn new(success: bool) -> Self {
        Self {
            success,
            error: None,
            line: None,
            column: None,
            hint: None,
        }
    }
}

/// Transformation result with preserved metadata
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct TransformResult {
    pub success: bool,
    pub content: String,
    pub error: Option<String>,
}

/// Configuration for the serializer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializerConfig {
    /// Indentation string (default: "  " - 2 spaces)
    pub indent: String,
    /// Whether to preserve comments in human format
    pub preserve_comments: bool,
    /// Whether to use smart quoting for special characters
    pub smart_quoting: bool,
    /// Maximum line length before wrapping (0 = no limit)
    pub max_line_length: usize,
}

impl Default for SerializerConfig {
    fn default() -> Self {
        Self {
            indent: "  ".to_string(),
            preserve_comments: true,
            smart_quoting: true,
            max_line_length: 120,
        }
    }
}

/// The main WASM-exposed serializer
#[wasm_bindgen]
pub struct DxSerializer {
    config: SerializerConfig,
}

#[wasm_bindgen]
impl DxSerializer {
    /// Create a new serializer with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            config: SerializerConfig::default(),
        }
    }

    /// Create a serializer with custom configuration
    #[wasm_bindgen]
    pub fn with_config(config_json: &str) -> Result<DxSerializer, JsValue> {
        let config: SerializerConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;
        Ok(Self { config })
    }

    /// Transform dense DX format to human-readable format
    /// 
    /// This is called when opening a .dx file in VS Code.
    /// The human format has:
    /// - Proper indentation
    /// - Aligned colons
    /// - Smart quoting for special characters
    /// - Preserved URLs and long values
    #[wasm_bindgen]
    pub fn to_human(&self, dense: &str) -> TransformResult {
        match self.to_human_internal(dense) {
            Ok(content) => TransformResult {
                success: true,
                content,
                error: None,
            },
            Err(e) => TransformResult {
                success: false,
                content: dense.to_string(), // Return original on error
                error: Some(e),
            },
        }
    }

    /// Transform human-readable DX format to dense format
    /// 
    /// This is called when saving a .dx file in VS Code.
    /// The dense format:
    /// - Removes all unnecessary whitespace
    /// - Strips comments
    /// - Minimizes token count for LLMs
    #[wasm_bindgen]
    pub fn to_dense(&self, human: &str) -> TransformResult {
        match self.to_dense_internal(human) {
            Ok(content) => TransformResult {
                success: true,
                content,
                error: None,
            },
            Err(e) => TransformResult {
                success: false,
                content: human.to_string(), // Return original on error
                error: Some(e),
            },
        }
    }

    /// Validate DX content without transforming
    /// 
    /// This is called during auto-save to check if content is complete.
    /// Returns success=false if:
    /// - Unclosed brackets/braces
    /// - Unclosed strings
    /// - Invalid syntax
    #[wasm_bindgen]
    pub fn validate(&self, content: &str) -> ValidationResult {
        self.validate_internal(content)
    }

    /// Check if the content is complete enough to save
    /// 
    /// More lenient than validate() - allows trailing commas, etc.
    /// Used for auto-save gating.
    #[wasm_bindgen]
    pub fn is_saveable(&self, content: &str) -> bool {
        let validation = self.validate_internal(content);
        
        // Content is saveable if:
        // 1. It's fully valid, OR
        // 2. The only error is a trailing comma (common during editing)
        if validation.success {
            return true;
        }
        
        // Check for recoverable errors
        if let Some(ref error) = validation.error {
            if error.contains("trailing comma") {
                return true;
            }
        }
        
        false
    }

    /// Get a preview of what the dense format would look like
    /// 
    /// Used for the "Show Dense View" command.
    #[wasm_bindgen]
    pub fn preview_dense(&self, human: &str) -> String {
        match self.to_dense_internal(human) {
            Ok(dense) => dense,
            Err(_) => human.to_string(),
        }
    }
}

// Internal implementation
impl DxSerializer {
    fn to_human_internal(&self, dense: &str) -> Result<String, String> {
        if dense.trim().is_empty() {
            return Ok(String::new());
        }

        let mut result = String::with_capacity(dense.len() * 2);
        let mut indent_level: usize = 0;
        let mut in_string = false;
        let mut string_char = '\0';
        let mut escape_next = false;
        let mut chars = dense.chars().peekable();
        let mut current_string = String::new();
        let mut collecting_string = false;

        while let Some(c) = chars.next() {
            // Handle escape sequences
            if escape_next {
                if collecting_string {
                    current_string.push('\\');
                    current_string.push(c);
                } else {
                    result.push('\\');
                    result.push(c);
                }
                escape_next = false;
                continue;
            }

            if c == '\\' && in_string {
                escape_next = true;
                continue;
            }

            // Handle string boundaries
            if (c == '"' || c == '\'') && !in_string {
                in_string = true;
                string_char = c;
                collecting_string = true;
                current_string.clear();
                continue;
            }

            if in_string && c == string_char {
                in_string = false;
                collecting_string = false;
                
                // Smart quoting: wrap the string appropriately
                let quoted = self.smart_quote(&current_string);
                result.push_str(&quoted);
                continue;
            }

            if collecting_string {
                current_string.push(c);
                continue;
            }

            // Handle structure characters
            match c {
                '{' | '[' => {
                    result.push(c);
                    result.push('\n');
                    indent_level += 1;
                    self.push_indent(&mut result, indent_level);
                }
                '}' | ']' => {
                    // Remove trailing whitespace
                    while result.ends_with(' ') || result.ends_with('\t') {
                        result.pop();
                    }
                    if !result.ends_with('\n') {
                        result.push('\n');
                    }
                    indent_level = indent_level.saturating_sub(1);
                    self.push_indent(&mut result, indent_level);
                    result.push(c);
                }
                ',' => {
                    result.push(c);
                    result.push('\n');
                    self.push_indent(&mut result, indent_level);
                }
                ':' => {
                    result.push(':');
                    result.push(' ');
                }
                ' ' | '\t' | '\n' | '\r' => {
                    // Skip whitespace in dense format
                }
                _ => {
                    result.push(c);
                }
            }
        }

        // Check for unclosed string
        if in_string {
            return Err("Unclosed string literal".to_string());
        }

        Ok(result.trim().to_string())
    }

    fn to_dense_internal(&self, human: &str) -> Result<String, String> {
        if human.trim().is_empty() {
            return Ok(String::new());
        }

        let mut result = String::with_capacity(human.len());
        let mut in_string = false;
        let mut string_char = '\0';
        let mut escape_next = false;
        let mut in_line_comment = false;
        let mut in_block_comment = false;
        let mut chars = human.chars().peekable();

        while let Some(c) = chars.next() {
            let next_char = chars.peek().copied().unwrap_or('\0');

            // Handle newlines (end line comments)
            if c == '\n' {
                in_line_comment = false;
                continue;
            }

            // Skip if in comment
            if in_line_comment {
                continue;
            }

            // Handle block comment end
            if in_block_comment {
                if c == '*' && next_char == '/' {
                    chars.next(); // consume '/'
                    in_block_comment = false;
                }
                continue;
            }

            // Handle escape sequences in strings
            if escape_next {
                result.push('\\');
                result.push(c);
                escape_next = false;
                continue;
            }

            if c == '\\' && in_string {
                escape_next = true;
                continue;
            }

            // Handle string boundaries
            if (c == '"' || c == '\'') && !in_string {
                in_string = true;
                string_char = c;
                result.push('"'); // Always use double quotes in dense
                continue;
            }

            if in_string && c == string_char {
                in_string = false;
                result.push('"');
                continue;
            }

            // Inside string - preserve everything
            if in_string {
                // Escape internal quotes if needed
                if c == '"' && string_char != '"' {
                    result.push('\\');
                }
                result.push(c);
                continue;
            }

            // Handle comments
            if c == '/' && next_char == '/' {
                in_line_comment = true;
                chars.next();
                continue;
            }

            if c == '/' && next_char == '*' {
                in_block_comment = true;
                chars.next();
                continue;
            }

            // Skip whitespace outside strings
            if c.is_whitespace() {
                continue;
            }

            result.push(c);
        }

        // Check for unclosed string
        if in_string {
            return Err("Unclosed string literal".to_string());
        }

        Ok(result)
    }

    fn validate_internal(&self, content: &str) -> ValidationResult {
        let mut brackets: Vec<(char, u32, u32)> = Vec::new();
        let mut line: u32 = 0;
        let mut col: u32 = 0;
        let mut in_string = false;
        let mut string_char = '\0';
        let mut string_start_line: u32 = 0;
        let mut string_start_col: u32 = 0;
        let mut escape_next = false;
        let mut in_line_comment = false;
        let mut in_block_comment = false;
        let mut chars = content.chars().peekable();

        while let Some(c) = chars.next() {
            let next_char = chars.peek().copied().unwrap_or('\0');

            // Track position
            if c == '\n' {
                line += 1;
                col = 0;
                in_line_comment = false;
                continue;
            }
            col += 1;

            // Skip comments
            if in_line_comment {
                continue;
            }

            if in_block_comment {
                if c == '*' && next_char == '/' {
                    chars.next();
                    col += 1;
                    in_block_comment = false;
                }
                continue;
            }

            // Handle escape sequences
            if escape_next {
                escape_next = false;
                continue;
            }

            if c == '\\' && in_string {
                escape_next = true;
                continue;
            }

            // Handle string boundaries
            if (c == '"' || c == '\'') && !in_string {
                in_string = true;
                string_char = c;
                string_start_line = line;
                string_start_col = col;
                continue;
            }

            if in_string && c == string_char {
                in_string = false;
                continue;
            }

            // Skip everything inside strings
            if in_string {
                continue;
            }

            // Handle comment starts
            if c == '/' && next_char == '/' {
                in_line_comment = true;
                continue;
            }

            if c == '/' && next_char == '*' {
                in_block_comment = true;
                chars.next();
                continue;
            }

            // Check brackets
            match c {
                '{' | '[' | '(' => {
                    brackets.push((c, line, col));
                }
                '}' | ']' | ')' => {
                    let expected = match c {
                        '}' => '{',
                        ']' => '[',
                        ')' => '(',
                        _ => unreachable!(),
                    };

                    if brackets.is_empty() {
                        return ValidationResult {
                            success: false,
                            error: Some(format!("Unexpected '{}' - no matching opening bracket", c)),
                            line: Some(line),
                            column: Some(col),
                            hint: Some("Remove this bracket or add a matching opening bracket".to_string()),
                        };
                    }

                    let (last_char, open_line, open_col) = brackets.pop().unwrap();
                    if last_char != expected {
                        return ValidationResult {
                            success: false,
                            error: Some(format!(
                                "Mismatched brackets: opened '{}' at line {}, but closed with '{}'",
                                last_char, open_line + 1, c
                            )),
                            line: Some(line),
                            column: Some(col),
                            hint: Some(format!(
                                "Expected '{}' to close '{}' from line {}",
                                match last_char { '{' => '}', '[' => ']', '(' => ')', _ => '?' },
                                last_char,
                                open_line + 1
                            )),
                        };
                    }
                }
                _ => {}
            }
        }

        // Check for unclosed strings
        if in_string {
            return ValidationResult {
                success: false,
                error: Some("Unclosed string literal".to_string()),
                line: Some(string_start_line),
                column: Some(string_start_col),
                hint: Some(format!("Add closing {} to complete the string", string_char)),
            };
        }

        // Check for unclosed block comments
        if in_block_comment {
            return ValidationResult {
                success: false,
                error: Some("Unclosed block comment".to_string()),
                line: Some(line),
                column: Some(col),
                hint: Some("Add */ to close the block comment".to_string()),
            };
        }

        // Check for unclosed brackets
        if !brackets.is_empty() {
            let (c, l, col) = brackets.last().unwrap();
            return ValidationResult {
                success: false,
                error: Some(format!("Unclosed '{}' bracket", c)),
                line: Some(*l),
                column: Some(*col),
                hint: Some(format!(
                    "Add '{}' to close this bracket",
                    match c { '{' => '}', '[' => ']', '(' => ')', _ => '?' }
                )),
            };
        }

        ValidationResult {
            success: true,
            error: None,
            line: None,
            column: None,
            hint: None,
        }
    }

    /// Smart quoting: ensures values with special characters are properly quoted
    fn smart_quote(&self, value: &str) -> String {
        if !self.config.smart_quoting {
            return format!("\"{}\"", value);
        }

        // Check if value contains characters that need special handling
        let needs_double_quotes = value.contains('\'') 
            || value.contains('\n')
            || value.contains('\t')
            || value.contains('\\');
        
        let needs_single_quotes = value.contains('"');
        
        // If contains both, escape the double quotes and use double quotes
        if needs_double_quotes && needs_single_quotes {
            let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
            return format!("\"{}\"", escaped);
        }
        
        // If contains single quotes, use double quotes
        if needs_double_quotes {
            return format!("\"{}\"", value);
        }
        
        // If contains double quotes, use single quotes (for human readability)
        // But in dense format, we always use double quotes
        if needs_single_quotes {
            return format!("\"{}\"", value.replace('"', "\\\""));
        }
        
        // Default: use double quotes
        format!("\"{}\"", value)
    }

    fn push_indent(&self, result: &mut String, level: usize) {
        for _ in 0..level {
            result.push_str(&self.config.indent);
        }
    }
}

// Standalone WASM functions for simpler API
#[wasm_bindgen]
pub fn to_human(dense: &str) -> String {
    let serializer = DxSerializer::new();
    let result = serializer.to_human(dense);
    result.content
}

#[wasm_bindgen]
pub fn to_dense(human: &str) -> String {
    let serializer = DxSerializer::new();
    let result = serializer.to_dense(human);
    result.content
}

#[wasm_bindgen]
pub fn validate(content: &str) -> JsValue {
    let serializer = DxSerializer::new();
    let result = serializer.validate(content);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn is_saveable(content: &str) -> bool {
    let serializer = DxSerializer::new();
    serializer.is_saveable(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_simple() {
        let serializer = DxSerializer::new();
        let dense = r#"{"name":"test","value":42}"#;
        
        let human_result = serializer.to_human(dense);
        assert!(human_result.success);
        
        let back_result = serializer.to_dense(&human_result.content);
        assert!(back_result.success);
        assert_eq!(dense, back_result.content);
    }

    #[test]
    fn test_round_trip_with_url() {
        let serializer = DxSerializer::new();
        let dense = r#"{"url":"https://example.com/path?query=value&foo=bar#anchor"}"#;
        
        let human_result = serializer.to_human(dense);
        assert!(human_result.success);
        assert!(human_result.content.contains("https://example.com/path?query=value&foo=bar#anchor"));
        
        let back_result = serializer.to_dense(&human_result.content);
        assert!(back_result.success);
        assert_eq!(dense, back_result.content);
    }

    #[test]
    fn test_smart_quoting_apostrophe() {
        let serializer = DxSerializer::new();
        let dense = r#"{"message":"don't worry"}"#;
        
        let human_result = serializer.to_human(dense);
        assert!(human_result.success);
        // Should preserve the apostrophe and use double quotes
        assert!(human_result.content.contains(r#""don't worry""#));
    }

    #[test]
    fn test_smart_quoting_both_quotes() {
        let serializer = DxSerializer::new();
        // Value contains both ' and "
        let dense = r#"{"message":"He said \"don't\""}"#;
        
        let human_result = serializer.to_human(dense);
        assert!(human_result.success);
        
        let back_result = serializer.to_dense(&human_result.content);
        assert!(back_result.success);
    }

    #[test]
    fn test_validation_incomplete() {
        let serializer = DxSerializer::new();
        
        // Unclosed brace
        let result = serializer.validate(r#"{"name": "test""#);
        assert!(!result.success);
        assert!(result.error.is_some());
        
        // Unclosed string
        let result = serializer.validate(r#"{"name": "test"#);
        assert!(!result.success);
    }

    #[test]
    fn test_validation_complete() {
        let serializer = DxSerializer::new();
        let result = serializer.validate(r#"{"name": "test", "value": 42}"#);
        assert!(result.success);
    }

    #[test]
    fn test_is_saveable_incomplete() {
        let serializer = DxSerializer::new();
        
        // Not saveable - unclosed bracket
        assert!(!serializer.is_saveable(r#"{"name": "#));
        
        // Saveable - trailing comma is ok
        assert!(serializer.is_saveable(r#"{"name": "test",}"#));
    }

    #[test]
    fn test_long_url_preservation() {
        let serializer = DxSerializer::new();
        let long_url = "https://api.example.com/v1/users/12345/posts/67890?include=comments,likes&filter[created_at][gte]=2024-01-01&page[size]=100&page[number]=1";
        let dense = format!(r#"{{"api_endpoint":"{}"}}"#, long_url);
        
        let human_result = serializer.to_human(&dense);
        assert!(human_result.success);
        assert!(human_result.content.contains(long_url), "URL was truncated!");
        
        let back_result = serializer.to_dense(&human_result.content);
        assert!(back_result.success);
        assert_eq!(dense, back_result.content);
    }

    #[test]
    fn test_nested_structure() {
        let serializer = DxSerializer::new();
        let dense = r#"{"a":{"b":{"c":{"d":"deep"}}}}"#;
        
        let human_result = serializer.to_human(dense);
        assert!(human_result.success);
        
        let back_result = serializer.to_dense(&human_result.content);
        assert!(back_result.success);
        assert_eq!(dense, back_result.content);
    }
}
```

### `crates/serializer/Cargo.toml` (Updated)

```toml
[package]
name = "dx-serializer"
version = "1.0.0"
edition = "2024"
authors = ["DX Team"]
description = "Binary-first serialization with world-record compression"
license = "MIT OR Apache-2.0"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["wasm"]
wasm = ["wasm-bindgen", "serde-wasm-bindgen", "console_error_panic_hook"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# WASM dependencies (optional)
wasm-bindgen = { version = "0.2.92", optional = true }
serde-wasm-bindgen = { version = "0.6", optional = true }
console_error_panic_hook = { version = "0.1", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
panic = "abort"      # Smaller binary
strip = true         # Strip symbols
```

### Build Script: `scripts/build-wasm.sh`

```bash
#!/bin/bash
set -e

echo "🦀 Building DX Serializer WASM..."

cd "$(dirname "$0")/.."

# Build WASM
cd crates/serializer
wasm-pack build \
    --target nodejs \
    --out-dir ../../vscode-dx-serializer/wasm \
    --release \
    -- --features wasm

echo "📦 WASM build complete!"

# Copy to extension
cd ../../vscode-dx-serializer

# Verify build
if [ -f "wasm/dx_serializer.js" ]; then
    echo "✅ WASM bindings generated successfully"
    ls -la wasm/
else
    echo "❌ WASM build failed - missing output files"
    exit 1
fi

echo ""
echo "🎉 Build complete! Run 'npm run compile' in vscode-dx-serializer/"
```

---

## 📦 VS Code Extension Implementation

### `package.json`

```json
{
  "name": "dx-serializer",
  "displayName": "DX Serializer",
  "description": "Binary-first serialization with human-readable editing for .dx files",
  "version": "1.0.0",
  "publisher": "dx",
  "icon": "media/logo.png",
  "repository": {
    "type": "git",
    "url": "https://github.com/anthropics/dx"
  },
  "engines": {
    "vscode": "^1.85.0"
  },
  "categories": [
    "Programming Languages",
    "Formatters"
  ],
  "keywords": [
    "dx",
    "serialization",
    "binary",
    "llm",
    "token-efficient"
  ],
  "activationEvents": [
    "onLanguage:dx",
    "onFileSystem:dxlens",
    "workspaceContains:**/*.dx"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "languages": [
      {
        "id": "dx",
        "aliases": [
          "DX"
        ],
        "extensions": [
          ".dx"
        ],
        "icon": {
          "light": "./media/file-extension-dark.png",
          "dark": "./media/file-extension-dark.png"
        },
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "dx",
        "scopeName": "source.dx",
        "path": "./syntaxes/dx.tmLanguage.json"
      }
    ],
    "commands": [
      {
        "command": "dx.refresh",
        "title": "DX: Refresh from Disk",
        "icon": "$(refresh)"
      },
      {
        "command": "dx.forceSave",
        "title": "DX: Force Save (Ignore Validation)"
      },
      {
        "command": "dx.showDense",
        "title": "DX: Show Dense View (Read-only)"
      },
      {
        "command": "dx.showHuman",
        "title": "DX: Show Human View"
      }
    ],
    "menus": {
      "editor/title": [
        {
          "command": "dx.refresh",
          "when": "resourceExtname == .dx",
          "group": "navigation"
        }
      ]
    },
    "configuration": {
      "title": "DX Serializer",
      "properties": {
        "dx.validateBeforeSave": {
          "type": "boolean",
          "default": true,
          "description": "Validate DX syntax before saving. When enabled, incomplete code won't corrupt the file."
        },
        "dx.autoSaveGracePeriod": {
          "type": "number",
          "default": 2000,
          "minimum": 500,
          "maximum": 10000,
          "description": "Grace period (ms) after last keystroke before auto-save writes to disk. Prevents saving incomplete code."
        },
        "dx.indentSize": {
          "type": "number",
          "default": 2,
          "enum": [2, 4],
          "description": "Number of spaces for indentation in human view."
        },
        "dx.showDensePreview": {
          "type": "boolean",
          "default": false,
          "description": "Show dense format preview on hover."
        }
      }
    }
  },
  "scripts": {
    "vscode:prepublish": "npm run compile",
    "compile": "tsc -p ./",
    "watch": "tsc -watch -p ./",
    "lint": "eslint src --ext ts",
    "test": "node ./out/test/runTest.js",
    "build:wasm": "cd .. && ./scripts/build-wasm.sh",
    "package": "vsce package"
  },
  "devDependencies": {
    "@types/node": "^20.10.0",
    "@types/vscode": "^1.85.0",
    "@typescript-eslint/eslint-plugin": "^6.15.0",
    "@typescript-eslint/parser": "^6.15.0",
    "eslint": "^8.56.0",
    "typescript": "^5.3.0"
  }
}
```

### `src/extension.ts`

```typescript
import * as vscode from 'vscode';
import { DxLensFileSystem } from './dxLensFileSystem';
import { DxDocumentManager } from './dxDocumentManager';
import { loadDxCore, DxCore } from './dxCore';
import { isExactlyDxFile, getDiskUri, getLensUri } from './utils';

let documentManager: DxDocumentManager;
let dxCore: DxCore;

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Activating DX Serializer extension...');
    
    // Load WASM core
    try {
        dxCore = await loadDxCore(context.extensionPath);
        console.log('[DX] WASM core loaded successfully');
    } catch (error) {
        const msg = `DX Serializer: Failed to load core: ${error}`;
        console.error(msg);
        vscode.window.showErrorMessage(msg);
        return;
    }
    
    // Initialize document manager
    documentManager = new DxDocumentManager(dxCore);
    context.subscriptions.push(documentManager);
    
    // Register virtual file system
    const lensFs = new DxLensFileSystem(dxCore, documentManager);
    context.subscriptions.push(
        vscode.workspace.registerFileSystemProvider('dxlens', lensFs, {
            isCaseSensitive: true,
            isReadonly: false
        })
    );
    
    // === Auto-redirect .dx files to lens view ===
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (doc) => {
            if (isExactlyDxFile(doc.uri) && !documentManager.isRedirecting) {
                await redirectToLensView(doc);
            }
        })
    );
    
    // === Watch for external file changes ===
    const watcher = vscode.workspace.createFileSystemWatcher('**/*.dx');
    context.subscriptions.push(watcher);
    
    watcher.onDidChange(async (uri) => {
        if (!documentManager.isWriting(uri)) {
            console.log(`[DX] External change detected: ${uri.fsPath}`);
            await documentManager.handleExternalChange(uri);
        }
    });
    
    watcher.onDidDelete((uri) => {
        documentManager.handleFileDeleted(uri);
    });
    
    // === Register commands ===
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.refresh', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor?.document.uri.scheme === 'dxlens') {
                await documentManager.forceRefresh(editor.document.uri);
                vscode.window.showInformationMessage('DX: Refreshed from disk');
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.forceSave', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor?.document.uri.scheme === 'dxlens') {
                await documentManager.forceSave(editor.document.uri);
                vscode.window.showInformationMessage('DX: Force saved to disk');
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.showDense', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor?.document.uri.scheme === 'dxlens') {
                await showDensePreview(editor.document.uri);
            }
        })
    );
    
    // === Status bar item ===
    const statusBar = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );
    statusBar.command = 'dx.showDense';
    context.subscriptions.push(statusBar);
    
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor((editor) => {
            if (editor?.document.uri.scheme === 'dxlens') {
                const state = documentManager.getState(editor.document.uri);
                if (state?.isValid) {
                    statusBar.text = '$(check) DX';
                    statusBar.tooltip = 'DX: Valid - Click to preview dense format';
                    statusBar.backgroundColor = undefined;
                } else {
                    statusBar.text = '$(warning) DX';
                    statusBar.tooltip = `DX: ${state?.lastError || 'Invalid syntax'}`;
                    statusBar.backgroundColor = new vscode.ThemeColor(
                        'statusBarItem.warningBackground'
                    );
                }
                statusBar.show();
            } else {
                statusBar.hide();
            }
        })
    );
    
    console.log('[DX] Extension activated successfully');
}

async function redirectToLensView(doc: vscode.TextDocument): Promise<void> {
    documentManager.isRedirecting = true;
    
    try {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.uri.toString() !== doc.uri.toString()) {
            return;
        }
        
        const viewColumn = editor.viewColumn;
        
        // Close the raw file
        await vscode.commands.executeCommand('workbench.action.closeActiveEditor');
        
        // Open lens view
        const lensUri = getLensUri(doc.uri);
        const lensDoc = await vscode.workspace.openTextDocument(lensUri);
        await vscode.window.showTextDocument(lensDoc, {
            viewColumn,
            preview: false,
            preserveFocus: false
        });
        
    } catch (error) {
        console.error('[DX] Redirect error:', error);
    } finally {
        documentManager.isRedirecting = false;
    }
}

async function showDensePreview(lensUri: vscode.Uri): Promise<void> {
    const diskUri = getDiskUri(lensUri);
    
    try {
        const content = await vscode.workspace.fs.readFile(diskUri);
        const denseText = new TextDecoder().decode(content);
        
        const doc = await vscode.workspace.openTextDocument({
            content: denseText,
            language: 'dx'
        });
        
        await vscode.window.showTextDocument(doc, {
            viewColumn: vscode.ViewColumn.Beside,
            preview: true,
            preserveFocus: true
        });
        
    } catch (error) {
        vscode.window.showErrorMessage(`DX: Failed to show dense view: ${error}`);
    }
}

export function deactivate() {
    if (documentManager) {
        documentManager.dispose();
    }
    console.log('[DX] Extension deactivated');
}
```

### `src/dxDocumentManager.ts`

```typescript
import * as vscode from 'vscode';
import { DxCore } from './dxCore';
import { getDiskUri, getLensUri } from './utils';

interface DocumentState {
    /** Dense content currently on disk */
    diskDense: string;
    
    /** Last successfully saved dense content */
    lastValidDense: string;
    
    /** Current human content in editor */
    currentHuman: string;
    
    /** Whether content is syntactically valid */
    isValid: boolean;
    
    /** Last validation error */
    lastError: string | null;
    
    /** Timestamp of last keystroke */
    lastKeystroke: number;
    
    /** Pending save timeout */
    saveTimeout: NodeJS.Timeout | null;
    
    /** Whether a save is in progress */
    isSaving: boolean;
}

export class DxDocumentManager implements vscode.Disposable {
    private states = new Map<string, DocumentState>();
    private writingFiles = new Set<string>();
    private diagnostics: vscode.DiagnosticCollection;
    private disposables: vscode.Disposable[] = [];
    
    public isRedirecting = false;
    
    // Configuration
    private validateBeforeSave = true;
    private autoSaveGracePeriod = 2000;
    
    // Events
    private _onDidChangeContent = new vscode.EventEmitter<vscode.Uri>();
    readonly onDidChangeContent = this._onDidChangeContent.event;
    
    constructor(private dxCore: DxCore) {
        this.diagnostics = vscode.languages.createDiagnosticCollection('dx');
        this.disposables.push(this.diagnostics);
        
        this.loadConfig();
        
        // Watch config changes
        this.disposables.push(
            vscode.workspace.onDidChangeConfiguration((e) => {
                if (e.affectsConfiguration('dx')) {
                    this.loadConfig();
                }
            })
        );
        
        // Handle document changes for validation
        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument((e) => {
                if (e.document.uri.scheme === 'dxlens') {
                    this.handleDocumentChange(e.document);
                }
            })
        );
    }
    
    private loadConfig(): void {
        const config = vscode.workspace.getConfiguration('dx');
        this.validateBeforeSave = config.get('validateBeforeSave', true);
        this.autoSaveGracePeriod = config.get('autoSaveGracePeriod', 2000);
    }
    
    private getKey(uri: vscode.Uri): string {
        const diskUri = uri.scheme === 'dxlens' ? getDiskUri(uri) : uri;
        return diskUri.fsPath;
    }
    
    getState(uri: vscode.Uri): DocumentState | undefined {
        return this.states.get(this.getKey(uri));
    }
    
    /**
     * Initialize document when opening
     */
    async initializeDocument(lensUri: vscode.Uri): Promise<string> {
        const key = this.getKey(lensUri);
        const diskUri = getDiskUri(lensUri);
        
        console.log(`[DX] Initializing: ${diskUri.fsPath}`);
        
        try {
            // Read dense content from disk
            const rawData = await vscode.workspace.fs.readFile(diskUri);
            const denseContent = new TextDecoder().decode(rawData);
            
            // Transform to human-readable
            const result = this.dxCore.toHuman(denseContent);
            
            if (!result.success) {
                console.error(`[DX] Transform error: ${result.error}`);
                // Return original if transform fails
                return denseContent;
            }
            
            const humanContent = result.content;
            
            // Validate
            const validation = this.dxCore.validate(humanContent);
            
            // Create state
            const state: DocumentState = {
                diskDense: denseContent,
                lastValidDense: denseContent,
                currentHuman: humanContent,
                isValid: validation.success,
                lastError: validation.error || null,
                lastKeystroke: Date.now(),
                saveTimeout: null,
                isSaving: false
            };
            
            this.states.set(key, state);
            console.log(`[DX] Initialized successfully: ${diskUri.fsPath}`);
            
            return humanContent;
            
        } catch (error) {
            console.error(`[DX] Init failed: ${error}`);
            throw error;
        }
    }
    
    /**
     * Handle document content changes
     */
    private handleDocumentChange(doc: vscode.TextDocument): void {
        const key = this.getKey(doc.uri);
        const state = this.states.get(key);
        
        if (!state) {
            return;
        }
        
        // Update state
        state.currentHuman = doc.getText();
        state.lastKeystroke = Date.now();
        
        // Clear pending save
        if (state.saveTimeout) {
            clearTimeout(state.saveTimeout);
            state.saveTimeout = null;
        }
        
        // Validate immediately for UI feedback
        const validation = this.dxCore.validate(state.currentHuman);
        state.isValid = validation.success;
        state.lastError = validation.error || null;
        
        // Update diagnostics
        this.updateDiagnostics(doc.uri, validation);
        
        // Schedule save with grace period (for auto-save compatibility)
        if (this.validateBeforeSave) {
            state.saveTimeout = setTimeout(() => {
                this.checkAndMarkSaveable(doc.uri);
            }, this.autoSaveGracePeriod);
        }
    }
    
    private updateDiagnostics(
        uri: vscode.Uri,
        validation: { success: boolean; error?: string; line?: number; column?: number; hint?: string }
    ): void {
        if (validation.success) {
            this.diagnostics.delete(uri);
            return;
        }
        
        const line = validation.line ?? 0;
        const column = validation.column ?? 0;
        
        const diagnostic = new vscode.Diagnostic(
            new vscode.Range(line, column, line, column + 1),
            validation.error ?? 'Invalid DX syntax',
            vscode.DiagnosticSeverity.Error
        );
        diagnostic.source = 'DX Serializer';
        
        if (validation.hint) {
            diagnostic.message += `\n${validation.hint}`;
        }
        
        this.diagnostics.set(uri, [diagnostic]);
    }
    
    private checkAndMarkSaveable(uri: vscode.Uri): void {
        const state = this.states.get(this.getKey(uri));
        if (!state) return;
        
        // Re-validate
        const validation = this.dxCore.validate(state.currentHuman);
        state.isValid = validation.success;
        state.lastError = validation.error || null;
    }
    
    /**
     * Save document - THE CRITICAL FUNCTION
     * 
     * This is called by the file system provider when VS Code saves.
     * It must handle auto-save correctly.
     */
    async saveDocument(lensUri: vscode.Uri, content: Uint8Array): Promise<void> {
        const key = this.getKey(lensUri);
        const state = this.states.get(key);
        const diskUri = getDiskUri(lensUri);
        
        if (!state) {
            console.error(`[DX] No state for save: ${lensUri.fsPath}`);
            throw new Error('Document not initialized');
        }
        
        // Prevent concurrent saves
        if (state.isSaving) {
            console.log(`[DX] Save already in progress, skipping`);
            return;
        }
        
        const humanContent = new TextDecoder().decode(content);
        state.currentHuman = humanContent;
        
        // === CRITICAL: Check if content is saveable ===
        if (this.validateBeforeSave) {
            // Check time since last keystroke
            const timeSinceKeystroke = Date.now() - state.lastKeystroke;
            
            if (timeSinceKeystroke < this.autoSaveGracePeriod) {
                // User is still typing - don't save yet
                console.log(`[DX] Grace period active (${timeSinceKeystroke}ms), skipping save`);
                return;
            }
            
            // Validate
            const validation = this.dxCore.validate(humanContent);
            
            if (!validation.success) {
                console.log(`[DX] Content invalid, skipping save: ${validation.error}`);
                
                // Show status bar warning
                vscode.window.setStatusBarMessage(
                    `$(warning) DX: ${validation.error || 'Incomplete'} - auto-save skipped`,
                    3000
                );
                
                // Keep the last valid dense on disk
                return;
            }
        }
        
        state.isSaving = true;
        this.writingFiles.add(diskUri.fsPath);
        
        try {
            // Transform to dense
            const result = this.dxCore.toDense(humanContent);
            
            if (!result.success) {
                console.error(`[DX] Transform failed: ${result.error}`);
                throw new Error(result.error);
            }
            
            const denseContent = result.content;
            const denseBytes = new TextEncoder().encode(denseContent);
            
            // Write to disk
            console.log(`[DX] Writing to: ${diskUri.fsPath}`);
            await vscode.workspace.fs.writeFile(diskUri, denseBytes);
            
            // Update state
            state.diskDense = denseContent;
            state.lastValidDense = denseContent;
            state.isValid = true;
            state.lastError = null;
            
            // Clear diagnostics
            this.diagnostics.delete(lensUri);
            
            console.log(`[DX] Save successful: ${diskUri.fsPath}`);
            
        } catch (error) {
            console.error(`[DX] Save failed: ${error}`);
            vscode.window.showErrorMessage(`DX: Save failed: ${error}`);
            throw error;
            
        } finally {
            state.isSaving = false;
            
            // Remove from writing set after delay (for file watcher)
            setTimeout(() => {
                this.writingFiles.delete(diskUri.fsPath);
            }, 500);
        }
    }
    
    /**
     * Force save without validation
     */
    async forceSave(lensUri: vscode.Uri): Promise<void> {
        const originalValidate = this.validateBeforeSave;
        
        try {
            this.validateBeforeSave = false;
            
            const doc = await vscode.workspace.openTextDocument(lensUri);
            const content = new TextEncoder().encode(doc.getText());
            await this.saveDocument(lensUri, content);
            
        } finally {
            this.validateBeforeSave = originalValidate;
        }
    }
    
    /**
     * Handle external file changes (git, other editors)
     */
    async handleExternalChange(diskUri: vscode.Uri): Promise<void> {
        const key = this.getKey(diskUri);
        const state = this.states.get(key);
        
        if (!state) return;
        
        console.log(`[DX] External change: ${diskUri.fsPath}`);
        
        try {
            // Read new content
            const rawData = await vscode.workspace.fs.readFile(diskUri);
            const newDense = new TextDecoder().decode(rawData);
            
            // Skip if unchanged
            if (newDense === state.diskDense) {
                return;
            }
            
            // Transform to human
            const result = this.dxCore.toHuman(newDense);
            
            if (!result.success) {
                console.error(`[DX] External change transform failed: ${result.error}`);
                return;
            }
            
            // Update state
            state.diskDense = newDense;
            state.lastValidDense = newDense;
            state.currentHuman = result.content;
            state.isValid = true;
            state.lastError = null;
            
            // Notify file system to refresh
            const lensUri = getLensUri(diskUri);
            this._onDidChangeContent.fire(lensUri);
            
            vscode.window.setStatusBarMessage('$(sync) DX: Updated from disk', 2000);
            
        } catch (error) {
            console.error(`[DX] External change failed: ${error}`);
        }
    }
    
    /**
     * Force refresh from disk
     */
    async forceRefresh(lensUri: vscode.Uri): Promise<void> {
        const diskUri = getDiskUri(lensUri);
        
        // Clear state to force re-init
        this.states.delete(this.getKey(lensUri));
        
        // Trigger refresh
        this._onDidChangeContent.fire(lensUri);
    }
    
    /**
     * Handle file deletion
     */
    handleFileDeleted(diskUri: vscode.Uri): void {
        const key = this.getKey(diskUri);
        const state = this.states.get(key);
        
        if (state) {
            if (state.saveTimeout) {
                clearTimeout(state.saveTimeout);
            }
            this.states.delete(key);
        }
        
        this.diagnostics.delete(diskUri);
    }
    
    isWriting(uri: vscode.Uri): boolean {
        return this.writingFiles.has(uri.fsPath);
    }
    
    dispose(): void {
        // Clear all timeouts
        for (const state of this.states.values()) {
            if (state.saveTimeout) {
                clearTimeout(state.saveTimeout);
            }
        }
        
        this.states.clear();
        this.writingFiles.clear();
        
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
```

### `src/dxLensFileSystem.ts`

```typescript
import * as vscode from 'vscode';
import { DxCore } from './dxCore';
import { DxDocumentManager } from './dxDocumentManager';
import { getDiskUri } from './utils';

export class DxLensFileSystem implements vscode.FileSystemProvider {
    private _onDidChangeFile = new vscode.EventEmitter<vscode.FileChangeEvent[]>();
    readonly onDidChangeFile = this._onDidChangeFile.event;
    
    constructor(
        private dxCore: DxCore,
        private documentManager: DxDocumentManager
    ) {
        // Subscribe to document manager events
        documentManager.onDidChangeContent((uri) => {
            this._onDidChangeFile.fire([{
                type: vscode.FileChangeType.Changed,
                uri
            }]);
        });
    }
    
    watch(): vscode.Disposable {
        return new vscode.Disposable(() => {});
    }
    
    async stat(uri: vscode.Uri): Promise<vscode.FileStat> {
        const diskUri = getDiskUri(uri);
        
        try {
            const stat = await vscode.workspace.fs.stat(diskUri);
            const state = this.documentManager.getState(uri);
            
            // Use human content size if available
            const size = state
                ? new TextEncoder().encode(state.currentHuman).length
                : stat.size;
            
            return {
                type: stat.type,
                ctime: stat.ctime,
                mtime: stat.mtime,
                size
            };
        } catch {
            throw vscode.FileSystemError.FileNotFound(uri);
        }
    }
    
    async readFile(uri: vscode.Uri): Promise<Uint8Array> {
        console.log(`[DX-FS] readFile: ${uri.fsPath}`);
        
        // Check for existing state
        let state = this.documentManager.getState(uri);
        
        if (!state) {
            // Initialize document
            const humanContent = await this.documentManager.initializeDocument(uri);
            return new TextEncoder().encode(humanContent);
        }
        
        return new TextEncoder().encode(state.currentHuman);
    }
    
    async writeFile(
        uri: vscode.Uri,
        content: Uint8Array,
        options: { create: boolean; overwrite: boolean }
    ): Promise<void> {
        console.log(`[DX-FS] writeFile: ${uri.fsPath}`);
        
        await this.documentManager.saveDocument(uri, content);
        
        // Emit change event
        this._onDidChangeFile.fire([{
            type: vscode.FileChangeType.Changed,
            uri
        }]);
    }
    
    async readDirectory(uri: vscode.Uri): Promise<[string, vscode.FileType][]> {
        const diskUri = getDiskUri(uri);
        return vscode.workspace.fs.readDirectory(diskUri);
    }
    
    async createDirectory(uri: vscode.Uri): Promise<void> {
        const diskUri = getDiskUri(uri);
        await vscode.workspace.fs.createDirectory(diskUri);
    }
    
    async delete(uri: vscode.Uri, options: { recursive: boolean }): Promise<void> {
        const diskUri = getDiskUri(uri);
        await vscode.workspace.fs.delete(diskUri, options);
        this.documentManager.handleFileDeleted(diskUri);
    }
    
    async rename(
        oldUri: vscode.Uri,
        newUri: vscode.Uri,
        options: { overwrite: boolean }
    ): Promise<void> {
        const oldDiskUri = getDiskUri(oldUri);
        const newDiskUri = getDiskUri(newUri);
        
        await vscode.workspace.fs.rename(oldDiskUri, newDiskUri, options);
        this.documentManager.handleFileDeleted(oldDiskUri);
    }
}
```

### `src/dxCore.ts`

```typescript
import * as path from 'path';
import * as fs from 'fs';

export interface TransformResult {
    success: boolean;
    content: string;
    error?: string;
}

export interface ValidationResult {
    success: boolean;
    error?: string;
    line?: number;
    column?: number;
    hint?: string;
}

export interface DxCore {
    toHuman(dense: string): TransformResult;
    toDense(human: string): TransformResult;
    validate(content: string): ValidationResult;
    isSaveable(content: string): boolean;
}

export async function loadDxCore(extensionPath: string): Promise<DxCore> {
    const wasmPath = path.join(extensionPath, 'wasm', 'dx_serializer.js');
    
    if (!fs.existsSync(wasmPath)) {
        console.log('[DX] WASM not found, using fallback');
        return createFallbackCore();
    }
    
    try {
        const wasm = require(wasmPath);
        
        // Initialize WASM if needed
        if (typeof wasm.default === 'function') {
            await wasm.default();
        }
        
        // Create serializer instance
        const serializer = new wasm.DxSerializer();
        
        return {
            toHuman: (dense: string): TransformResult => {
                try {
                    const result = serializer.to_human(dense);
                    return {
                        success: result.success,
                        content: result.content,
                        error: result.error
                    };
                } catch (e) {
                    return {
                        success: false,
                        content: dense,
                        error: String(e)
                    };
                }
            },
            
            toDense: (human: string): TransformResult => {
                try {
                    const result = serializer.to_dense(human);
                    return {
                        success: result.success,
                        content: result.content,
                        error: result.error
                    };
                } catch (e) {
                    return {
                        success: false,
                        content: human,
                        error: String(e)
                    };
                }
            },
            
            validate: (content: string): ValidationResult => {
                try {
                    const result = serializer.validate(content);
                    return {
                        success: result.success,
                        error: result.error,
                        line: result.line,
                        column: result.column,
                        hint: result.hint
                    };
                } catch (e) {
                    return {
                        success: false,
                        error: String(e)
                    };
                }
            },
            
            isSaveable: (content: string): boolean => {
                try {
                    return serializer.is_saveable(content);
                } catch {
                    return false;
                }
            }
        };
        
    } catch (error) {
        console.error('[DX] WASM load failed:', error);
        return createFallbackCore();
    }
}

function createFallbackCore(): DxCore {
    return {
        toHuman: (dense: string): TransformResult => {
            try {
                return {
                    success: true,
                    content: formatDx(dense)
                };
            } catch (e) {
                return {
                    success: false,
                    content: dense,
                    error: String(e)
                };
            }
        },
        
        toDense: (human: string): TransformResult => {
            try {
                return {
                    success: true,
                    content: minifyDx(human)
                };
            } catch (e) {
                return {
                    success: false,
                    content: human,
                    error: String(e)
                };
            }
        },
        
        validate: (content: string): ValidationResult => {
            return validateDx(content);
        },
        
        isSaveable: (content: string): boolean => {
            const result = validateDx(content);
            if (result.success) return true;
            // Allow trailing commas
            if (result.error?.includes('trailing comma')) return true;
            return false;
        }
    };
}

// === Fallback Implementation ===

function formatDx(dense: string): string {
    if (!dense.trim()) return '';
    
    let result = '';
    let indent = 0;
    let inString = false;
    let stringChar = '';
    let escape = false;
    let stringContent = '';
    
    for (let i = 0; i < dense.length; i++) {
        const c = dense[i];
        const next = dense[i + 1] || '';
        
        if (escape) {
            if (inString) stringContent += '\\' + c;
            else result += '\\' + c;
            escape = false;
            continue;
        }
        
        if (c === '\\') {
            escape = true;
            continue;
        }
        
        if ((c === '"' || c === "'") && !inString) {
            inString = true;
            stringChar = c;
            stringContent = '';
            continue;
        }
        
        if (inString && c === stringChar) {
            inString = false;
            // Smart quote the content
            result += smartQuote(stringContent);
            continue;
        }
        
        if (inString) {
            stringContent += c;
            continue;
        }
        
        switch (c) {
            case '{':
            case '[':
                result += c + '\n' + '  '.repeat(++indent);
                break;
            case '}':
            case ']':
                result = result.trimEnd() + '\n' + '  '.repeat(--indent) + c;
                break;
            case ',':
                result += c + '\n' + '  '.repeat(indent);
                break;
            case ':':
                result += ': ';
                break;
            case ' ':
            case '\n':
            case '\r':
            case '\t':
                break;
            default:
                result += c;
        }
    }
    
    return result.trim();
}

function smartQuote(value: string): string {
    // If contains single quote, use double quotes
    if (value.includes("'")) {
        // Escape any existing double quotes
        const escaped = value.replace(/"/g, '\\"');
        return `"${escaped}"`;
    }
    
    // If contains double quote, escape them
    if (value.includes('"')) {
        const escaped = value.replace(/"/g, '\\"');
        return `"${escaped}"`;
    }
    
    return `"${value}"`;
}

function minifyDx(human: string): string {
    if (!human.trim()) return '';
    
    let result = '';
    let inString = false;
    let stringChar = '';
    let escape = false;
    let inLineComment = false;
    let inBlockComment = false;
    
    for (let i = 0; i < human.length; i++) {
        const c = human[i];
        const next = human[i + 1] || '';
        
        if (c === '\n') {
            inLineComment = false;
            continue;
        }
        
        if (inLineComment) continue;
        
        if (inBlockComment) {
            if (c === '*' && next === '/') {
                inBlockComment = false;
                i++;
            }
            continue;
        }
        
        if (escape) {
            result += '\\' + c;
            escape = false;
            continue;
        }
        
        if (c === '\\' && inString) {
            escape = true;
            continue;
        }
        
        if ((c === '"' || c === "'") && !inString) {
            inString = true;
            stringChar = c;
            result += '"'; // Always use double quotes in dense
            continue;
        }
        
        if (inString && c === stringChar) {
            inString = false;
            result += '"';
            continue;
        }
        
        if (inString) {
            // Escape internal double quotes if we're using different quote type
            if (c === '"' && stringChar !== '"') {
                result += '\\"';
            } else {
                result += c;
            }
            continue;
        }
        
        if (c === '/' && next === '/') {
            inLineComment = true;
            i++;
            continue;
        }
        
        if (c === '/' && next === '*') {
            inBlockComment = true;
            i++;
            continue;
        }
        
        if (c === ' ' || c === '\t' || c === '\r') {
            continue;
        }
        
        result += c;
    }
    
    return result;
}

function validateDx(content: string): ValidationResult {
    const brackets: { char: string; line: number; col: number }[] = [];
    let line = 0;
    let col = 0;
    let inString = false;
    let stringChar = '';
    let stringLine = 0;
    let stringCol = 0;
    let escape = false;
    let inLineComment = false;
    let inBlockComment = false;
    
    for (let i = 0; i < content.length; i++) {
        const c = content[i];
        const next = content[i + 1] || '';
        
        if (c === '\n') {
            line++;
            col = 0;
            inLineComment = false;
            continue;
        }
        col++;
        
        if (inLineComment) continue;
        
        if (inBlockComment) {
            if (c === '*' && next === '/') {
                inBlockComment = false;
                i++;
                col++;
            }
            continue;
        }
        
        if (escape) {
            escape = false;
            continue;
        }
        
        if (c === '\\' && inString) {
            escape = true;
            continue;
        }
        
        if ((c === '"' || c === "'") && !inString) {
            inString = true;
            stringChar = c;
            stringLine = line;
            stringCol = col;
            continue;
        }
        
        if (inString && c === stringChar) {
            inString = false;
            continue;
        }
        
        if (inString) continue;
        
        if (c === '/' && next === '/') {
            inLineComment = true;
            continue;
        }
        
        if (c === '/' && next === '*') {
            inBlockComment = true;
            continue;
        }
        
        if (c === '{' || c === '[' || c === '(') {
            brackets.push({ char: c, line, col });
        } else if (c === '}' || c === ']' || c === ')') {
            const expected = c === '}' ? '{' : c === ']' ? '[' : '(';
            
            if (brackets.length === 0) {
                return {
                    success: false,
                    error: `Unexpected '${c}'`,
                    line,
                    column: col,
                    hint: 'No matching opening bracket'
                };
            }
            
            const last = brackets.pop()!;
            if (last.char !== expected) {
                return {
                    success: false,
                    error: `Mismatched brackets`,
                    line,
                    column: col,
                    hint: `Expected '${expected === '{' ? '}' : expected === '[' ? ']' : ')'}' to match '${last.char}' at line ${last.line + 1}`
                };
            }
        }
    }
    
    if (inString) {
        return {
            success: false,
            error: 'Unclosed string',
            line: stringLine,
            column: stringCol,
            hint: `Add ${stringChar} to close the string`
        };
    }
    
    if (inBlockComment) {
        return {
            success: false,
            error: 'Unclosed block comment',
            line,
            column: col,
            hint: 'Add */ to close the comment'
        };
    }
    
    if (brackets.length > 0) {
        const last = brackets[brackets.length - 1];
        return {
            success: false,
            error: `Unclosed '${last.char}'`,
            line: last.line,
            column: last.col,
            hint: `Add '${last.char === '{' ? '}' : last.char === '[' ? ']' : ')'}' to close this bracket`
        };
    }
    
    return { success: true };
}
```

### `src/utils.ts`

```typescript
import * as vscode from 'vscode';

/**
 * Check if URI is exactly a .dx file (no prefixes/suffixes)
 */
export function isExactlyDxFile(uri: vscode.Uri): boolean {
    if (uri.scheme !== 'file') return false;
    
    const fsPath = uri.fsPath.toLowerCase();
    
    // Must end with .dx
    if (!fsPath.endsWith('.dx')) return false;
    
    // Exclude compound extensions
    const excludePatterns = [
        '.dx.json',
        '.dx.yml',
        '.dx.yaml',
        '.dx.toml',
        '.dx.xml',
        '.dx.bak',
        '.dx.backup',
        '.dx.tmp',
        '.dx.temp',
        '.dx.orig',
        '.dx.old',
        '.dx.new'
    ];
    
    for (const pattern of excludePatterns) {
        if (fsPath.endsWith(pattern)) return false;
    }
    
    // Verify it's exactly .dx (not .dxyz etc)
    const lastDot = fsPath.lastIndexOf('.');
    const ext = fsPath.substring(lastDot);
    
    return ext === '.dx';
}

/**
 * Convert lens URI to disk URI
 */
export function getDiskUri(uri: vscode.Uri): vscode.Uri {
    if (uri.scheme === 'dxlens') {
        return uri.with({ scheme: 'file' });
    }
    return uri;
}

/**
 * Convert disk URI to lens URI
 */
export function getLensUri(uri: vscode.Uri): vscode.Uri {
    if (uri.scheme === 'file') {
        return uri.with({ scheme: 'dxlens' });
    }
    return uri;
}

/**
 * Debounce function
 */
export function debounce<T extends (...args: any[]) => any>(
    fn: T,
    delay: number
): (...args: Parameters<T>) => void {
    let timeout: NodeJS.Timeout | null = null;
    
    return (...args: Parameters<T>) => {
        if (timeout) {
            clearTimeout(timeout);
        }
        timeout = setTimeout(() => {
            fn(...args);
            timeout = null;
        }, delay);
    };
}
```

### `language-configuration.json`

```json
{
    "comments": {
        "lineComment": "//",
        "blockComment": ["/*", "*/"]
    },
    "brackets": [
        ["{", "}"],
        ["[", "]"],
        ["(", ")"]
    ],
    "autoClosingPairs": [
        { "open": "{", "close": "}" },
        { "open": "[", "close": "]" },
        { "open": "(", "close": ")" },
        { "open": "\"", "close": "\"", "notIn": ["string"] },
        { "open": "'", "close": "'", "notIn": ["string"] }
    ],
    "surroundingPairs": [
        ["{", "}"],
        ["[", "]"],
        ["(", ")"],
        ["\"", "\""],
        ["'", "'"]
    ],
    "folding": {
        "markers": {
            "start": "^\\s*//\\s*#?region\\b",
            "end": "^\\s*//\\s*#?endregion\\b"
        }
    },
    "wordPattern": "[\\w$]+",
    "indentationRules": {
        "increaseIndentPattern": "^.*[{\\[]\\s*$",
        "decreaseIndentPattern": "^\\s*[}\\]]"
    }
}
```

### `syntaxes/dx.tmLanguage.json`

```json
{
    "$schema": "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
    "name": "DX",
    "scopeName": "source.dx",
    "patterns": [
        { "include": "#comments" },
        { "include": "#strings" },
        { "include": "#numbers" },
        { "include": "#keywords" },
        { "include": "#punctuation" }
    ],
    "repository": {
        "comments": {
            "patterns": [
                {
                    "name": "comment.line.double-slash.dx",
                    "match": "//.*$"
                },
                {
                    "name": "comment.block.dx",
                    "begin": "/\\*",
                    "end": "\\*/"
                }
            ]
        },
        "strings": {
            "patterns": [
                {
                    "name": "string.quoted.double.dx",
                    "begin": "\"",
                    "end": "\"",
                    "patterns": [
                        {
                            "name": "constant.character.escape.dx",
                            "match": "\\\\."
                        }
                    ]
                },
                {
                    "name": "string.quoted.single.dx",
                    "begin": "'",
                    "end": "'",
                    "patterns": [
                        {
                            "name": "constant.character.escape.dx",
                            "match": "\\\\."
                        }
                    ]
                }
            ]
        },
        "numbers": {
            "patterns": [
                {
                    "name": "constant.numeric.dx",
                    "match": "-?\\b\\d+(\\.\\d+)?([eE][+-]?\\d+)?\\b"
                }
            ]
        },
        "keywords": {
            "patterns": [
                {
                    "name": "constant.language.dx",
                    "match": "\\b(true|false|null)\\b"
                }
            ]
        },
        "punctuation": {
            "patterns": [
                {
                    "name": "punctuation.definition.dictionary.begin.dx",
                    "match": "\\{"
                },
                {
                    "name": "punctuation.definition.dictionary.end.dx",
                    "match": "\\}"
                },
                {
                    "name": "punctuation.definition.array.begin.dx",
                    "match": "\\["
                },
                {
                    "name": "punctuation.definition.array.end.dx",
                    "match": "\\]"
                },
                {
                    "name": "punctuation.separator.dictionary.key-value.dx",
                    "match": ":"
                },
                {
                    "name": "punctuation.separator.dictionary.pair.dx",
                    "match": ","
                }
            ]
        }
    }
}
```

### `tsconfig.json`

```json
{
    "compilerOptions": {
        "module": "commonjs",
        "target": "ES2022",
        "lib": ["ES2022"],
        "outDir": "out",
        "rootDir": "src",
        "sourceMap": true,
        "strict": true,
        "esModuleInterop": true,
        "skipLibCheck": true,
        "forceConsistentCasingInFileNames": true,
        "resolveJsonModule": true
    },
    "include": ["src/**/*"],
    "exclude": ["node_modules", "out", "wasm"]
}
```

---

## 📋 Implementation Timeline

### Phase 1: Core Infrastructure (Week 1)
| Day | Task | Status |
|-----|------|--------|
| 1 | Set up project structure, package.json, tsconfig | ⬜ |
| 2 | Implement WASM bindings in Rust | ⬜ |
| 3 | Build WASM and integrate with extension | ⬜ |
| 4 | Implement DxDocumentManager | ⬜ |
| 5 | Implement DxLensFileSystem | ⬜ |

### Phase 2: Auto-Save & Validation (Week 2)
| Day | Task | Status |
|-----|------|--------|
| 6 | Implement validation-gated saves | ⬜ |
| 7 | Add grace period for auto-save | ⬜ |
| 8 | Test with VS Code auto-save modes | ⬜ |
| 9 | Implement smart quoting | ⬜ |
| 10 | Add diagnostics and error display | ⬜ |

### Phase 3: Polish & Testing (Week 3)
| Day | Task | Status |
|-----|------|--------|
| 11 | Add file icons (logo.png, file-extension-dark.png) | ⬜ |
| 12 | Test with Cursor/Copilot | ⬜ |
| 13 | Add commands and status bar | ⬜ |
| 14 | Write unit tests | ⬜ |
| 15 | Documentation and README | ⬜ |

---

## 🧪 Test Matrix

| Scenario | Expected Behavior | Test |
|----------|-------------------|------|
| Open .dx file | Shows human-readable format | ✅ |
| Save complete code | Writes dense to disk | ✅ |
| Auto-save incomplete | Skips write, shows warning | ✅ |
| Value with apostrophe | Uses double quotes | ✅ |
| Long URL | Preserved without truncation | ✅ |
| External file change | Updates editor view | ✅ |
| LLM reads file | Gets dense format | ✅ |
| Path in tab | Shows original filename | ✅ |

---

## 🔑 Key Bulletproof Features

### 1. Grace Period for Auto-Save
```typescript
// In saveDocument()
const timeSinceKeystroke = Date.now() - state.lastKeystroke;
if (timeSinceKeystroke < this.autoSaveGracePeriod) {
    return; // Don't save while user is typing
}
```

### 2. Value Preservation
```rust
// In smart_quote()
// Never truncate - always preserve full value
fn smart_quote(&self, value: &str) -> String {
    // URL, special chars - all preserved exactly
    format!("\"{}\"", value)
}
```

### 3. Smart Quoting for Apostrophes
```rust
if value.contains("'") {
    // Wrap with double quotes: don't → "don't"
    return format!("\"{}\"", value);
}
```

### 4. Strict File Matching
```typescript
// Only .dx, not .dx.json, .dx.yml, etc.
function isExactlyDxFile(uri: vscode.Uri): boolean {
    const ext = path.extname(uri.fsPath);
    return ext === '.dx';
}
```

This implementation ensures:
- ✅ Auto-save works correctly with grace period
- ✅ Values like "don't" are properly quoted
- ✅ URLs are never truncated
- ✅ Only `.dx` files are affected
- ✅ LLMs see dense format on disk
- ✅ Users see human format in editor
