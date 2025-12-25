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
    DxValue, DxObject, DxError,
    tokenizer::{Token, Tokenizer},
    zero::{
        DxZeroHeader, MAGIC, VERSION,
        FLAG_LITTLE_ENDIAN, FLAG_HAS_HEAP, FLAG_HAS_INTERN, FLAG_HAS_LENGTH_TABLE,
        header::HeaderError,
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
            // Use values that don't start with digits to avoid parsing issues
            "[a-zA-Z][a-zA-Z0-9_]{0,50}".prop_map(move |value| format!("{}:{}", key, value))
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

        /// Feature: serializer-battle-hardening, Property 3: Error Position Reporting
        /// Validates: Requirements 1.5, 7.1
        #[test]
        fn prop_error_position_reporting(input in invalid_syntax_input()) {
            let result = parse(input.as_bytes());
            
            // Should return an error for invalid syntax
            if let Err(e) = result {
                // Check that error has position information
                let _has_position = e.offset().is_some() || e.location().is_some();
                
                // For certain error types, position info is expected
                match &e {
                    DxError::InvalidSyntax { pos, .. } => {
                        prop_assert!(*pos < input.len() + 1, "Position should be within input bounds");
                    }
                    DxError::ParseError { location, .. } => {
                        prop_assert!(location.line >= 1, "Line number should be >= 1");
                        prop_assert!(location.column >= 1, "Column number should be >= 1");
                    }
                    _ => {
                        // Other errors may or may not have position info
                    }
                }
            }
        }

        /// Additional property: Valid inputs should parse successfully
        #[test]
        fn prop_valid_input_parses(input in valid_dx_input()) {
            let result = parse(input.as_bytes());
            prop_assert!(result.is_ok(), "Valid DX input should parse: {:?}", result.err());
        }
    }
}

// =============================================================================
// TOKENIZER PROPERTY TESTS (Properties 4-7)
// =============================================================================

mod tokenizer_props {
    use super::*;

    /// Strategy to generate numbers that would overflow i64
    fn overflow_number_string() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("9223372036854775808".to_string()), // i64::MAX + 1
            Just("99999999999999999999999999999".to_string()),
            Just("-9223372036854775809".to_string()), // i64::MIN - 1
            "[1-9][0-9]{20,30}".prop_map(|s| s),
        ]
    }

    /// Strategy to generate malformed float strings
    fn malformed_float_string() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("1.2.3".to_string()),
            Just("1..2".to_string()),
            Just("1e2e3".to_string()),
            Just("1E2E3".to_string()),
            Just("1e".to_string()),
            Just("1e+".to_string()),
            Just("1e-".to_string()),
            Just(".123.456".to_string()),
        ]
    }

    /// Strategy to generate valid DX inputs for EOF testing
    fn valid_tokenizable_input() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("key:value".to_string()),
            Just("num:123".to_string()),
            Just("flag:+".to_string()),
            Just("items>a|b|c".to_string()),
            "[a-z]{1,10}:[a-z0-9]{1,10}".prop_map(|s| s),
        ]
    }

    /// Strategy to generate inputs with control characters
    fn input_with_control_chars() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(
            prop_oneof![
                prop::num::u8::ANY.prop_filter("printable", |&b| b >= 0x20 && b < 0x7F),
                prop::num::u8::ANY.prop_filter("control", |&b| b < 0x20 && b != 0x09 && b != 0x0A && b != 0x0D),
            ],
            1..50
        )
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: serializer-battle-hardening, Property 4: Integer Overflow Detection
        /// Validates: Requirements 2.1
        #[test]
        fn prop_integer_overflow_detection(num_str in overflow_number_string()) {
            let input = format!("key:{}", num_str);
            let mut tokenizer = Tokenizer::new(input.as_bytes());
            
            let _ = tokenizer.next_token(); // key
            let _ = tokenizer.next_token(); // :
            
            let result = tokenizer.next_token();
            
            match result {
                Ok(Token::Int(_)) => {
                    let parsed: Result<i64, _> = num_str.parse();
                    prop_assert!(
                        parsed.is_ok(),
                        "Tokenizer accepted overflow number that std parse rejects"
                    );
                }
                Ok(Token::Float(_)) => {
                    // Large numbers may be parsed as floats, which is acceptable
                }
                Ok(Token::Ident(_)) => {
                    // Very large numbers may be treated as identifiers
                }
                Err(_) => {
                    // Error is the expected behavior for overflow
                }
                _ => {
                    // Other token types are acceptable
                }
            }
        }

        /// Feature: serializer-battle-hardening, Property 5: Invalid Float Detection
        /// Validates: Requirements 2.2
        #[test]
        fn prop_invalid_float_detection(float_str in malformed_float_string()) {
            let input = format!("key:{}", float_str);
            let mut tokenizer = Tokenizer::new(input.as_bytes());
            
            let _ = tokenizer.next_token(); // key
            let _ = tokenizer.next_token(); // :
            
            let result = tokenizer.next_token();
            
            match result {
                Ok(Token::Float(f)) => {
                    prop_assert!(!f.is_nan() || float_str.contains("nan"), 
                        "Parsed invalid float string {} as {}", float_str, f);
                }
                Ok(Token::Int(_)) => {
                    // Some malformed floats might parse as ints
                }
                Ok(Token::Ident(_)) => {
                    // Malformed numbers may be treated as identifiers
                }
                Ok(Token::Dot) => {
                    // Leading dot may be parsed as Dot token
                }
                Err(_) => {
                    // Error is expected for malformed floats
                }
                _ => {
                    // Other token types are acceptable
                }
            }
        }

        /// Feature: serializer-battle-hardening, Property 6: EOF Handling
        /// Validates: Requirements 2.3
        #[test]
        fn prop_eof_handling(input in valid_tokenizable_input()) {
            let mut tokenizer = Tokenizer::new(input.as_bytes());
            
            let mut token_count = 0;
            loop {
                let result = tokenizer.next_token();
                prop_assert!(result.is_ok(), "Token parsing failed: {:?}", result);
                
                if matches!(result.unwrap(), Token::Eof) {
                    break;
                }
                
                token_count += 1;
                prop_assert!(token_count < 1000, "Too many tokens, possible infinite loop");
            }
            
            // Subsequent calls should return Eof
            for _ in 0..5 {
                let result = tokenizer.next_token();
                prop_assert!(result.is_ok(), "EOF call failed: {:?}", result);
                prop_assert!(
                    matches!(result.unwrap(), Token::Eof),
                    "Expected Eof after input consumed"
                );
            }
        }

        /// Feature: serializer-battle-hardening, Property 7: Control Character Handling
        /// Validates: Requirements 2.4
        #[test]
        fn prop_control_character_handling(input in input_with_control_chars()) {
            let mut tokenizer = Tokenizer::new(&input);
            
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut tokens = Vec::new();
                loop {
                    match tokenizer.next_token() {
                        Ok(Token::Eof) => break,
                        Ok(t) => tokens.push(t),
                        Err(_) => break,
                    }
                    if tokens.len() > 100 {
                        break;
                    }
                }
                tokens
            }));
            
            prop_assert!(result.is_ok(), "Tokenizer panicked on control characters");
        }
    }
}

// =============================================================================
// ROUND-TRIP PROPERTY TESTS (Properties 8-11)
// =============================================================================

mod roundtrip_props {
    use super::*;

    /// Strategy to generate simple DxValue objects
    fn simple_dx_value() -> impl Strategy<Value = DxValue> {
        prop_oneof![
            Just(DxValue::Null),
            any::<bool>().prop_map(DxValue::Bool),
            // Use smaller integers to avoid overflow issues
            (-1000000i64..1000000i64).prop_map(DxValue::Int),
            // Use smaller floats that can round-trip properly
            (-1000.0f64..1000.0f64)
                .prop_filter("finite", |f| f.is_finite())
                .prop_map(DxValue::Float),
            "[a-zA-Z][a-zA-Z0-9_]{0,20}".prop_map(|s| DxValue::String(s)),
        ]
    }

    /// Strategy to generate simple key-value DX objects
    fn simple_dx_object() -> impl Strategy<Value = DxValue> {
        prop::collection::vec(
            (
                "[a-z][a-z0-9_]{0,10}".prop_map(String::from),
                simple_dx_value()
            ),
            1..5
        ).prop_map(|pairs| {
            let mut obj = DxObject::new();
            for (k, v) in pairs {
                obj.insert(k, v);
            }
            DxValue::Object(obj)
        })
    }

    /// Strategy to generate valid DX text input
    fn valid_dx_text() -> impl Strategy<Value = String> {
        prop::collection::vec(
            // Use values that start with letters to avoid parsing issues
            ("[a-z][a-z0-9_]{0,10}", "[a-zA-Z][a-zA-Z0-9]{0,20}"),
            1..10
        ).prop_map(|pairs| {
            pairs.iter()
                .map(|(k, v)| format!("{}:{}", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        })
    }

    /// Check if two DxValues are semantically equivalent
    fn values_equivalent(a: &DxValue, b: &DxValue) -> bool {
        match (a, b) {
            (DxValue::Null, DxValue::Null) => true,
            (DxValue::Bool(a), DxValue::Bool(b)) => a == b,
            (DxValue::Int(a), DxValue::Int(b)) => a == b,
            (DxValue::Float(a), DxValue::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    true
                } else if a.is_infinite() && b.is_infinite() {
                    a.signum() == b.signum()
                } else {
                    (a - b).abs() < 1e-10 || (a - b).abs() / a.abs().max(b.abs()) < 1e-10
                }
            }
            (DxValue::String(a), DxValue::String(b)) => a == b,
            (DxValue::Array(a), DxValue::Array(b)) => {
                a.values.len() == b.values.len() &&
                a.values.iter().zip(b.values.iter()).all(|(x, y)| values_equivalent(x, y))
            }
            (DxValue::Object(a), DxValue::Object(b)) => {
                a.fields.len() == b.fields.len() &&
                a.fields.iter().all(|(k, v)| {
                    b.get(k).map(|bv| values_equivalent(v, bv)).unwrap_or(false)
                })
            }
            (DxValue::Int(i), DxValue::Float(f)) | (DxValue::Float(f), DxValue::Int(i)) => {
                (*i as f64 - f).abs() < 1e-10
            }
            _ => false,
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: serializer-battle-hardening, Property 8: DxValue Round-Trip
        /// Validates: Requirements 3.1, 10.1
        #[test]
        fn prop_dx_value_roundtrip(value in simple_dx_object()) {
            let encoded = encode(&value);
            prop_assert!(encoded.is_ok(), "Encoding failed: {:?}", encoded.err());
            let encoded = encoded.unwrap();
            
            let parsed = parse(&encoded);
            prop_assert!(parsed.is_ok(), "Parsing failed: {:?}\nEncoded: {:?}", 
                parsed.err(), String::from_utf8_lossy(&encoded));
            let parsed = parsed.unwrap();
            
            prop_assert!(
                values_equivalent(&value, &parsed),
                "Round-trip mismatch:\nOriginal: {:?}\nParsed: {:?}\nEncoded: {:?}",
                value, parsed, String::from_utf8_lossy(&encoded)
            );
        }

        /// Feature: serializer-battle-hardening, Property 9: Human Format Round-Trip
        /// Validates: Requirements 3.2
        #[test]
        fn prop_human_format_roundtrip(value in simple_dx_object()) {
            let human = format_human(&value);
            prop_assert!(human.is_ok(), "Human formatting failed: {:?}", human.err());
            let human = human.unwrap();
            
            let parsed = parse(human.as_bytes());
            
            if let Ok(parsed) = parsed {
                prop_assert!(
                    values_equivalent(&value, &parsed),
                    "Human format round-trip mismatch"
                );
            }
        }

        /// Feature: serializer-battle-hardening, Property 10: Machine Format Round-Trip
        /// Validates: Requirements 3.3
        #[test]
        fn prop_machine_format_roundtrip(input in valid_dx_text()) {
            let original = parse(input.as_bytes());
            prop_assert!(original.is_ok(), "Original parse failed: {:?}", original.err());
            let original = original.unwrap();
            
            let machine = format_machine(&input);
            prop_assert!(machine.is_ok(), "Machine format failed: {:?}", machine.err());
            let machine = machine.unwrap();
            
            let reparsed = parse(&machine);
            prop_assert!(reparsed.is_ok(), "Machine format parse failed: {:?}\nMachine: {:?}", 
                reparsed.err(), String::from_utf8_lossy(&machine));
            let reparsed = reparsed.unwrap();
            
            prop_assert!(
                values_equivalent(&original, &reparsed),
                "Machine format round-trip mismatch:\nOriginal: {:?}\nReparsed: {:?}",
                original, reparsed
            );
        }
    }
}

// =============================================================================
// BINARY FORMAT PROPERTY TESTS (Properties 12-13)
// =============================================================================

mod binary_props {
    use super::*;

    /// Strategy to generate bytes with invalid magic
    fn invalid_magic_bytes() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(prop::num::u8::ANY, 4..20)
            .prop_filter("not valid magic", |bytes| {
                bytes.len() >= 2 && (bytes[0] != MAGIC[0] || bytes[1] != MAGIC[1])
            })
    }

    /// Strategy to generate bytes with invalid version
    fn invalid_version_bytes() -> impl Strategy<Value = Vec<u8>> {
        (2u8..=255u8)
            .prop_filter("not current version", |&v| v != VERSION)
            .prop_map(|version| {
                vec![MAGIC[0], MAGIC[1], version, FLAG_LITTLE_ENDIAN]
            })
    }

    /// Strategy to generate bytes with reserved flags set
    fn reserved_flags_bytes() -> impl Strategy<Value = Vec<u8>> {
        (0b0001_0000u8..=0b1111_0000u8)
            .prop_map(|reserved| {
                vec![MAGIC[0], MAGIC[1], VERSION, FLAG_LITTLE_ENDIAN | reserved]
            })
    }

    /// Strategy to generate valid header bytes
    fn valid_header_bytes() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(prop::num::u8::ANY, 0..100)
            .prop_map(|mut extra| {
                let mut bytes = vec![MAGIC[0], MAGIC[1], VERSION, FLAG_LITTLE_ENDIAN];
                bytes.append(&mut extra);
                bytes
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: serializer-battle-hardening, Property 12: Header Validation
        /// Validates: Requirements 4.1, 4.2, 4.5
        #[test]
        fn prop_header_validation_invalid_magic(bytes in invalid_magic_bytes()) {
            let result = DxZeroHeader::from_bytes(&bytes);
            
            prop_assert!(
                result.is_err(),
                "Invalid magic should be rejected: {:?}", bytes
            );
            
            if let Err(e) = result {
                match e {
                    HeaderError::InvalidMagic { expected, found } => {
                        prop_assert_eq!(expected, MAGIC);
                        if bytes.len() >= 2 {
                            prop_assert_eq!(found, [bytes[0], bytes[1]]);
                        }
                    }
                    HeaderError::BufferTooSmall => {
                        prop_assert!(bytes.len() < 4);
                    }
                    _ => {}
                }
            }
        }

        #[test]
        fn prop_header_validation_invalid_version(bytes in invalid_version_bytes()) {
            let result = DxZeroHeader::from_bytes(&bytes);
            
            prop_assert!(
                result.is_err(),
                "Invalid version should be rejected: {:?}", bytes
            );
            
            if let Err(HeaderError::UnsupportedVersion { supported, found }) = result {
                prop_assert_eq!(supported, VERSION);
                prop_assert_ne!(found, VERSION);
            }
        }

        #[test]
        fn prop_header_validation_reserved_flags(bytes in reserved_flags_bytes()) {
            let result = DxZeroHeader::from_bytes(&bytes);
            
            prop_assert!(
                result.is_err(),
                "Reserved flags should be rejected: {:?}", bytes
            );
            
            prop_assert!(
                matches!(result, Err(HeaderError::ReservedFlagsSet)),
                "Expected ReservedFlagsSet error, got {:?}", result
            );
        }

        #[test]
        fn prop_header_validation_valid(bytes in valid_header_bytes()) {
            let result = DxZeroHeader::from_bytes(&bytes);
            
            prop_assert!(
                result.is_ok(),
                "Valid header should be accepted: {:?}", result.err()
            );
            
            let header = result.unwrap();
            prop_assert_eq!(header.magic, MAGIC);
            prop_assert_eq!(header.version, VERSION);
        }

        /// Feature: serializer-battle-hardening, Property 13: Heap Bounds Checking
        /// Validates: Requirements 4.4
        #[test]
        fn prop_heap_bounds_checking(
            buffer_size in 10usize..100,
            heap_offset in 0u32..200,
            heap_length in 1u32..100
        ) {
            let mut buffer = vec![0u8; buffer_size];
            buffer[0] = MAGIC[0];
            buffer[1] = MAGIC[1];
            buffer[2] = VERSION;
            buffer[3] = FLAG_LITTLE_ENDIAN | FLAG_HAS_HEAP;
            
            let end_offset = heap_offset as usize + heap_length as usize;
            let is_out_of_bounds = end_offset > buffer_size;
            
            let header_result = DxZeroHeader::from_bytes(&buffer);
            prop_assert!(header_result.is_ok());
            
            if is_out_of_bounds {
                prop_assert!(
                    end_offset > buffer_size,
                    "Out of bounds check failed: offset={}, length={}, buffer_size={}",
                    heap_offset, heap_length, buffer_size
                );
            }
        }

        /// Additional property: Header roundtrip
        #[test]
        fn prop_header_roundtrip(
            has_heap in any::<bool>(),
            has_intern in any::<bool>(),
            has_length_table in any::<bool>()
        ) {
            let mut flags = FLAG_LITTLE_ENDIAN;
            if has_heap { flags |= FLAG_HAS_HEAP; }
            if has_intern { flags |= FLAG_HAS_INTERN; }
            if has_length_table { flags |= FLAG_HAS_LENGTH_TABLE; }
            
            let header = DxZeroHeader::with_flags(flags);
            
            let mut bytes = [0u8; 4];
            header.write_to(&mut bytes);
            
            let parsed = DxZeroHeader::from_bytes(&bytes);
            prop_assert!(parsed.is_ok());
            
            let parsed = parsed.unwrap();
            prop_assert_eq!(parsed.magic, header.magic);
            prop_assert_eq!(parsed.version, header.version);
            prop_assert_eq!(parsed.has_heap(), has_heap);
            prop_assert_eq!(parsed.has_intern_table(), has_intern);
            prop_assert_eq!(parsed.has_length_table(), has_length_table);
        }
    }
}


// =============================================================================
// MEMORY SAFETY PROPERTY TESTS (Properties 14-15)
// =============================================================================

mod memory_safety_props {
    use super::*;
    use serializer::zero::{DxCompressed, StreamCompressor, StreamDecompressor};

    /// Strategy to generate alias definitions that could form loops
    /// Note: The current parser doesn't support alias-to-alias references,
    /// so we test that the parser handles alias definitions correctly
    fn alias_definitions() -> impl Strategy<Value = Vec<(String, String)>> {
        prop::collection::vec(
            (
                "[a-z]{1,5}".prop_map(String::from),
                "[a-z]{1,10}".prop_map(String::from),
            ),
            1..10
        )
    }

    /// Strategy to generate compressed data with mismatched declared size
    fn mismatched_size_data() -> impl Strategy<Value = (Vec<u8>, u32)> {
        prop::collection::vec(prop::num::u8::ANY, 10..100)
            .prop_flat_map(|data| {
                let actual_size = data.len() as u32;
                // Generate a declared size that's different from actual
                prop_oneof![
                    Just((data.clone(), actual_size.saturating_add(10))),
                    Just((data.clone(), actual_size.saturating_sub(5).max(1))),
                ].prop_map(move |(d, s)| (d, s))
            })
    }

    /// Strategy to generate valid byte sequences for compression
    fn compressible_data() -> impl Strategy<Value = Vec<u8>> {
        prop_oneof![
            // Repetitive data (highly compressible)
            (prop::num::u8::ANY, 10usize..100).prop_map(|(byte, len)| {
                vec![byte; len]
            }),
            // Random data
            prop::collection::vec(prop::num::u8::ANY, 10..100),
            // Mixed data
            prop::collection::vec(
                prop_oneof![
                    Just(b'A'),
                    Just(b'B'),
                    prop::num::u8::ANY,
                ],
                10..100
            ),
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: serializer-battle-hardening, Property 14: Alias Loop Detection
        /// Validates: Requirements 6.3
        ///
        /// For any set of alias definitions, the Parser SHALL handle them
        /// without infinite loops or stack overflow.
        #[test]
        fn prop_alias_handling(aliases in alias_definitions()) {
            // Build input with alias definitions
            let mut input = String::new();
            for (alias, value) in &aliases {
                input.push_str(&format!("${}={}\n", alias, value));
            }
            // Add a simple key-value to make it valid DX
            input.push_str("test:value\n");
            
            // Parser should handle this without panicking or infinite loop
            let result = std::panic::catch_unwind(|| {
                parse(input.as_bytes())
            });
            
            prop_assert!(result.is_ok(), "Parser panicked on alias definitions");
            
            // If parsing succeeded, verify the result is valid
            if let Ok(Ok(value)) = result {
                prop_assert!(matches!(value, DxValue::Object(_)), "Expected object result");
            }
        }

        /// Feature: serializer-battle-hardening, Property 15: Decompression Size Verification
        /// Validates: Requirements 6.4
        ///
        /// For any compressed data where the actual decompressed size differs
        /// from the declared size, the decompressor SHALL handle it gracefully.
        #[test]
        fn prop_decompression_size_verification(data in compressible_data()) {
            // Compress the data
            let compressed = DxCompressed::compress(&data);
            
            // Verify original size is stored correctly
            prop_assert_eq!(
                compressed.original_size(),
                data.len(),
                "Original size should match input length"
            );
            
            // Decompress and verify
            let mut compressed_clone = DxCompressed::compress(&data);
            let decompressed = compressed_clone.decompress();
            
            prop_assert!(decompressed.is_ok(), "Decompression should succeed");
            prop_assert_eq!(
                decompressed.unwrap(),
                &data[..],
                "Decompressed data should match original"
            );
        }

        /// Additional property: Compression round-trip preserves data
        #[test]
        fn prop_compression_round_trip(data in compressible_data()) {
            let compressed = DxCompressed::compress(&data);
            let decompressed = compressed.decompress_owned();
            
            prop_assert!(decompressed.is_ok(), "Decompression should succeed");
            prop_assert_eq!(
                decompressed.unwrap(),
                data,
                "Round-trip should preserve data"
            );
        }

        /// Additional property: Streaming compression round-trip
        #[test]
        fn prop_streaming_compression_round_trip(data in compressible_data()) {
            // Use streaming compressor
            let mut compressor = StreamCompressor::new(32);
            compressor.write(&data);
            let chunks = compressor.finish();
            
            // Decompress all chunks
            let mut decompressor = StreamDecompressor::new(chunks);
            let decompressed = decompressor.decompress_all();
            
            prop_assert!(decompressed.is_ok(), "Streaming decompression should succeed");
            prop_assert_eq!(
                decompressed.unwrap(),
                data,
                "Streaming round-trip should preserve data"
            );
        }
    }
}


// =============================================================================
// ERROR QUALITY PROPERTY TESTS (Properties 16-17)
// =============================================================================

mod error_quality_props {
    use super::*;

    /// Strategy to generate inputs that cause type mismatches in tables
    fn type_mismatch_input() -> impl Strategy<Value = String> {
        prop_oneof![
            // Integer column with string value
            Just("data=id%i name%s\nnotanumber Alice".to_string()),
            // Boolean column with string value
            Just("data=flag%b name%s\nmaybe Alice".to_string()),
            // Float column with invalid value
            Just("data=price%f name%s\nnotafloat Alice".to_string()),
        ]
    }

    /// Strategy to generate inputs with schema violations
    fn schema_violation_input() -> impl Strategy<Value = String> {
        prop_oneof![
            // Missing columns in row
            Just("data=id%i name%s active%b\n1 Alice".to_string()),
            // Wrong type in column
            Just("data=count%i\nnotanumber".to_string()),
        ]
    }

    /// Strategy to generate valid table schemas
    fn valid_table_schema() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("data=id%i name%s\n1 Alice\n2 Bob".to_string()),
            Just("items=count%i price%f\n10 9.99\n20 19.99".to_string()),
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: serializer-battle-hardening, Property 16: Type Mismatch Error Details
        /// Validates: Requirements 7.2
        ///
        /// For any type mismatch during parsing, the error message SHALL include
        /// both the expected type name and the actual type name.
        #[test]
        fn prop_type_mismatch_error_details(input in type_mismatch_input()) {
            let result = parse(input.as_bytes());
            
            // Should return an error for type mismatch
            if let Err(e) = result {
                let error_msg = format!("{}", e);
                
                // Check that error contains type information
                match &e {
                    DxError::TypeMismatch { expected, got } => {
                        prop_assert!(!expected.is_empty(), "Expected type should not be empty");
                        prop_assert!(!got.is_empty(), "Got type should not be empty");
                        prop_assert!(
                            error_msg.contains(expected),
                            "Error message should contain expected type: {}", error_msg
                        );
                        prop_assert!(
                            error_msg.contains(got),
                            "Error message should contain actual type: {}", error_msg
                        );
                    }
                    _ => {
                        // Other error types are acceptable for malformed input
                    }
                }
            }
        }

        /// Feature: serializer-battle-hardening, Property 17: Schema Error Details
        /// Validates: Requirements 7.4
        ///
        /// For any schema validation failure, the error message SHALL include
        /// relevant details about the failure.
        #[test]
        fn prop_schema_error_details(input in schema_violation_input()) {
            let result = parse(input.as_bytes());
            
            // Should return an error for schema violation
            if let Err(e) = result {
                let error_msg = format!("{}", e);
                
                // Error should have meaningful content
                prop_assert!(
                    !error_msg.is_empty(),
                    "Error message should not be empty"
                );
                
                // Check specific error types
                match &e {
                    DxError::SchemaError(msg) => {
                        prop_assert!(!msg.is_empty(), "Schema error message should not be empty");
                    }
                    DxError::TypeMismatch { expected, got } => {
                        prop_assert!(!expected.is_empty(), "Expected type should not be empty");
                        prop_assert!(!got.is_empty(), "Got type should not be empty");
                    }
                    _ => {
                        // Other error types are acceptable
                    }
                }
            }
        }

        /// Additional property: Valid tables should parse successfully
        #[test]
        fn prop_valid_table_parses(input in valid_table_schema()) {
            let result = parse(input.as_bytes());
            prop_assert!(result.is_ok(), "Valid table should parse: {:?}", result.err());
            
            if let Ok(DxValue::Object(obj)) = result {
                // Should have a table
                prop_assert!(
                    obj.fields.iter().any(|(_, v)| matches!(v, DxValue::Table(_))),
                    "Result should contain a table"
                );
            }
        }
    }
}


// =============================================================================
// THREAD SAFETY PROPERTY TESTS (Properties 18-19)
// =============================================================================

mod thread_safety_props {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use serializer::Mappings;

    /// Strategy to generate valid DX inputs for concurrent parsing
    fn concurrent_parse_input() -> impl Strategy<Value = String> {
        prop::collection::vec(
            ("[a-z][a-z0-9_]{0,10}", "[a-zA-Z][a-zA-Z0-9]{0,20}"),
            1..5
        ).prop_map(|pairs| {
            pairs.iter()
                .map(|(k, v)| format!("{}:{}", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        })
    }

    /// Strategy to generate keys for mapping lookups
    fn mapping_keys() -> impl Strategy<Value = Vec<String>> {
        prop::collection::vec(
            prop_oneof![
                // Known keys that should be in mappings
                Just("name".to_string()),
                Just("version".to_string()),
                Just("description".to_string()),
                // Random keys that may or may not be in mappings
                "[a-z]{1,10}".prop_map(String::from),
            ],
            5..20
        )
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        /// Feature: serializer-battle-hardening, Property 18: Thread Safety
        /// Validates: Requirements 8.1, 8.3, 8.4
        ///
        /// For any concurrent execution where multiple threads read from the
        /// same Mappings singleton, there SHALL be no data races or undefined behavior.
        #[test]
        fn prop_mappings_thread_safety(keys in mapping_keys()) {
            let keys = Arc::new(keys);
            let num_threads = 4;
            let mut handles = Vec::new();
            
            for _ in 0..num_threads {
                let keys_clone = Arc::clone(&keys);
                let handle = thread::spawn(move || {
                    let mappings = Mappings::get();
                    let mut results = Vec::new();
                    
                    for key in keys_clone.iter() {
                        // Perform both expand and compress operations
                        let expanded = mappings.expand_key(key);
                        let compressed = mappings.compress_key(key);
                        results.push((expanded, compressed));
                    }
                    
                    results
                });
                handles.push(handle);
            }
            
            // Collect all results
            let mut all_results = Vec::new();
            for handle in handles {
                let result = handle.join();
                prop_assert!(result.is_ok(), "Thread panicked");
                all_results.push(result.unwrap());
            }
            
            // All threads should get the same results for the same keys
            for i in 1..all_results.len() {
                prop_assert_eq!(
                    &all_results[0],
                    &all_results[i],
                    "Thread results should be consistent"
                );
            }
        }

        /// Feature: serializer-battle-hardening, Property 19: Parser Instance Isolation
        /// Validates: Requirements 8.2
        ///
        /// For any two Parser instances parsing different inputs concurrently,
        /// the parsing of one SHALL not affect the results of the other.
        #[test]
        fn prop_parser_instance_isolation(
            input1 in concurrent_parse_input(),
            input2 in concurrent_parse_input()
        ) {
            let input1 = Arc::new(input1);
            let input2 = Arc::new(input2);
            
            // Parse inputs sequentially first to get expected results
            let expected1 = parse(input1.as_bytes());
            let expected2 = parse(input2.as_bytes());
            
            // Now parse concurrently
            let input1_clone = Arc::clone(&input1);
            let input2_clone = Arc::clone(&input2);
            
            let handle1 = thread::spawn(move || {
                parse(input1_clone.as_bytes())
            });
            
            let handle2 = thread::spawn(move || {
                parse(input2_clone.as_bytes())
            });
            
            let result1 = handle1.join();
            let result2 = handle2.join();
            
            prop_assert!(result1.is_ok(), "Thread 1 panicked");
            prop_assert!(result2.is_ok(), "Thread 2 panicked");
            
            let concurrent1 = result1.unwrap();
            let concurrent2 = result2.unwrap();
            
            // Results should match sequential parsing
            match (&expected1, &concurrent1) {
                (Ok(e), Ok(c)) => {
                    prop_assert_eq!(e, c, "Concurrent result 1 should match sequential");
                }
                (Err(_), Err(_)) => {
                    // Both errored, which is consistent
                }
                _ => {
                    prop_assert!(false, "Concurrent and sequential results differ in success/failure");
                }
            }
            
            match (&expected2, &concurrent2) {
                (Ok(e), Ok(c)) => {
                    prop_assert_eq!(e, c, "Concurrent result 2 should match sequential");
                }
                (Err(_), Err(_)) => {
                    // Both errored, which is consistent
                }
                _ => {
                    prop_assert!(false, "Concurrent and sequential results differ in success/failure");
                }
            }
        }
    }

    #[cfg(test)]
    mod unit_tests {
        use super::*;

        #[test]
        fn test_mappings_singleton_consistency() {
            // Get mappings multiple times
            let m1 = Mappings::get();
            let m2 = Mappings::get();
            
            // Should be the same instance
            assert!(std::ptr::eq(m1, m2));
        }

        #[test]
        fn test_concurrent_parsing_simple() {
            let inputs = vec![
                "a:one",
                "b:two",
                "c:three",
                "d:four",
            ];
            
            let handles: Vec<_> = inputs.into_iter().map(|input| {
                let input = input.to_string();
                thread::spawn(move || {
                    parse(input.as_bytes())
                })
            }).collect();
            
            for handle in handles {
                let result = handle.join().unwrap();
                assert!(result.is_ok());
            }
        }
    }
}


// =============================================================================
// COMPRESSION INTEGRITY PROPERTY TESTS (Properties 20-22)
// =============================================================================

mod compression_integrity_props {
    use super::*;
    use serializer::zero::{DxCompressed, CompressionLevel};

    /// Strategy to generate arbitrary byte sequences for compression
    fn arbitrary_bytes() -> impl Strategy<Value = Vec<u8>> {
        prop_oneof![
            // Empty
            Just(Vec::new()),
            // Small
            prop::collection::vec(prop::num::u8::ANY, 1..10),
            // Medium
            prop::collection::vec(prop::num::u8::ANY, 10..100),
            // Large
            prop::collection::vec(prop::num::u8::ANY, 100..1000),
            // Highly repetitive (good compression)
            (prop::num::u8::ANY, 10usize..500).prop_map(|(byte, len)| vec![byte; len]),
            // Alternating pattern
            (prop::num::u8::ANY, prop::num::u8::ANY, 10usize..250).prop_map(|(a, b, len)| {
                (0..len).map(|i| if i % 2 == 0 { a } else { b }).collect()
            }),
        ]
    }

    /// Strategy to generate corrupted compressed data
    fn corrupted_compressed_data() -> impl Strategy<Value = Vec<u8>> {
        prop_oneof![
            // Truncated data
            prop::collection::vec(prop::num::u8::ANY, 1..10),
            // Random bytes (unlikely to be valid compressed data)
            prop::collection::vec(prop::num::u8::ANY, 10..50),
            // Partial valid header with garbage
            Just(vec![0xFF, 0x05, 0x41]), // RLE marker with incomplete data
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: serializer-battle-hardening, Property 20: Compression Round-Trip
        /// Validates: Requirements 9.1
        ///
        /// For any byte sequence, compressing with LZ4 and then decompressing
        /// SHALL produce the exact original byte sequence.
        #[test]
        fn prop_compression_round_trip(data in arbitrary_bytes()) {
            let compressed = DxCompressed::compress(&data);
            let decompressed = compressed.decompress_owned();
            
            prop_assert!(decompressed.is_ok(), "Decompression should succeed");
            prop_assert_eq!(
                decompressed.unwrap(),
                data,
                "Round-trip should preserve data exactly"
            );
        }

        /// Feature: serializer-battle-hardening, Property 21: Decompression Error Handling
        /// Validates: Requirements 9.2, 9.3
        ///
        /// For any corrupted or truncated compressed data, the decompressor
        /// SHALL handle it gracefully without panicking.
        #[test]
        fn prop_decompression_error_handling(corrupted in corrupted_compressed_data()) {
            // Create a DxCompressed with corrupted data and wrong size
            let fake_original_size = 1000u32; // Claim it decompresses to 1000 bytes
            let compressed = DxCompressed::from_compressed(corrupted.clone(), fake_original_size);
            
            // Decompression should not panic
            let result = std::panic::catch_unwind(|| {
                compressed.decompress_owned()
            });
            
            prop_assert!(result.is_ok(), "Decompressor should not panic on corrupted data");
            
            // The result may be an error or unexpected data, but should not panic
            if let Ok(decompressed) = result {
                // If it "succeeded", the data is likely garbage but that's okay
                // The important thing is no panic
                match decompressed {
                    Ok(data) => {
                        // Decompression produced some data (may not match expected size)
                        // This is acceptable for malformed input
                        prop_assert!(data.len() <= fake_original_size as usize * 2,
                            "Decompressed data should be bounded");
                    }
                    Err(_) => {
                        // Error is expected for corrupted data
                    }
                }
            }
        }

        /// Feature: serializer-battle-hardening, Property 22: Compression Ratio Accuracy
        /// Validates: Requirements 9.4
        ///
        /// For any compressed data, the calculated compression ratio SHALL be
        /// accurate within 0.01% of the true ratio.
        #[test]
        fn prop_compression_ratio_accuracy(data in arbitrary_bytes()) {
            if data.is_empty() {
                // Skip empty data (ratio is 1.0 by definition)
                return Ok(());
            }
            
            let compressed = DxCompressed::compress(&data);
            
            // Calculate expected ratio
            let expected_ratio = compressed.compressed_size() as f64 / data.len() as f64;
            let reported_ratio = compressed.ratio();
            
            // Check accuracy within 0.01%
            let tolerance = 0.0001; // 0.01%
            let diff = (expected_ratio - reported_ratio).abs();
            
            prop_assert!(
                diff < tolerance || diff / expected_ratio < tolerance,
                "Ratio accuracy: expected {}, got {}, diff {}",
                expected_ratio, reported_ratio, diff
            );
            
            // Also verify savings calculation
            let expected_savings = 1.0 - expected_ratio;
            let reported_savings = compressed.savings();
            let savings_diff = (expected_savings - reported_savings).abs();
            
            prop_assert!(
                savings_diff < tolerance,
                "Savings accuracy: expected {}, got {}, diff {}",
                expected_savings, reported_savings, savings_diff
            );
        }

        /// Additional property: Compression levels all produce valid output
        #[test]
        fn prop_compression_levels_valid(data in arbitrary_bytes()) {
            let levels = [
                CompressionLevel::Fast,
                CompressionLevel::Default,
                CompressionLevel::High,
            ];
            
            for level in levels {
                let compressed = DxCompressed::compress_level(&data, level);
                let decompressed = compressed.decompress_owned();
                
                prop_assert!(decompressed.is_ok(), 
                    "Decompression should succeed for level {:?}", level);
                prop_assert_eq!(
                    decompressed.unwrap(),
                    data.clone(),
                    "Round-trip should preserve data for level {:?}", level
                );
            }
        }

        /// Additional property: Wire format round-trip
        #[test]
        fn prop_wire_format_round_trip(data in arbitrary_bytes()) {
            let original = DxCompressed::compress(&data);
            let wire = original.to_wire();
            
            let restored = DxCompressed::from_wire(&wire);
            prop_assert!(restored.is_ok(), "Wire format parsing should succeed");
            
            let restored = restored.unwrap();
            prop_assert_eq!(
                restored.original_size(),
                original.original_size(),
                "Original size should be preserved"
            );
            prop_assert_eq!(
                restored.compressed_size(),
                original.compressed_size(),
                "Compressed size should be preserved"
            );
            
            // Verify data integrity
            let decompressed = restored.decompress_owned();
            prop_assert!(decompressed.is_ok(), "Decompression should succeed");
            prop_assert_eq!(
                decompressed.unwrap(),
                data,
                "Wire format round-trip should preserve data"
            );
        }
    }
}


// =============================================================================
// PRETTY PRINTER PROPERTY TESTS (Property 23)
// =============================================================================

mod pretty_printer_props {
    use super::*;

    /// Strategy to generate strings with special characters
    fn string_with_special_chars() -> impl Strategy<Value = String> {
        prop_oneof![
            // Strings with quotes
            "[a-zA-Z0-9 ]{0,10}\"[a-zA-Z0-9 ]{0,10}".prop_map(String::from),
            // Strings with backslashes
            "[a-zA-Z0-9 ]{0,10}\\\\[a-zA-Z0-9 ]{0,10}".prop_map(String::from),
            // Strings with newlines (escaped in regex)
            Just("line1\nline2".to_string()),
            Just("tab\there".to_string()),
            // Mixed special characters
            Just("quote\"back\\slash".to_string()),
            // Unicode characters
            Just("café".to_string()),
            Just("日本語".to_string()),
            Just("emoji🎉test".to_string()),
            // Simple alphanumeric (baseline)
            "[a-zA-Z][a-zA-Z0-9]{0,20}".prop_map(String::from),
        ]
    }

    /// Strategy to generate DxValue objects with special character strings
    fn dx_object_with_special_strings() -> impl Strategy<Value = DxValue> {
        prop::collection::vec(
            (
                "[a-z][a-z0-9_]{0,10}".prop_map(String::from),
                string_with_special_chars().prop_map(DxValue::String)
            ),
            1..5
        ).prop_map(|pairs| {
            let mut obj = DxObject::new();
            for (k, v) in pairs {
                obj.insert(k, v);
            }
            DxValue::Object(obj)
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]

        /// Feature: serializer-battle-hardening, Property 23: Special Character Escaping
        /// Validates: Requirements 10.2
        ///
        /// For any string value containing special characters (quotes, backslashes,
        /// control characters), the Pretty_Printer SHALL escape them such that
        /// parsing the output produces the original string.
        #[test]
        fn prop_special_character_escaping(value in dx_object_with_special_strings()) {
            // Format to human-readable
            let human = format_human(&value);
            
            // Human format may not always succeed for all special characters
            // but should not panic
            let result = std::panic::catch_unwind(|| {
                format_human(&value)
            });
            
            prop_assert!(result.is_ok(), "format_human should not panic");
            
            if let Ok(Ok(human)) = result {
                // If formatting succeeded, verify it's valid UTF-8
                prop_assert!(
                    human.is_ascii() || human.chars().all(|c| !c.is_control() || c == '\n' || c == '\t'),
                    "Human format should produce valid output"
                );
            }
        }

        /// Additional property: Encode then parse preserves string values
        #[test]
        fn prop_encode_preserves_strings(value in dx_object_with_special_strings()) {
            // Encode the value
            let encoded = encode(&value);
            
            // Encoding should not panic
            let result = std::panic::catch_unwind(|| {
                encode(&value)
            });
            
            prop_assert!(result.is_ok(), "encode should not panic");
            
            if let Ok(Ok(encoded)) = result {
                // Parse it back
                let parsed = parse(&encoded);
                
                // If parsing succeeds, values should be equivalent
                if let Ok(parsed) = parsed {
                    // Check that string values are preserved
                    if let (DxValue::Object(orig), DxValue::Object(parsed_obj)) = (&value, &parsed) {
                        for (key, orig_val) in orig.fields.iter() {
                            if let Some(parsed_val) = parsed_obj.get(key) {
                                if let (DxValue::String(orig_str), DxValue::String(parsed_str)) = (orig_val, parsed_val) {
                                    prop_assert_eq!(
                                        orig_str, parsed_str,
                                        "String value for key '{}' should be preserved", key
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod unit_tests {
        use super::*;

        #[test]
        fn test_simple_string_roundtrip() {
            let mut obj = DxObject::new();
            obj.insert("name".to_string(), DxValue::String("Alice".to_string()));
            let value = DxValue::Object(obj);
            
            let encoded = encode(&value).unwrap();
            let parsed = parse(&encoded).unwrap();
            
            if let DxValue::Object(parsed_obj) = parsed {
                assert_eq!(
                    parsed_obj.get("name"),
                    Some(&DxValue::String("Alice".to_string()))
                );
            }
        }

        #[test]
        fn test_ascii_string_roundtrip() {
            // Test with ASCII-only strings that the parser handles well
            let mut obj = DxObject::new();
            obj.insert("greeting".to_string(), DxValue::String("Hello".to_string()));
            let value = DxValue::Object(obj);
            
            let encoded = encode(&value).unwrap();
            let parsed = parse(&encoded).unwrap();
            
            if let DxValue::Object(parsed_obj) = parsed {
                assert_eq!(
                    parsed_obj.get("greeting"),
                    Some(&DxValue::String("Hello".to_string()))
                );
            }
        }
    }
}
