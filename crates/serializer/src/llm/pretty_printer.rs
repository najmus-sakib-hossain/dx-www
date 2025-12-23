//! Pretty Printer for Human Format V2
//!
//! Provides a validated formatter that ensures output is always parseable.
//! The PrettyPrinter wraps HumanFormatterV2 and validates the output
//! by parsing it back to ensure round-trip consistency.
//!
//! ## Example
//!
//! ```rust
//! use serializer::llm::pretty_printer::PrettyPrinter;
//! use serializer::llm::types::DxDocument;
//!
//! let printer = PrettyPrinter::new();
//! let doc = DxDocument::new();
//! let output = printer.format(&doc).unwrap();
//! // Output is guaranteed to be parseable
//! ```

use crate::llm::human_formatter_v2::{HumanFormatV2Config, HumanFormatterV2};
use crate::llm::human_parser::HumanParser;
use crate::llm::types::DxDocument;
use thiserror::Error;

/// Errors that can occur during pretty printing
#[derive(Debug, Error)]
pub enum PrettyPrintError {
    /// Output validation failed - the formatted output could not be parsed back
    #[error("Output validation failed: {msg}")]
    ValidationFailed { msg: String },

    /// Round-trip consistency check failed
    #[error("Round-trip consistency failed: {msg}")]
    RoundTripFailed { msg: String },
}

/// Configuration for the PrettyPrinter
#[derive(Debug, Clone)]
pub struct PrettyPrinterConfig {
    /// Underlying formatter config
    pub formatter_config: HumanFormatV2Config,
    /// Validate output by parsing it back
    pub validate_output: bool,
    /// Check round-trip consistency (requires validate_output)
    pub check_round_trip: bool,
}

impl Default for PrettyPrinterConfig {
    fn default() -> Self {
        Self {
            formatter_config: HumanFormatV2Config::default(),
            validate_output: true,
            check_round_trip: true,
        }
    }
}

impl PrettyPrinterConfig {
    /// Create a new config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the max line width
    pub fn with_max_width(mut self, width: usize) -> Self {
        self.formatter_config.max_line_width = width;
        self
    }

    /// Set whether to expand keys
    pub fn with_expand_keys(mut self, expand: bool) -> Self {
        self.formatter_config.expand_keys = expand;
        self
    }

    /// Set whether to validate output
    pub fn with_validation(mut self, validate: bool) -> Self {
        self.validate_output = validate;
        self
    }

    /// Set whether to check round-trip consistency
    pub fn with_round_trip_check(mut self, check: bool) -> Self {
        self.check_round_trip = check;
        self
    }
}

/// Pretty Printer that wraps HumanFormatterV2 with validation
///
/// The PrettyPrinter ensures that all output is valid and parseable.
/// It optionally validates the output by parsing it back and checking
/// for round-trip consistency.
pub struct PrettyPrinter {
    config: PrettyPrinterConfig,
    formatter: HumanFormatterV2,
    parser: HumanParser,
}

impl PrettyPrinter {
    /// Create a new PrettyPrinter with default config
    pub fn new() -> Self {
        let config = PrettyPrinterConfig::default();
        Self {
            formatter: HumanFormatterV2::with_config(config.formatter_config.clone()),
            parser: HumanParser::new(),
            config,
        }
    }

    /// Create a PrettyPrinter with custom config
    pub fn with_config(config: PrettyPrinterConfig) -> Self {
        Self {
            formatter: HumanFormatterV2::with_config(config.formatter_config.clone()),
            parser: HumanParser::new(),
            config,
        }
    }

    /// Format a DxDocument to a pretty-printed Human format V2 string
    ///
    /// If validation is enabled, the output is parsed back to ensure
    /// it is valid. If round-trip checking is enabled, the parsed
    /// document is compared to the original.
    pub fn format(&self, doc: &DxDocument) -> Result<String, PrettyPrintError> {
        // Format the document
        let output = self.formatter.format(doc);

        // Validate if enabled
        if self.config.validate_output {
            self.validate_output(&output, doc)?;
        }

        Ok(output)
    }

    /// Format a DxDocument without validation (faster but no guarantees)
    pub fn format_unchecked(&self, doc: &DxDocument) -> String {
        self.formatter.format(doc)
    }

    /// Validate that the output can be parsed back
    fn validate_output(&self, output: &str, original: &DxDocument) -> Result<(), PrettyPrintError> {
        // Try to parse the output
        let parsed = self.parser.parse(output).map_err(|e| {
            PrettyPrintError::ValidationFailed {
                msg: format!("Failed to parse formatted output: {}", e),
            }
        })?;

        // Check round-trip consistency if enabled
        if self.config.check_round_trip {
            self.check_round_trip(original, &parsed)?;
        }

        Ok(())
    }

    /// Check that the parsed document matches the original
    fn check_round_trip(
        &self,
        original: &DxDocument,
        parsed: &DxDocument,
    ) -> Result<(), PrettyPrintError> {
        // Check context
        if original.context.len() != parsed.context.len() {
            return Err(PrettyPrintError::RoundTripFailed {
                msg: format!(
                    "Context size mismatch: original={}, parsed={}",
                    original.context.len(),
                    parsed.context.len()
                ),
            });
        }

        // Check that all context values match
        for (key, value) in &original.context {
            if let Some(parsed_value) = parsed.context.get(key) {
                if !values_equal(value, parsed_value) {
                    return Err(PrettyPrintError::RoundTripFailed {
                        msg: format!(
                            "Context value mismatch for key '{}': original={:?}, parsed={:?}",
                            key, value, parsed_value
                        ),
                    });
                }
            } else {
                return Err(PrettyPrintError::RoundTripFailed {
                    msg: format!("Context key '{}' missing in parsed document", key),
                });
            }
        }

        // Check sections
        if original.sections.len() != parsed.sections.len() {
            return Err(PrettyPrintError::RoundTripFailed {
                msg: format!(
                    "Section count mismatch: original={}, parsed={}",
                    original.sections.len(),
                    parsed.sections.len()
                ),
            });
        }

        for (id, section) in &original.sections {
            if let Some(parsed_section) = parsed.sections.get(id) {
                // Check schema
                if section.schema != parsed_section.schema {
                    return Err(PrettyPrintError::RoundTripFailed {
                        msg: format!(
                            "Schema mismatch for section '{}': original={:?}, parsed={:?}",
                            id, section.schema, parsed_section.schema
                        ),
                    });
                }

                // Check row count
                if section.rows.len() != parsed_section.rows.len() {
                    return Err(PrettyPrintError::RoundTripFailed {
                        msg: format!(
                            "Row count mismatch for section '{}': original={}, parsed={}",
                            id,
                            section.rows.len(),
                            parsed_section.rows.len()
                        ),
                    });
                }

                // Check each row
                for (row_idx, (orig_row, parsed_row)) in
                    section.rows.iter().zip(parsed_section.rows.iter()).enumerate()
                {
                    if orig_row.len() != parsed_row.len() {
                        return Err(PrettyPrintError::RoundTripFailed {
                            msg: format!(
                                "Column count mismatch in section '{}' row {}: original={}, parsed={}",
                                id, row_idx, orig_row.len(), parsed_row.len()
                            ),
                        });
                    }

                    for (col_idx, (orig_val, parsed_val)) in
                        orig_row.iter().zip(parsed_row.iter()).enumerate()
                    {
                        if !values_equal(orig_val, parsed_val) {
                            return Err(PrettyPrintError::RoundTripFailed {
                                msg: format!(
                                    "Value mismatch in section '{}' row {} col {}: original={:?}, parsed={:?}",
                                    id, row_idx, col_idx, orig_val, parsed_val
                                ),
                            });
                        }
                    }
                }
            } else {
                return Err(PrettyPrintError::RoundTripFailed {
                    msg: format!("Section '{}' missing in parsed document", id),
                });
            }
        }

        Ok(())
    }

    /// Get the config
    pub fn config(&self) -> &PrettyPrinterConfig {
        &self.config
    }
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Compare two DxLlmValue instances for equality
fn values_equal(a: &crate::llm::types::DxLlmValue, b: &crate::llm::types::DxLlmValue) -> bool {
    use crate::llm::types::DxLlmValue;

    match (a, b) {
        (DxLlmValue::Str(s1), DxLlmValue::Str(s2)) => s1 == s2,
        (DxLlmValue::Num(n1), DxLlmValue::Num(n2)) => {
            // Handle floating point comparison
            (n1 - n2).abs() < f64::EPSILON || (n1.is_nan() && n2.is_nan())
        }
        (DxLlmValue::Bool(b1), DxLlmValue::Bool(b2)) => b1 == b2,
        (DxLlmValue::Null, DxLlmValue::Null) => true,
        (DxLlmValue::Ref(r1), DxLlmValue::Ref(r2)) => r1 == r2,
        (DxLlmValue::Arr(arr1), DxLlmValue::Arr(arr2)) => {
            if arr1.len() != arr2.len() {
                return false;
            }
            arr1.iter().zip(arr2.iter()).all(|(v1, v2)| values_equal(v1, v2))
        }
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::types::{DxLlmValue, DxSection};

    #[test]
    fn test_pretty_printer_empty_document() {
        let printer = PrettyPrinter::new();
        let doc = DxDocument::new();
        let result = printer.format(&doc);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_pretty_printer_with_config() {
        let printer = PrettyPrinter::new();
        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));
        doc.context.insert("ct".to_string(), DxLlmValue::Num(42.0));

        let result = printer.format(&doc);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("[config]"));
        assert!(output.contains("name")); // Expanded key
        assert!(output.contains("Test"));
    }

    #[test]
    fn test_pretty_printer_with_section() {
        let printer = PrettyPrinter::new();
        let mut doc = DxDocument::new();

        let mut section = DxSection::new(vec!["id".to_string(), "nm".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Str("Alpha".to_string()),
        ]);
        section.rows.push(vec![
            DxLlmValue::Num(2.0),
            DxLlmValue::Str("Beta".to_string()),
        ]);
        doc.sections.insert('d', section);

        let result = printer.format(&doc);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("[data]"));
        assert!(output.contains("Alpha"));
        assert!(output.contains("Beta"));
    }

    #[test]
    fn test_pretty_printer_round_trip() {
        let printer = PrettyPrinter::new();
        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));
        doc.context.insert("ct".to_string(), DxLlmValue::Num(42.0));
        doc.context.insert("ac".to_string(), DxLlmValue::Bool(true));

        let mut section = DxSection::new(vec!["id".to_string(), "vl".to_string()]);
        section.rows.push(vec![
            DxLlmValue::Num(1.0),
            DxLlmValue::Str("Alpha".to_string()),
        ]);
        doc.sections.insert('d', section);

        // Format and validate
        let result = printer.format(&doc);
        assert!(result.is_ok(), "Pretty printer should succeed: {:?}", result.err());
    }

    #[test]
    fn test_pretty_printer_unchecked() {
        let printer = PrettyPrinter::new();
        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));

        // Unchecked format should always succeed
        let output = printer.format_unchecked(&doc);
        assert!(output.contains("name"));
        assert!(output.contains("Test"));
    }

    #[test]
    fn test_pretty_printer_no_validation() {
        let config = PrettyPrinterConfig::new()
            .with_validation(false);
        let printer = PrettyPrinter::with_config(config);

        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));

        let result = printer.format(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pretty_printer_with_arrays() {
        let printer = PrettyPrinter::new();
        let mut doc = DxDocument::new();
        doc.context.insert(
            "ws".to_string(),
            DxLlmValue::Arr(vec![
                DxLlmValue::Str("frontend/www".to_string()),
                DxLlmValue::Str("frontend/mobile".to_string()),
            ]),
        );

        let result = printer.format(&doc);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("frontend/www, frontend/mobile"));
    }

    #[test]
    fn test_values_equal() {
        // Strings
        assert!(values_equal(
            &DxLlmValue::Str("test".to_string()),
            &DxLlmValue::Str("test".to_string())
        ));
        assert!(!values_equal(
            &DxLlmValue::Str("test".to_string()),
            &DxLlmValue::Str("other".to_string())
        ));

        // Numbers
        assert!(values_equal(&DxLlmValue::Num(42.0), &DxLlmValue::Num(42.0)));
        assert!(!values_equal(&DxLlmValue::Num(42.0), &DxLlmValue::Num(43.0)));

        // Booleans
        assert!(values_equal(&DxLlmValue::Bool(true), &DxLlmValue::Bool(true)));
        assert!(!values_equal(&DxLlmValue::Bool(true), &DxLlmValue::Bool(false)));

        // Null
        assert!(values_equal(&DxLlmValue::Null, &DxLlmValue::Null));

        // Arrays
        assert!(values_equal(
            &DxLlmValue::Arr(vec![DxLlmValue::Num(1.0), DxLlmValue::Num(2.0)]),
            &DxLlmValue::Arr(vec![DxLlmValue::Num(1.0), DxLlmValue::Num(2.0)])
        ));
        assert!(!values_equal(
            &DxLlmValue::Arr(vec![DxLlmValue::Num(1.0)]),
            &DxLlmValue::Arr(vec![DxLlmValue::Num(2.0)])
        ));

        // Different types
        assert!(!values_equal(&DxLlmValue::Num(42.0), &DxLlmValue::Str("42".to_string())));
    }
}

/// Property-based tests for PrettyPrinter
///
/// **Feature: dx-serializer-human-format-v2, Property 11: Pretty printer round-trip**
/// **Validates: Requirements 10.1, 10.2, 10.3**
#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::llm::types::{DxLlmValue, DxSection};
    use proptest::prelude::*;
    use std::collections::HashMap;

    /// Generate a random DxLlmValue (non-recursive for simplicity)
    fn arb_simple_value() -> impl Strategy<Value = DxLlmValue> {
        prop_oneof![
            Just(DxLlmValue::Bool(true)),
            Just(DxLlmValue::Bool(false)),
            Just(DxLlmValue::Null),
            (-1000i64..1000i64).prop_map(|n| DxLlmValue::Num(n as f64)),
            "[a-zA-Z][a-zA-Z0-9]{0,10}".prop_map(DxLlmValue::Str),
        ]
    }

    /// Generate a random key (valid identifier, using abbreviated forms)
    fn arb_key() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("nm".to_string()),
            Just("tt".to_string()),
            Just("ds".to_string()),
            Just("st".to_string()),
            Just("ct".to_string()),
            Just("ac".to_string()),
            Just("id".to_string()),
            Just("vl".to_string()),
        ]
    }

    /// Generate a random section ID
    fn arb_section_id() -> impl Strategy<Value = char> {
        prop_oneof![
            Just('d'),
            Just('f'),
            Just('o'),
            Just('p'),
            Just('u'),
        ]
    }

    /// Generate a random context map
    fn arb_context() -> impl Strategy<Value = HashMap<String, DxLlmValue>> {
        proptest::collection::hash_map(arb_key(), arb_simple_value(), 0..4)
    }

    /// Generate a random section with consistent schema and rows
    fn arb_section() -> impl Strategy<Value = DxSection> {
        proptest::collection::vec(arb_key(), 1..4).prop_flat_map(|schema| {
            let schema_len = schema.len();
            let row_strategy = proptest::collection::vec(arb_simple_value(), schema_len..=schema_len);
            let rows_strategy = proptest::collection::vec(row_strategy, 0..4);
            
            rows_strategy.prop_map(move |rows| {
                let mut section = DxSection::new(schema.clone());
                for row in rows {
                    let _ = section.add_row(row);
                }
                section
            })
        })
    }

    /// Generate a random DxDocument
    fn arb_document() -> impl Strategy<Value = DxDocument> {
        (
            arb_context(),
            proptest::collection::hash_map(arb_section_id(), arb_section(), 0..2),
        )
            .prop_map(|(context, sections)| {
                let mut doc = DxDocument::new();
                doc.context = context;
                doc.sections = sections;
                doc
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Property 11: Pretty Printer Round-Trip
        /// For any valid DxDocument, formatting with the Pretty_Printer and then
        /// parsing with the Human_Parser SHALL produce an equivalent document.
        ///
        /// **Feature: dx-serializer-human-format-v2, Property 11: Pretty printer round-trip**
        /// **Validates: Requirements 10.1, 10.2, 10.3**
        #[test]
        fn prop_pretty_printer_round_trip(doc in arb_document()) {
            let printer = PrettyPrinter::new();
            
            // Format with validation (this already checks round-trip internally)
            let result = printer.format(&doc);
            
            // The format should succeed for all valid documents
            prop_assert!(
                result.is_ok(),
                "PrettyPrinter should succeed for valid document: {:?}\nError: {:?}",
                doc, result.err()
            );
        }

        /// Property: PrettyPrinter output is always parseable
        ///
        /// **Feature: dx-serializer-human-format-v2, Property 11: Pretty printer round-trip**
        /// **Validates: Requirements 10.1, 10.2**
        #[test]
        fn prop_pretty_printer_output_parseable(doc in arb_document()) {
            let printer = PrettyPrinter::with_config(
                PrettyPrinterConfig::new()
                    .with_validation(false) // Don't validate internally
            );
            let parser = crate::llm::human_parser::HumanParser::new();
            
            // Format without validation
            let output = printer.format(&doc).unwrap();
            
            // Output should always be parseable
            let parsed = parser.parse(&output);
            prop_assert!(
                parsed.is_ok(),
                "PrettyPrinter output should be parseable:\nOutput: {}\nError: {:?}",
                output, parsed.err()
            );
        }

        /// Property: PrettyPrinter preserves context values
        ///
        /// **Feature: dx-serializer-human-format-v2, Property 11: Pretty printer round-trip**
        /// **Validates: Requirements 10.1, 10.2, 10.3**
        #[test]
        fn prop_pretty_printer_preserves_context(context in arb_context()) {
            let printer = PrettyPrinter::new();
            let parser = crate::llm::human_parser::HumanParser::new();
            
            let mut doc = DxDocument::new();
            doc.context = context.clone();
            
            let output = printer.format(&doc).unwrap();
            let parsed = parser.parse(&output).unwrap();
            
            // All context values should be preserved
            prop_assert_eq!(
                doc.context.len(),
                parsed.context.len(),
                "Context size should be preserved"
            );
        }

        /// Property: PrettyPrinter preserves section data
        ///
        /// **Feature: dx-serializer-human-format-v2, Property 11: Pretty printer round-trip**
        /// **Validates: Requirements 10.1, 10.2, 10.3**
        #[test]
        fn prop_pretty_printer_preserves_sections(section in arb_section()) {
            let printer = PrettyPrinter::new();
            let parser = crate::llm::human_parser::HumanParser::new();
            
            let mut doc = DxDocument::new();
            doc.sections.insert('d', section.clone());
            
            let output = printer.format(&doc).unwrap();
            let parsed = parser.parse(&output).unwrap();
            
            // Section should be preserved
            prop_assert!(
                parsed.sections.contains_key(&'d'),
                "Section 'd' should be preserved"
            );
            
            let parsed_section = parsed.sections.get(&'d').unwrap();
            prop_assert_eq!(
                section.rows.len(),
                parsed_section.rows.len(),
                "Row count should be preserved"
            );
        }
    }
}
