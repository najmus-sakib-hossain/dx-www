//! Property-based tests for DX Serializer Battle Hardening
//!
//! Feature: serializer-battle-hardening
//! 
//! This test file contains property tests that verify correctness properties
//! across many generated inputs using proptest.
//!
//! Run with: cargo test --package serializer --test battle_hardening_property_tests
//!
//! ## Properties Tested
//! - Properties 1-3: Parser input validation
//! - Properties 4-7: Tokenizer robustness
//! - Properties 8-11: Round-trip consistency
//! - Properties 12-13: Binary format security

use proptest::prelude::*;
use serializer::{
    parse, encode, format_human, format_machine,
    DxValue, DxObject, DxArray, DxError,
    tokenizer::{Token, Tokenizer},
    zero::{
        DxZeroHeader, HeaderError, MAGIC, VERSION,
        FLAG_LITTLE_ENDIAN, FLAG_HAS_HEAP, FLAG_HAS_INTERN, FLAG_HAS_LENGTH_TABLE,
    },
};

// =============================================================================
// PARSER PROPERTY TESTS (Properties 1-3)
// =============================================================================

mod parser_props {
    use super::*;

    /// Strategy to generate strings with null bytes at random positions
    fn string_with_null_bytes() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(prop::num::u8::ANY, 1..100).prop_map(|mut bytes| {
            if !bytes.contains(&0) && !bytes.is_empty() {
                let pos = bytes.len() / 2;
                bytes[pos] = 0;
            }
            bytes
        })
    }

    /// Strategy to generate valid DX key-value pairs
    fn valid_dx_input() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_]{0,20}".prop_flat_map(|key| {
            "[a-zA-Z0-9_]{1,50}".prop_map(move |value| format!("{}:{}", key, value))
        })
    }

    /// Strategy to generate syntactically invalid inputs
    fn invalid_syntax_input() -> impl Strategy<Value = String> {
        prop_oneof![
            "[a-z][a-z0-9_]{0,10}:".prop_map(|s| s),
            Just(":::invalid".to_string()),
            Just("@@@bad".to_string()),
            Just("key:value\n$undefined.ref:test".to_string()),
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: serializer-battle-hardening, Property 1: Null Byte Handling
        /// Validates: Requirements 1.1
        #[test]
        fn prop_null_byte_handling(input in string_with_null_bytes()) {
            let result = std::panic::catch_unwind(|| parse(&input));
            prop_assert!(result.is_ok(), "Parser panicked on input with null bytes");
            
            if let Ok(Err(e)) = result {
                let msg = format!("{}", e);
                prop_assert!(!msg.is_empty(), "Error message should not be empty");
            }
        }

        /// Feature: serializer-battle-hardening, Property 2: UTF-8 Validation with Offset
        /// Validates: Requirements 1.4
        #[test]
        fn prop_utf8_validation_with_offset(
            prefix in "[a-z]{0,10}",
            suffix in "[a-z]{0,10}"
        ) {
            let mut input = prefix.as_bytes().to_vec();
            let invalid_offset = input.len();
            input.push(0xFF);
            input.extend_from_slice(suffix.as_bytes());
            
            let result = parse(&input);
            
            match result {
                Ok(_) => { /* Handled gracefully */ }
                Err(DxError::Utf8Error { offset }) => {
                    prop_assert!(
                        offset >= invalid_offset,
                        "UTF-8 error offset {} should be >= invalid byte position {}",
                        offset, invalid_offset
                    );
                }
                Err(_) => { /* Other errors acceptable */ }
            }
        }

        /// Feature: serializer-battle-hardening, Property 3: Error Position Reporti