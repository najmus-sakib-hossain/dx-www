# Design Document: Human Format V3 Fix

## Overview

This design document specifies how to fix the Human Format V3 output inconsistency between the WASM module and TypeScript fallback. The WASM module currently outputs the OLD format with decorative comments and Unicode box-drawing tables, while the TypeScript fallback outputs the correct TOML-like format.

**ROOT CAUSE**: The Rust serializer's `toHuman()` function uses the "hologram" formatter which outputs the old YAML-like format with Unicode tables. The TypeScript `formatDocumentV3()` function outputs the correct TOML-like format.

**SOLUTION**: Update `dxCore.ts` to use the TypeScript formatter for `toHuman()` even when WASM is loaded, while still using WASM for parsing, validation, and `toDense()`. This is a surgical fix that doesn't require modifying the Rust crate.

## Architecture

The fix follows a hybrid approach where WASM is used for parsing/validation but TypeScript is used for formatting:

```
┌─────────────────────────────────────────────────────────────────┐
│                      Current (Broken) Flow                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  LLM Format ──► WASM toHuman() ──► OLD Format (with tables)     │
│                                                                 │
│  Example Output:                                                │
│  # ════════════════════════════════════════════════════════     │
│  #                        CONFIGURATION                         │
│  # ════════════════════════════════════════════════════════     │
│  [config]                                                       │
│  name = "dx"                                                    │
│  ...                                                            │
│  [data]                                                         │
│  ┌──────────┐                                                   │
│  │ path     │                                                   │
│  ├──────────┤                                                   │
│  │ @/driven │                                                   │
│  └──────────┘                                                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                       Fixed Flow                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  LLM Format ──► WASM parse() ──► DxDocument ──► TS formatV3()   │
│                                                                 │
│  Example Output:                                                │
│  author              = essensefromexistence                     │
│  name                = dx                                       │
│  version             = 0.0.1                                    │
│                                                                 │
│  [stack]                                                        │
│  js                  = javascript/typescript | bun | tsc        │
│                                                                 │
│  [driven]                                                       │
│  path                = @/driven                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Updated WasmDxCore Class

The `WasmDxCore` class will be updated to use TypeScript formatting:

```typescript
class WasmDxCore implements DxCore {
    private serializer: WasmSerializer;
    readonly isWasm = true;

    constructor(serializer: WasmSerializer) {
        this.serializer = serializer;
    }

    toHuman(dense: string): TransformResult {
        // FIXED: Use TypeScript formatter instead of WASM
        // WASM outputs old format with Unicode tables
        // TypeScript outputs correct TOML-like format
        try {
            const content = formatDx(dense, 2, 20);
            return { success: true, content };
        } catch (error) {
            return {
                success: false,
                content: '',
                error: error instanceof Error ? error.message : String(error),
            };
        }
    }

    toDense(human: string): TransformResult {
        // Keep using WASM for toDense - it works correctly
        const result = this.serializer.toDense(human);
        return {
            success: result.success,
            content: result.content,
            error: result.error,
        };
    }

    validate(content: string): ValidationResult {
        // Keep using WASM for validation - it has battle-hardened limits
        const result = this.serializer.validate(content);
        return {
            success: result.success,
            error: result.error,
            line: result.line,
            column: result.column,
            hint: result.hint,
        };
    }

    isSaveable(content: string): boolean {
        return this.serializer.isSaveable(content);
    }
}
```

### 2. Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    OPENING A FILE (Fixed)                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  .dx file (LLM format on disk)                                  │
│       │                                                         │
│       ▼                                                         │
│  ┌─────────────────┐                                            │
│  │   parseLlm()    │  (TypeScript - unchanged)                  │
│  │ (llmParser.ts)  │                                            │
│  └────────┬────────┘                                            │
│           │ DxDocument                                          │
│           ▼                                                     │
│  ┌─────────────────────┐                                        │
│  │  formatDocumentV3() │  (TypeScript - Human Format V3)        │
│  │ (humanFormatterV3)  │                                        │
│  └────────┬────────────┘                                        │
│           │                                                     │
│           ▼                                                     │
│  Human V3 format (editor via dxlens://)                         │
│                                                                 │
│  Output Example:                                                │
│  name                = dx                                       │
│  version             = 0.0.1                                    │
│                                                                 │
│  [stack]                                                        │
│  js                  = javascript/typescript | bun | tsc        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                    SAVING A FILE (Unchanged)                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Human V3 format (editor)                                       │
│       │                                                         │
│       ▼                                                         │
│  ┌─────────────────┐                                            │
│  │  WASM toDense() │  (WASM - battle-hardened)                  │
│  │  or TS fallback │                                            │
│  └────────┬────────┘                                            │
│           │                                                     │
│           ▼                                                     │
│  .dx file (LLM format on disk)                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do.*

### Property 1: No Unicode Box-Drawing in Output

*For any* valid LLM format input, the `toHuman()` output SHALL NOT contain any Unicode box-drawing characters (`┌`, `┐`, `└`, `┘`, `─`, `│`, `├`, `┤`, `┬`, `┴`, `┼`).

**Validates: Requirements 1.2**

### Property 2: No Decorative Comments in Output

*For any* valid LLM format input, the `toHuman()` output SHALL NOT contain decorative comment headers matching the pattern `# ═+`.

**Validates: Requirements 1.1**

### Property 3: TOML-like Section Headers

*For any* valid LLM format input with sections, the `toHuman()` output SHALL contain section headers in the format `[section_name]` without decorative borders.

**Validates: Requirements 1.3**

### Property 4: Aligned Key-Value Pairs

*For any* valid LLM format input, the `toHuman()` output SHALL have all `=` signs aligned within each section (keys padded to consistent width).

**Validates: Requirements 1.4**

### Property 5: Pipe Array Separator

*For any* valid LLM format input with arrays, the `toHuman()` output SHALL use ` | ` (space-pipe-space) as the array separator, not commas.

**Validates: Requirements 1.5**

## Error Handling

The fix is surgical and low-risk:
- If TypeScript formatting fails, return the original content (graceful degradation)
- WASM is still used for validation and toDense, so security limits are preserved
- No changes to the complex auto-save, document manager, or file system provider

## Testing Strategy

### Unit Tests

1. **No Box-Drawing Test** - Verify output contains no Unicode table characters
2. **No Decorative Comments Test** - Verify output contains no `# ═` patterns
3. **Section Header Format Test** - Verify sections use `[name]` format
4. **Key Alignment Test** - Verify `=` signs are aligned
5. **Array Separator Test** - Verify arrays use ` | ` separator

### Property Tests

1. **Round-Trip Consistency** - Parse LLM → format to Human V3 → parse Human V3 → serialize to LLM should preserve data
2. **Format Consistency** - WASM and TypeScript should produce identical Human V3 output

## Files to Modify

### Modified Files

| File | Changes |
|------|---------|
| `crates/vscode-dx-serializer/src/dxCore.ts` | Update `WasmDxCore.toHuman()` to use TypeScript formatter |

### Preserved Files (DO NOT MODIFY)

| File | Reason |
|------|--------|
| `dxDocumentManager.ts` | Complex document state management |
| `dxLensFileSystem.ts` | Virtual file system provider |
| `extension.ts` | Activation and auto-save logic |
| `cacheManager.ts` | Cache file generation |
| `humanFormatterV3.ts` | Already outputs correct format |
| `humanParserV3.ts` | Already parses Human V3 correctly |

## Implementation Notes

1. **Why not fix the Rust crate?** - The Rust crate's "hologram" formatter is designed for the old format. Changing it would require significant refactoring and could break other consumers. Using TypeScript for formatting is a quick, safe fix.

2. **Why keep WASM for toDense and validate?** - WASM provides battle-hardened security limits (100 MB input, 1000 recursion depth, 10M table rows) that are important for validation. The TypeScript toDense also works correctly, so either can be used.

3. **Performance impact** - Minimal. Formatting is fast in TypeScript, and the WASM overhead for parsing is avoided by using TypeScript's parseLlm directly.

