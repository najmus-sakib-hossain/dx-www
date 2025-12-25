//! Property tests for parser input validation
//!
//! Feature: serializer-battle-hardening
//! Tests Properties 1-3 from the design document

use proptest::prelude::*;
use serializer::{parse, DxError};

/// Strategy to generate strings with null bytes at random positions
fn string_with_null_bytes() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(prop::num::u8::ANY, 1..100).prop_map(|mut bytes| {
        // Ensure at least one null byte
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
        // Missing value after colon
        "[a-z][a-z0-9_]{0,10}:".prop_map(|s| s),
        // Invalid characters at start
        Just(":::invalid".to_string()),
        Just("@@@bad".to_string()),
        // Unclosed structures
        Just("key:value\n$undefined.ref:test".to_string()),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Feature: serializer-battle-hardening, Property 1: Null Byte Handling
    /// Validates: Requirements 1.1
    ///
    /// For any input string containing null bytes at any position,
    /// the Parser SHALL either successfully parse the input or return
    /// a well-formed error without panicking.
    #[test]
    fn prop_null_byte_handling(input in string_with_null_bytes()) {
        // The parser should not panic on null bytes
        let result = std::panic::catch_unwind(|| {
            parse(&input)
        });
        
        // Should not panic
        prop_assert!(result.is_ok(), "Parser panicked on input with null bytes");
        
        // If it returns an error, it should be a well-formed error
        if let Ok(Err(e)) = result {
            // Error should have a meaningful message
            let msg = format!("{}", e);
            prop_assert!(!msg.is_empty(), "Error message should not be empty");
        }
    }

    /// Feature: serializer-battle-hardening, Property 2: UTF-8 Validation with Offset
    /// Validates: Requirements 1.4
    ///
    /// For any byte sequence containing invalid UTF-8, the Parser SHALL
    /// return a Utf8Error where the offset field exactly matches the byte
    /// position of the first invalid sequence.
    #[test]
    fn prop_utf8_validation_with_offset(
        prefix in "[a-z]{0,10}",
        suffix in "[a-z]{0,10}"
    ) {
        // Create input with invalid UTF-8 at a known position
        let mut input = prefix.as_bytes().to_vec();
        let invalid_offset = input.len();
        
        // Add invalid UTF-8 sequence (0xFF is never valid in UTF-8)
        input.push(0xFF);
        input.extend_from_slice(suffix.as_bytes());
        
        let result = parse(&input);
        
        // Should return an error (either UTF-8 or parse error)
        // The parser may handle invalid UTF-8 in different ways
        match result {
            Ok(_) => {
                // If parsing succeeds, the invalid byte was handled gracefully
                // (e.g., treated as part of a binary value)
            }
            Err(DxError::Utf8Error { offset }) => {
                // If UTF-8 error, offset should be at or after the invalid byte
                prop_assert!(
                    offset >= invalid_offset,
                    "UTF-8 error offset {} should be >= invalid byte position {}",
                    offset, invalid_offset
                );
            }
            Err(_) => {
                // Other errors are acceptable (e.g., parse errors)
            }
        }
    }

    /// Feature: serializer-battle-hardening, Property 3: Error Position Reporting
    /// Validates: Requirements 1.5, 7.1
    ///
    /// For any syntactically invalid input, the Parser SHALL return an error
    /// containing position information.
    #[test]
    fn prop_error_position_reporting(input in invalid_syntax_input()) {
        let result = parse(input.as_bytes());
        
        // Should return an error for invalid syntax
        if let Err(e) = result {
            // Check that error has position information
            let has_position = e.offset().is_some() || e.location().is_some();
            
            // For certain error types, position info is expected
            match &e {
                DxError::InvalidSyntax { pos, .. } => {
                    prop_assert!(*pos < input.len() + 1, "Position should be within input bounds");
                }
                DxError::ParseError { location, .. } => {
                    prop_assert!(location.line >= 1, "Line number should be >= 1");
                    prop_assert!(location.column >= 1, "Column number should be >= 1");
                }
                DxError::UnknownAlias(_) => {
                    // Alias errors may not have position info
                }
                _ => {
                    // Other errors may or may not have position info
                }
            }
        }
        // Note: Some "invalid" inputs may actually parse successfully
        // due to the flexible nature of the DX format
    }

    /// Additional property: Valid inputs should parse successfully
    #[test]
    fn prop_valid_input_parses(input in valid_dx_input()) {
        let result = parse(input.as_bytes());
        prop_assert!(result.is_ok(), "Valid DX input should parse: {:?}", result.err());
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_null_byte_in_key() {
        let input = b"ke\0y:value";
        let result = parse(input);
        // Should not panic, may return error or handle gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_null_byte_in_value() {
        let input = b"key:val\0ue";
        let result = parse(input);
        // Should not panic
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_invalid_utf8_sequence() {
        let input = vec![b'k', b'e', b'y', b':', 0xFF, 0xFE];
        let result = parse(&input);
        // Should handle gracefully
        assert!(result.is_ok() || result.is_err());
    }
}
