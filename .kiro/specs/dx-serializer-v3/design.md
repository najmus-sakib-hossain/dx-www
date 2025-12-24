# Design Document: DX Serializer V3

## Overview

DX Serializer V3 introduces a new vertical key-value human format, multi-format input support (JSON, YAML, TOML, CSV), and automatic cache generation. The extension transforms any supported format to DX LLM format on disk, displays a clean human format in the editor, and maintains cache files for human and machine versions.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        VS Code Extension                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐  │
│  │   Format    │    │   DxCore    │    │   Cache Manager     │  │
│  │  Detector   │───▶│  Transform  │───▶│   (.dx/cache/)      │  │
│  └─────────────┘    └─────────────┘    └─────────────────────┘  │
│         │                  │                      │              │
│         ▼                  ▼                      ▼              │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐  │
│  │ JSON/YAML/  │    │ LLM Parser  │    │  Human Format V3    │  │
│  │ TOML/CSV    │    │ & Formatter │    │  Machine Format     │  │
│  │  Parsers    │    └─────────────┘    └─────────────────────┘  │
│  └─────────────┘                                                 │
└─────────────────────────────────────────────────────────────────┘

File Flow:
┌──────────────┐     ┌──────────────┐     ┌──────────────────────┐
│  Input File  │────▶│  LLM Format  │────▶│  .dx/cache/          │
│  (any format)│     │  (on disk)   │     │  ├── dx.human        │
└──────────────┘     └──────────────┘     │  └── dx.machine      │
                            ▲             └──────────────────────┘
                            │
                     ┌──────────────┐
                     │ Human Format │
                     │ (in editor)  │
                     └──────────────┘
```

## Components and Interfaces

### 1. Format Detector (`formatDetector.ts`)

```typescript
export type SourceFormat = 'json' | 'yaml' | 'toml' | 'csv' | 'llm' | 'human' | 'unknown';

export interface DetectionResult {
    format: SourceFormat;
    confidence: number; // 0-1
}

export function detectFormat(content: string): DetectionResult;
```

### 2. Format Converters (`converters/`)

```typescript
// converters/jsonConverter.ts
export function jsonToDxDocument(json: string): DxDocument;

// converters/yamlConverter.ts  
export function yamlToDxDocument(yaml: string): DxDocument;

// converters/tomlConverter.ts
export function tomlToDxDocument(toml: string): DxDocument;

// converters/csvConverter.ts
export function csvToDxDocument(csv: string, sectionId: string): DxDocument;
```

### 3. Human Format V3 (`humanFormatterV3.ts`)

```typescript
export interface HumanFormatV3Config {
    keyPadding: number;      // Minimum padding for keys (default: 20)
    arraySeparator: string;  // Separator for arrays (default: ' | ')
    quoteStringsWithSpaces: boolean;
}

export function formatDocumentV3(doc: DxDocument, config?: HumanFormatV3Config): string;
export function parseHumanV3(content: string): HumanParseResult;
```

### 4. Machine Format (`machineFormat.ts`)

```typescript
export function documentToMachine(doc: DxDocument): Buffer;
export function machineToDocument(buffer: Buffer): DxDocument;
```

### 5. Cache Manager (`cacheManager.ts`)

```typescript
export interface CacheManager {
    ensureCacheDir(): Promise<void>;
    writeHumanCache(sourcePath: string, content: string): Promise<void>;
    writeMachineCache(sourcePath: string, content: Buffer): Promise<void>;
    readHumanCache(sourcePath: string): Promise<string | null>;
    readMachineCache(sourcePath: string): Promise<Buffer | null>;
    deleteCacheFiles(sourcePath: string): Promise<void>;
}
```

## Data Models

### Human Format V3 Structure

The new human format uses a vertical key-value layout:

```toml
# Config values (no section header)
name                 = dx
version              = 0.0.1
title                = Enhanced Developing Experience
description          = "Orchestrate don't just own your code"
author               = essensefromexistence
license              = MIT

# Data sections with schema header
[stack]              = Lang | Runtime | Compiler | Bundler | PM | Framework
javascript           = javascript/typescript | bun | tsc | vite | bun | react
python               = python | cpython | - | - | uv | django
rust                 = rust | native | rustc | - | cargo | -

[forge]
repository           = https://dx.vercel.app/essensefromexistence/dx
container            = none
pipeline             = none
tasks                = none
items                = cli | docs | examples | packages | scripts | style | tests

[style]
path                 = @/style
engine               = atomic | enhanced | logical
themes               = dx | vercel | claude
```

### Key Formatting Rules

1. **Key Padding**: All keys padded to 20 characters (or longest key + 1)
2. **Array Separator**: Use ` | ` instead of `, `
3. **Section Schema**: `[section] = Col1 | Col2 | Col3` for multi-row sections
4. **Single Row Sections**: Display as vertical key-value pairs
5. **Quoted Strings**: Strings with spaces wrapped in double quotes

### Machine Format Structure

Binary format for efficient machine parsing:

```
Header (8 bytes):
  - Magic: "DXMF" (4 bytes)
  - Version: uint16 (2 bytes)
  - Flags: uint16 (2 bytes)

Context Section:
  - Count: uint16
  - For each entry:
    - Key length: uint8
    - Key: utf8 bytes
    - Value type: uint8 (0=string, 1=number, 2=bool, 3=null, 4=array)
    - Value length: uint16
    - Value: bytes

Data Sections:
  - Section count: uint8
  - For each section:
    - Section ID: uint8
    - Schema column count: uint8
    - Schema columns: [length:uint8, name:utf8][]
    - Row count: uint16
    - Rows: [values][]
```


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Format Detection Accuracy

*For any* valid JSON, YAML, TOML, CSV, LLM, or Human format content, the format detector SHALL correctly identify the format type with high confidence.

**Validates: Requirements 5.1, 5.2, 5.3, 5.4, 5.5, 5.6**

### Property 2: Format Conversion Preserves Data

*For any* valid JSON, YAML, TOML, or CSV content, converting to DxDocument and back to the original format SHALL preserve all data values (though structure may differ).

**Validates: Requirements 1.1, 1.2, 1.3, 1.4**

### Property 3: Human Format V3 Round-Trip

*For any* valid DxDocument, formatting to Human V3 and parsing back SHALL produce an equivalent document.

**Validates: Requirements 2.1-2.7, 3.1**

### Property 4: Key Alignment Consistency

*For any* Human V3 formatted output, all `=` signs SHALL appear at the same column position (longest key length + padding).

**Validates: Requirements 2.3**

### Property 5: Array Separator Consistency

*For any* array value in Human V3 format, the separator SHALL be ` | ` and not `, `.

**Validates: Requirements 2.4**

### Property 6: No Table Formatting

*For any* Human V3 formatted output, the content SHALL NOT contain Unicode table border characters (`┌`, `┐`, `└`, `┘`, `─`, `│`, `├`, `┤`, `┬`, `┴`, `┼`).

**Validates: Requirements 2.6**

### Property 7: String Quoting

*For any* string value containing spaces, the Human V3 format SHALL wrap it in double quotes.

**Validates: Requirements 2.7**

### Property 8: Cache Path Preservation

*For any* source file path with subdirectories, the cache files SHALL preserve the relative path structure under `.dx/cache/`.

**Validates: Requirements 4.4**

### Property 9: Machine Format Round-Trip

*For any* valid DxDocument, serializing to machine format and deserializing back SHALL produce an identical document.

**Validates: Requirements 1.7, 3.4**

## Error Handling

1. **Invalid Format Detection**: Return `unknown` format with low confidence
2. **Parse Errors**: Return detailed error with line/column information
3. **Conversion Errors**: Preserve original content, show error message
4. **Cache Write Errors**: Log warning, continue without caching
5. **File System Errors**: Show user-friendly error message

## Testing Strategy

### Unit Tests
- Format detection for each supported format
- Individual converter functions
- Human V3 formatter edge cases
- Machine format serialization

### Property-Based Tests
- Round-trip consistency (Property 3, 9)
- Format detection accuracy (Property 1)
- Formatting rules (Properties 4, 5, 6, 7)

### Integration Tests
- Full pipeline: paste → detect → convert → save → cache
- Bidirectional sync: edit human → update LLM → regenerate cache
- Cache file management
