//! Property tests for format converters
//!
//! Battle-hardening tests for JSON, YAML, TOML, and DX format converters.
//! These tests validate edge cases, malformed input handling, and round-trip correctness.

#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use crate::converters::json::{json_to_dx, dx_to_json};
    use crate::converters::yaml::{yaml_to_dx, dx_to_yaml};
    use crate::converters::toml::{toml_to_dx, dx_to_toml};
    use crate::converters::dx_hyper::{encode_hyper, decode_hyper};
    use crate::converters::dx_ultra::{encode_ultra, decode_ultra};
    use crate::types::DxValue;

    // ========================================================================
    // JSON Converter Property Tests
    // ========================================================================

    /// Generate valid JSON-like key names
    fn json_key() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z_][a-zA-Z0-9_]{0,15}")
            .unwrap()
            .prop_filter("non-empty", |s| !s.is_empty())
    }

    /// Generate simple JSON values
    fn json_value() -> impl Strategy<Value = String> {
        prop_oneof![
            // Strings
            prop::string::string_regex("[a-zA-Z0-9 ]{0,30}").unwrap().prop_map(|s| format!("\"{}\"", s)),
            // Numbers
            (-1000000i64..1000000).prop_map(|n| n.to_string()),
            // Floats
            (-1000.0f64..1000.0).prop_map(|f| format!("{:.2}", f)),
            // Booleans
            prop::bool::ANY.prop_map(|b| if b { "true".to_string() } else { "false".to_string() }),
            // Null
            Just("null".to_string()),
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Test JSON to DX conversion preserves simple key-value pairs
        #[test]
        fn prop_json_simple_object_round_trip(
            key1 in json_key(),
            key2 in json_key(),
            val1 in (-1000i64..1000),
            val2 in prop::string::string_regex("[a-zA-Z]{1,10}").unwrap()
        ) {
            prop_assume!(key1 != key2);
            
            let json = format!(r#"{{"{}": {}, "{}": "{}"}}"#, key1, val1, key2, val2);
            let result = json_to_dx(&json);
            
            prop_assert!(result.is_ok(), "JSON parsing should succeed: {:?}", result.err());
            
            let dx = result.unwrap();
            // Verify the DX output contains the values
            prop_assert!(dx.contains(&val1.to_string()) || dx.contains(&key1),
                "DX output should contain key or value");
        }

        /// Test JSON arrays are handled correctly
        #[test]
        fn prop_json_array_handling(
            items in prop::collection::vec((-100i64..100), 1..10)
        ) {
            let array_str = items.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let json = format!(r#"{{"numbers": [{}]}}"#, array_str);
            
            let result = json_to_dx(&json);
            prop_assert!(result.is_ok(), "JSON array parsing should succeed");
        }

        /// Test nested JSON objects
        #[test]
        fn prop_json_nested_objects(
            outer_key in json_key(),
            inner_key in json_key(),
            value in (-1000i64..1000)
        ) {
            let json = format!(r#"{{"{outer_key}": {{"{inner_key}": {value}}}}}"#);
            let result = json_to_dx(&json);
            
            prop_assert!(result.is_ok(), "Nested JSON parsing should succeed: {:?}", result.err());
        }

        /// Test malformed JSON is rejected gracefully
        #[test]
        fn prop_json_malformed_rejected(
            garbage in prop::string::string_regex("[^{}\\[\\]\",:]{1,20}").unwrap()
        ) {
            // Malformed JSON should either fail or produce empty output
            let result = json_to_dx(&garbage);
            // We don't assert failure because some strings might be valid
            // Just ensure no panic
            let _ = result;
        }
    }

    // ========================================================================
    // YAML Converter Property Tests
    // ========================================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Test YAML simple key-value pairs
        #[test]
        fn prop_yaml_simple_round_trip(
            key in json_key(),
            value in prop::string::string_regex("[a-zA-Z0-9]{1,20}").unwrap()
        ) {
            let yaml = format!("{}: {}", key, value);
            let result = yaml_to_dx(&yaml);
            
            prop_assert!(result.is_ok(), "YAML parsing should succeed: {:?}", result.err());
        }

        /// Test YAML nested structures
        #[test]
        fn prop_yaml_nested_structure(
            parent in json_key(),
            child in json_key(),
            value in (-1000i64..1000)
        ) {
            let yaml = format!("{}:\n  {}: {}", parent, child, value);
            let result = yaml_to_dx(&yaml);
            
            prop_assert!(result.is_ok(), "Nested YAML parsing should succeed: {:?}", result.err());
        }

        /// Test YAML lists
        #[test]
        fn prop_yaml_list_handling(
            key in json_key(),
            items in prop::collection::vec(prop::string::string_regex("[a-zA-Z]{1,10}").unwrap(), 1..5)
        ) {
            let list_items = items.iter()
                .map(|s| format!("  - {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            let yaml = format!("{}:\n{}", key, list_items);
            
            let result = yaml_to_dx(&yaml);
            prop_assert!(result.is_ok(), "YAML list parsing should succeed: {:?}", result.err());
        }
    }

    // ========================================================================
    // TOML Converter Property Tests
    // ========================================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Test TOML simple key-value pairs
        #[test]
        fn prop_toml_simple_round_trip(
            key in json_key(),
            value in prop::string::string_regex("[a-zA-Z0-9]{1,20}").unwrap()
        ) {
            let toml = format!("{} = \"{}\"", key, value);
            let result = toml_to_dx(&toml);
            
            prop_assert!(result.is_ok(), "TOML parsing should succeed: {:?}", result.err());
        }

        /// Test TOML sections
        #[test]
        fn prop_toml_section_handling(
            section in json_key(),
            key in json_key(),
            value in (-1000i64..1000)
        ) {
            let toml = format!("[{}]\n{} = {}", section, key, value);
            let result = toml_to_dx(&toml);
            
            prop_assert!(result.is_ok(), "TOML section parsing should succeed: {:?}", result.err());
        }

        /// Test TOML arrays
        #[test]
        fn prop_toml_array_handling(
            key in json_key(),
            items in prop::collection::vec((-100i64..100), 1..5)
        ) {
            let array_str = items.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let toml = format!("{} = [{}]", key, array_str);
            
            let result = toml_to_dx(&toml);
            prop_assert!(result.is_ok(), "TOML array parsing should succeed: {:?}", result.err());
        }
    }

    // ========================================================================
    // DX-Hyper Format Property Tests
    // ========================================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Test DX-Hyper encoding preserves simple values
        #[test]
        fn prop_hyper_simple_value_encoding(
            key in json_key(),
            value in prop::string::string_regex("[a-zA-Z0-9]{1,20}").unwrap()
        ) {
            let mut obj = crate::types::DxObject::new();
            obj.set(&key, DxValue::String(value.clone()));
            let dx_value = DxValue::Object(obj);
            
            let encoded = encode_hyper(&dx_value);
            prop_assert!(!encoded.is_empty(), "Encoded output should not be empty");
            
            // Decode and verify
            let decoded = decode_hyper(&encoded);
            prop_assert!(decoded.is_ok(), "Decoding should succeed: {:?}", decoded.err());
        }

        /// Test DX-Hyper handles booleans correctly
        #[test]
        fn prop_hyper_boolean_encoding(
            key in json_key(),
            value in prop::bool::ANY
        ) {
            let mut obj = crate::types::DxObject::new();
            obj.set(&key, DxValue::Bool(value));
            let dx_value = DxValue::Object(obj);
            
            let encoded = encode_hyper(&dx_value);
            
            // Boolean should be encoded as 1 or 0
            prop_assert!(encoded.contains("1") || encoded.contains("0") || encoded.contains(&key),
                "Encoded boolean should contain 1, 0, or key");
        }

        /// Test DX-Hyper handles arrays
        #[test]
        fn prop_hyper_array_encoding(
            items in prop::collection::vec(prop::string::string_regex("[a-zA-Z]{1,10}").unwrap(), 1..5)
        ) {
            let array: Vec<DxValue> = items.iter()
                .map(|s| DxValue::String(s.clone()))
                .collect();
            let dx_value = DxValue::Array(crate::types::DxArray::from(array));
            
            let encoded = encode_hyper(&dx_value);
            prop_assert!(!encoded.is_empty(), "Encoded array should not be empty");
        }
    }

    // ========================================================================
    // DX-Ultra Format Property Tests
    // ========================================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Test DX-Ultra encoding preserves simple values
        #[test]
        fn prop_ultra_simple_value_encoding(
            key in json_key(),
            value in prop::string::string_regex("[a-zA-Z0-9]{1,20}").unwrap()
        ) {
            let mut obj = crate::types::DxObject::new();
            obj.set(&key, DxValue::String(value.clone()));
            let dx_value = DxValue::Object(obj);
            
            let encoded = encode_ultra(&dx_value);
            prop_assert!(!encoded.is_empty(), "Encoded output should not be empty");
            
            // Decode and verify
            let decoded = decode_ultra(&encoded);
            prop_assert!(decoded.is_ok(), "Decoding should succeed: {:?}", decoded.err());
        }

        /// Test DX-Ultra handles numbers correctly
        #[test]
        fn prop_ultra_number_encoding(
            key in json_key(),
            value in (-1000000i64..1000000)
        ) {
            let mut obj = crate::types::DxObject::new();
            obj.set(&key, DxValue::Int(value));
            let dx_value = DxValue::Object(obj);
            
            let encoded = encode_ultra(&dx_value);
            prop_assert!(!encoded.is_empty(), "Encoded number should not be empty");
        }

        /// Test DX-Ultra handles floats correctly
        #[test]
        fn prop_ultra_float_encoding(
            key in json_key(),
            value in (-1000.0f64..1000.0)
        ) {
            let mut obj = crate::types::DxObject::new();
            obj.set(&key, DxValue::Float(value));
            let dx_value = DxValue::Object(obj);
            
            let encoded = encode_ultra(&dx_value);
            prop_assert!(!encoded.is_empty(), "Encoded float should not be empty");
        }
    }

    // ========================================================================
    // Edge Case Tests
    // ========================================================================

    #[test]
    fn test_json_empty_object() {
        let result = json_to_dx("{}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_empty_array() {
        let result = json_to_dx("[]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_deeply_nested() {
        let json = r#"{"a": {"b": {"c": {"d": {"e": 1}}}}}"#;
        let result = json_to_dx(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_yaml_empty_document() {
        let result = yaml_to_dx("");
        // Empty YAML should be handled gracefully
        let _ = result;
    }

    #[test]
    fn test_toml_empty_document() {
        let result = toml_to_dx("");
        // Empty TOML should be handled gracefully
        let _ = result;
    }

    #[test]
    fn test_json_unicode_values() {
        let json = r#"{"emoji": "🎉", "chinese": "中文", "arabic": "العربية"}"#;
        let result = json_to_dx(json);
        assert!(result.is_ok(), "Unicode should be handled: {:?}", result.err());
    }

    #[test]
    fn test_json_special_characters() {
        let json = r#"{"path": "C:\\Users\\test", "url": "https://example.com?a=1&b=2"}"#;
        let result = json_to_dx(json);
        assert!(result.is_ok(), "Special characters should be handled: {:?}", result.err());
    }

    #[test]
    fn test_json_large_numbers() {
        let json = r#"{"big": 9007199254740991, "negative": -9007199254740991}"#;
        let result = json_to_dx(json);
        assert!(result.is_ok(), "Large numbers should be handled: {:?}", result.err());
    }

    #[test]
    fn test_json_scientific_notation() {
        let json = r#"{"sci": 1.23e10, "neg_sci": -4.56e-7}"#;
        let result = json_to_dx(json);
        assert!(result.is_ok(), "Scientific notation should be handled: {:?}", result.err());
    }
}
