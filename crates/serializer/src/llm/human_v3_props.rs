//! Property-based tests for Human V3 parser
//!
//! These tests verify the correctness properties defined in the design document.

use proptest::prelude::*;

use super::human_v3_parser::{HumanV3Parser, SectionHeader};
use super::section_names::SectionNameDict;
use super::types::DxLlmValue;

// ============================================================
// Property 6: Quoted String Handling
// For any string value (quoted or unquoted), parsing SHALL preserve
// the content exactly, removing quotes if present.
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_quoted_string_preserves_content(s in "[a-zA-Z0-9_@/.-]{1,50}") {
        let parser = HumanV3Parser::new();

        // Double-quoted string
        let double_quoted = format!("\"{}\"", s);
        let result = parser.parse_value(&double_quoted);
        prop_assert_eq!(result, DxLlmValue::Str(s.clone()));

        // Single-quoted string
        let single_quoted = format!("'{}'", s);
        let result = parser.parse_value(&single_quoted);
        prop_assert_eq!(result, DxLlmValue::Str(s.clone()));

        // Unquoted string (if no special characters)
        if !s.contains(' ') && !s.parse::<f64>().is_ok() && s != "true" && s != "false" && s != "-" && s != "~" && s != "none" {
            let result = parser.parse_value(&s);
            prop_assert_eq!(result, DxLlmValue::Str(s));
        }
    }
}

// ============================================================
// Property 7: Value Type Detection
// For any value string matching a numeric pattern, parsing SHALL
// produce a number type; for pipe-separated values, an array type.
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_numeric_values_parsed_as_numbers(n in -1000000i64..1000000i64) {
        let parser = HumanV3Parser::new();
        let input = n.to_string();
        let result = parser.parse_value(&input);
        prop_assert_eq!(result, DxLlmValue::Num(n as f64));
    }

    #[test]
    fn prop_float_values_parsed_as_numbers(n in -1000.0f64..1000.0f64) {
        let parser = HumanV3Parser::new();
        let input = format!("{:.2}", n);
        let result = parser.parse_value(&input);
        match result {
            DxLlmValue::Num(parsed) => {
                // Allow small floating point differences
                prop_assert!((parsed - input.parse::<f64>().unwrap()).abs() < 0.01);
            }
            _ => prop_assert!(false, "Expected number, got {:?}", result),
        }
    }

    #[test]
    fn prop_pipe_separated_values_parsed_as_arrays(
        parts in prop::collection::vec("[a-zA-Z_][a-zA-Z0-9_]{0,9}", 2..6)
    ) {
        let parser = HumanV3Parser::new();
        let input = parts.join(" | ");
        let result = parser.parse_value(&input);

        match result {
            DxLlmValue::Arr(arr) => {
                prop_assert_eq!(arr.len(), parts.len());
                for (i, part) in parts.iter().enumerate() {
                    prop_assert_eq!(arr[i].clone(), DxLlmValue::Str(part.clone()));
                }
            }
            _ => prop_assert!(false, "Expected array, got {:?}", result),
        }
    }
}

// ============================================================
// Property 2: Array Parsing Consistency
// For any value containing N pipe separators (` | `), parsing SHALL
// produce an array with exactly N+1 elements.
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_array_element_count(n in 2usize..10) {
        let parser = HumanV3Parser::new();

        // Generate n elements
        let parts: Vec<String> = (0..n).map(|i| format!("elem{}", i)).collect();
        let input = parts.join(" | ");

        // Count pipe separators
        let pipe_count = input.matches(" | ").count();

        let result = parser.parse_value(&input);
        match result {
            DxLlmValue::Arr(arr) => {
                // N pipe separators should produce N+1 elements
                prop_assert_eq!(arr.len(), pipe_count + 1);
            }
            _ => prop_assert!(false, "Expected array, got {:?}", result),
        }
    }
}

// ============================================================
// Property 4: Section Name Mapping
// For any section header with a full name, parsing SHALL map it
// to the correct abbreviated section ID.
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_section_name_round_trip(idx in 0usize..16) {
        let dict = SectionNameDict::new();
        let names = [
            "config", "forge", "stack", "style", "ui", "media", "i18n",
            "icon", "font", "driven", "generator", "scripts", "dependencies",
            "js", "python", "rust",
        ];

        let name = names[idx % names.len()];
        let id = dict.name_to_id(name);
        let back = dict.id_to_name(&id);

        prop_assert_eq!(back, name);
    }
}

// ============================================================
// Property 1: Config Parsing
// For any Human V3 document with key-value pairs before any section
// header, parsing SHALL place those values in the context map.
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_config_values_in_context(
        name in "[a-zA-Z][a-zA-Z0-9_-]{0,20}",
        version in "[0-9]+\\.[0-9]+\\.[0-9]+"
    ) {
        let parser = HumanV3Parser::new();

        let input = format!("name = {}\nversion = {}\n", name, version);
        let result = parser.parse(&input);

        prop_assert!(result.is_ok());
        let doc = result.unwrap();

        // name should be compressed to "nm"
        prop_assert!(doc.context.contains_key("nm"));
        prop_assert_eq!(doc.context.get("nm"), Some(&DxLlmValue::Str(name)));

        // version should be compressed to "vr"
        prop_assert!(doc.context.contains_key("vr"));
        prop_assert_eq!(doc.context.get("vr"), Some(&DxLlmValue::Str(version)));
    }
}

// ============================================================
// Section Header Parsing Tests
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_simple_section_header_parsing(name in "[a-zA-Z][a-zA-Z0-9_]{0,20}") {
        let parser = HumanV3Parser::new();
        let header_str = format!("[{}]", name);
        let result = parser.parse_section_header(&header_str);

        prop_assert_eq!(result, Some(SectionHeader::Simple(name)));
    }

    #[test]
    fn prop_nested_section_header_parsing(
        parent in "[a-zA-Z][a-zA-Z0-9_]{0,10}",
        child in "[a-zA-Z][a-zA-Z0-9_]{0,10}"
    ) {
        let parser = HumanV3Parser::new();
        let header_str = format!("[{}.{}]", parent, child);
        let result = parser.parse_section_header(&header_str);

        prop_assert_eq!(result, Some(SectionHeader::Nested(parent, child)));
    }
}

// ============================================================
// Boolean and Null Value Tests
// ============================================================

#[test]
fn test_boolean_values() {
    let parser = HumanV3Parser::new();

    assert_eq!(parser.parse_value("true"), DxLlmValue::Bool(true));
    assert_eq!(parser.parse_value("false"), DxLlmValue::Bool(false));
}

#[test]
fn test_null_values() {
    let parser = HumanV3Parser::new();

    assert_eq!(parser.parse_value("-"), DxLlmValue::Null);
    assert_eq!(parser.parse_value("~"), DxLlmValue::Null);
    assert_eq!(parser.parse_value("none"), DxLlmValue::Null);
}
