# Design Document: DX Serializer LLM and Human Formats

## Overview

This design extends the DX Serializer crate to support three interconvertible formats:

1. **LLM Format** - Token-optimized using sigils (`#c`, `#:`, `#<letter>`), references (`^key`), and abbreviated keys
2. **Human Format** - Beautiful TOML-like display with Unicode tables, expanded keys, and summaries
3. **Machine Format** - Binary format for runtime (already implemented)

The architecture follows a "hub and spoke" model where all formats convert through a common internal representation (`DxDocument`), ensuring consistent round-trip behavior.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         DX SERIALIZER ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│                           ┌─────────────────┐                               │
│                           │   DxDocument    │                               │
│                           │  (Internal IR)  │                               │
│                           └────────┬────────┘                               │
│                                    │                                        │
│              ┌─────────────────────┼─────────────────────┐                  │
│              │                     │                     │                  │
│              ▼                     ▼                     ▼                  │
│   ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐         │
│   │   LLM Format     │  │  Human Format    │  │ Machine Format   │         │
│   │   (Token-opt)    │  │  (Beautiful)     │  │   (Binary)       │         │
│   └──────────────────┘  └──────────────────┘  └──────────────────┘         │
│                                                                             │
│   Sigils: #c #: #x     TOML-like tables      Zero-copy binary              │
│   Refs: ^key           Unicode box-drawing   0.70ns access                 │
│   Bools: + -           ✓ ✗ symbols                                         │
│   Null: ~              — symbol                                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### Core Data Types

```rust
/// Internal document representation (the "hub")
#[derive(Debug, Clone, PartialEq)]
pub struct DxDocument {
    /// Context/config section (#c)
    pub context: HashMap<String, DxValue>,
    /// Reference definitions (#:)
    pub refs: HashMap<String, String>,
    /// Data sections (#<letter>)
    pub sections: HashMap<char, DxSection>,
}

/// A data section with schema and rows
#[derive(Debug, Clone, PartialEq)]
pub struct DxSection {
    /// Column names from schema
    pub schema: Vec<String>,
    /// Row data
    pub rows: Vec<Vec<DxValue>>,
}

/// Value types in DX format
#[derive(Debug, Clone, PartialEq)]
pub enum DxValue {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Arr(Vec<DxValue>),
    Ref(String),  // Reference pointer (^key)
}
```

### LLM Format Parser

```rust
/// Parse LLM-optimized format into DxDocument
pub struct LlmParser;

impl LlmParser {
    /// Parse LLM format string into DxDocument
    pub fn parse(input: &str) -> Result<DxDocument, ParseError>;
    
    /// Parse a single line
    fn parse_line(line: &str, doc: &mut DxDocument) -> Result<(), ParseError>;
    
    /// Parse context section: #c:key|val;key|val
    fn parse_context(content: &str) -> Result<HashMap<String, DxValue>, ParseError>;
    
    /// Parse reference: #:key|value
    fn parse_reference(content: &str) -> Result<(String, String), ParseError>;
    
    /// Parse data section header: #x(col|col|col)
    fn parse_section_header(id: char, content: &str) -> Result<DxSection, ParseError>;
    
    /// Parse data row: val|val|val
    fn parse_row(line: &str, schema: &[String]) -> Result<Vec<DxValue>, ParseError>;
    
    /// Parse a single value
    fn parse_value(s: &str) -> DxValue;
}
```

### LLM Format Serializer

```rust
/// Serialize DxDocument to LLM-optimized format
pub struct LlmSerializer {
    abbrev: AbbrevDict,
}

impl LlmSerializer {
    /// Serialize DxDocument to LLM format string
    pub fn serialize(&self, doc: &DxDocument) -> String;
    
    /// Find repeated strings for reference optimization
    fn find_repeated_strings(&self, doc: &DxDocument) -> HashMap<String, String>;
    
    /// Serialize context section
    fn serialize_context(&self, context: &HashMap<String, DxValue>, refs: &HashMap<String, String>) -> String;
    
    /// Serialize a data section
    fn serialize_section(&self, id: char, section: &DxSection, refs: &HashMap<String, String>) -> String;
    
    /// Serialize a single value
    fn serialize_value(&self, value: &DxValue, refs: &HashMap<String, String>) -> String;
}
```

### Human Format Formatter

```rust
/// Configuration for human format output
#[derive(Debug, Clone)]
pub struct HumanFormatConfig {
    pub table_style: TableStyle,
    pub indent_size: usize,
    pub max_width: usize,
    pub show_references: bool,
    pub show_summaries: bool,
    pub expand_abbreviations: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum TableStyle {
    Unicode,   // ┌─┬─┐ │ ├─┼─┤ └─┴─┘
    Ascii,     // +-+-+ | +-+-+ +-+-+
    Markdown,  // | --- | --- |
    Minimal,   // No borders
}

/// Format DxDocument to beautiful human-readable format
pub struct HumanFormatter {
    config: HumanFormatConfig,
    abbrev: AbbrevDict,
}

impl HumanFormatter {
    /// Format DxDocument to human-readable string
    pub fn format(&self, doc: &DxDocument) -> String;
    
    /// Format section header with box-drawing
    fn format_section_header(&self, title: &str) -> String;
    
    /// Format config section
    fn format_config(&self, context: &HashMap<String, DxValue>, refs: &HashMap<String, String>) -> String;
    
    /// Format data section as table
    fn format_data_section(&self, id: char, section: &DxSection, refs: &HashMap<String, String>) -> String;
    
    /// Build Unicode box-drawn table
    fn build_table(&self, section: &DxSection, context: &str, refs: &HashMap<String, String>) -> String;
    
    /// Format cell value for display
    fn format_cell_value(&self, value: &DxValue, refs: &HashMap<String, String>) -> String;
    
    /// Generate summary footer
    fn generate_summary(&self, section: &DxSection, context: &str) -> String;
}
```

### Human Format Parser

```rust
/// Parse human-readable format back to DxDocument
pub struct HumanParser;

impl HumanParser {
    /// Parse human format string into DxDocument
    pub fn parse(input: &str) -> Result<DxDocument, ParseError>;
    
    /// Parse section header: [section_name]
    fn parse_section_header(line: &str) -> Option<String>;
    
    /// Parse key-value pair: key = "value"
    fn parse_key_value(line: &str) -> Option<(String, DxValue)>;
    
    /// Parse Unicode table
    fn parse_table(lines: &[&str]) -> Result<DxSection, ParseError>;
    
    /// Parse table cell value
    fn parse_cell_value(s: &str) -> DxValue;
}
```

### Abbreviation Dictionary

```rust
/// Bidirectional key abbreviation dictionary
pub struct AbbrevDict {
    /// Short → Full (for expansion)
    global: HashMap<&'static str, &'static str>,
    /// Context-specific expansions
    contextual: HashMap<(&'static str, &'static str), &'static str>,
    /// Full → Short (for compression)
    reverse: HashMap<&'static str, &'static str>,
}

impl AbbrevDict {
    /// Create dictionary with all standard mappings
    pub fn new() -> Self;
    
    /// Expand abbreviated key to full name
    pub fn expand(&self, abbrev: &str, context: &str) -> String;
    
    /// Compress full key to abbreviation
    pub fn compress(&self, full: &str) -> String;
}
```

## Data Models

### LLM Format Syntax

```
# Context section
#c:<key>|<val>;<key>|<val>

# Reference definitions
#:<ref_key>|<ref_value>

# Data section with schema
#<id>(<col>|<col>|<col>)
<val>|<val>|<val>
<val>|<val>|<val>

# Special values
+       → boolean true
-       → boolean false
~       → null
^<key>  → reference pointer
*a,b,c  → inline array
```

### Human Format Syntax

```toml
# ═══════════════════════════════════════════════════════════════════════════════
#                                   SECTION NAME
# ═══════════════════════════════════════════════════════════════════════════════

[config]
    key      = "value"
    long_key = "another value"

[references]
    A = "Some Value"

[data_section]
    # Schema: col1 | col2 | col3
    
    ┌──────┬──────┬──────┐
    │ Col1 │ Col2 │ Col3 │
    ├──────┼──────┼──────┤
    │ val  │ val  │  ✓   │
    │ val  │ val  │  ✗   │
    └──────┴──────┴──────┘
    
    Total: 2 rows | Summary info
```

### Key Abbreviation Mappings

The dictionary contains 50+ standard mappings organized by category:

| Category | Abbreviations |
|----------|---------------|
| Identity | `id`, `nm`→name, `tt`→title, `ds`→description |
| State | `st`→status, `ac`→active, `en`→enabled |
| Timestamps | `cr`→created, `up`→updated, `dl`→deleted, `dt`→date, `tm`→time, `ts`→timestamp |
| Metrics | `ct`→count, `tl`→total, `am`→amount, `pr`→price, `qt`→quantity, `km`→kilometers, `el`→elevation |
| Dimensions | `w`→width, `h`→height, `sz`→size |
| Web | `ur`→url, `pt`→path |
| Contact | `em`→email, `ph`→phone, `ad`→address |
| Location | `cy`→city, `co`→country, `l`→location, `la`→latitude, `lo`→longitude |
| Visual | `cl`→color, `im`→image |
| Relations | `pa`→parent, `ch`→children, `us`→user, `ow`→owner, `au`→author |
| Classification | `ca`→category, `tg`→tags, `t`→type, `v`→value |
| Commerce | `sk`→sku, `cu`→customer, `sh`→shipping, `pd`→paid |

Context-aware expansions for ambiguous keys:
- `s` → `sunny` (hikes), `status` (orders), `season` (config)
- `w` → `with` (hikes), `width` (images), `weight` (products)
- `t` → `task` (config), `type` (products), `time` (events)

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: LLM Format Round-Trip

*For any* valid DxDocument, serializing to LLM format and parsing back SHALL produce a semantically equivalent DxDocument.

**Validates: Requirements 1.1-1.8, 2.1-2.7, 9.1**

### Property 2: Human Format Round-Trip

*For any* valid DxDocument, formatting to Human format and parsing back SHALL produce a semantically equivalent DxDocument.

**Validates: Requirements 3.1-3.8, 4.1-4.6, 9.2**

### Property 3: LLM↔Human Conversion Round-Trip

*For any* valid LLM format string, converting to Human format and back to LLM format SHALL produce semantically equivalent output.

**Validates: Requirements 6.1-6.5, 7.1-7.5, 9.3**

### Property 4: Special Value Preservation

*For any* DxValue containing booleans (`+`/`-`/`true`/`false`/`✓`/`✗`) or null (`~`/`—`/`null`), converting through any format sequence SHALL preserve the semantic value.

**Validates: Requirements 1.5-1.7, 2.4-2.6, 3.4-3.6, 4.4-4.6, 6.3, 7.3**

### Property 5: Reference Resolution Correctness

*For any* DxDocument with references, resolving `^key` pointers SHALL always produce the correct referenced value, and creating references for repeated strings SHALL reduce output size.

**Validates: Requirements 1.4, 2.2, 3.7, 6.2, 7.2**

### Property 6: Key Abbreviation Round-Trip

*For any* key in the abbreviation dictionary, compressing then expanding SHALL return the original key. *For any* abbreviation in the dictionary, expanding then compressing SHALL return the original abbreviation.

**Validates: Requirements 5.1-5.3, 6.1, 7.1**

## Error Handling

### Parse Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid sigil at position {pos}: expected #c, #:, or #<letter>")]
    InvalidSigil { pos: usize },
    
    #[error("Malformed context section: {msg}")]
    MalformedContext { msg: String },
    
    #[error("Undefined reference: ^{key}")]
    UndefinedReference { key: String },
    
    #[error("Invalid value format: {value}")]
    InvalidValue { value: String },
    
    #[error("Schema mismatch: expected {expected} columns, got {got}")]
    SchemaMismatch { expected: usize, got: usize },
    
    #[error("Invalid table format at line {line}")]
    InvalidTable { line: usize },
}
```

### Graceful Degradation

- Unknown sigils are preserved as-is in output
- Unknown abbreviations pass through unchanged
- Malformed table rows are skipped with warning
- Missing references resolve to `^key` literal

## Testing Strategy

### Unit Tests

Unit tests verify specific examples and edge cases:

1. **Parser edge cases**: Empty input, single values, nested structures
2. **Serializer edge cases**: Empty documents, special characters, Unicode
3. **Formatter edge cases**: Long strings, many columns, empty tables
4. **Dictionary edge cases**: Unknown keys, context switching

### Property-Based Tests

Property tests verify universal properties using the `proptest` crate:

1. **Round-trip tests**: Generate random documents, convert, verify equivalence
2. **Value preservation**: Generate random values, convert through formats
3. **Reference optimization**: Generate documents with repeated strings
4. **Key abbreviation**: Generate random keys from dictionary

Configuration:
- Minimum 100 iterations per property test
- Use `proptest` crate for Rust
- Tag format: `Feature: dx-serializer-llm-human, Property N: <description>`

### Integration Tests

1. **File-based tests**: Read `.dx` files, convert, verify output
2. **CLI tests**: Test command-line conversion tools
3. **Playground tests**: Verify example files convert correctly
