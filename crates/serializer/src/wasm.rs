//! WASM Bindings for DX Serializer VS Code Extension
//!
//! Provides the DxSerializer interface for the VS Code extension with:
//! - `to_human`: Transform dense format to human-readable format
//! - `to_dense`: Transform human-readable format to dense format
//! - `validate`: Validate content syntax with detailed error info
//! - `is_saveable`: Check if content is complete enough to save
//! - Security limits: max_input_size, max_recursion_depth, max_table_rows
//!
//! ## Usage from JavaScript
//!
//! ```javascript
//! import init, { DxSerializer, TransformResult, ValidationResult } from 'dx_serializer';
//!
//! await init();
//!
//! const serializer = new DxSerializer();
//!
//! // Transform dense to human (for editor display)
//! const result = serializer.toHuman('server#host:localhost#port:5432');
//! if (result.success) {
//!     console.log(result.content);
//! }
//!
//! // Transform human to dense (for disk storage)
//! const denseResult = serializer.toDense(humanContent);
//!
//! // Validate content
//! const validation = serializer.validate(content);
//! if (!validation.success) {
//!     console.log(`Error at line ${validation.line}: ${validation.error}`);
//!     console.log(`Hint: ${validation.hint}`);
//! }
//!
//! // Get security limits
//! console.log(`Max input size: ${serializer.maxInputSize()} bytes`);
//! console.log(`Max recursion depth: ${serializer.maxRecursionDepth()}`);
//! console.log(`Max table rows: ${serializer.maxTableRows()}`);
//! ```

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::error::{MAX_INPUT_SIZE, MAX_RECURSION_DEPTH, MAX_TABLE_ROWS};
use crate::llm::{human_to_llm, llm_to_human};

/// Result of a transformation operation
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct TransformResult {
    success: bool,
    content: String,
    error: Option<String>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl TransformResult {
    /// Whether the transformation succeeded
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn success(&self) -> bool {
        self.success
    }

    /// The transformed content (empty if failed)
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// Error message if transformation failed
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

impl TransformResult {
    /// Create a successful result
    pub fn ok(content: String) -> Self {
        Self {
            success: true,
            content,
            error: None,
        }
    }

    /// Create a failed result
    pub fn err(error: String) -> Self {
        Self {
            success: false,
            content: String::new(),
            error: Some(error),
        }
    }
}

/// Result of a validation operation
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct ValidationResult {
    success: bool,
    error: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
    hint: Option<String>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl ValidationResult {
    /// Whether the content is valid
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn success(&self) -> bool {
        self.success
    }

    /// Error message if validation failed
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }

    /// Line number where error occurred (1-indexed)
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn line(&self) -> Option<u32> {
        self.line
    }

    /// Column number where error occurred (1-indexed)
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn column(&self) -> Option<u32> {
        self.column
    }

    /// Actionable hint for fixing the error
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn hint(&self) -> Option<String> {
        self.hint.clone()
    }
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn valid() -> Self {
        Self {
            success: true,
            error: None,
            line: None,
            column: None,
            hint: None,
        }
    }

    /// Create a failed validation result
    pub fn invalid(error: String, line: u32, column: u32, hint: String) -> Self {
        Self {
            success: false,
            error: Some(error),
            line: Some(line),
            column: Some(column),
            hint: Some(hint),
        }
    }
}

/// Serializer configuration for the VS Code extension
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct SerializerConfig {
    /// Indentation size (2 or 4 spaces)
    indent_size: usize,
    /// Whether to preserve comments
    preserve_comments: bool,
    /// Whether to use smart quoting for special characters
    smart_quoting: bool,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl SerializerConfig {
    /// Create a new configuration with defaults
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            indent_size: 2,
            preserve_comments: true,
            smart_quoting: true,
        }
    }

    /// Set the indent size (2 or 4)
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = setIndentSize))]
    pub fn set_indent_size(&mut self, size: usize) {
        self.indent_size = if size == 4 { 4 } else { 2 };
    }

    /// Set whether to preserve comments
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = setPreserveComments))]
    pub fn set_preserve_comments(&mut self, preserve: bool) {
        self.preserve_comments = preserve;
    }

    /// Set whether to use smart quoting
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = setSmartQuoting))]
    pub fn set_smart_quoting(&mut self, smart: bool) {
        self.smart_quoting = smart;
    }
}

impl Default for SerializerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// DX Serializer for VS Code extension
///
/// Provides transformation between LLM (disk) and Human (editor) formats
/// with validation support. Uses the llm module for format conversion.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct DxSerializer {
    config: SerializerConfig,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl DxSerializer {
    /// Create a new DxSerializer with default configuration
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        let config = SerializerConfig::new();
        Self { config }
    }

    /// Create a DxSerializer with custom configuration
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = withConfig))]
    pub fn with_config(config: SerializerConfig) -> Self {
        Self { config }
    }

    /// Transform LLM format to human-readable format
    ///
    /// This is called when opening a .dx file in the editor.
    /// Converts sigil-based LLM format (#c, #:, #<letter>) to beautiful
    /// human-readable format with Unicode tables and expanded keys.
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = toHuman))]
    pub fn to_human(&self, llm_input: &str) -> TransformResult {
        // Handle empty input
        if llm_input.trim().is_empty() {
            return TransformResult::ok(String::new());
        }

        match llm_to_human(llm_input) {
            Ok(human) => TransformResult::ok(human),
            Err(e) => TransformResult::err(format!("Parse error: {}", e)),
        }
    }

    /// Transform human-readable format to LLM format
    ///
    /// This is called when saving a .dx file in the editor.
    /// Converts human-readable format back to token-optimized LLM format.
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = toDense))]
    pub fn to_dense(&self, human_input: &str) -> TransformResult {
        // Handle empty input
        if human_input.trim().is_empty() {
            return TransformResult::ok(String::new());
        }

        match human_to_llm(human_input) {
            Ok(llm) => TransformResult::ok(llm),
            Err(e) => TransformResult::err(format!("Parse error: {}", e)),
        }
    }

    /// Validate content syntax
    ///
    /// Returns detailed error information including line, column, and hints.
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn validate(&self, content: &str) -> ValidationResult {
        // Track bracket/quote state for validation
        let mut bracket_stack: Vec<(char, u32, u32)> = Vec::new();
        let mut in_string = false;
        let mut string_char = '"';
        let mut string_start: Option<(u32, u32)> = None;

        for (line_idx, line) in content.lines().enumerate() {
            let line_num = (line_idx + 1) as u32;
            let mut col = 0u32;
            let mut chars = line.chars().peekable();

            while let Some(ch) = chars.next() {
                col += 1;

                // Handle escape sequences in strings
                if in_string && ch == '\\' {
                    chars.next(); // Skip escaped character
                    col += 1;
                    continue;
                }

                // Handle string boundaries
                if !in_string && (ch == '"' || ch == '\'') {
                    in_string = true;
                    string_char = ch;
                    string_start = Some((line_num, col));
                    continue;
                }

                if in_string && ch == string_char {
                    in_string = false;
                    string_start = None;
                    continue;
                }

                // Skip bracket checking inside strings
                if in_string {
                    continue;
                }

                // Track brackets
                match ch {
                    '{' | '[' | '(' => {
                        bracket_stack.push((ch, line_num, col));
                    }
                    '}' | ']' | ')' => {
                        let expected = match ch {
                            '}' => '{',
                            ']' => '[',
                            ')' => '(',
                            _ => unreachable!(),
                        };

                        if let Some((open_char, open_line, open_col)) = bracket_stack.pop() {
                            if open_char != expected {
                                return ValidationResult::invalid(
                                    format!(
                                        "Mismatched bracket: expected '{}' but found '{}'",
                                        matching_close(open_char),
                                        ch
                                    ),
                                    line_num,
                                    col,
                                    format!(
                                        "Opening '{}' at line {}, column {} expects '{}'",
                                        open_char,
                                        open_line,
                                        open_col,
                                        matching_close(open_char)
                                    ),
                                );
                            }
                        } else {
                            return ValidationResult::invalid(
                                format!("Unexpected closing bracket '{}'", ch),
                                line_num,
                                col,
                                format!("No matching opening bracket for '{}'", ch),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        // Check for unclosed strings
        if in_string {
            if let Some((line, col)) = string_start {
                return ValidationResult::invalid(
                    format!("Unclosed string starting with '{}'", string_char),
                    line,
                    col,
                    format!(
                        "Add a closing '{}' to complete the string",
                        string_char
                    ),
                );
            }
        }

        // Check for unclosed brackets
        if let Some((ch, line, col)) = bracket_stack.pop() {
            return ValidationResult::invalid(
                format!("Unclosed bracket '{}'", ch),
                line,
                col,
                format!(
                    "Add a closing '{}' to match the opening '{}'",
                    matching_close(ch),
                    ch
                ),
            );
        }

        ValidationResult::valid()
    }

    /// Check if content is complete enough to save
    ///
    /// Returns true if the content has no unclosed brackets or strings.
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = isSaveable))]
    pub fn is_saveable(&self, content: &str) -> bool {
        self.validate(content).success
    }

    /// Get the maximum input size limit (100 MB)
    ///
    /// Files larger than this will be rejected to prevent memory exhaustion.
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = maxInputSize))]
    pub fn max_input_size(&self) -> usize {
        MAX_INPUT_SIZE
    }

    /// Get the maximum recursion depth limit (1000 levels)
    ///
    /// Structures nested deeper than this will be rejected to prevent stack overflow.
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = maxRecursionDepth))]
    pub fn max_recursion_depth(&self) -> usize {
        MAX_RECURSION_DEPTH
    }

    /// Get the maximum table rows limit (10 million rows)
    ///
    /// Tables with more rows than this will be rejected to prevent memory exhaustion.
    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = maxTableRows))]
    pub fn max_table_rows(&self) -> usize {
        MAX_TABLE_ROWS
    }
}

impl Default for DxSerializer {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the matching closing bracket for an opening bracket
fn matching_close(open: char) -> char {
    match open {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        _ => open,
    }
}

/// Apply smart quoting to a string value
///
/// - If string contains apostrophe ('), wrap in double quotes
/// - If string contains both ' and ", use double quotes with escaped "
pub fn smart_quote(value: &str) -> String {
    let has_single = value.contains('\'');
    let has_double = value.contains('"');

    if !has_single && !has_double {
        // No quotes needed for simple strings without spaces/special chars
        if !value.contains(' ')
            && !value.contains('#')
            && !value.contains('|')
            && !value.contains('^')
            && !value.contains(':')
        {
            return value.to_string();
        }
        // Default to double quotes
        return format!("\"{}\"", value);
    }

    if has_single && !has_double {
        // Contains apostrophe - use double quotes
        return format!("\"{}\"", value);
    }

    if has_double && !has_single {
        // Contains double quotes - use single quotes
        return format!("'{}'", value);
    }

    // Contains both - use double quotes with escaped double quotes
    let escaped = value.replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

/// Initialize WASM module
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn init_wasm() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Get version information
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "serializerVersion")]
pub fn serializer_version() -> String {
    format!(
        "dx-serializer v{} ({})",
        env!("CARGO_PKG_VERSION"),
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_result() {
        let ok = TransformResult::ok("content".to_string());
        assert!(ok.success());
        assert_eq!(ok.content(), "content");
        assert!(ok.error().is_none());

        let err = TransformResult::err("error".to_string());
        assert!(!err.success());
        assert!(err.content().is_empty());
        assert_eq!(err.error(), Some("error".to_string()));
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::valid();
        assert!(valid.success());
        assert!(valid.error().is_none());

        let invalid = ValidationResult::invalid(
            "error".to_string(),
            1,
            5,
            "hint".to_string(),
        );
        assert!(!invalid.success());
        assert_eq!(invalid.error(), Some("error".to_string()));
        assert_eq!(invalid.line(), Some(1));
        assert_eq!(invalid.column(), Some(5));
        assert_eq!(invalid.hint(), Some("hint".to_string()));
    }

    #[test]
    fn test_serializer_to_human() {
        let serializer = DxSerializer::new();
        // Use LLM format: #c: for context section with pipe-separated key|value pairs
        let result = serializer.to_human("#c:host|localhost;port|5432");
        assert!(result.success(), "to_human failed: {:?}", result.error());
        assert!(result.content().contains("host") || result.content().contains("localhost"));
    }

    #[test]
    fn test_serializer_to_dense() {
        let serializer = DxSerializer::new();
        // Use LLM format for input
        let human = serializer.to_human("#c:debug|+;prod|-");
        assert!(human.success(), "to_human failed: {:?}", human.error());

        // Then deflate back
        let dense = serializer.to_dense(&human.content());
        assert!(dense.success(), "to_dense failed: {:?}", dense.error());
        // The result should contain the LLM format sigil
        assert!(dense.content().contains("#c:") || dense.content().contains("debug"));
    }

    #[test]
    fn test_validate_valid_content() {
        let serializer = DxSerializer::new();
        let result = serializer.validate("key: value\nother: data");
        assert!(result.success());
    }

    #[test]
    fn test_validate_unclosed_bracket() {
        let serializer = DxSerializer::new();
        let result = serializer.validate("data: {\n  key: value");
        assert!(!result.success());
        assert!(result.error().unwrap().contains("Unclosed bracket"));
        assert_eq!(result.line(), Some(1));
        assert!(result.hint().is_some());
    }

    #[test]
    fn test_validate_unclosed_string() {
        let serializer = DxSerializer::new();
        let result = serializer.validate("key: \"unclosed string");
        assert!(!result.success());
        assert!(result.error().unwrap().contains("Unclosed string"));
        assert!(result.hint().is_some());
    }

    #[test]
    fn test_validate_mismatched_brackets() {
        let serializer = DxSerializer::new();
        let result = serializer.validate("data: [value}");
        assert!(!result.success());
        assert!(result.error().unwrap().contains("Mismatched bracket"));
    }

    #[test]
    fn test_is_saveable() {
        let serializer = DxSerializer::new();
        assert!(serializer.is_saveable("key: value"));
        assert!(!serializer.is_saveable("key: {unclosed"));
        assert!(!serializer.is_saveable("key: \"unclosed"));
    }

    #[test]
    fn test_smart_quote_simple() {
        assert_eq!(smart_quote("hello"), "hello");
        assert_eq!(smart_quote("hello world"), "\"hello world\"");
    }

    #[test]
    fn test_smart_quote_apostrophe() {
        // Strings with apostrophes should use double quotes
        assert_eq!(smart_quote("don't"), "\"don't\"");
        assert_eq!(smart_quote("it's working"), "\"it's working\"");
    }

    #[test]
    fn test_smart_quote_double_quotes() {
        // Strings with double quotes should use single quotes
        assert_eq!(smart_quote("say \"hello\""), "'say \"hello\"'");
    }

    #[test]
    fn test_smart_quote_both() {
        // Strings with both should escape double quotes
        assert_eq!(
            smart_quote("don't say \"hello\""),
            "\"don't say \\\"hello\\\"\""
        );
    }

    #[test]
    fn test_smart_quote_special_chars() {
        assert_eq!(smart_quote("key:value"), "\"key:value\"");
        assert_eq!(smart_quote("a|b|c"), "\"a|b|c\"");
        assert_eq!(smart_quote("a#b"), "\"a#b\"");
    }

    #[test]
    fn test_config() {
        let mut config = SerializerConfig::new();
        assert_eq!(config.indent_size, 2);

        config.set_indent_size(4);
        assert_eq!(config.indent_size, 4);

        config.set_indent_size(3); // Invalid, should default to 2
        assert_eq!(config.indent_size, 2);
    }

    #[test]
    fn test_empty_input() {
        let serializer = DxSerializer::new();

        let human = serializer.to_human("");
        assert!(human.success());
        assert!(human.content().is_empty());

        let dense = serializer.to_dense("");
        assert!(dense.success());
        assert!(dense.content().is_empty());
    }
}


#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Generators for valid LLM format content

    /// Generate a valid abbreviated key (2-3 lowercase letters)
    fn valid_abbrev_key() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-z]{2,3}")
            .unwrap()
            .prop_filter("non-empty key", |s| !s.is_empty())
    }

    /// Generate a simple value for LLM format
    fn simple_value() -> impl Strategy<Value = String> {
        prop_oneof![
            // Simple strings (alphanumeric)
            prop::string::string_regex("[a-zA-Z][a-zA-Z0-9]{0,15}").unwrap(),
            // Numbers
            (1i32..10000).prop_map(|n| n.to_string()),
        ]
    }

    /// Generate a boolean value in LLM format (+ or -)
    fn llm_bool() -> impl Strategy<Value = String> {
        prop::bool::ANY.prop_map(|b| if b { "+".to_string() } else { "-".to_string() })
    }

    /// Generate a context section in LLM format: #c:key|val;key|val
    fn llm_context_section() -> impl Strategy<Value = String> {
        prop::collection::vec(
            (valid_abbrev_key(), simple_value()),
            1..4,
        )
            .prop_map(|pairs| {
                let content: String = pairs
                    .into_iter()
                    .map(|(k, v)| format!("{}|{}", k, v))
                    .collect::<Vec<_>>()
                    .join(";");
                format!("#c:{}", content)
            })
    }

    /// Generate a data section in LLM format: #d(schema)\nrow\nrow
    fn llm_data_section() -> impl Strategy<Value = String> {
        (
            prop::string::string_regex("[a-z]").unwrap(), // section id (single letter)
            prop::collection::vec(valid_abbrev_key(), 2..4), // schema columns
            prop::collection::vec(simple_value(), 2..4), // row values
        )
            .prop_filter("schema and row same length", |(_, schema, row)| schema.len() == row.len())
            .prop_map(|(id, schema, row)| {
                let schema_str = schema.join("|");
                let row_str = row.join("|");
                format!("#{}({})\n{}", id, schema_str, row_str)
            })
    }

    /// Generate valid LLM format content
    fn valid_llm_content() -> impl Strategy<Value = String> {
        prop_oneof![
            llm_context_section(),
            llm_data_section(),
        ]
    }

    // Feature: dx-serializer-extension-fix, Property 1: LLM to Human to LLM Round-Trip
    // For any valid LLM format string, converting to human format and back to LLM format
    // SHALL produce a document with equivalent data.
    // **Validates: Requirements 1.1-1.9, 2.1-2.6, 3.1-3.5, 3.6**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_llm_round_trip_context(
            pairs in prop::collection::vec(
                (valid_abbrev_key(), simple_value()),
                1..3,
            )
        ) {
            // Skip if keys are duplicated
            let keys: Vec<_> = pairs.iter().map(|(k, _)| k.clone()).collect();
            let unique_keys: std::collections::HashSet<_> = keys.iter().collect();
            prop_assume!(keys.len() == unique_keys.len());

            let serializer = DxSerializer::new();
            let content: String = pairs
                .iter()
                .map(|(k, v)| format!("{}|{}", k, v))
                .collect::<Vec<_>>()
                .join(";");
            let llm = format!("#c:{}", content);

            // Transform to human
            let human_result = serializer.to_human(&llm);
            prop_assert!(human_result.success(), "to_human failed: {:?}", human_result.error());

            // Transform back to LLM
            let llm_result = serializer.to_dense(&human_result.content());
            prop_assert!(llm_result.success(), "to_dense failed: {:?}", llm_result.error());

            // Verify values are preserved
            let result = llm_result.content();
            for (_, value) in &pairs {
                prop_assert!(
                    result.contains(value),
                    "Value '{}' not found in result: '{}'", value, result
                );
            }
        }

        #[test]
        fn prop_llm_round_trip_booleans(
            key1 in valid_abbrev_key(),
            key2 in valid_abbrev_key(),
            bool1 in prop::bool::ANY,
            bool2 in prop::bool::ANY
        ) {
            prop_assume!(key1 != key2);

            let serializer = DxSerializer::new();
            let b1 = if bool1 { "+" } else { "-" };
            let b2 = if bool2 { "+" } else { "-" };
            let llm = format!("#c:{}|{};{}|{}", key1, b1, key2, b2);

            // Transform to human
            let human_result = serializer.to_human(&llm);
            prop_assert!(human_result.success(), "to_human failed: {:?}", human_result.error());

            // Human format should show true/false
            let human = human_result.content();
            if bool1 {
                prop_assert!(human.contains("true") || human.contains("✓"),
                    "Boolean true not found in human format: '{}'", human);
            } else {
                prop_assert!(human.contains("false") || human.contains("✗"),
                    "Boolean false not found in human format: '{}'", human);
            }

            // Transform back to LLM
            let llm_result = serializer.to_dense(&human);
            prop_assert!(llm_result.success(), "to_dense failed: {:?}", llm_result.error());

            // LLM format should have + or -
            let result = llm_result.content();
            prop_assert!(
                result.contains("|+") || result.contains("|-"),
                "Boolean markers not found in LLM result: '{}'", result
            );
        }

        #[test]
        fn prop_empty_content_round_trip(content in "\\s*") {
            let serializer = DxSerializer::new();

            let human_result = serializer.to_human(&content);
            prop_assert!(human_result.success());

            let dense_result = serializer.to_dense(&human_result.content());
            prop_assert!(dense_result.success());
        }
    }
}


#[cfg(test)]
mod string_preservation_tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: dx-serializer-extension, Property 3: String value preservation
    // For any string value (including URLs with query parameters, strings with
    // apostrophes, strings with both quote types, and strings with escape sequences),
    // transforming through the serializer SHALL preserve the exact string content.
    // **Validates: Requirements 2.1, 2.2, 2.3, 2.4, 2.5**

    /// Generate URL-like strings with query parameters
    fn url_string() -> impl Strategy<Value = String> {
        (
            prop::string::string_regex("https?://[a-z]+\\.[a-z]{2,4}").unwrap(),
            prop::string::string_regex("/[a-z]+").unwrap(),
            prop::collection::vec(
                (
                    prop::string::string_regex("[a-z]+").unwrap(),
                    prop::string::string_regex("[a-zA-Z0-9]+").unwrap(),
                ),
                0..3,
            ),
        )
            .prop_map(|(base, path, params)| {
                if params.is_empty() {
                    format!("{}{}", base, path)
                } else {
                    let query: String = params
                        .into_iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join("&");
                    format!("{}{}?{}", base, path, query)
                }
            })
    }

    /// Generate strings with apostrophes
    fn apostrophe_string() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("don't".to_string()),
            Just("it's".to_string()),
            Just("won't".to_string()),
            Just("can't".to_string()),
            Just("I'm".to_string()),
            prop::string::string_regex("[A-Z][a-z]+'s [a-z]+").unwrap(),
        ]
    }

    /// Generate strings with double quotes
    fn double_quote_string() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("say \"hello\"".to_string()),
            Just("the \"best\" way".to_string()),
            prop::string::string_regex("[a-z]+ \"[a-z]+\" [a-z]+").unwrap(),
        ]
    }

    /// Generate strings with both quote types
    fn mixed_quote_string() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("don't say \"hello\"".to_string()),
            Just("it's \"great\"".to_string()),
            Just("can't \"stop\"".to_string()),
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_url_preservation(url in url_string()) {
            // Test that URLs are preserved through smart_quote
            let quoted = smart_quote(&url);

            // Extract the content (remove quotes if present)
            let extracted = if quoted.starts_with('"') && quoted.ends_with('"') {
                quoted[1..quoted.len()-1].to_string()
            } else if quoted.starts_with('\'') && quoted.ends_with('\'') {
                quoted[1..quoted.len()-1].to_string()
            } else {
                quoted.clone()
            };

            prop_assert_eq!(
                url.clone(), extracted.clone(),
                "URL not preserved: original='{}', quoted='{}', extracted='{}'",
                url, quoted, extracted
            );
        }

        #[test]
        fn prop_apostrophe_uses_double_quotes(s in apostrophe_string()) {
            let quoted = smart_quote(&s);

            // Strings with apostrophes should use double quotes
            prop_assert!(
                quoted.starts_with('"') && quoted.ends_with('"'),
                "String with apostrophe should use double quotes: '{}' -> '{}'",
                s, quoted
            );

            // Content should be preserved
            let extracted = &quoted[1..quoted.len()-1];
            prop_assert_eq!(
                s.clone(), extracted.to_string(),
                "Apostrophe string not preserved: original='{}', extracted='{}'",
                s, extracted
            );
        }

        #[test]
        fn prop_double_quote_uses_single_quotes(s in double_quote_string()) {
            let quoted = smart_quote(&s);

            // Strings with double quotes should use single quotes
            prop_assert!(
                quoted.starts_with('\'') && quoted.ends_with('\''),
                "String with double quotes should use single quotes: '{}' -> '{}'",
                s, quoted
            );

            // Content should be preserved
            let extracted = &quoted[1..quoted.len()-1];
            prop_assert_eq!(
                s.clone(), extracted.to_string(),
                "Double quote string not preserved: original='{}', extracted='{}'",
                s, extracted
            );
        }

        #[test]
        fn prop_mixed_quotes_escapes_double(s in mixed_quote_string()) {
            let quoted = smart_quote(&s);

            // Should use double quotes with escaped internal double quotes
            prop_assert!(
                quoted.starts_with('"') && quoted.ends_with('"'),
                "Mixed quote string should use double quotes: '{}' -> '{}'",
                s, quoted
            );

            // Content should be preserved (after unescaping)
            let extracted = quoted[1..quoted.len()-1].replace("\\\"", "\"");
            prop_assert_eq!(
                s.clone(), extracted.clone(),
                "Mixed quote string not preserved: original='{}', extracted='{}'",
                s, extracted
            );
        }

        #[test]
        fn prop_simple_string_no_quotes(
            s in prop::string::string_regex("[a-zA-Z][a-zA-Z0-9]{0,15}").unwrap()
        ) {
            let quoted = smart_quote(&s);

            // Simple strings without special chars should not be quoted
            prop_assert_eq!(
                s.clone(), quoted.clone(),
                "Simple string should not be quoted: '{}' -> '{}'",
                s, quoted
            );
        }

        #[test]
        fn prop_string_with_spaces_quoted(
            word1 in prop::string::string_regex("[a-z]+").unwrap(),
            word2 in prop::string::string_regex("[a-z]+").unwrap()
        ) {
            let s = format!("{} {}", word1, word2);
            let quoted = smart_quote(&s);

            // Strings with spaces should be quoted
            prop_assert!(
                (quoted.starts_with('"') && quoted.ends_with('"')) ||
                (quoted.starts_with('\'') && quoted.ends_with('\'')),
                "String with spaces should be quoted: '{}' -> '{}'",
                s, quoted
            );
        }

        #[test]
        fn prop_special_chars_quoted(
            prefix in prop::string::string_regex("[a-z]+").unwrap(),
            suffix in prop::string::string_regex("[a-z]+").unwrap(),
            special in prop::sample::select(vec!['#', '|', '^', ':'])
        ) {
            let s = format!("{}{}{}", prefix, special, suffix);
            let quoted = smart_quote(&s);

            // Strings with special DX chars should be quoted
            prop_assert!(
                (quoted.starts_with('"') && quoted.ends_with('"')) ||
                (quoted.starts_with('\'') && quoted.ends_with('\'')),
                "String with special char '{}' should be quoted: '{}' -> '{}'",
                special, s, quoted
            );
        }
    }
}
