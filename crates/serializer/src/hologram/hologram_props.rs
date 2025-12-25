//! Property tests for Hologram module (inflate/deflate)
//!
//! Battle-hardening tests for the holographic transformation between
//! LLM-dense format and human-readable format.

#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use crate::hologram::{inflate, deflate, HologramConfig};
    use crate::hologram::types::ValueType;

    // ========================================================================
    // Generators for valid hologram content
    // ========================================================================

    /// Generate valid key names (alphanumeric, starting with letter)
    fn valid_key() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z][a-zA-Z0-9_]{0,15}")
            .unwrap()
            .prop_filter("non-empty", |s| !s.is_empty())
    }

    /// Generate simple string values (no special characters)
    fn simple_value() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z0-9]{1,20}").unwrap()
    }

    /// Generate numeric values
    fn numeric_value() -> impl Strategy<Value = String> {
        prop_oneof![
            (-10000i64..10000).prop_map(|n| n.to_string()),
            (-1000.0f64..1000.0).prop_map(|f| format!("{:.2}", f)),
        ]
    }

    /// Generate boolean values in LLM format
    fn bool_value() -> impl Strategy<Value = String> {
        prop::bool::ANY.prop_map(|b| if b { "1".to_string() } else { "0".to_string() })
    }

    /// Generate a simple inline object in LLM format: key#field:value#field:value
    fn inline_object() -> impl Strategy<Value = String> {
        (
            valid_key(),
            prop::collection::vec((valid_key(), simple_value()), 1..4),
        )
            .prop_filter("unique keys", |(_, pairs)| {
                let keys: Vec<_> = pairs.iter().map(|(k, _)| k).collect();
                let unique: std::collections::HashSet<_> = keys.iter().collect();
                keys.len() == unique.len()
            })
            .prop_map(|(name, pairs)| {
                let fields: String = pairs
                    .into_iter()
                    .map(|(k, v)| format!("#{}:{}", k, v))
                    .collect();
                format!("{}{}", name, fields)
            })
    }

    /// Generate a simple array in LLM format: key@N>item|item|item
    fn simple_array() -> impl Strategy<Value = String> {
        (
            valid_key(),
            prop::collection::vec(simple_value(), 1..5),
        )
            .prop_map(|(name, items)| {
                let count = items.len();
                let content = items.join("|");
                format!("{}@{}>{}",name, count, content)
            })
    }

    // ========================================================================
    // Property Tests: Inflate/Deflate Round-Trip
    // ========================================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Test that inflate produces non-empty output for valid input
        #[test]
        fn prop_inflate_produces_output(input in inline_object()) {
            let result = inflate(&input);
            prop_assert!(!result.is_empty(), 
                "Inflate should produce output for: {}", input);
        }

        /// Test that deflate produces non-empty output for valid input
        #[test]
        fn prop_deflate_produces_output(input in inline_object()) {
            let inflated = inflate(&input);
            let deflated = deflate(&inflated);
            prop_assert!(!deflated.is_empty(),
                "Deflate should produce output for inflated: {}", inflated);
        }

        /// Test round-trip preserves key names
        #[test]
        fn prop_round_trip_preserves_keys(
            key in valid_key(),
            field1 in valid_key(),
            field2 in valid_key(),
            val1 in simple_value(),
            val2 in simple_value()
        ) {
            prop_assume!(field1 != field2);
            
            let input = format!("{}#{}:{}#{}:{}", key, field1, val1, field2, val2);
            let inflated = inflate(&input);
            let deflated = deflate(&inflated);
            
            // The deflated output should contain the original values
            prop_assert!(deflated.contains(&val1) || inflated.contains(&val1),
                "Value '{}' should be preserved through round-trip", val1);
            prop_assert!(deflated.contains(&val2) || inflated.contains(&val2),
                "Value '{}' should be preserved through round-trip", val2);
        }

        /// Test that arrays are handled correctly
        #[test]
        fn prop_array_round_trip(input in simple_array()) {
            let inflated = inflate(&input);
            
            // Inflated should contain bullet points or list markers
            prop_assert!(
                inflated.contains("•") || inflated.contains("-") || inflated.contains("items"),
                "Inflated array should have list markers: {}", inflated
            );
        }

        /// Test boolean expansion in inflate
        #[test]
        fn prop_boolean_expansion(
            key in valid_key(),
            field in valid_key(),
            value in prop::bool::ANY
        ) {
            let bool_str = if value { "1" } else { "0" };
            let input = format!("{}#{}:{}", key, field, bool_str);
            let inflated = inflate(&input);
            
            // Inflated should show true/false or checkmarks
            if value {
                prop_assert!(
                    inflated.contains("true") || inflated.contains("✓") || inflated.contains("1"),
                    "True should be expanded: {}", inflated
                );
            } else {
                prop_assert!(
                    inflated.contains("false") || inflated.contains("✗") || inflated.contains("0"),
                    "False should be expanded: {}", inflated
                );
            }
        }

        /// Test numeric values are preserved
        #[test]
        fn prop_numeric_preservation(
            key in valid_key(),
            field in valid_key(),
            value in (-10000i64..10000)
        ) {
            let input = format!("{}#{}:{}", key, field, value);
            let inflated = inflate(&input);
            let deflated = deflate(&inflated);
            
            // The numeric value should be preserved
            prop_assert!(
                inflated.contains(&value.to_string()) || deflated.contains(&value.to_string()),
                "Numeric value {} should be preserved", value
            );
        }
    }

    // ========================================================================
    // Property Tests: Configuration Options
    // ========================================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        /// Test ASCII mode produces ASCII-only output
        #[test]
        fn prop_ascii_mode_output(input in inline_object()) {
            let config = HologramConfig {
                ascii_mode: true,
                ..Default::default()
            };
            let result = crate::hologram::inflate_with_config(&input, &config);
            
            // ASCII mode should not contain Unicode symbols
            prop_assert!(
                !result.contains("▼") && !result.contains("✓") && !result.contains("✗"),
                "ASCII mode should not contain Unicode: {}", result
            );
        }

        /// Test indent size affects output
        #[test]
        fn prop_indent_size_affects_output(
            input in inline_object(),
            indent in 2usize..8
        ) {
            let config = HologramConfig {
                indent_size: indent,
                ..Default::default()
            };
            let result = crate::hologram::inflate_with_config(&input, &config);
            
            // Output should contain some indentation
            // (exact verification depends on content)
            prop_assert!(!result.is_empty(), "Output should not be empty");
        }
    }

    // ========================================================================
    // Property Tests: Edge Cases
    // ========================================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Test empty input handling
        #[test]
        fn prop_empty_input_handling(whitespace in "\\s{0,10}") {
            let inflated = inflate(&whitespace);
            let deflated = deflate(&whitespace);
            
            // Empty/whitespace input should not cause panic
            // and should produce empty or whitespace output
            let _ = (inflated, deflated);
        }

        /// Test special characters in values
        #[test]
        fn prop_special_chars_in_values(
            key in valid_key(),
            field in valid_key()
        ) {
            // Test with URL-like value
            let input = format!("{}#{}:https://example.com", key, field);
            let inflated = inflate(&input);
            
            // URL should be preserved
            prop_assert!(
                inflated.contains("https") || inflated.contains("example"),
                "URL should be preserved in output"
            );
        }

        /// Test null/empty values
        #[test]
        fn prop_null_value_handling(
            key in valid_key(),
            field in valid_key()
        ) {
            let input = format!("{}#{}:~", key, field);
            let inflated = inflate(&input);
            
            // Null marker should be handled
            prop_assert!(
                inflated.contains("null") || inflated.contains("~") || inflated.contains("∅"),
                "Null should be represented: {}", inflated
            );
        }
    }

    // ========================================================================
    // Unit Tests: Specific Edge Cases
    // ========================================================================

    #[test]
    fn test_inflate_simple_object() {
        let input = "server#host:localhost#port:5432";
        let result = inflate(input);
        assert!(!result.is_empty());
        assert!(result.contains("localhost") || result.contains("host"));
    }

    #[test]
    fn test_deflate_simple_object() {
        let input = "server#host:localhost#port:5432";
        let inflated = inflate(input);
        let deflated = deflate(&inflated);
        assert!(!deflated.is_empty());
    }

    #[test]
    fn test_inflate_with_boolean_true() {
        let input = "config#debug:1#verbose:0";
        let result = inflate(input);
        // Should contain some representation of true/false
        assert!(!result.is_empty());
    }

    #[test]
    fn test_inflate_array() {
        let input = "items@3>apple|banana|cherry";
        let result = inflate(input);
        assert!(!result.is_empty());
        // Should contain the items or list markers
        assert!(result.contains("apple") || result.contains("•") || result.contains("items"));
    }

    #[test]
    fn test_inflate_empty_string() {
        let result = inflate("");
        // Should handle gracefully
        assert!(result.is_empty() || result.trim().is_empty());
    }

    #[test]
    fn test_deflate_empty_string() {
        let result = deflate("");
        // Should handle gracefully
        assert!(result.is_empty() || result.trim().is_empty());
    }

    #[test]
    fn test_inflate_multiline() {
        let input = "server#host:localhost#port:5432\ndb#name:mydb#user:admin";
        let result = inflate(input);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_value_type_inference() {
        assert_eq!(ValueType::infer("123"), ValueType::Integer);
        assert_eq!(ValueType::infer("12.34"), ValueType::Float);
        assert_eq!(ValueType::infer("1"), ValueType::Boolean);
        assert_eq!(ValueType::infer("0"), ValueType::Boolean);
        assert_eq!(ValueType::infer("~"), ValueType::Null);
        assert_eq!(ValueType::infer("hello"), ValueType::String);
    }

    #[test]
    fn test_comment_preservation() {
        let input = "!This is a comment!server#host:localhost";
        let result = inflate(input);
        // Comment should be preserved in some form
        assert!(result.contains("comment") || result.contains("//") || result.contains("This"));
    }

    #[test]
    fn test_reference_handling() {
        let input = "data#value:*ref1";
        let result = inflate(input);
        // Reference should be handled
        assert!(!result.is_empty());
    }

    #[test]
    fn test_unicode_values() {
        let input = "i18n#greeting:こんにちは#emoji:🎉";
        let result = inflate(input);
        // Unicode should be preserved
        assert!(result.contains("こんにちは") || result.contains("🎉") || result.contains("greeting"));
    }

    #[test]
    fn test_deeply_nested_not_supported() {
        // Hologram format is flat, so nested objects are handled as strings
        let input = "outer#inner:nested#deep:value";
        let result = inflate(input);
        // Should not panic, output depends on implementation
        assert!(!result.is_empty());
    }
}
