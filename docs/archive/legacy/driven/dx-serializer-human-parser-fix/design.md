# Design Document: DX Serializer Human Parser Fix

## Overview

This design document describes the fixes needed for the DX Serializer VSCode extension's human format parser (`humanParserV3.ts`). The current parser has critical bugs that cause file corruption during format-on-save operations. The main issues are:

1. **Stack section parsing bug**: Lines like `js = javascript/typescript | bun | tsc` are incorrectly parsed, with values being stored with `"= value"` prefix
2. **Key-value line parsing bug**: The `parseKeyValueLineV3` function incorrectly handles the `=` sign
3. **Nested section reconstruction bug**: Nested sections like `[i18n.locales]` are not properly reconstructed

## Architecture

The fix involves modifying the existing parser architecture without changing the overall structure:

```
┌─────────────────────────────────────────────────────────────────┐
│                    Human Format Input                            │
│  name = dx                                                       │
│  [stack]                                                         │
│  js = javascript/typescript | bun | tsc | vite | bun | react    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  parseHumanV3() - FIXED                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ parseKeyValueLineV3() - FIX: Split on FIRST = only          ││
│  │ - Extract key: text before first =                          ││
│  │ - Extract value: text after first = (trimmed)               ││
│  └─────────────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Stack Section Handling - FIX: Store refs correctly          ││
│  │ - Join pipe-separated values with | (no spaces)             ││
│  │ - Do NOT include = in stored value                          ││
│  └─────────────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Nested Section Handling - FIX: Preserve all fields          ││
│  │ - Track parent.child relationship                           ││
│  │ - Store with correct prefix (e.g., locales_path)            ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DxDocument                                  │
│  context: Map<string, DxValue>  (config key-values)             │
│  refs: Map<string, string>      (stack section)                 │
│  sections: Map<string, DxSection> (data sections)               │
│  sectionOrder: string[]         (preserve order)                │
└─────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### Modified Functions

#### 1. `parseKeyValueLineV3(line: string): [string, string] | null`

**Current Bug**: The function correctly splits on `=`, but the issue is in how the value is used downstream.

**Fix**: Ensure the returned value is properly trimmed and does not include any `=` prefix.

```typescript
export function parseKeyValueLineV3(line: string): [string, string] | null {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//')) return null;
    if (trimmed.startsWith('[')) return null;
    
    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) return null;
    
    const key = trimmed.substring(0, eqIndex).trim();
    const value = trimmed.substring(eqIndex + 1).trim();
    
    if (!key) return null;
    
    // Validate: value should not start with '=' (indicates double = bug)
    if (value.startsWith('=')) {
        console.warn(`Warning: Value starts with '=' for key '${key}', possible parsing issue`);
    }
    
    return [key, value];
}
```

#### 2. Stack Section Parsing in `parseHumanV3()`

**Current Bug**: The stack section parsing stores values incorrectly. When parsing `js = javascript/typescript | bun | tsc`, the value is being corrupted.

**Fix**: In the stack section handling block, properly extract and store the pipe-separated values:

```typescript
} else if (isStackSection) {
    // Stack section - store as refs with pipe-separated values
    // value is already the text after '=' from parseKeyValueLineV3
    // Join with | (no spaces) for LLM format storage
    const refValue = value.split(' | ').map(v => v.trim()).filter(v => v).join('|');
    doc.refs.set(key, refValue);
}
```

#### 3. Nested Section Reconstruction

**Current Bug**: Nested sections are not properly reconstructed, causing data loss.

**Fix**: Ensure the nested section reconstruction preserves all fields:

```typescript
// Convert nested sections to proper DxSection format
for (const [parentName, subsections] of nestedSections) {
    const sectionId = NESTED_SECTION_PARENTS[parentName] || compressSectionName(parentName);
    const schema: string[] = [];
    const row: DxValue[] = [];

    const subsectionOrder = nestedSectionOrder.get(parentName) || [];
    for (const subsectionName of subsectionOrder) {
        const fields = subsections.get(subsectionName);
        if (fields) {
            for (const field of fields) {
                const prefixedKey = `${subsectionName}_${field.key}`;
                schema.push(prefixedKey);
                row.push(field.value);
            }
        }
    }

    if (schema.length > 0) {
        const section = createSection(sectionId, schema);
        section.rows.push(row);
        doc.sections.set(sectionId, section);
        if (!doc.sectionOrder.includes(sectionId)) {
            doc.sectionOrder.push(sectionId);
        }
    }
}
```

### Re-enabling Format-on-Save

After the parser fixes are verified, format-on-save will be re-enabled in `dxLensFileSystem.ts`:

```typescript
async writeFile(
    uri: vscode.Uri,
    content: Uint8Array,
    options: { create: boolean; overwrite: boolean }
): Promise<void> {
    // ... existing validation code ...

    const humanContent = new TextDecoder().decode(content);
    
    // Parse and re-format for consistent alignment
    const parseResult = parseHumanV3(humanContent);
    if (parseResult.success && parseResult.document) {
        const formattedContent = formatDocumentV3(parseResult.document, DEFAULT_CONFIG);
        // Use formatted content for saving
        content = new TextEncoder().encode(formattedContent);
    }
    
    // Save through document manager
    const saved = await this.documentManager.saveDocument(uri, content);
    // ...
}
```

## Data Models

No changes to data models. The existing `DxDocument`, `DxSection`, and `DxValue` types remain unchanged.

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Round-Trip Consistency

*For any* valid human format document, parsing with `parseHumanV3` then formatting with `formatDocumentV3` SHALL produce a semantically equivalent document where all keys and values are preserved.

**Validates: Requirements 4.1, 4.2, 4.3, 4.4, 6.2, 6.3**

### Property 2: Key-Value Parsing Correctness

*For any* key-value line in format `key = value` (with optional whitespace), the parser SHALL extract the key as the text before the first `=` sign (trimmed) and the value as the text after the first `=` sign (trimmed), correctly handling values that contain additional `=` signs.

**Validates: Requirements 1.1, 1.4, 2.1, 5.1**

### Property 3: No Value Corruption

*For any* parsed human format document, no stored value in the resulting DxDocument (in context, refs, or sections) SHALL start with `=` or contain the pattern `"= "` as a prefix.

**Validates: Requirements 1.2, 2.2**

### Property 4: Nested Section Preservation

*For any* human format document containing nested sections (e.g., `[i18n.locales]`, `[js.dependencies]`), parsing SHALL preserve all key-value pairs with correct prefixes, and the reconstructed DxSection SHALL contain all original data.

**Validates: Requirements 3.1, 3.2, 3.3**

## Error Handling

### Parse Errors

The parser will continue to use the existing error handling structure:

```typescript
interface HumanParseResultV3 {
    success: boolean;
    document?: DxDocument;
    sectionOrder?: string[];
    error?: {
        message: string;
        line: number;
        column: number;
        hint?: string;
    };
}
```

### Format-on-Save Error Handling

When format-on-save encounters a parsing error:
1. The original content is preserved (not overwritten)
2. An error message is displayed to the user via VS Code's notification system
3. The error includes line number and hint for fixing

## Testing Strategy

### Unit Tests

Unit tests will verify specific examples and edge cases:

1. **Stack section parsing examples**:
   - `js = javascript/typescript | bun | tsc | vite | bun | react`
   - `python = py | python | cpython | python | uv | django`

2. **Nested section examples**:
   - `[i18n.locales]` with `path = @/locales`
   - `[js.dependencies]` with `react = 19.0.1`

3. **Edge cases**:
   - Values containing `=` signs (e.g., `url = https://example.com?foo=bar`)
   - Empty values
   - Whitespace variations

### Property-Based Tests

Property-based tests will use a testing library (e.g., fast-check for TypeScript) to verify the correctness properties:

1. **Round-trip property test**: Generate random valid human format documents, parse them, format them, and verify equivalence
2. **Key-value parsing test**: Generate random key-value lines and verify correct extraction
3. **No corruption test**: Generate random documents and verify no values contain corruption patterns
4. **Nested section test**: Generate random nested sections and verify preservation

Each property test should run a minimum of 100 iterations to ensure comprehensive coverage.

**Test Annotation Format**:
```typescript
// Feature: dx-serializer-human-parser-fix, Property 1: Round-Trip Consistency
// Validates: Requirements 4.1, 4.2, 4.3, 4.4, 6.2, 6.3
```
