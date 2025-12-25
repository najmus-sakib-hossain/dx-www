//! Battle Hardening Tests for DX Serializer
//!
//! These tests probe edge cases, boundary conditions, and potential weaknesses
//! to ensure the serializer is production-ready and robust.

use serializer::{format_machine, parse, Mappings};

/// Helper to parse string input
fn parse_str(input: &str) -> serializer::Result<serializer::DxValue> {
    parse(input.as_bytes())
}

// ============================================================================
// PARSER EDGE CASES
// ============================================================================

mod parser_edge_cases {
    use super::*;

    #[test]
    fn test_empty_input() {
        let result = parse(b"");
        assert!(result.is_ok(), "Empty input should parse successfully");
    }

    #[test]
    fn test_whitespace_only() {
        let result = parse(b"   \n\t\n   ");
        assert!(result.is_ok(), "Whitespace-only input should parse");
    }

    #[test]
    fn test_comments_only() {
        let result = parse(b"# comment 1\n# comment 2\n# comment 3");
        assert!(result.is_ok(), "Comments-only input should parse");
    }

    #[test]
    fn test_very_long_key() {
        let long_key = "a".repeat(10000);
        let input = format!("{}:value", long_key);
        let result = parse_str(&input);
        assert!(result.is_ok(), "Very long keys should be handled");
    }

    #[test]
    fn test_very_long_value() {
        let long_value = "x".repeat(100000);
        let input = format!("key:{}", long_value);
        let result = parse_str(&input);
        assert!(result.is_ok(), "Very long values should be handled");
    }

    #[test]
    fn test_deeply_nested_keys() {
        // Test deeply nested dotted keys
        let deep_key = (0..100).map(|i| format!("level{}", i)).collect::<Vec<_>>().join(".");
        let input = format!("{}:value", deep_key);
        let result = parse_str(&input);
        assert!(result.is_ok(), "Deeply nested keys should be handled");
    }

    #[test]
    fn test_unicode_keys() {
        let input = "名前:太郎\nバージョン:1.0";
        let result = parse_str(input);
        assert!(result.is_ok(), "Unicode keys should be handled");
    }

    #[test]
    fn test_unicode_values() {
        let input = "name:日本語テスト\nemoji:🎉🚀✨";
        let result = parse_str(input);
        assert!(result.is_ok(), "Unicode values should be handled");
    }

    #[test]
    fn test_mixed_line_endings() {
        let input = "key1:val1\r\nkey2:val2\nkey3:val3\rkey4:val4";
        let result = parse_str(input);
        assert!(result.is_ok(), "Mixed line endings should be handled");
    }

    #[test]
    fn test_trailing_whitespace() {
        let input = "key:value   \n   key2:value2   ";
        let result = parse_str(input);
        assert!(result.is_ok(), "Trailing whitespace should be handled");
    }

    #[test]
    fn test_multiple_colons_in_value() {
        let input = "url:https://example.com:8080/path";
        let result = parse_str(input);
        assert!(result.is_ok(), "Multiple colons in value should be handled");
    }

    #[test]
    fn test_special_characters_in_value() {
        let input = "special:!@#$%^&*()[]{}|\\;',.<>?";
        let result = parse_str(input);
        assert!(result.is_ok(), "Special characters should be handled");
    }

    #[test]
    fn test_numeric_keys() {
        let input = "123:value\n456:another";
        let result = parse_str(input);
        assert!(result.is_ok(), "Numeric keys should be handled");
    }

    #[test]
    fn test_empty_value() {
        let input = "key:\nkey2:value";
        let result = parse_str(input);
        assert!(result.is_ok(), "Empty values should be handled");
    }

    #[test]
    fn test_value_with_only_spaces() {
        let input = "key:   ";
        let result = parse_str(input);
        assert!(result.is_ok(), "Value with only spaces should be handled");
    }
}

// ============================================================================
// COMPRESSION EDGE CASES
// ============================================================================

mod compression_edge_cases {
    use super::*;

    #[test]
    fn test_compress_empty_input() {
        let result = format_machine("");
        assert!(result.is_ok(), "Empty input should compress");
        assert!(result.unwrap().is_empty(), "Empty input should produce empty output");
    }

    #[test]
    fn test_compress_whitespace_only() {
        let result = format_machine("   \n\t\n   ");
        assert!(result.is_ok(), "Whitespace-only should compress");
    }

    #[test]
    fn test_compress_preserves_special_values() {
        let input = "key:value with spaces\nurl:https://example.com";
        let result = format_machine(input).unwrap();
        let output = String::from_utf8(result).unwrap();
        
        assert!(output.contains("value with spaces"), "Spaces in values should be preserved");
        assert!(output.contains("https://example.com"), "URLs should be preserved");
    }

    #[test]
    fn test_compress_handles_caret_prefix() {
        let input = "context.name:test\n^version:1.0";
        let result = format_machine(input);
        assert!(result.is_ok(), "Caret prefix should be handled");
    }

    #[test]
    fn test_compress_array_with_empty_items() {
        let input = "items > a | | b | | c";
        let result = format_machine(input);
        assert!(result.is_ok(), "Arrays with empty items should be handled");
    }

    #[test]
    fn test_compress_array_single_item() {
        let input = "items > single";
        let result = format_machine(input);
        assert!(result.is_ok(), "Single-item arrays should be handled");
    }

    #[test]
    fn test_compress_very_long_array() {
        let items: Vec<String> = (0..1000).map(|i| format!("item{}", i)).collect();
        let input = format!("items > {}", items.join(" | "));
        let result = format_machine(&input);
        assert!(result.is_ok(), "Very long arrays should be handled");
    }
}

// ============================================================================
// MAPPINGS EDGE CASES
// ============================================================================

mod mappings_edge_cases {
    use super::*;

    #[test]
    fn test_mappings_empty_key() {
        let mappings = Mappings::get();
        let result = mappings.compress_key("");
        assert_eq!(result, "", "Empty key should return empty string");
    }

    #[test]
    fn test_mappings_single_char_key() {
        let mappings = Mappings::get();
        // Single char that's not a mapping should stay as-is
        let result = mappings.compress_key("x");
        assert_eq!(result, "x", "Unknown single char should stay as-is");
    }

    #[test]
    fn test_mappings_case_sensitivity() {
        let mappings = Mappings::get();
        
        // "name" should compress to "n"
        assert_eq!(mappings.compress_key("name"), "n");
        
        // "Name" (capitalized) should NOT compress (case sensitive)
        assert_eq!(mappings.compress_key("Name"), "Name");
        
        // "NAME" (uppercase) should NOT compress
        assert_eq!(mappings.compress_key("NAME"), "NAME");
    }

    #[test]
    fn test_mappings_bidirectional_consistency() {
        let mappings = Mappings::get();
        
        // For all default mappings, compress then expand should give original
        for (full, short) in &mappings.compress {
            let compressed = mappings.compress_key(full);
            assert_eq!(&compressed, short, "Compression mismatch for {}", full);
            
            let expanded = mappings.expand_key(&compressed);
            assert_eq!(&expanded, full, "Expansion mismatch for {}", short);
        }
    }

    #[test]
    fn test_mappings_unknown_keys_preserved() {
        let mappings = Mappings::get();
        
        let unknown_keys = vec![
            "unknownKey",
            "myCustomField",
            "x_y_z",
            "CamelCaseKey",
            "key123",
            "123key",
        ];
        
        for key in unknown_keys {
            let compressed = mappings.compress_key(key);
            assert_eq!(compressed, key, "Unknown key '{}' should be preserved", key);
            
            let expanded = mappings.expand_key(key);
            assert_eq!(expanded, key, "Unknown key '{}' should be preserved on expand", key);
        }
    }
}

// ============================================================================
// TABLE PARSING EDGE CASES
// ============================================================================

mod table_edge_cases {
    use super::*;

    #[test]
    fn test_table_empty_rows() {
        let input = "data=id%i name%s\n";
        let result = parse_str(input);
        assert!(result.is_ok(), "Table with no rows should parse");
    }

    #[test]
    fn test_table_single_column() {
        let input = "data=id%i\n1\n2\n3";
        let result = parse_str(input);
        assert!(result.is_ok(), "Single-column table should parse");
    }

    #[test]
    fn test_table_many_columns() {
        let cols: Vec<String> = (0..50).map(|i| format!("col{}%s", i)).collect();
        let input = format!("data={}\nvalue1 value2", cols.join(" "));
        let result = parse_str(&input);
        assert!(result.is_ok(), "Table with many columns should parse");
    }

    #[test]
    fn test_table_ditto_first_row() {
        // Ditto on first row should fail gracefully
        let input = "data=id%i name%s\n_ Alice";
        let result = parse_str(input);
        // This should error because ditto has no previous value
        assert!(result.is_err(), "Ditto on first row should error");
    }

    #[test]
    fn test_table_ditto_chain() {
        let input = "data=id%i name%s\n1 Alice\n_ Bob\n_ Charlie";
        let result = parse_str(input);
        assert!(result.is_ok(), "Ditto chain should work");
    }
}

// ============================================================================
// STREAM ARRAY EDGE CASES
// ============================================================================

mod stream_array_edge_cases {
    use super::*;

    #[test]
    fn test_stream_empty() {
        let input = "items>";
        let result = parse_str(input);
        assert!(result.is_ok(), "Empty stream should parse");
    }

    #[test]
    fn test_stream_single_item() {
        let input = "items>single";
        let result = parse_str(input);
        assert!(result.is_ok(), "Single item stream should parse");
    }

    #[test]
    fn test_stream_with_pipes_in_values() {
        // This is tricky - pipes are delimiters
        let input = "items>a|b|c";
        let result = parse_str(input);
        assert!(result.is_ok(), "Stream with pipes should parse");
    }

    #[test]
    fn test_stream_numeric_values() {
        let input = "numbers>1|2|3|4|5";
        let result = parse_str(input);
        assert!(result.is_ok(), "Numeric stream should parse");
    }

    #[test]
    fn test_stream_mixed_types() {
        let input = "mixed>hello|123|+|-|~";
        let result = parse_str(input);
        assert!(result.is_ok(), "Mixed type stream should parse");
    }
}

// ============================================================================
// ALIAS EDGE CASES
// ============================================================================

mod alias_edge_cases {
    use super::*;

    #[test]
    fn test_alias_simple() {
        let input = "$c=context\n$c.name:test";
        let result = parse_str(input);
        assert!(result.is_ok(), "Simple alias should work");
    }

    #[test]
    fn test_alias_undefined() {
        let input = "$undefined.key:value";
        let result = parse_str(input);
        assert!(result.is_err(), "Undefined alias should error");
    }

    #[test]
    fn test_alias_redefinition() {
        let input = "$c=context\n$c=other\n$c.name:test";
        let result = parse_str(input);
        // Redefinition should work (last definition wins)
        assert!(result.is_ok(), "Alias redefinition should work");
    }

    #[test]
    fn test_alias_single_char() {
        let input = "$x=expanded\n$x.key:value";
        let result = parse_str(input);
        assert!(result.is_ok(), "Single char alias should work");
    }
}

// ============================================================================
// BASE62 EDGE CASES
// ============================================================================

mod base62_edge_cases {
    use serializer::base62::{encode_base62, decode_base62};

    #[test]
    fn test_base62_zero() {
        assert_eq!(encode_base62(0), "0");
        assert_eq!(decode_base62("0").unwrap(), 0);
    }

    #[test]
    fn test_base62_max_u64() {
        let max = u64::MAX;
        let encoded = encode_base62(max);
        let decoded = decode_base62(&encoded).unwrap();
        assert_eq!(decoded, max, "Max u64 should round-trip");
    }

    #[test]
    fn test_base62_boundary_values() {
        let boundaries = vec![
            0, 1, 9, 10, 35, 36, 61, 62, 63,
            100, 1000, 10000, 100000,
            u32::MAX as u64,
        ];
        
        for n in boundaries {
            let encoded = encode_base62(n);
            let decoded = decode_base62(&encoded).unwrap();
            assert_eq!(decoded, n, "Boundary {} should round-trip", n);
        }
    }

    #[test]
    fn test_base62_invalid_chars() {
        let invalid_inputs = vec![
            "!invalid",
            "hello world",
            "abc-def",
            "123_456",
            "αβγ",  // Greek letters
        ];
        
        for input in invalid_inputs {
            let result = decode_base62(input);
            assert!(result.is_err(), "Invalid input '{}' should error", input);
        }
    }

    #[test]
    fn test_base62_empty_string() {
        let result = decode_base62("");
        assert_eq!(result.unwrap(), 0, "Empty string should decode to 0");
    }
}

// ============================================================================
// BOOLEAN AND NULL EDGE CASES
// ============================================================================

mod boolean_null_edge_cases {
    use super::*;
    use serializer::types::DxValue;

    #[test]
    fn test_boolean_true_variants() {
        let input = "a:+\nb:true";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("a"), Some(&DxValue::Bool(true)));
            // "true" is parsed as string, not bool (+ is the bool marker)
        }
    }

    #[test]
    fn test_boolean_false_variants() {
        let input = "a:-";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("a"), Some(&DxValue::Bool(false)));
        }
    }

    #[test]
    fn test_null_variants() {
        let input = "a:~";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("a"), Some(&DxValue::Null));
        }
    }

    #[test]
    fn test_implicit_true() {
        let input = "enabled!";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("enabled"), Some(&DxValue::Bool(true)));
        }
    }

    #[test]
    fn test_implicit_null() {
        let input = "missing?";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("missing"), Some(&DxValue::Null));
        }
    }
}

// ============================================================================
// NUMBER PARSING EDGE CASES
// ============================================================================

mod number_edge_cases {
    use super::*;
    use serializer::types::DxValue;

    #[test]
    fn test_integer_zero() {
        let input = "num:0";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("num"), Some(&DxValue::Int(0)));
        }
    }

    #[test]
    fn test_negative_integer() {
        let input = "num:-42";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("num"), Some(&DxValue::Int(-42)));
        }
    }

    #[test]
    fn test_large_integer() {
        let input = "num:9223372036854775807"; // i64::MAX
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("num"), Some(&DxValue::Int(i64::MAX)));
        }
    }

    #[test]
    fn test_float_zero() {
        let input = "num:0.0";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("num"), Some(&DxValue::Float(0.0)));
        }
    }

    #[test]
    fn test_float_scientific() {
        let input = "num:1e10";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            assert_eq!(obj.get("num"), Some(&DxValue::Float(1e10)));
        }
    }

    #[test]
    fn test_negative_float() {
        let input = "num:-3.14159";
        let result = parse_str(input).unwrap();
        
        if let DxValue::Object(obj) = result {
            if let Some(DxValue::Float(f)) = obj.get("num") {
                assert!((f - (-3.14159)).abs() < 0.00001);
            } else {
                panic!("Expected float");
            }
        }
    }
}

// ============================================================================
// STRESS TESTS
// ============================================================================

mod stress_tests {
    use super::*;

    #[test]
    fn test_many_keys() {
        let mut input = String::new();
        for i in 0..1000 {
            input.push_str(&format!("key{}:value{}\n", i, i));
        }
        
        let result = parse_str(&input);
        assert!(result.is_ok(), "Many keys should parse");
    }

    #[test]
    fn test_large_table() {
        let mut input = String::from("data=id%i name%s score%i\n");
        for i in 0..1000 {
            input.push_str(&format!("{} User{} {}\n", i, i, i * 10));
        }
        
        let result = parse_str(&input);
        assert!(result.is_ok(), "Large table should parse");
    }

    #[test]
    fn test_compression_large_input() {
        let mut input = String::new();
        for i in 0..1000 {
            input.push_str(&format!("context.item{}: value{}\n", i, i));
        }
        
        let result = format_machine(&input);
        assert!(result.is_ok(), "Large input should compress");
        
        let compressed = result.unwrap();
        // Compressed should be smaller due to key abbreviation
        assert!(compressed.len() < input.len(), "Compression should reduce size");
    }
}

// ============================================================================
// ROUNDTRIP TESTS
// ============================================================================

mod roundtrip_tests {
    use super::*;

    #[test]
    fn test_simple_roundtrip() {
        let original = "name:test\nversion:1";
        
        // Parse
        let parsed = parse_str(original).unwrap();
        
        // The parsed value should contain the data
        if let serializer::types::DxValue::Object(obj) = parsed {
            assert!(obj.contains_key("name"));
            assert!(obj.contains_key("version"));
        }
    }

    #[test]
    fn test_compression_preserves_data() {
        let original = "name:myapp\nversion:2\nauthor:Team";
        
        let compressed = format_machine(original).unwrap();
        let compressed_str = String::from_utf8(compressed).unwrap();
        
        // Key data should be present (possibly abbreviated)
        assert!(compressed_str.contains("myapp"));
        assert!(compressed_str.contains("2"));
        assert!(compressed_str.contains("Team"));
    }
}
