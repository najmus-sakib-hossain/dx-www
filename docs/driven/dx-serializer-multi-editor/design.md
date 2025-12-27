# Design Document: DX Serializer Multi-Editor Extensions

## Overview

This design document describes the architecture and implementation approach for creating DX serializer extensions for Neovim, Zed, JetBrains IDEs, and Xcode. The design emphasizes code reuse through a shared core library while adapting to each editor's plugin architecture.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     DX Core Library                              │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌────────────┐ │
│  │  llmParser  │ │humanParserV3│ │humanFormatter│ │ serializer │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └────────────┘ │
│                           │                                      │
│              ┌────────────┴────────────┐                        │
│              ▼                         ▼                        │
│     ┌─────────────┐           ┌─────────────┐                   │
│     │  npm package │           │ WASM module │                   │
│     │ (TypeScript) │           │   (Rust)    │                   │
│     └─────────────┘           └─────────────┘                   │
└─────────────────────────────────────────────────────────────────┘
         │                              │
         ▼                              ▼
┌─────────────────┐            ┌─────────────────┐
│  Neovim Plugin  │            │  Zed Extension  │
│     (Lua)       │            │  (Rust/WASM)    │
└─────────────────┘            └─────────────────┘
         │                              │
         ▼                              ▼
┌─────────────────┐            ┌─────────────────┐
│ JetBrains Plugin│            │ Xcode Extension │
│    (Kotlin)     │            │    (Swift)      │
└─────────────────┘            └─────────────────┘
```

## Components and Interfaces

### 1. DX Core Library (`dx-core`)

The core library extracts the parsing and formatting logic from the VS Code extension into a standalone package.

```typescript
// dx-core/index.ts
export interface DxDocument {
    context: Map<string, DxValue>;
    refs: Map<string, string>;
    sections: Map<string, DxSection>;
    sectionOrder?: string[];
}

export interface TransformResult {
    success: boolean;
    content: string;
    error?: string;
}

export interface ValidationResult {
    success: boolean;
    error?: string;
    line?: number;
    column?: number;
    hint?: string;
}

// Core functions
export function parseLlm(input: string): ParseResult;
export function parseHumanV3(input: string): HumanParseResult;
export function formatDocumentV3(doc: DxDocument, config?: FormatConfig): string;
export function serializeToLlmV3(doc: DxDocument): string;
export function validateDx(content: string): ValidationResult;

// High-level transformations
export function llmToHuman(llmContent: string): TransformResult;
export function humanToLlm(humanContent: string): TransformResult;
```

### 2. Neovim Plugin (`dx.nvim`)

```
crates/neovim-dx-serializer/
├── lua/
│   └── dx/
│       ├── init.lua           # Plugin entry point
│       ├── core.lua           # Core transformation logic
│       ├── commands.lua       # User commands
│       ├── autocmds.lua       # Auto-commands for file events
│       └── config.lua         # Configuration options
├── queries/
│   └── dx/
│       └── highlights.scm     # Tree-sitter highlight queries
├── syntax/
│   └── dx.vim                 # Fallback Vim syntax file
├── ftdetect/
│   └── dx.vim                 # Filetype detection
├── ftplugin/
│   └── dx.vim                 # Filetype-specific settings
└── README.md
```

**Key Components:**

```lua
-- lua/dx/init.lua
local M = {}

M.setup = function(opts)
    opts = opts or {}
    M.config = vim.tbl_deep_extend("force", {
        format_on_save = true,
        key_padding = 20,
    }, opts)
    
    require("dx.autocmds").setup()
    require("dx.commands").setup()
end

return M
```

### 3. Zed Extension (`dx-zed`)

```
crates/zed-dx-serializer/
├── src/
│   ├── lib.rs                 # Extension entry point
│   ├── language.rs            # Language server implementation
│   └── transform.rs           # Transformation logic
├── languages/
│   └── dx/
│       ├── config.toml        # Language configuration
│       └── highlights.scm     # Tree-sitter highlights
├── grammars/
│   └── dx/
│       ├── grammar.js         # Tree-sitter grammar
│       └── src/
│           └── parser.c       # Generated parser
├── extension.toml             # Extension manifest
└── Cargo.toml
```

**Key Components:**

```rust
// src/lib.rs
use zed_extension_api::{self as zed, Extension, LanguageServerId};

struct DxExtension;

impl Extension for DxExtension {
    fn new() -> Self {
        DxExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Return language server command
    }
}

zed::register_extension!(DxExtension);
```

### 4. JetBrains Plugin (`dx-intellij`)

```
crates/jetbrains-dx-serializer/
├── src/main/
│   ├── kotlin/
│   │   └── com/dx/intellij/
│   │       ├── DxFileType.kt          # File type registration
│   │       ├── DxLanguage.kt          # Language definition
│   │       ├── DxSyntaxHighlighter.kt # Syntax highlighting
│   │       ├── DxParserDefinition.kt  # Parser definition
│   │       ├── DxAnnotator.kt         # Error annotations
│   │       ├── DxFormattingService.kt # Format-on-save
│   │       └── DxVirtualFileSystem.kt # Virtual file system
│   └── resources/
│       ├── META-INF/
│       │   └── plugin.xml             # Plugin descriptor
│       └── icons/
│           └── dx.svg                 # File icon
├── src/test/
│   └── kotlin/
│       └── com/dx/intellij/
│           └── DxParserTest.kt        # Parser tests
├── build.gradle.kts                   # Gradle build file
└── README.md
```

**Key Components:**

```kotlin
// DxFileType.kt
class DxFileType : LanguageFileType(DxLanguage.INSTANCE) {
    override fun getName() = "DX"
    override fun getDescription() = "DX Configuration File"
    override fun getDefaultExtension() = "dx"
    override fun getIcon() = DxIcons.FILE
}

// DxVirtualFileSystem.kt
class DxVirtualFileSystem : VirtualFileSystem() {
    // Transform LLM to Human on read
    override fun contentsToByteArray(file: VirtualFile): ByteArray {
        val llmContent = file.inputStream.bufferedReader().readText()
        val humanContent = DxCore.llmToHuman(llmContent)
        return humanContent.toByteArray()
    }
    
    // Transform Human to LLM on write
    override fun getOutputStream(file: VirtualFile): OutputStream {
        return DxOutputStream(file)
    }
}
```

### 5. Xcode Extension (`DXSerializer`)

```
crates/xcode-dx-serializer/
├── DXSerializer/
│   ├── DXSerializer.xcodeproj/
│   ├── DXSerializer/
│   │   ├── AppDelegate.swift          # App delegate
│   │   ├── MainMenu.xib               # Main menu
│   │   └── Info.plist                 # App info
│   └── DXSerializerExtension/
│       ├── SourceEditorExtension.swift # Extension entry
│       ├── SourceEditorCommand.swift   # Editor commands
│       ├── DXCore.swift               # Core transformation
│       ├── DXParser.swift             # Parser implementation
│       ├── DXFormatter.swift          # Formatter implementation
│       └── Info.plist                 # Extension info
└── README.md
```

**Key Components:**

```swift
// SourceEditorCommand.swift
class FormatDXCommand: NSObject, XCSourceEditorCommand {
    func perform(with invocation: XCSourceEditorCommandInvocation,
                 completionHandler: @escaping (Error?) -> Void) {
        let buffer = invocation.buffer
        let content = buffer.completeBuffer
        
        // Transform based on current format
        if content.hasPrefix("#") {
            // LLM format - convert to Human
            let result = DXCore.llmToHuman(content)
            if result.success {
                buffer.completeBuffer = result.content
            }
        } else {
            // Human format - format and optionally convert to LLM
            let result = DXCore.formatHuman(content)
            if result.success {
                buffer.completeBuffer = result.content
            }
        }
        
        completionHandler(nil)
    }
}
```

## Data Models

All extensions share the same data model from the core library:

```typescript
// DxValue - represents any value in a DX document
interface DxValue {
    type: 'string' | 'number' | 'bool' | 'null' | 'array' | 'ref';
    value: string | number | boolean | null | DxValue[];
    refKey?: string;
}

// DxSection - represents a data section
interface DxSection {
    id: string;
    schema: string[];
    rows: DxValue[][];
}

// DxDocument - the complete document
interface DxDocument {
    context: Map<string, DxValue>;
    refs: Map<string, string>;
    sections: Map<string, DxSection>;
    sectionOrder?: string[];
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: LLM to Human Round-Trip Consistency

*For any* valid LLM format content, transforming to Human format and back to LLM format SHALL produce semantically equivalent content (same data, possibly different formatting).

**Validates: Requirements 2.3, 3.3, 4.3, 5.3**

### Property 2: Human to LLM Round-Trip Consistency

*For any* valid Human format content, transforming to LLM format and back to Human format SHALL produce semantically equivalent content.

**Validates: Requirements 2.4, 3.4, 4.4, 5.4**

### Property 3: Syntax Highlighting Tokenization

*For any* valid DX Human format content, the Tree-sitter grammar SHALL correctly tokenize section headers, keys, values, pipe separators, and quoted strings into distinct token types.

**Validates: Requirements 6.2, 6.3, 6.4, 6.5**

### Property 4: Validation Error Line Numbers

*For any* invalid DX content with a syntax error, the validation result SHALL include the correct line number where the error occurs.

**Validates: Requirements 7.1, 7.4**

### Property 5: Content Preservation on Failure

*For any* content that fails validation or parsing, the save operation SHALL preserve the original content unchanged.

**Validates: Requirements 7.3, 8.5**

### Property 6: Format-on-Save Key Alignment

*For any* valid Human format content after format-on-save, all keys within the same section SHALL be aligned to the same column position.

**Validates: Requirements 8.1, 8.2**

### Property 7: Section Order Preservation

*For any* DX document with multiple sections, format-on-save SHALL preserve the original order of sections.

**Validates: Requirements 8.3**

### Property 8: LLM Format Detection

*For any* content that starts with `#`, the extension SHALL detect it as LLM format and NOT apply Human formatting transformations.

**Validates: Requirements 8.4**

### Property 9: Malformed Value Warning

*For any* key-value pair where the value starts with `=`, the parser SHALL emit a warning about potential malformed input.

**Validates: Requirements 7.5**

## Error Handling

### Parse Errors

Each extension handles parse errors according to its editor's conventions:

| Editor | Error Display |
|--------|---------------|
| Neovim | Quickfix list with line numbers |
| Zed | Inline diagnostics |
| JetBrains | Problems tool window |
| Xcode | Issue navigator (when possible) |

### Graceful Degradation

1. If parsing fails, preserve original content
2. If transformation fails, show error and don't save
3. If syntax highlighting fails, fall back to plain text

## Testing Strategy

### Unit Tests

Each extension includes unit tests for:
- Parser correctness
- Formatter output
- Validation error detection
- Edge cases (empty files, malformed input)

### Property-Based Tests

The core library includes property-based tests using fast-check:
- Round-trip consistency (Properties 1, 2)
- Tokenization correctness (Property 3)
- Error line number accuracy (Property 4)
- Content preservation (Property 5)
- Key alignment (Property 6)
- Section order (Property 7)

### Integration Tests

Each extension includes integration tests:
- File open/save workflow
- Format-on-save behavior
- Error display in editor UI

### Test Configuration

- Minimum 100 iterations per property test
- Tests tagged with: **Feature: dx-serializer-multi-editor, Property N: {property_text}**
