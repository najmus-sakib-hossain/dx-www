//! Property-based tests for error handling
//!
//! **Property 12: Error Messages with Location**
//! **Validates: Requirements 6.1, 6.2, 6.6, 6.7, 6.8**

#[cfg(test)]
mod property_tests {
    use crate::error::{DxError, SourceLocation, DX_MAGIC, DX_VERSION};
    use proptest::prelude::*;

    /// Generate random input with newlines for location testing
    fn arb_multiline_input() -> impl Strategy<Value = Vec<u8>> {
        proptest::collection::vec(
            prop_oneof![
                Just(b'\n'),
                (0x20u8..0x7Eu8), // Printable ASCII
            ],
            10..100,
        )
    }

    /// Generate a random byte offset within input bounds
    fn arb_offset_for_input(input: &[u8]) -> impl Strategy<Value = usize> {
        0..input.len().max(1)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Property 12a: Error location line numbers SHALL be 1-indexed
        ///
        /// **Property 12: Error Messages with Location**
        /// **Validates: Requirements 6.1**
        #[test]
        fn prop_error_location_line_is_1_indexed(input in arb_multiline_input()) {
            let loc = SourceLocation::from_offset(&input, 0);
            prop_assert!(loc.line >= 1, "Line number should be >= 1, got {}", loc.line);
        }

        /// Property 12b: Error location column numbers SHALL be 1-indexed
        ///
        /// **Property 12: Error Messages with Location**
        /// **Validates: Requirements 6.1**
        #[test]
        fn prop_error_location_column_is_1_indexed(input in arb_multiline_input()) {
            let loc = SourceLocation::from_offset(&input, 0);
            prop_assert!(loc.column >= 1, "Column number should be >= 1, got {}", loc.column);
        }

        /// Property 12c: Line number SHALL increment after each newline
        ///
        /// **Property 12: Error Messages with Location**
        /// **Validates: Requirements 6.1**
        #[test]
        fn prop_line_increments_after_newline(
            prefix in "[a-z]{1,10}",
            suffix in "[a-z]{1,10}"
        ) {
            let input = format!("{}\n{}", prefix, suffix);
            let bytes = input.as_bytes();
            
            // Location before newline
            let loc_before = SourceLocation::from_offset(bytes, prefix.len());
            // Location after newline (first char of second line)
            let loc_after = SourceLocation::from_offset(bytes, prefix.len() + 1);
            
            prop_assert_eq!(loc_before.line, 1, "Before newline should be line 1");
            prop_assert_eq!(loc_after.line, 2, "After newline should be line 2");
            prop_assert_eq!(loc_after.column, 1, "First char of new line should be column 1");
        }

        /// Property 12d: Column SHALL reset to 1 after newline
        ///
        /// **Property 12: Error Messages with Location**
        /// **Validates: Requirements 6.1**
        #[test]
        fn prop_column_resets_after_newline(
            line1 in "[a-z]{5,15}",
            line2 in "[a-z]{3,10}"
        ) {
            let input = format!("{}\n{}", line1, line2);
            let bytes = input.as_bytes();
            
            // Check column at start of second line
            let offset = line1.len() + 1; // After newline
            let loc = SourceLocation::from_offset(bytes, offset);
            
            prop_assert_eq!(loc.column, 1, "Column should reset to 1 after newline");
        }

        /// Property 12e: Offset SHALL be preserved in SourceLocation
        ///
        /// **Property 12: Error Messages with Location**
        /// **Validates: Requirements 6.1**
        #[test]
        fn prop_offset_preserved(input in arb_multiline_input()) {
            if input.is_empty() {
                return Ok(());
            }
            let offset = input.len() / 2;
            let loc = SourceLocation::from_offset(&input, offset);
            prop_assert_eq!(loc.offset, offset, "Offset should be preserved");
        }

        /// Property 13: Invalid magic bytes SHALL produce InvalidMagic error
        ///
        /// **Property 13: Binary Header Validation**
        /// **Validates: Requirements 6.2**
        #[test]
        fn prop_invalid_magic_error_contains_bytes(byte0 in 0u8..255, byte1 in 0u8..255) {
            // Skip valid magic bytes
            if byte0 == DX_MAGIC[0] && byte1 == DX_MAGIC[1] {
                return Ok(());
            }
            
            let err = DxError::invalid_magic(byte0, byte1);
            let msg = err.to_string();
            
            prop_assert!(
                msg.contains(&format!("{:#04X}", byte0)) || msg.contains(&format!("{:#02X}", byte0)),
                "Error message should contain first byte: {}", msg
            );
        }

        /// Property 14: UTF-8 errors SHALL include byte offset
        ///
        /// **Property 14: Invalid UTF-8 Handling**
        /// **Validates: Requirements 6.6**
        #[test]
        fn prop_utf8_error_has_offset(offset in 0usize..10000) {
            let err = DxError::utf8_error(offset);
            prop_assert_eq!(err.offset(), Some(offset), "UTF-8 error should have offset");
        }

        /// Property 15: Buffer too small errors SHALL include required and available sizes
        ///
        /// **Property 15: Buffer Size Error**
        /// **Validates: Requirements 6.7**
        #[test]
        fn prop_buffer_error_has_sizes(required in 1usize..10000, available in 0usize..10000) {
            let err = DxError::buffer_too_small(required, available);
            let msg = err.to_string();
            
            prop_assert!(
                msg.contains(&required.to_string()),
                "Error should contain required size {}: {}", required, msg
            );
            prop_assert!(
                msg.contains(&available.to_string()),
                "Error should contain available size {}: {}", available, msg
            );
        }

        /// Property: Unsupported version errors SHALL include found and expected versions
        ///
        /// **Validates: Requirements 6.2**
        #[test]
        fn prop_version_error_has_versions(found in 0u8..255) {
            if found == DX_VERSION {
                return Ok(());
            }
            
            let err = DxError::unsupported_version(found);
            let msg = err.to_string();
            
            prop_assert!(
                msg.contains(&found.to_string()),
                "Error should contain found version {}: {}", found, msg
            );
            prop_assert!(
                msg.contains(&DX_VERSION.to_string()),
                "Error should contain expected version {}: {}", DX_VERSION, msg
            );
        }

        /// Property: Base62 errors SHALL include character and position
        ///
        /// **Validates: Requirements 6.8**
        #[test]
        fn prop_base62_error_has_details(
            char in proptest::char::any(),
            position in 0usize..1000
        ) {
            let err = DxError::base62_error(char, position, "test error");
            let msg = err.to_string();
            
            prop_assert!(
                msg.contains(&position.to_string()),
                "Error should contain position {}: {}", position, msg
            );
        }
    }

    #[test]
    fn test_source_location_multiline() {
        let input = b"line1\nline2\nline3\nline4";
        
        // Test various positions
        let cases = vec![
            (0, 1, 1),   // Start of line 1
            (3, 1, 4),   // Middle of line 1
            (5, 1, 6),   // End of line 1 (before newline)
            (6, 2, 1),   // Start of line 2
            (11, 2, 6),  // End of line 2
            (12, 3, 1),  // Start of line 3
            (18, 4, 1),  // Start of line 4
        ];
        
        for (offset, expected_line, expected_col) in cases {
            let loc = SourceLocation::from_offset(input, offset);
            assert_eq!(
                loc.line, expected_line,
                "Offset {} should be line {}, got {}",
                offset, expected_line, loc.line
            );
            assert_eq!(
                loc.column, expected_col,
                "Offset {} should be column {}, got {}",
                offset, expected_col, loc.column
            );
        }
    }

    #[test]
    fn test_error_offset_extraction() {
        // Errors with offsets
        assert_eq!(DxError::UnexpectedEof(42).offset(), Some(42));
        assert_eq!(DxError::utf8_error(100).offset(), Some(100));
        assert_eq!(DxError::DittoNoPrevious(50).offset(), Some(50));
        
        let parse_err = DxError::parse_error(b"test\ninput", 7, "test");
        assert_eq!(parse_err.offset(), Some(7));
        
        // Errors without offsets
        assert_eq!(DxError::SchemaError("test".into()).offset(), None);
        assert_eq!(DxError::IntegerOverflow.offset(), None);
        assert_eq!(DxError::Io("test".into()).offset(), None);
    }

    #[test]
    fn test_error_is_recoverable() {
        // Recoverable errors
        assert!(DxError::UnknownAlias("test".into()).is_recoverable());
        assert!(DxError::UnknownAnchor("test".into()).is_recoverable());
        assert!(DxError::TypeMismatch {
            expected: "int".into(),
            got: "string".into()
        }.is_recoverable());
        
        // Non-recoverable errors
        assert!(!DxError::UnexpectedEof(0).is_recoverable());
        assert!(!DxError::InvalidMagic(0, 0).is_recoverable());
        assert!(!DxError::IntegerOverflow.is_recoverable());
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let dx_err: DxError = io_err.into();
        
        match dx_err {
            DxError::Io(msg) => assert!(msg.contains("not found")),
            _ => panic!("Expected Io error"),
        }
    }

    #[test]
    fn test_error_from_utf8() {
        // Create invalid UTF-8
        let invalid = vec![0xFF, 0xFE];
        let result = std::str::from_utf8(&invalid);
        
        if let Err(utf8_err) = result {
            let dx_err: DxError = utf8_err.into();
            match dx_err {
                DxError::Utf8Error { offset } => assert_eq!(offset, 0),
                _ => panic!("Expected Utf8Error"),
            }
        }
    }
}
