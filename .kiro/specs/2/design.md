# Design Document: DX Serializer Quantum Entanglement

## Overview

This design document describes the architecture and implementation approach for enhancing the DX Serializer to achieve "quantum entanglement" between its three format modes (Human, LLM, Machine), implementing platform-specific async I/O, and ensuring production-grade reliability.

The design follows a "hub and spoke" architecture where all formats convert through a common internal representation (`DxDocument`), ensuring consistent round-trip behavior and enabling seamless format switching.

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         DX Serializer Architecture                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                   │
│  │   Human     │     │    LLM      │     │   Machine   │                   │
│  │   Format    │     │   Format    │     │   Format    │                   │
│  │  (Editor)   │     │   (Disk)    │     │  (Runtime)  │                   │
│  └──────┬──────┘     └──────┬──────┘     └──────┬──────┘                   │
│         │                   │                   │                           │
│         │    inflate        │    inflate        │                           │
│         ▼                   ▼                   ▼                           │
│  ┌─────────────────────────────────────────────────────┐                   │
│  │                    DxDocument                        │                   │
│  │              (Internal Representation)               │                   │
│  │  - context: HashMap<String, DxLlmValue>             │                   │
│  │  - sections: HashMap<char, DxSection>               │                   │
│  │  - refs: HashMap<String, String>                    │                   │
│  └─────────────────────────────────────────────────────┘                   │
│         │                   │                   │                           │
│         │    deflate        │    deflate        │                           │
│         ▼                   ▼                   ▼                           │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                   │
│  │   Human     │     │    LLM      │     │   Machine   │                   │
│  │   Output    │     │   Output    │     │   Output    │                   │
│  └─────────────┘     └─────────────┘     └─────────────┘                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Platform-Specific I/O Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Platform I/O Abstraction                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────┐                   │
│  │              DxAsyncIO (Unified API)                │                   │
│  │  - async fn read_file(path) -> Result<Vec<u8>>     │                   │
│  │  - async fn write_file(path, data) -> Result<()>   │                   │
│  │  - async fn read_batch(paths) -> Result<Vec<...>>  │                   │
│  └─────────────────────────────────────────────────────┘                   │
│                           │                                                 │
│         ┌─────────────────┼─────────────────┐                              │
│         ▼                 ▼                 ▼                              │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐                      │
│  │  io_uring   │   │   kqueue    │   │    IOCP     │                      │
│  │  (Linux)    │   │  (macOS)    │   │  (Windows)  │                      │
│  └─────────────┘   └─────────────┘   └─────────────┘                      │
│                                                                             │
│  Fallback: std::fs (blocking) when platform API unavailable                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Format Conversion Module (`src/hologram/`)

The hologram module provides the "quantum entanglement" - seamless conversion between formats.

```rust
/// Unified format conversion interface
pub trait FormatConverter {
    /// Convert to internal document representation
    fn to_document(&self, input: &[u8]) -> Result<DxDocument, ConvertError>;
    
    /// Convert from internal document representation
    fn from_document(&self, doc: &DxDocument) -> Result<Vec<u8>, ConvertError>;
}

/// Human format converter
pub struct HumanConverter {
    config: HumanFormatV2Config,
}

/// LLM format converter  
pub struct LlmConverter {
    abbrev: AbbrevDict,
    min_ref_length: usize,
    min_ref_count: usize,
}

/// Machine format converter
pub struct MachineConverter {
    compression: Option<CompressionLevel>,
}
```

### 2. Platform I/O Module (`src/io/`)

New module for platform-specific async I/O.

```rust
/// Platform-agnostic async I/O trait
#[async_trait]
pub trait AsyncFileIO: Send + Sync {
    /// Read entire file asynchronously
    async fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    
    /// Write entire file asynchronously
    async fn write(&self, path: &Path, data: &[u8]) -> io::Result<()>;
    
    /// Read multiple files in batch
    async fn read_batch(&self, paths: &[&Path]) -> io::Result<Vec<Vec<u8>>>;
    
    /// Write multiple files in batch
    async fn write_batch(&self, files: &[(&Path, &[u8])]) -> io::Result<()>;
}

/// Linux io_uring implementation
#[cfg(target_os = "linux")]
pub struct IoUringIO {
    ring: IoUring,
    ring_size: u32,
}

/// macOS kqueue implementation
#[cfg(target_os = "macos")]
pub struct KqueueIO {
    kq: RawFd,
}

/// Windows IOCP implementation
#[cfg(target_os = "windows")]
pub struct IocpIO {
    completion_port: HANDLE,
}

/// Fallback blocking implementation
pub struct BlockingIO;

/// Auto-detect and create best available I/O backend
pub fn create_async_io() -> Box<dyn AsyncFileIO> {
    #[cfg(target_os = "linux")]
    if IoUringIO::is_available() {
        return Box::new(IoUringIO::new().unwrap());
    }
    
    #[cfg(target_os = "macos")]
    return Box::new(KqueueIO::new().unwrap());
    
    #[cfg(target_os = "windows")]
    return Box::new(IocpIO::new().unwrap());
    
    Box::new(BlockingIO)
}
```

### 3. LLM Token Efficiency Module (`src/llm/`)

Enhanced LLM serialization for 3x+ token efficiency.

```rust
/// Token efficiency optimizations
pub struct TokenOptimizer {
    /// Key abbreviation dictionary (100+ mappings)
    abbrev: AbbrevDict,
    
    /// String reference tracker
    refs: RefTracker,
    
    /// Base62 encoder for large integers
    base62: Base62Encoder,
}

/// Reference tracker for automatic string deduplication
pub struct RefTracker {
    /// String -> reference key mapping
    string_to_ref: HashMap<String, String>,
    
    /// Minimum string length to consider
    min_length: usize,
    
    /// Minimum occurrences to create reference
    min_count: usize,
}

/// Base62 encoder for compact integer representation
pub struct Base62Encoder;

impl Base62Encoder {
    /// Encode integer to base62 (0-9, a-z, A-Z)
    pub fn encode(n: u64) -> String;
    
    /// Decode base62 to integer
    pub fn decode(s: &str) -> Result<u64, DecodeError>;
}
```

### 4. Machine Format Module (`src/zero/`)

Enhanced zero-copy binary format.

```rust
/// Quantum reader with compile-time offsets
pub struct QuantumReader<'a> {
    data: &'a [u8],
}

impl<'a> QuantumReader<'a> {
    /// Read u64 at compile-time offset (0.7ns)
    #[inline(always)]
    pub fn read_u64<const OFFSET: usize>(&self) -> u64;
    
    /// Unchecked read for maximum performance
    #[inline(always)]
    pub unsafe fn read_u64_unchecked<const OFFSET: usize>(&self) -> u64;
}

/// Arena allocator for batch serialization
pub struct DxArena {
    buffer: Vec<u8>,
    offset: usize,
}

/// SIMD batch operations
pub mod simd512 {
    /// Sum u64 values using best available SIMD
    pub fn sum_u64s(data: &[u8]) -> u64;
    
    /// Auto-dispatch to AVX-512, AVX2, or scalar
    pub mod dispatch {
        pub fn sum_u64s(data: &[u8]) -> u64;
    }
}
```

### 5. Compression Module (`src/zero/compress.rs`)

Integrated LZ4 compression.

```rust
/// Compression levels
#[derive(Clone, Copy)]
pub enum CompressionLevel {
    Fast,      // Fastest compression, larger output
    Balanced,  // Good balance of speed and size
    Maximum,   // Best compression, slower
}

/// Compressed data wrapper
pub struct DxCompressed {
    compressed: Vec<u8>,
    original_size: usize,
}

impl DxCompressed {
    /// Compress data with specified level
    pub fn compress(data: &[u8], level: CompressionLevel) -> Self;
    
    /// Decompress to original data
    pub fn decompress(&self) -> Result<Vec<u8>, DecompressError>;
    
    /// Get compression savings ratio
    pub fn savings(&self) -> f64;
}

/// Streaming compressor for large files
pub struct StreamCompressor {
    level: CompressionLevel,
    buffer: Vec<u8>,
}

/// Streaming decompressor
pub struct StreamDecompressor {
    buffer: Vec<u8>,
}
```

## Data Models

### DxDocument (Internal Representation)

```rust
/// The hub of the "hub and spoke" architecture
#[derive(Debug, Clone, PartialEq)]
pub struct DxDocument {
    /// Context/metadata key-value pairs
    pub context: HashMap<String, DxLlmValue>,
    
    /// Data sections (keyed by single char ID)
    pub sections: HashMap<char, DxSection>,
    
    /// String references for deduplication
    pub refs: HashMap<String, String>,
}

/// A data section with schema and rows
#[derive(Debug, Clone, PartialEq)]
pub struct DxSection {
    /// Column names/schema
    pub schema: Vec<String>,
    
    /// Data rows
    pub rows: Vec<Vec<DxLlmValue>>,
}

/// Value types supported in all formats
#[derive(Debug, Clone, PartialEq)]
pub enum DxLlmValue {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<DxLlmValue>),
    Ref(String),  // Reference to string in refs map
}
```

### Binary Format Layout

```
┌─────────────────────────────────────────┐
│ HEADER (4 bytes)                        │
│ - Magic: 0x5A 0x44 ("ZD")              │
│ - Version: 0x01                         │
│ - Flags: has_heap, little_endian, etc.  │
├─────────────────────────────────────────┤
│ FIXED SECTION (variable size)           │
│ - Primitive fields packed               │
│ - u8, u16, u32, u64, i*, f32, f64, bool │
├─────────────────────────────────────────┤
│ VARIABLE SLOTS (16 bytes × N)           │
│ - Inline (marker=0x00):                 │
│   [len, data[0..14], 0x00]              │
│ - Heap (marker=0xFF):                   │
│   [offset, length, reserved, 0xFF]      │
├─────────────────────────────────────────┤
│ HEAP SECTION (variable size)            │
│ - Contiguous packed data                │
│ - No headers or padding                 │
└─────────────────────────────────────────┘
```



## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Format Round-Trip Preservation

*For any* valid DxDocument, converting through any sequence of format transformations (Human ↔ LLM ↔ Machine) and back to the original format SHALL produce a semantically equivalent document.

**Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7**

### Property 2: LLM Token Efficiency vs TOON

*For any* dataset containing 100 or more records, the token count of the LLM format output SHALL be at most one-third (≤33%) of the token count of the equivalent TOON format output.

**Validates: Requirements 2.1**

### Property 3: LLM Compact Value Serialization

*For any* boolean value, the LLM serialization SHALL be exactly one character ("+" for true, "-" for false). *For any* null value, the LLM serialization SHALL be exactly one character ("~").

**Validates: Requirements 2.3, 2.4**

### Property 4: LLM Automatic Reference Creation

*For any* DxDocument containing a string that appears 2 or more times and has length >= 5 characters, the LLM serialization SHALL include a reference definition for that string and use reference markers (^key) in place of repeated occurrences.

**Validates: Requirements 2.5**

### Property 5: LLM Base62 Efficiency

*For any* integer value greater than 61, the base62 encoding SHALL produce a string with fewer characters than the decimal representation.

**Validates: Requirements 2.7**

### Property 6: Human Format Key Expansion

*For any* abbreviated key in the abbreviation dictionary, the Human format output SHALL contain the full expanded key name instead of the abbreviation.

**Validates: Requirements 4.1**

### Property 7: Human Format Structure

*For any* DxDocument with sections, the Human format output SHALL:
- Contain TOML-like section headers with square brackets (e.g., [section_name])
- Display tabular data with Unicode box-drawing characters (─, │, ┌, ┐, └, ┘, ├, ┤, ┬, ┴, ┼)
- Use comma-separated values for arrays
- Include row counts after tables in the format "Total: N rows"

**Validates: Requirements 4.2, 4.3, 4.4, 4.5**

### Property 8: Human Format Keyboard-Accessible Characters

*For any* Human format output, all characters SHALL be either:
- Standard ASCII printable characters (0x20-0x7E)
- Specific Unicode box-drawing characters for tables
- Standard newline characters

**Validates: Requirements 4.6**

### Property 9: Human Format Flat Paths

*For any* nested data structure, the Human format output SHALL use dot-notation key paths (e.g., "parent.child.value") instead of indentation-based nesting.

**Validates: Requirements 4.8**

### Property 10: Machine Format String Storage

*For any* string value:
- If length <= 14 bytes, the slot marker byte SHALL be 0x00 (inline storage)
- If length > 14 bytes, the slot marker byte SHALL be 0xFF (heap storage)

**Validates: Requirements 3.6, 6.3**

### Property 11: Batch File Operations Correctness

*For any* set of files processed in batch, the batch operation SHALL produce the same results as processing each file individually.

**Validates: Requirements 5.6**

### Property 12: Error Messages with Location

*For any* parse error on invalid input, the error message SHALL contain both line number and column number indicating the error location.

**Validates: Requirements 6.1**

### Property 13: Binary Header Validation

*For any* byte sequence that does not start with magic bytes [0x5A, 0x44] or has invalid version byte, the Machine format parser SHALL return an InvalidMagic or UnsupportedVersion error before attempting field access.

**Validates: Requirements 6.2**

### Property 14: Invalid UTF-8 Handling

*For any* byte sequence containing invalid UTF-8 in a string position, the parser SHALL return a specific Utf8ValidationError.

**Validates: Requirements 6.6**

### Property 15: Buffer Size Error

*For any* buffer smaller than the required size for a Machine format operation, the error SHALL be BufferTooSmall and SHALL include the required size.

**Validates: Requirements 6.7**

### Property 16: Compression Round-Trip

*For any* byte sequence, compressing and then decompressing SHALL produce the exact original byte sequence.

**Validates: Requirements 7.5**

### Property 17: Compression Ratio

*For any* typical configuration data (structured text with repeated patterns), LZ4 compression SHALL achieve at least 40% size reduction (compressed size <= 60% of original).

**Validates: Requirements 7.2**

### Property 18: SIMD/Scalar Equivalence

*For any* batch operation (sum, search, etc.), the SIMD implementation SHALL produce the exact same result as the scalar implementation.

**Validates: Requirements 8.3**

### Property 19: Mmap/Regular Read Equivalence

*For any* valid Machine format file, reading via memory-mapping SHALL produce the exact same data as reading via regular file I/O.

**Validates: Requirements 9.1**

### Property 20: Header Validation Before Access

*For any* memory-mapped file, the validate_header() method SHALL be called and succeed before any field access is permitted.

**Validates: Requirements 9.3**

### Property 21: Batch Iteration Correctness

*For any* memory-mapped file containing N records, batch iteration SHALL yield exactly N records in order, each with correct data.

**Validates: Requirements 9.4**

## Error Handling

### Error Types

```rust
/// Unified error type for DX Serializer
#[derive(Debug, thiserror::Error)]
pub enum DxError {
    // Parse errors
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },
    
    // Binary format errors
    #[error("Invalid magic bytes: expected [0x5A, 0x44], found [{0:#04x}, {1:#04x}]")]
    InvalidMagic(u8, u8),
    
    #[error("Unsupported version: found {found}, supported: {supported}")]
    UnsupportedVersion { found: u8, supported: u8 },
    
    #[error("Buffer too small: required {required} bytes, actual {actual} bytes")]
    BufferTooSmall { required: usize, actual: usize },
    
    // Encoding errors
    #[error("Invalid UTF-8 at byte offset {offset}: {message}")]
    Utf8Error { offset: usize, message: String },
    
    #[error("Invalid base62 character: '{0}'")]
    Base62Error(char),
    
    // I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    // Conversion errors
    #[error("Conversion error: {0}")]
    ConvertError(String),
    
    // Compression errors
    #[error("Compression error: {0}")]
    CompressionError(String),
    
    #[error("Decompression error: {0}")]
    DecompressionError(String),
    
    // Platform errors
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
}
```

### Error Handling Strategy

1. **Never panic** - All errors are returned as Result types
2. **Descriptive messages** - Include context (line/column, byte offset, expected vs actual)
3. **Error chaining** - Preserve underlying error causes
4. **Recoverable by default** - Most errors allow the caller to retry or handle gracefully

## Testing Strategy

### Dual Testing Approach

The testing strategy combines unit tests for specific examples and edge cases with property-based tests for universal correctness guarantees.

### Unit Tests

Unit tests focus on:
- Specific examples demonstrating correct behavior
- Edge cases (empty input, maximum sizes, boundary values)
- Error conditions (invalid input, malformed data)
- Integration points between components

### Property-Based Tests

Property-based tests use the `proptest` crate with minimum 100 iterations per property.

```rust
use proptest::prelude::*;

// Property 1: Format Round-Trip Preservation
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    /// Feature: dx-serializer-quantum-entanglement, Property 1: Format Round-Trip
    /// Validates: Requirements 1.1-1.7
    #[test]
    fn prop_format_round_trip(doc in arb_dx_document()) {
        // Human -> LLM -> Human
        let human1 = document_to_human(&doc)?;
        let llm = human_to_llm(&human1)?;
        let human2 = llm_to_human(&llm)?;
        let doc2 = human_to_document(&human2)?;
        prop_assert_eq!(doc, doc2);
        
        // LLM -> Machine -> LLM
        let machine = llm_to_machine(&llm)?;
        let llm2 = machine_to_llm(&machine)?;
        let doc3 = llm_to_document(&llm2)?;
        prop_assert_eq!(doc, doc3);
    }
}

// Property 3: LLM Compact Value Serialization
proptest! {
    /// Feature: dx-serializer-quantum-entanglement, Property 3: Compact Values
    /// Validates: Requirements 2.3, 2.4
    #[test]
    fn prop_llm_compact_values(b in any::<bool>()) {
        let serialized = serialize_bool(b);
        prop_assert_eq!(serialized.len(), 1);
        prop_assert!(serialized == "+" || serialized == "-");
    }
}

// Property 10: Machine Format String Storage
proptest! {
    /// Feature: dx-serializer-quantum-entanglement, Property 10: String Storage
    /// Validates: Requirements 3.6, 6.3
    #[test]
    fn prop_string_storage(s in ".*") {
        let slot = write_string_slot(&s);
        let marker = slot[15];
        
        if s.len() <= 14 {
            prop_assert_eq!(marker, 0x00, "String <= 14 bytes should be inline");
        } else {
            prop_assert_eq!(marker, 0xFF, "String > 14 bytes should be heap");
        }
    }
}

// Property 16: Compression Round-Trip
proptest! {
    /// Feature: dx-serializer-quantum-entanglement, Property 16: Compression Round-Trip
    /// Validates: Requirements 7.5
    #[test]
    fn prop_compression_round_trip(data in prop::collection::vec(any::<u8>(), 0..10000)) {
        let compressed = DxCompressed::compress(&data, CompressionLevel::Balanced);
        let decompressed = compressed.decompress()?;
        prop_assert_eq!(data, decompressed);
    }
}

// Property 18: SIMD/Scalar Equivalence
proptest! {
    /// Feature: dx-serializer-quantum-entanglement, Property 18: SIMD/Scalar Equivalence
    /// Validates: Requirements 8.3
    #[test]
    fn prop_simd_scalar_equivalence(values in prop::collection::vec(any::<u64>(), 0..10000)) {
        let bytes: Vec<u8> = values.iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        
        let simd_sum = simd512::dispatch::sum_u64s(&bytes);
        let scalar_sum: u64 = values.iter().sum();
        
        prop_assert_eq!(simd_sum, scalar_sum);
    }
}
```

### Test Generators

```rust
/// Generate arbitrary DxDocument for property testing
fn arb_dx_document() -> impl Strategy<Value = DxDocument> {
    (
        prop::collection::hash_map(arb_key(), arb_value(), 0..10),
        prop::collection::hash_map(arb_section_id(), arb_section(), 0..5),
        prop::collection::hash_map(arb_ref_key(), arb_string(), 0..10),
    ).prop_map(|(context, sections, refs)| {
        DxDocument { context, sections, refs }
    })
}

/// Generate arbitrary DxLlmValue
fn arb_value() -> impl Strategy<Value = DxLlmValue> {
    prop_oneof![
        Just(DxLlmValue::Null),
        any::<bool>().prop_map(DxLlmValue::Bool),
        any::<f64>().prop_map(DxLlmValue::Num),
        arb_string().prop_map(DxLlmValue::Str),
        prop::collection::vec(arb_value_leaf(), 0..5).prop_map(DxLlmValue::Arr),
    ]
}
```

### Benchmark Tests

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_format_conversion(c: &mut Criterion) {
    let doc = create_test_document(100); // 100 records
    
    c.bench_function("human_to_llm", |b| {
        b.iter(|| human_to_llm(&doc))
    });
    
    c.bench_function("llm_to_machine", |b| {
        b.iter(|| llm_to_machine(&doc))
    });
    
    c.bench_function("machine_field_access", |b| {
        let machine = llm_to_machine(&doc).unwrap();
        let reader = QuantumReader::new(&machine);
        b.iter(|| reader.read_u64::<4>())
    });
}
```
