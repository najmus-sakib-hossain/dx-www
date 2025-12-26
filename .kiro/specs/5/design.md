# Design Document: Human V3 Rust Parser

## Overview

This design adds a new `HumanV3Parser` to the Rust serializer crate that can parse Human Format V3 into `DxDocument`. The parser will be integrated into the WASM `toDense` function, replacing the current `HumanParser` which only supports the old format with `[config]` headers.

Human V3 format is a TOML-like structure where:
- Config values appear at the top without any section header
- Sections use full names like `[forge]`, `[style]`, `[media]`
- Nested sections use dot notation like `[i18n.locales]`, `[js.dependencies]`
- Arrays use pipe separators: `workspace = @/www | @/backend`
- The `[stack]` section contains reference definitions

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      WASM toDense()                              │
├─────────────────────────────────────────────────────────────────┤
│  1. Detect format (LLM vs Human V3)                             │
│  2. If LLM format (starts with #), return as-is                 │
│  3. If Human V3, parse with HumanV3Parser                       │
│  4. Serialize DxDocument to LLM format                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     HumanV3Parser                                │
├─────────────────────────────────────────────────────────────────┤
│  - parse_config_section() - key-value pairs before any header   │
│  - parse_section_header() - [section] or [parent.child]         │
│  - parse_stack_section() - reference definitions                │
│  - parse_data_section() - regular data sections                 │
│  - parse_nested_section() - [parent.child] sections             │
│  - parse_value() - strings, numbers, bools, nulls, arrays       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DxDocument                                  │
├─────────────────────────────────────────────────────────────────┤
│  - context: HashMap<String, DxLlmValue>  (config values)        │
│  - refs: HashMap<String, String>          (stack references)    │
│  - sections: HashMap<String, DxSection>   (data sections)       │
│  - section_order: Vec<String>             (preserve order)      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     LlmSerializer                                │
├─────────────────────────────────────────────────────────────────┤
│  - serialize_context() - #c:key|value;key|value                 │
│  - serialize_refs() - #:key|value                               │
│  - serialize_section() - #id(schema)\nrow|data                  │
└─────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### HumanV3Parser

```rust
pub struct HumanV3Parser {
    abbrev: AbbrevDict,
    section_names: SectionNameDict,
}

impl HumanV3Parser {
    pub fn new() -> Self;
    pub fn parse(&self, input: &str) -> Result<DxDocument, HumanV3ParseError>;
    
    fn parse_config_section(&self, lines: &[&str]) -> Result<(Context, usize), Error>;
    fn parse_section_header(&self, line: &str) -> Option<SectionHeader>;
    fn parse_stack_section(&self, lines: &[&str]) -> Result<(Refs, usize), Error>;
    fn parse_data_section(&self, lines: &[&str], header: &SectionHeader) -> Result<(Section, usize), Error>;
    fn parse_nested_section(&self, lines: &[&str], parent: &str, child: &str) -> Result<(NestedData, usize), Error>;
    fn parse_key_value(&self, line: &str) -> Result<Option<(String, DxLlmValue)>, Error>;
    fn parse_value(&self, raw: &str) -> DxLlmValue;
}
```

### SectionNameDict

```rust
pub struct SectionNameDict {
    name_to_id: HashMap<String, String>,
    id_to_name: HashMap<String, String>,
}

impl SectionNameDict {
    pub fn new() -> Self;
    pub fn name_to_id(&self, name: &str) -> String;
    pub fn id_to_name(&self, id: &str) -> String;
}

// Mappings:
// config -> c, forge -> f, stack -> k, style -> y, ui -> u
// media -> m, i18n -> i, icon -> o, font -> t, driven -> d
// generator -> g, scripts -> s, dependencies -> x
// js -> j, python -> p, rust -> r
```

### SectionHeader

```rust
pub enum SectionHeader {
    Simple(String),           // [forge] -> "forge"
    Nested(String, String),   // [i18n.locales] -> ("i18n", "locales")
}
```

## Data Models

### DxDocument (Extended)

The existing `DxDocument` will be extended to track section order:

```rust
pub struct DxDocument {
    pub context: HashMap<String, DxLlmValue>,
    pub refs: HashMap<String, String>,
    pub sections: HashMap<String, DxSection>,
    pub section_order: Vec<String>,  // NEW: preserve order
}
```

### NestedSectionData

For tracking nested sections before merging:

```rust
struct NestedSectionData {
    parent_id: String,
    subsections: HashMap<String, Vec<(String, DxLlmValue)>>,
    subsection_order: Vec<String>,
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Config Parsing

*For any* Human V3 document with key-value pairs before any section header, parsing SHALL place those values in the context map with keys compressed to their abbreviated forms.

**Validates: Requirements 1.1, 1.2**

### Property 2: Array Parsing Consistency

*For any* value containing N pipe separators (` | `), parsing SHALL produce an array with exactly N+1 elements.

**Validates: Requirements 1.3, 6.4**

### Property 3: Stack Section Preservation

*For any* `[stack]` section, parsing SHALL place all entries in the refs map with keys preserved (not abbreviated) and values joined with `|`.

**Validates: Requirements 2.1, 2.2, 2.3**

### Property 4: Section Name Mapping

*For any* section header with a full name, parsing SHALL map it to the correct abbreviated section ID and compress all keys in the schema.

**Validates: Requirements 3.1, 3.2, 3.3**

### Property 5: Nested Section Merging

*For any* set of nested sections sharing a parent, parsing SHALL merge them into a single section with keys prefixed by subsection name.

**Validates: Requirements 4.1, 4.2, 4.3**

### Property 6: Quoted String Handling

*For any* string value (quoted or unquoted), parsing SHALL preserve the content exactly, removing quotes if present.

**Validates: Requirements 5.1, 5.2, 5.3**

### Property 7: Value Type Detection

*For any* value string matching a numeric pattern, parsing SHALL produce a number type; for pipe-separated values, an array type.

**Validates: Requirements 6.3, 6.4**

### Property 8: Round-Trip Consistency

*For any* valid LLM document, converting to Human V3 then back to LLM SHALL produce a document with equivalent context, refs, section data, and section order.

**Validates: Requirements 7.1, 7.2, 7.3**

### Property 9: Format Detection and Passthrough

*For any* input to `toDense`, if it starts with `#` (LLM format), it SHALL be returned unchanged; otherwise it SHALL be parsed as Human V3.

**Validates: Requirements 8.1, 8.2**

## Error Handling

The parser will return detailed errors with:
- Line number where the error occurred
- Column number (when applicable)
- Error message describing the issue
- Hint for how to fix the problem

```rust
pub struct HumanV3ParseError {
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub hint: Option<String>,
}
```

Error cases:
- Unclosed quotes
- Invalid section header format
- Malformed key-value pairs
- Invalid nested section syntax

## Testing Strategy

### Unit Tests

- Test each parsing function in isolation
- Test edge cases: empty input, single line, no sections
- Test error cases: malformed input, unclosed quotes

### Property-Based Tests

Using `proptest` crate with minimum 100 iterations per property:

1. **Config parsing**: Generate random key-value pairs, verify context map
2. **Array parsing**: Generate arrays with varying lengths, verify element count
3. **Section mapping**: Generate section names, verify ID mapping
4. **Nested sections**: Generate nested section combinations, verify merging
5. **Round-trip**: Generate LLM documents, verify Human V3 → LLM consistency
6. **Value types**: Generate values of each type, verify correct parsing

### Integration Tests

- Full document parsing with all section types
- WASM `toDense` function with Human V3 input
- Comparison with TypeScript parser output
