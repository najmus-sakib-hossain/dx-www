# Design Document: DX Serializer Extension Fix

## Overview

This design updates the DX Serializer VS Code extension to properly handle the DX LLM format with sigils (`#c`, `#:`, `#<letter>`) instead of the current hologram format (`key#field:value`). The extension will use the existing `llm` module from the serializer crate for conversions, with a TypeScript fallback implementation.

## Architecture

The extension follows a layered architecture:

```
┌─────────────────────────────────────────────────────────────────┐
│                     VS Code Extension                            │
├─────────────────────────────────────────────────────────────────┤
│  extension.ts          │  Entry point, command registration      │
│  dxLensFileSystem.ts   │  Virtual file system provider           │
│  dxDocumentManager.ts  │  Document state and save management     │
│  dxCore.ts             │  WASM wrapper + TypeScript fallback     │
│  utils.ts              │  File type detection, URI conversion    │
├─────────────────────────────────────────────────────────────────┤
│                     DxCore Interface                             │
├─────────────────────────────────────────────────────────────────┤
│  WASM Core (preferred)  │  TypeScript Fallback                   │
│  - llm_to_human()       │  - llmToHuman()                        │
│  - human_to_llm()       │  - humanToLlm()                        │
│  - validate()           │  - validate()                          │
└─────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. DxCore Interface

The core transformation interface that both WASM and TypeScript implementations must satisfy:

```typescript
interface DxCore {
    /** Transform LLM format to human-readable format */
    toHuman(llm: string): TransformResult;
    
    /** Transform human-readable format to LLM format */
    toDense(human: string): TransformResult;
    
    /** Validate content syntax */
    validate(content: string): ValidationResult;
    
    /** Check if content is complete enough to save */
    isSaveable(content: string): boolean;
    
    /** Whether this is using WASM or fallback */
    readonly isWasm: boolean;
}

interface TransformResult {
    success: boolean;
    content: string;
    error?: string;
}

interface ValidationResult {
    success: boolean;
    error?: string;
    line?: number;
    column?: number;
    hint?: string;
}
```

### 2. LLM Parser (TypeScript Fallback)

Parses LLM format into internal DxDocument representation:

```typescript
interface DxDocument {
    context: Map<string, DxValue>;
    refs: Map<string, string>;
    sections: Map<string, DxSection>;
}

interface DxSection {
    schema: string[];
    rows: DxValue[][];
}

type DxValue = 
    | { type: 'string'; value: string }
    | { type: 'number'; value: number }
    | { type: 'bool'; value: boolean }
    | { type: 'null' }
    | { type: 'array'; value: DxValue[] }
    | { type: 'ref'; key: string };
```

### 3. Human Formatter

Converts DxDocument to human-readable format with:
- Section headers with box-drawing characters
- Unicode tables for data sections
- Expanded key names
- Resolved references
- Summary footers

### 4. Human Parser

Parses human-readable format back to DxDocument:
- Recognizes `[section]` headers
- Parses key-value pairs with expanded names
- Parses Unicode tables
- Compresses key names back to abbreviations

### 5. File Type Filter

The `isExactlyDxFile()` function ensures only pure `.dx` files are handled:

```typescript
function isExactlyDxFile(uri: vscode.Uri): boolean {
    // Must be file:// or dxlens:// scheme
    if (uri.scheme !== 'file' && uri.scheme !== 'dxlens') {
        return false;
    }
    
    const filename = path.basename(uri.fsPath);
    
    // Must end with exactly .dx
    if (!filename.endsWith('.dx')) {
        return false;
    }
    
    // Must not have compound extension
    const parts = filename.split('.');
    if (parts.length > 2) {
        // Check if second-to-last part is 'dx' (compound like file.dx.json)
        if (parts[parts.length - 2] === 'dx') {
            return false;
        }
    }
    
    return true;
}
```

## Data Models

### LLM Format Syntax

```
#c:<key>|<val>;<key>|<val>     # Context section
#:<ref>|<value>                 # Reference definition
#<id>(<schema>)                 # Data section header
<row1>                          # Data rows (pipe-separated)
<row2>
```

**Value Types:**
- String: plain text or quoted
- Number: integer or decimal
- Boolean: `+` (true) or `-` (false)
- Null: `~`
- Array: `*item1,item2,item3`
- Reference: `^key`

### Human Format Syntax

```toml
# ═══════════════════════════════════════════════════════════════════════════════
#                                   CONFIG
# ═══════════════════════════════════════════════════════════════════════════════

[config]
    name     = "Test"
    count    = 42
    active   = true

# ═══════════════════════════════════════════════════════════════════════════════
#                                   DATA
# ═══════════════════════════════════════════════════════════════════════════════

[data]
    # Schema: id | value | enabled
    
    ┌────┬───────┬─────────┐
    │ ID │ Value │ Enabled │
    ├────┼───────┼─────────┤
    │  1 │ Alpha │    ✓    │
    │  2 │ Beta  │    ✗    │
    └────┴───────┴─────────┘
    
    Total: 2 items
```

### Abbreviation Dictionary

Standard mappings for key compression/expansion:

| Abbreviation | Full Name | Category |
|--------------|-----------|----------|
| `nm` | name | Identity |
| `tt` | title | Identity |
| `ds` | description | Identity |
| `id` | id | Identity |
| `st` | status | State |
| `ac` | active | State |
| `en` | enabled | State |
| `ct` | count | Metrics |
| `tl` | total | Metrics |
| `pr` | price | Metrics |
| `am` | amount | Metrics |
| `qt` | quantity | Metrics |
| `em` | email | Contact |
| `ph` | phone | Contact |
| `ur` | url | Web |
| `pt` | path | Web |

Context-aware mappings:
- `s` → `sunny` (in hikes), `status` (in orders), `season` (in config)
- `w` → `with` (in hikes), `width` (in images), `weight` (in products)
- `t` → `task` (in config), `type` (in products), `time` (in events)

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: LLM to Human to LLM Round-Trip

*For any* valid LLM format string, converting to human format and back to LLM format SHALL produce a document with equivalent data (same context values, same references, same section data).

**Validates: Requirements 1.1-1.9, 2.1-2.6, 3.1-3.5, 3.6**

### Property 2: Human to LLM to Human Round-Trip

*For any* valid human format string, converting to LLM format and back to human format SHALL produce a document with equivalent data (same context values, same section data).

**Validates: Requirements 3.1-3.5, 3.6**

### Property 3: Boolean Value Preservation

*For any* document containing boolean values, converting between LLM format (`+`/`-`) and human format (`true`/`false` or `✓`/`✗`) SHALL preserve the boolean semantics.

**Validates: Requirements 1.6, 2.4, 3.3**

### Property 4: Null Value Preservation

*For any* document containing null values, converting between LLM format (`~`) and human format (`null` or `—`) SHALL preserve the null semantics.

**Validates: Requirements 1.7, 2.5, 3.4**

### Property 5: Reference Resolution

*For any* document with references, the human format SHALL display resolved values, and converting back to LLM format SHALL recreate equivalent references.

**Validates: Requirements 1.8, 2.6, 3.5**

### Property 6: Key Abbreviation Consistency

*For any* key in the abbreviation dictionary, expanding then compressing (or vice versa) SHALL produce the original key.

**Validates: Requirements 2.1, 3.2, 6.1-6.9**

### Property 7: File Extension Filtering

*For any* file path, `isExactlyDxFile()` SHALL return true if and only if the path ends with exactly `.dx` (not compound extensions like `.dx.json`).

**Validates: Requirements 4.1-4.7**

### Property 8: WASM and TypeScript Equivalence

*For any* valid input, the WASM core and TypeScript fallback SHALL produce equivalent output for all transformation functions.

**Validates: Requirements 5.3, 5.4**

## Error Handling

### Parse Errors

| Error Type | Detection | Message Format |
|------------|-----------|----------------|
| Invalid sigil | Unknown character after `#` | "Invalid sigil '#x' at line N" |
| Malformed reference | `^` not followed by valid key | "Malformed reference at line N" |
| Schema mismatch | Row column count ≠ schema length | "Row has M columns, expected N" |
| Unclosed string | EOF while in string | "Unclosed string starting at line N" |
| Unclosed bracket | EOF with bracket stack | "Unclosed bracket '[' at line N" |

### Validation Errors

All validation errors include:
- Line number (1-indexed)
- Column number (1-indexed)
- Actionable hint for fixing

## Testing Strategy

### Unit Tests

- Test individual parser functions (parseContext, parseRef, parseSection)
- Test individual formatter functions (formatSection, formatTable)
- Test abbreviation dictionary lookups
- Test file extension filtering with various inputs

### Property-Based Tests

Using a property-based testing library (e.g., fast-check for TypeScript):

1. **Round-trip tests**: Generate random valid documents, convert through formats, verify equivalence
2. **Value preservation tests**: Generate documents with specific value types, verify preservation
3. **File filtering tests**: Generate random file paths, verify correct filtering

**Configuration:**
- Minimum 100 iterations per property test
- Each test tagged with: `Feature: dx-serializer-extension-fix, Property N: <description>`

### Integration Tests

- Test full extension activation
- Test file open/save cycle
- Test auto-save behavior
- Test external file change detection
