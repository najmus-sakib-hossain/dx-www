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

// ============================================================
// Property 3: Stack Section Preservation
// For any `[stack]` section, parsing SHALL place all entries in the
// refs map with keys preserved (not abbreviated) and values joined with `|`.
// **Validates: Requirements 2.1, 2.2, 2.3**
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_stack_section_preserves_keys(
        key in "[a-z]{2,10}",
        values in prop::collection::vec("[a-zA-Z][a-zA-Z0-9_/-]{0,15}", 1..5)
    ) {
        let parser = HumanV3Parser::new();

        // Create a stack section with pipe-separated values
        let value_str = values.join(" | ");
        let input = format!("[stack]\n{} = {}\n", key, value_str);

        let result = parser.parse(&input);
        prop_assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let doc = result.unwrap();

        // Key should be preserved (not abbreviated)
        prop_assert!(doc.refs.contains_key(&key), "Key '{}' not found in refs", key);

        // Values should be joined with | (no spaces)
        let expected_value = values.join("|");
        prop_assert_eq!(
            doc.refs.get(&key),
            Some(&expected_value),
            "Expected '{}', got '{:?}'",
            expected_value,
            doc.refs.get(&key)
        );
    }

    #[test]
    fn prop_stack_section_multiple_entries(
        entries in prop::collection::vec(
            ("[a-z]{2,8}", prop::collection::vec("[a-zA-Z0-9_/-]{1,10}", 1..4)),
            1..4
        )
    ) {
        // Ensure unique keys
        let mut seen_keys = std::collections::HashSet::new();
        let unique_entries: Vec<_> = entries
            .into_iter()
            .filter(|(k, _)| seen_keys.insert(k.clone()))
            .collect();

        prop_assume!(!unique_entries.is_empty());

        let parser = HumanV3Parser::new();

        // Build stack section
        let mut input = String::from("[stack]\n");
        for (key, values) in &unique_entries {
            let value_str = values.join(" | ");
            input.push_str(&format!("{} = {}\n", key, value_str));
        }

        let result = parser.parse(&input);
        prop_assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let doc = result.unwrap();

        // All entries should be in refs
        for (key, values) in &unique_entries {
            prop_assert!(doc.refs.contains_key(key), "Key '{}' not found in refs", key);
            let expected_value = values.join("|");
            prop_assert_eq!(
                doc.refs.get(key),
                Some(&expected_value),
                "For key '{}': expected '{}', got '{:?}'",
                key,
                expected_value,
                doc.refs.get(key)
            );
        }
    }
}


// ============================================================
// Property 5: Nested Section Merging
// For any set of nested sections sharing a parent, parsing SHALL
// merge them into a single section with keys prefixed by subsection name.
// **Validates: Requirements 4.1, 4.2, 4.3**
// ============================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_nested_sections_merged_with_prefix(
        subsection1 in "[a-z]{3,8}",
        subsection2 in "[a-z]{3,8}",
        key1 in "[a-z]{2,6}",
        key2 in "[a-z]{2,6}",
        value1 in "[a-zA-Z0-9_/-]{1,15}",
        value2 in "[a-zA-Z0-9_/-]{1,15}"
    ) {
        // Ensure subsections are different
        prop_assume!(subsection1 != subsection2);

        let parser = HumanV3Parser::new();

        // Create nested sections under i18n parent
        let input = format!(
            "[i18n.{}]\n{} = {}\n\n[i18n.{}]\n{} = {}\n",
            subsection1, key1, value1,
            subsection2, key2, value2
        );

        let result = parser.parse(&input);
        prop_assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let doc = result.unwrap();

        // i18n section should exist (mapped to 'i')
        prop_assert!(doc.sections.contains_key(&'i'), "i18n section not found");

        let section = doc.sections.get(&'i').unwrap();

        // Schema should contain prefixed keys
        let has_sub1_key = section.schema.iter().any(|k| k.contains(&subsection1));
        let has_sub2_key = section.schema.iter().any(|k| k.contains(&subsection2));

        prop_assert!(has_sub1_key, "Schema missing key with prefix '{}': {:?}", subsection1, section.schema);
        prop_assert!(has_sub2_key, "Schema missing key with prefix '{}': {:?}", subsection2, section.schema);

        // Should have exactly one row with all values
        prop_assert_eq!(section.rows.len(), 1, "Expected 1 row, got {}", section.rows.len());
    }

    #[test]
    fn prop_nested_section_order_preserved(
        subsections in prop::collection::vec("[a-z]{3,6}", 2..5)
    ) {
        // Ensure unique subsection names
        let mut seen = std::collections::HashSet::new();
        let unique_subs: Vec<_> = subsections
            .into_iter()
            .filter(|s| seen.insert(s.clone()))
            .collect();

        prop_assume!(unique_subs.len() >= 2);

        let parser = HumanV3Parser::new();

        // Build nested sections under i18n
        let mut input = String::new();
        for (i, sub) in unique_subs.iter().enumerate() {
            input.push_str(&format!("[i18n.{}]\nkey{} = value{}\n\n", sub, i, i));
        }

        let result = parser.parse(&input);
        prop_assert!(result.is_ok(), "Parse failed: {:?}", result.err());

        let doc = result.unwrap();
        prop_assert!(doc.sections.contains_key(&'i'), "i18n section not found");

        let section = doc.sections.get(&'i').unwrap();

        // Schema should have keys in the order subsections were defined
        // Each subsection contributes one key, so schema length should match
        prop_assert_eq!(
            section.schema.len(),
            unique_subs.len(),
            "Schema length mismatch: expected {}, got {}",
            unique_subs.len(),
            section.schema.len()
        );
    }
}


// ============================================================
// Property 8: Round-Trip Consistency
// For any valid Human V3 document, parsing then serializing to LLM format
// SHALL produce a document with equivalent context, refs, and section data.
// **Validates: Requirements 7.1, 7.2, 7.3**
// ============================================================

use super::serializer::LlmSerializer;
use super::parser::LlmParser;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_human_v3_to_llm_preserves_context(
        name in "[a-zA-Z][a-zA-Z0-9_-]{0,15}",
        version in "[0-9]+\\.[0-9]+\\.[0-9]+"
    ) {
        let parser = HumanV3Parser::new();
        let serializer = LlmSerializer::new();

        // Create Human V3 input
        let human_v3 = format!("name = {}\nversion = {}\n", name, version);

        // Parse Human V3 -> DxDocument
        let doc = parser.parse(&human_v3);
        prop_assert!(doc.is_ok(), "Human V3 parse failed: {:?}", doc.err());
        let doc = doc.unwrap();

        // Serialize to LLM format
        let llm = serializer.serialize(&doc);

        // Parse LLM back to verify
        let round_trip_doc = LlmParser::parse(&llm);
        prop_assert!(round_trip_doc.is_ok(), "LLM parse failed: {:?}", round_trip_doc.err());
        let round_trip_doc = round_trip_doc.unwrap();

        // Verify context values are preserved
        prop_assert_eq!(
            doc.context.len(),
            round_trip_doc.context.len(),
            "Context size mismatch"
        );

        // Check name is preserved (key may be abbreviated)
        let has_name = round_trip_doc.context.values().any(|v| {
            matches!(v, DxLlmValue::Str(s) if s == &name)
        });
        prop_assert!(has_name, "Name '{}' not found in round-trip context", name);

        // Check version is preserved
        let has_version = round_trip_doc.context.values().any(|v| {
            matches!(v, DxLlmValue::Str(s) if s == &version)
        });
        prop_assert!(has_version, "Version '{}' not found in round-trip context", version);
    }

    #[test]
    fn prop_human_v3_to_llm_preserves_stack(
        key in "[a-z]{2,8}",
        values in prop::collection::vec("[a-zA-Z0-9_/-]{1,10}", 1..4)
    ) {
        let parser = HumanV3Parser::new();
        let serializer = LlmSerializer::new();

        // Create Human V3 input with stack section
        let value_str = values.join(" | ");
        let human_v3 = format!("[stack]\n{} = {}\n", key, value_str);

        // Parse Human V3 -> DxDocument
        let doc = parser.parse(&human_v3);
        prop_assert!(doc.is_ok(), "Human V3 parse failed: {:?}", doc.err());
        let doc = doc.unwrap();

        // Verify refs are set correctly
        prop_assert!(doc.refs.contains_key(&key), "Key '{}' not found in refs", key);
        let expected_value = values.join("|");
        prop_assert_eq!(
            doc.refs.get(&key),
            Some(&expected_value),
            "Refs value mismatch"
        );

        // Serialize to LLM format
        let llm = serializer.serialize(&doc);

        // Verify LLM contains the reference definition
        prop_assert!(
            llm.contains(&format!("#:{}|", key)),
            "LLM output missing reference definition for '{}'",
            key
        );
    }

    #[test]
    fn prop_human_v3_to_llm_preserves_sections(
        key1 in "[a-z]{2,6}",
        key2 in "[a-z]{2,6}",
        val1 in "[a-zA-Z0-9_]{1,10}",
        val2 in "[a-zA-Z0-9_]{1,10}"
    ) {
        prop_assume!(key1 != key2);

        let parser = HumanV3Parser::new();
        let serializer = LlmSerializer::new();

        // Create Human V3 input with forge section
        let human_v3 = format!("[forge]\n{} = {}\n{} = {}\n", key1, val1, key2, val2);

        // Parse Human V3 -> DxDocument
        let doc = parser.parse(&human_v3);
        prop_assert!(doc.is_ok(), "Human V3 parse failed: {:?}", doc.err());
        let doc = doc.unwrap();

        // Verify forge section exists (mapped to 'f')
        prop_assert!(doc.sections.contains_key(&'f'), "Forge section not found");

        let section = doc.sections.get(&'f').unwrap();
        prop_assert_eq!(section.schema.len(), 2, "Schema should have 2 columns");
        prop_assert_eq!(section.rows.len(), 1, "Should have 1 row");

        // Serialize to LLM format
        let llm = serializer.serialize(&doc);

        // Verify LLM contains the section
        prop_assert!(
            llm.contains("#f("),
            "LLM output missing forge section"
        );
    }

    #[test]
    fn prop_human_v3_to_llm_preserves_booleans(
        key in "[a-z]{2,6}"
    ) {
        let parser = HumanV3Parser::new();
        let serializer = LlmSerializer::new();

        // Create Human V3 input with boolean values
        let human_v3 = format!("{} = true\n", key);

        // Parse Human V3 -> DxDocument
        let doc = parser.parse(&human_v3);
        prop_assert!(doc.is_ok(), "Human V3 parse failed: {:?}", doc.err());
        let doc = doc.unwrap();

        // Verify boolean is parsed correctly
        let has_bool = doc.context.values().any(|v| {
            matches!(v, DxLlmValue::Bool(true))
        });
        prop_assert!(has_bool, "Boolean true not found in context");

        // Serialize to LLM format
        let llm = serializer.serialize(&doc);

        // Verify LLM contains the boolean marker
        prop_assert!(
            llm.contains("|+"),
            "LLM output missing boolean true marker (+)"
        );
    }
}


// ============================================================
// Property 9: Format Detection and Passthrough
// For any input to `toDense`, if it starts with LLM format sigils
// (#c:, #:, or #<letter>(), it SHALL be returned unchanged;
// otherwise it SHALL be parsed as Human V3.
// **Validates: Requirements 8.1, 8.2**
// ============================================================

use super::convert::{human_to_llm, is_llm_format};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_llm_format_passthrough_context(
        key in "[a-z]{2,6}",
        value in "[a-zA-Z0-9_]{1,15}"
    ) {
        // LLM format with context section
        let llm_input = format!("#c:{}|{}", key, value);

        // Should be detected as LLM format
        prop_assert!(is_llm_format(&llm_input), "Should detect as LLM format: {}", llm_input);

        // Should be returned unchanged
        let result = human_to_llm(&llm_input);
        prop_assert!(result.is_ok(), "human_to_llm failed: {:?}", result.err());
        prop_assert_eq!(result.unwrap(), llm_input, "LLM format should be returned unchanged");
    }

    #[test]
    fn prop_llm_format_passthrough_section(
        section_id in "[a-z]",
        col1 in "[a-z]{2,4}",
        col2 in "[a-z]{2,4}"
    ) {
        // LLM format with data section
        let llm_input = format!("#{}({}|{})\nval1|val2", section_id, col1, col2);

        // Should be detected as LLM format
        prop_assert!(is_llm_format(&llm_input), "Should detect as LLM format: {}", llm_input);

        // Should be returned unchanged
        let result = human_to_llm(&llm_input);
        prop_assert!(result.is_ok(), "human_to_llm failed: {:?}", result.err());
        prop_assert_eq!(result.unwrap(), llm_input, "LLM format should be returned unchanged");
    }

    #[test]
    fn prop_llm_format_passthrough_refs(
        key in "[A-Z]",
        value in "[a-zA-Z0-9_]{1,15}"
    ) {
        // LLM format with reference definition
        let llm_input = format!("#:{}|{}", key, value);

        // Should be detected as LLM format
        prop_assert!(is_llm_format(&llm_input), "Should detect as LLM format: {}", llm_input);

        // Should be returned unchanged
        let result = human_to_llm(&llm_input);
        prop_assert!(result.is_ok(), "human_to_llm failed: {:?}", result.err());
        prop_assert_eq!(result.unwrap(), llm_input, "LLM format should be returned unchanged");
    }

    #[test]
    fn prop_human_v3_format_parsed(
        name in "[a-zA-Z][a-zA-Z0-9_-]{0,15}",
        version in "[0-9]+\\.[0-9]+\\.[0-9]+"
    ) {
        // Human V3 format (no [config] header)
        let human_v3 = format!("name = {}\nversion = {}\n", name, version);

        // Should NOT be detected as LLM format
        prop_assert!(!is_llm_format(&human_v3), "Should NOT detect as LLM format: {}", human_v3);

        // Should be parsed and converted to LLM format
        let result = human_to_llm(&human_v3);
        prop_assert!(result.is_ok(), "human_to_llm failed: {:?}", result.err());

        let llm = result.unwrap();
        // Result should be LLM format (starts with #c:)
        prop_assert!(llm.starts_with("#c:"), "Result should be LLM format: {}", llm);
    }

    #[test]
    fn prop_comment_headers_not_llm_format(
        title in "[A-Z][A-Z ]{0,20}"
    ) {
        // Human format with comment headers (starts with # but not LLM format)
        let human_with_comments = format!("# ═══════════════════════════════════════════════════════════════════════════════\n#                                   {}\n# ═══════════════════════════════════════════════════════════════════════════════\n\n[config]\n    name = test\n", title);

        // Should NOT be detected as LLM format (comment headers are not LLM sigils)
        prop_assert!(!is_llm_format(&human_with_comments), "Comment headers should NOT be detected as LLM format");
    }
}
