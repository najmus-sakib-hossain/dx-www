# Design Document: Serializer Battle Hardening

## Overview

This design document specifies the implementation approach for battle-hardening the DX serializer codebase. The goal is to ensure the serializer is production-ready, secure, and robust against edge cases, malformed input, and adversarial attacks.

The implementation focuses on three key areas:
1. **Defensive Input Handling** - Validating and sanitizing all inputs
2. **Comprehensive Property Testing** - Using property-based testing to verify correctness invariants
3. **Error Quality Improvements** - Providing actionable, precise error messages

## Architecture

The battle-hardening effort touches multiple components of the serializer:

```
┌─────────────────────────────────────────────────────────────────┐
│                        Input Layer                               │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  Tokenizer   │  │   Parser     │  │  Converters  │          │
│  │  - UTF-8     │  │  - Recursion │  │  - JSON      │          │
│  │  - Overflow  │  │  - Aliases   │  │  - YAML      │          │
│  │  - Control   │  │  - Tables    │  │  - TOML      │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
├─────────────────────────────────────────────────────────────────┤
│                       Core Layer                                 │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   DxValue    │  │  DxDocument  │  │   Schema     │          │
│  │  - Types     │  │  - Sections  │  │  - Validate  │          │
│  │  - Equality  │  │  - Context   │  │  - Types     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
├─────────────────────────────────────────────────────────────────┤
│                      Binary Layer                                │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Header     │  │    Slots     │  │  Compression │          │
│  │  - Magic     │  │  - Inline    │  │  - LZ4       │          │
│  │  - Version   │  │  - Heap      │  │  - Verify    │          │
│  │  - Flags     │  │  - Bounds    │  │  - Ratio     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Input Validation Module

New validation functions to be added to existing modules:

```rust
// In tokenizer.rs
impl Tokenizer {
    /// Validate input size before parsing
    pub fn validate_input_size(input: &[u8], max_size: usize) -> Result<()>;
    
    /// Check for and handle control characters
    pub fn handle_control_char(&mut self, byte: u8) -> Result<Token>;
    
    /// Parse number with overflow detection
    pub fn read_number_checked(&mut self) -> Result<Token>;
}

// In parser.rs
impl Parser {
    /// Track recursion depth during parsing
    fn check_recursion_depth(&self, current: usize) -> Result<()>;
    
    /// Detect alias expansion loops
    fn check_alias_loop(&self, alias: &str, visited: &HashSet<String>) -> Result<()>;
}
```

### 2. Error Enhancement Module

Enhanced error types with detailed context:

```rust
// In error.rs
pub struct ParseErrorContext {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub expected: Vec<String>,
    pub found: String,
    pub context_snippet: String,
}

impl DxError {
    /// Create error with full position context
    pub fn with_context(self, input: &[u8], offset: usize) -> Self;
    
    /// Add suggestions for similar valid values
    pub fn with_suggestions(self, suggestions: Vec<String>) -> Self;
}
```

### 3. Binary Validation Module

Enhanced validation for DX-Zero format:

```rust
// In zero/deserialize.rs
impl DxMmap {
    /// Validate heap reference is within bounds
    pub fn validate_heap_ref(&self, offset: u32, length: u32) -> Result<()>;
    
    /// Validate all slot references before access
    pub fn validate_all_refs(&self) -> Result<()>;
}

// In zero/compress.rs
impl DxCompressed {
    /// Verify decompressed size matches declared size
    pub fn verify_size(&self, declared: usize) -> Result<()>;
    
    /// Detect truncated compressed data
    pub fn detect_truncation(&self) -> Result<()>;
}
```

## Data Models

### Validation Limits

```rust
/// Maximum input size (100 MB)
pub const MAX_INPUT_SIZE: usize = 100 * 1024 * 1024;

/// Maximum recursion depth for nested structures
pub const MAX_RECURSION_DEPTH: usize = 1000;

/// Maximum table row count
pub const MAX_TABLE_ROWS: usize = 10_000_000;

/// Maximum alias expansion depth
pub const MAX_ALIAS_DEPTH: usize = 100;
```

### Thread Safety Guarantees

The following components are guaranteed thread-safe:
- `Mappings` singleton (read-only after initialization)
- `DxMmap` (immutable after creation)
- `Parser` instances (no shared mutable state)

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Null Byte Handling

*For any* input string containing null bytes at any position, the Parser SHALL either successfully parse the input (treating null as part of string values) or return a well-formed error without panicking.

**Validates: Requirements 1.1**

### Property 2: UTF-8 Validation with Offset

*For any* byte sequence containing invalid UTF-8, the Parser SHALL return a Utf8Error where the offset field exactly matches the byte position of the first invalid sequence.

**Validates: Requirements 1.4**

### Property 3: Error Position Reporting

*For any* syntactically invalid input, the Parser SHALL return an error containing line number, column number, and byte offset that correctly identify the error location.

**Validates: Requirements 1.5, 7.1**

### Property 4: Integer Overflow Detection

*For any* numeric string representing a value outside the range [-2^63, 2^63-1], the Tokenizer SHALL return an IntegerOverflow error rather than silently truncating or wrapping.

**Validates: Requirements 2.1**

### Property 5: Invalid Float Detection

*For any* string that looks like a float but has invalid format (multiple decimal points, invalid exponent), the Tokenizer SHALL return an InvalidNumber error.

**Validates: Requirements 2.2**

### Property 6: EOF Handling

*For any* input, after all tokens have been consumed, subsequent calls to next_token() SHALL return Token::Eof without panicking or returning errors.

**Validates: Requirements 2.3**

### Property 7: Control Character Handling

*For any* input containing control characters (0x00-0x1F except 0x09, 0x0A, 0x0D), the Tokenizer SHALL handle them consistently—either as part of string values or by returning appropriate errors.

**Validates: Requirements 2.4**

### Property 8: DxValue Round-Trip

*For any* valid DxValue object, serializing to DX format and then parsing back SHALL produce a DxValue that is semantically equivalent to the original.

**Validates: Requirements 3.1, 10.1**

### Property 9: Human Format Round-Trip

*For any* valid DxDocument object, formatting to Human format and then parsing back SHALL produce a DxDocument that is semantically equivalent to the original.

**Validates: Requirements 3.2**

### Property 10: LLM Format Round-Trip

*For any* valid DxDocument object, formatting to LLM format and then parsing back SHALL produce a DxDocument that is semantically equivalent to the original.

**Validates: Requirements 3.3**

### Property 11: Binary Format Round-Trip

*For any* valid DX-Zero byte sequence, reading into memory and writing back SHALL produce an identical byte sequence.

**Validates: Requirements 3.4**

### Property 12: Header Validation

*For any* byte sequence where the first two bytes are not [0x5A, 0x44], OR the version byte is not 0x01, OR reserved flags are set, the Zero_Copy_Deserializer SHALL return an appropriate error before any data field access.

**Validates: Requirements 4.1, 4.2, 4.5**

### Property 13: Heap Bounds Checking

*For any* DX-Zero slot containing a heap reference, if the offset+length exceeds the buffer size, the Zero_Copy_Deserializer SHALL return an out-of-bounds error.

**Validates: Requirements 4.4**

### Property 14: Alias Loop Detection

*For any* set of alias definitions where expanding an alias would require expanding itself (directly or transitively), the Parser SHALL detect the cycle and return an error rather than infinite looping.

**Validates: Requirements 6.3**

### Property 15: Decompression Size Verification

*For any* compressed data where the actual decompressed size differs from the declared size, the Zero_Copy_Deserializer SHALL detect the mismatch and return an error.

**Validates: Requirements 6.4**

### Property 16: Type Mismatch Error Details

*For any* type mismatch during parsing (e.g., expected int, got string), the error message SHALL include both the expected type name and the actual type name.

**Validates: Requirements 7.2**

### Property 17: Schema Error Details

*For any* schema validation failure, the error message SHALL include the column name and the expected type for that column.

**Validates: Requirements 7.4**

### Property 18: Thread Safety

*For any* concurrent execution where multiple threads read from the same Mappings singleton or DxMmap instance, there SHALL be no data races or undefined behavior.

**Validates: Requirements 8.1, 8.3, 8.4**

### Property 19: Parser Instance Isolation

*For any* two Parser instances parsing different inputs concurrently, the parsing of one SHALL not affect the results of the other.

**Validates: Requirements 8.2**

### Property 20: Compression Round-Trip

*For any* byte sequence, compressing with LZ4 and then decompressing SHALL produce the exact original byte sequence.

**Validates: Requirements 9.1**

### Property 21: Decompression Error Handling

*For any* corrupted or truncated compressed data, the decompressor SHALL return a DecompressionError with details about the failure rather than producing incorrect output.

**Validates: Requirements 9.2, 9.3**

### Property 22: Compression Ratio Accuracy

*For any* compressed data, the calculated compression ratio SHALL be within 0.01% of the true ratio (compressed_size / original_size).

**Validates: Requirements 9.4**

### Property 23: Special Character Escaping

*For any* string value containing special characters (quotes, backslashes, control characters), the Pretty_Printer SHALL escape them such that parsing the output produces the original string.

**Validates: Requirements 10.2**

## Error Handling

### Error Categories

1. **Input Validation Errors** - Detected before parsing begins
   - `InputTooLarge` - Input exceeds MAX_INPUT_SIZE
   - `InvalidUtf8` - Input contains invalid UTF-8 sequences

2. **Parse Errors** - Detected during parsing
   - `UnexpectedToken` - Syntax error with position info
   - `RecursionLimitExceeded` - Nesting too deep
   - `AliasLoop` - Circular alias reference

3. **Binary Format Errors** - Detected during binary deserialization
   - `InvalidMagic` - Wrong magic bytes
   - `UnsupportedVersion` - Unknown version
   - `HeapOutOfBounds` - Invalid heap reference

4. **Compression Errors** - Detected during decompression
   - `DecompressionFailed` - LZ4 decompression error
   - `SizeMismatch` - Declared vs actual size mismatch
   - `Truncated` - Incomplete compressed data

### Error Recovery

The serializer follows a fail-fast approach:
- Errors are returned immediately upon detection
- No partial results are returned
- All errors include sufficient context for debugging

## Testing Strategy

### Dual Testing Approach

The testing strategy combines:
1. **Unit tests** - Specific examples and edge cases
2. **Property-based tests** - Universal properties across generated inputs

### Property-Based Testing Configuration

- **Library**: `proptest` (Rust)
- **Minimum iterations**: 100 per property
- **Shrinking**: Enabled for minimal counterexamples

### Test Organization

```
tests/
├── battle_hardening_tests.rs    # Existing edge case tests
├── property_tests/
│   ├── parser_props.rs          # Parser property tests
│   ├── tokenizer_props.rs       # Tokenizer property tests
│   ├── roundtrip_props.rs       # Round-trip property tests
│   ├── binary_props.rs          # Binary format property tests
│   ├── compression_props.rs     # Compression property tests
│   └── thread_safety_props.rs   # Concurrency property tests
└── integration/
    ├── converter_tests.rs       # Format converter tests
    └── stress_tests.rs          # Large input stress tests
```

### Property Test Annotations

Each property test must include:
```rust
/// Feature: serializer-battle-hardening, Property N: Property Name
/// Validates: Requirements X.Y, X.Z
#[test]
fn prop_property_name() { ... }
```
