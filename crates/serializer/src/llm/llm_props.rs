//! Property-based tests for LLM format round-trip
//!
//! **Feature: dx-serializer-llm-human, Property 1: LLM Format Round-Trip**
//! **Validates: Requirements 1.1-1.8, 2.1-2.7, 9.1**

#[cfg(test)]
mod property_tests {
    use crate::llm::parser::LlmParser;
    use crate::llm::serializer::LlmSerializer;
    use crate::llm::types::{DxDocument, DxLlmValue, DxSection};
    use proptest::prelude::*;
    use std::collections::HashMap;

    /// Generate a random DxLlmValue (non-recursive for simplicity)
    fn arb_simple_value() -> impl Strategy<Value = DxLlmValue> {
        prop_oneof![
            Just(DxLlmValue::Bool(true)),
            Just(DxLlmValue::Bool(false)),
            Just(DxLlmValue::Null),
            (-1000i64..1000i64).prop_map(|n| DxLlmValue::Num(n as f64)),
            "[a-zA-Z][a-zA-Z0-9_]{0,10}".prop_map(DxLlmValue::Str),
        ]
    }

    /// Generate a random key (valid identifier)
    fn arb_key() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_]{1,8}".prop_map(|s| s)
    }

    /// Generate a random section ID
    fn arb_section_id() -> impl Strategy<Value = char> {
        prop_oneof![
            Just('a'),
            Just('b'),
            Just('c'),
            Just('d'),
            Just('e'),
        ]
    }

    /// Generate a random context map
    fn arb_context() -> impl Strategy<Value = HashMap<String, DxLlmValue>> {
        proptest::collection::hash_map(arb_key(), arb_simple_value(), 0..5)
    }

    /// Generate a random section with consistent schema and rows
    fn arb_section() -> impl Strategy<Value = DxSection> {
        // Generate schema first, then rows that match the schema
        proptest::collection::vec(arb_key(), 1..4).prop_flat_map(|schema| {
            let schema_len = schema.len();
            let row_strategy = proptest::collection::vec(arb_simple_value(), schema_len..=schema_len);
            let rows_strategy = proptest::collection::vec(row_strategy, 0..5);
            
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
            proptest::collection::hash_map(arb_section_id(), arb_section(), 0..3),
        )
            .prop_map(|(context, sections)| {
                let mut doc = DxDocument::new();
                doc.context = context;
                doc.sections = sections;
                doc
            })
    }

    /// Compare two DxLlmValues for semantic equality
    /// (handles numeric precision and reference resolution)
    fn values_equal(a: &DxLlmValue, b: &DxLlmValue) -> bool {
        match (a, b) {
            (DxLlmValue::Num(x), DxLlmValue::Num(y)) => (x - y).abs() < 0.0001,
            (DxLlmValue::Str(x), DxLlmValue::Str(y)) => x == y,
            (DxLlmValue::Bool(x), DxLlmValue::Bool(y)) => x == y,
            (DxLlmValue::Null, DxLlmValue::Null) => true,
            (DxLlmValue::Arr(x), DxLlmValue::Arr(y)) => {
                x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| values_equal(a, b))
            }
            // References may be resolved to strings
            (DxLlmValue::Ref(_), DxLlmValue::Str(_)) => true,
            (DxLlmValue::Str(_), DxLlmValue::Ref(_)) => true,
            _ => false,
        }
    }

    /// Compare two documents for semantic equality
    /// Note: Keys may be abbreviated during serialization, so we compare
    /// using the abbreviation dictionary to normalize keys.
    fn documents_equal(a: &DxDocument, b: &DxDocument) -> bool {
        use crate::llm::abbrev::AbbrevDict;
        let abbrev = AbbrevDict::new();
        
        // Compare context - keys may be abbreviated
        if a.context.len() != b.context.len() {
            return false;
        }
        for (key_a, val_a) in &a.context {
            // Try to find the key in b, accounting for abbreviation
            let compressed_key = abbrev.compress(key_a);
            let val_b = b.context.get(key_a)
                .or_else(|| b.context.get(&compressed_key));
            
            if let Some(val_b) = val_b {
                if !values_equal(val_a, val_b) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Compare sections
        if a.sections.len() != b.sections.len() {
            return false;
        }
        for (id, section_a) in &a.sections {
            if let Some(section_b) = b.sections.get(id) {
                if section_a.rows.len() != section_b.rows.len() {
                    return false;
                }
                for (row_a, row_b) in section_a.rows.iter().zip(section_b.rows.iter()) {
                    if row_a.len() != row_b.len() {
                        return false;
                    }
                    for (val_a, val_b) in row_a.iter().zip(row_b.iter()) {
                        if !values_equal(val_a, val_b) {
                            return false;
                        }
                    }
                }
            } else {
                return false;
            }
        }

        true
    }


    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Property 1: LLM Format Round-Trip
        /// For any valid DxDocument, serializing to LLM format and parsing back
        /// SHALL produce a semantically equivalent DxDocument.
        ///
        /// **Feature: dx-serializer-llm-human, Property 1: LLM Format Round-Trip**
        /// **Validates: Requirements 1.1-1.8, 2.1-2.7, 9.1**
        #[test]
        fn prop_llm_round_trip(doc in arb_document()) {
            let serializer = LlmSerializer::new();
            
            // Serialize to LLM format
            let llm_string = serializer.serialize(&doc);
            
            // Parse back
            let parsed = LlmParser::parse(&llm_string);
            prop_assert!(parsed.is_ok(), "Failed to parse serialized LLM: {}", llm_string);
            
            let parsed_doc = parsed.unwrap();
            
            // Verify semantic equality
            prop_assert!(
                documents_equal(&doc, &parsed_doc),
                "Round-trip failed:\nOriginal: {:?}\nSerialized: {}\nParsed: {:?}",
                doc, llm_string, parsed_doc
            );
        }

        /// Property: Boolean values are preserved through round-trip
        ///
        /// **Feature: dx-serializer-llm-human, Property 1: LLM Format Round-Trip**
        /// **Validates: Requirements 1.5, 1.6, 2.4, 2.5**
        #[test]
        fn prop_boolean_round_trip(b in proptest::bool::ANY) {
            let serializer = LlmSerializer::new();
            let mut doc = DxDocument::new();
            doc.context.insert("flag".to_string(), DxLlmValue::Bool(b));
            
            let llm_string = serializer.serialize(&doc);
            let parsed = LlmParser::parse(&llm_string).unwrap();
            
            let parsed_value = parsed.context.get("flag").unwrap();
            prop_assert_eq!(parsed_value.as_bool(), Some(b));
        }

        /// Property: Null values are preserved through round-trip
        ///
        /// **Feature: dx-serializer-llm-human, Property 1: LLM Format Round-Trip**
        /// **Validates: Requirements 1.7, 2.6**
        #[test]
        fn prop_null_round_trip(_dummy in Just(())) {
            let serializer = LlmSerializer::new();
            let mut doc = DxDocument::new();
            doc.context.insert("empty".to_string(), DxLlmValue::Null);
            
            let llm_string = serializer.serialize(&doc);
            let parsed = LlmParser::parse(&llm_string).unwrap();
            
            let parsed_value = parsed.context.get("empty").unwrap();
            prop_assert!(parsed_value.is_null());
        }

        /// Property: Numeric values are preserved through round-trip
        ///
        /// **Feature: dx-serializer-llm-human, Property 1: LLM Format Round-Trip**
        /// **Validates: Requirements 1.1-1.8, 2.1-2.7**
        #[test]
        fn prop_numeric_round_trip(n in -10000i64..10000i64) {
            let serializer = LlmSerializer::new();
            let mut doc = DxDocument::new();
            // Use abbreviated key "num" since "number" gets compressed
            doc.context.insert("num".to_string(), DxLlmValue::Num(n as f64));
            
            let llm_string = serializer.serialize(&doc);
            let parsed = LlmParser::parse(&llm_string).unwrap();
            
            let parsed_value = parsed.context.get("num").unwrap();
            prop_assert_eq!(parsed_value.as_num(), Some(n as f64));
        }

        /// Property: String values are preserved through round-trip
        ///
        /// **Feature: dx-serializer-llm-human, Property 1: LLM Format Round-Trip**
        /// **Validates: Requirements 1.1-1.8, 2.1-2.7**
        #[test]
        fn prop_string_round_trip(s in "[a-zA-Z][a-zA-Z0-9_]{0,20}") {
            let serializer = LlmSerializer::new();
            let mut doc = DxDocument::new();
            // Use a key that won't be abbreviated (use the abbreviated form directly)
            doc.context.insert("txt".to_string(), DxLlmValue::Str(s.clone()));
            
            let llm_string = serializer.serialize(&doc);
            let parsed = LlmParser::parse(&llm_string).unwrap();
            
            // The key should remain "txt" since it's already abbreviated
            let parsed_value = parsed.context.get("txt").unwrap();
            prop_assert_eq!(parsed_value.as_str(), Some(s.as_str()));
        }
    }

    #[test]
    fn test_llm_round_trip_basic() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();
        
        doc.context.insert("name".to_string(), DxLlmValue::Str("Test".to_string()));
        doc.context.insert("count".to_string(), DxLlmValue::Num(42.0));
        doc.context.insert("active".to_string(), DxLlmValue::Bool(true));
        
        let mut section = DxSection::new(vec!["id".to_string(), "value".to_string()]);
        section.add_row(vec![DxLlmValue::Num(1.0), DxLlmValue::Str("Alpha".to_string())]).unwrap();
        section.add_row(vec![DxLlmValue::Num(2.0), DxLlmValue::Str("Beta".to_string())]).unwrap();
        doc.sections.insert('d', section);
        
        let llm_string = serializer.serialize(&doc);
        let parsed = LlmParser::parse(&llm_string).unwrap();
        
        assert!(documents_equal(&doc, &parsed));
    }

    #[test]
    fn test_special_values_round_trip() {
        let serializer = LlmSerializer::new();
        let mut doc = DxDocument::new();
        
        doc.context.insert("true_val".to_string(), DxLlmValue::Bool(true));
        doc.context.insert("false_val".to_string(), DxLlmValue::Bool(false));
        doc.context.insert("null_val".to_string(), DxLlmValue::Null);
        
        let llm_string = serializer.serialize(&doc);
        
        // Verify special characters are used
        assert!(llm_string.contains("|+"), "Should contain + for true");
        assert!(llm_string.contains("|-"), "Should contain - for false");
        assert!(llm_string.contains("|~"), "Should contain ~ for null");
        
        let parsed = LlmParser::parse(&llm_string).unwrap();
        
        assert_eq!(parsed.context.get("true_val").unwrap().as_bool(), Some(true));
        assert_eq!(parsed.context.get("false_val").unwrap().as_bool(), Some(false));
        assert!(parsed.context.get("null_val").unwrap().is_null());
    }
}
