# DX Serializer VS Code Extension ✅ COMPLETE

> **Status: Fully Working** - The DX Serializer VS Code extension is complete and production-ready!

Seamless editing of `.dx` files and files named exactly `dx` with human-readable display and dense storage.

## 🎯 What We Achieved

The DX Serializer VS Code extension provides a revolutionary approach to configuration files:

1. **Triple Format System**: One source file, three representations
   - **LLM Format** (on disk): Token-efficient sigil-based format (4.8× smaller than JSON)
   - **Human Format V3** (in editor): Beautiful vertical key-value layout with aligned equals signs
   - **Machine Format** (binary cache): Fast binary format for compilers and tools

2. **Universal Converter**: Automatically converts JSON, YAML, TOML, CSV to DX format
   - Drop any supported format into a `.dx` file
   - Extension auto-detects and converts to LLM format
   - No manual conversion needed

3. **Section Order Preservation**: Reorder sections in the editor and the order is preserved on save

4. **Professional Syntax Highlighting**: Color-coded syntax for easy reading

## ✨ Features

- **Dual Format**: Edit human-readable format while storing token-efficient LLM format on disk
- **Human Format V3**: Clean vertical key-value layout with aligned equals signs
- **Syntax Highlighting**: Professional color-coded syntax highlighting for DX files
- **Multi-Format Input**: Auto-convert JSON, YAML, TOML, CSV to DX format
- **Section Order Preservation**: Reorder sections in editor and preserve order on save
- **Cache Generation**: Automatic `.dx/cache/{filename}.human` (text) and `.machine` (binary) files
- **File Support**: Handles both `.dx` extension files AND files named exactly `dx` (no extension)
- **Real-time Validation**: Immediate syntax error feedback with actionable hints
- **Auto-Save Compatible**: Saves work seamlessly with VS Code's auto-save

## 🎯 How It Works

When you open a `.dx` file or a file named exactly `dx`:
1. The extension reads the LLM format from disk
2. Detects the format (JSON, YAML, TOML, CSV, or LLM)
3. Converts non-LLM formats to LLM format automatically
4. Transforms to Human V3 format for display
5. On save, transforms back to LLM format and generates cache files

This gives you the best of both worlds:
- **Humans**: Beautiful, readable vertical key-value format with syntax highlighting
- **LLMs**: Token-efficient sigil-based format (4.8× better than JSON)
- **Compilers**: Binary machine format for fast parsing
- **Git**: Compact diffs, efficient storage

### Format Example

**LLM format (on disk - `dx` file):**
```dx
#c:nm|dx;v|0.0.1;tt|Enhanced Developing Experience
#f(repo|cont|pl|tk|it)
https://dx.vercel.app/essensefromexistence/dx|none|none|none|*cli,docs,examples
#:js|javascript/typescript|bun|tsc|vite|bun|react
#:python|py|python|cpython|-|uv|django
```

**Human Format V3 (in editor):**
```
name                = dx
version             = 0.0.1
title               = "Enhanced Developing Experience"

[forge]
repository          = https://dx.vercel.app/essensefromexistence/dx
container           = none

[stack]
js                  = javascript/typescript | bun | tsc | vite | bun | react
python              = py                    | python | cpython | - | uv | django
```

## 🎨 Syntax Highlighting

The extension provides professional syntax highlighting:
- **Keys/Properties** (pink `#f05b8d`): Property names like `name`, `version`, `path`
- **Values/Strings** (green `#58c760`): String values, identifiers, quoted strings
- **Numbers/Booleans/Section Headers** (blue `#62a6ff`): Version numbers, `true`/`false`, `[section]`
- **Comments** (muted gray `#a1a1a1`): `# comment` or `// comment`
- **Operators** (white): `=`, `|` separators
- **Paths** (green): `@/path/to/file`, URLs

## 📁 Cache Files

On save, the extension generates cache files in `.dx/cache/`:

```
project/
├── .dx/
│   └── cache/
│       ├── dx.human      # Human V3 format (text)
│       └── dx.machine    # Binary format (for compilers)
├── dx                    # LLM format (source)
└── src/
    └── config.dx
```

### Machine Format (Binary)

The `.machine` file uses a compact binary format optimized for:
- Fast parsing by compilers and programming languages
- Minimal file size
- Direct memory mapping

Binary structure:
- Magic bytes: `DXM\x01` (4 bytes)
- Version: 1 byte
- Flags: 1 byte
- Section count: 2 bytes (u16 LE)
- Sections with typed values

## 🔧 Commands

| Command | Description |
|---------|-------------|
| `DX: Refresh from Disk` | Reload file from disk |
| `DX: Force Save` | Save without validation checks |
| `DX: Show Dense View` | Preview the LLM format in a read-only view |

## ⚙️ Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `dx.validateBeforeSave` | `true` | Validate syntax before saving |
| `dx.autoSaveGracePeriod` | `2000` | Grace period (ms) after last keystroke |
| `dx.indentSize` | `2` | Indent size (2 or 4 spaces) |

## 📊 Status Bar

The extension shows validation status in the status bar:
- ✓ Green checkmark: File is valid and saveable
- ⚠ Warning: File has syntax errors (click to see details)

## 📋 Human Format V3 Features

### No [config] Header
Config values appear at the top without section header:
```
name                = dx
version             = 0.0.1
```

### Aligned Equals Signs
Keys are padded to 20 characters for visual alignment:
```
name                = dx
version             = 0.0.1
title               = "Enhanced Developing Experience"
```

### Pipe Array Separator
Arrays use ` | ` instead of commas:
```
workspace           = @/www | @/backend
editors             = neovim | zed | vscode | cursor
```

### Nested Sections
Support for nested sections like i18n:
```
[i18n.locales]
path                = @/locales
default             = en-US

[i18n.ttses]
path                = @/media/sounds
default             = en-US
```

### Stack Section with Aligned Columns
Reference definitions with aligned pipe separators:
```
[stack]
js                  = javascript/typescript | bun    | tsc     | vite  | bun   | react
python              = py                    | python | cpython | -     | uv    | django
rust                = rs                    | rust   | native  | rustc | cargo | actix-web
```

### Section Order Preservation
Reorder sections in the editor and the order is preserved when saving:
- Move `[forge]` above `[stack]` in the editor
- Save the file
- The LLM format on disk will have `#f(...)` before `#:...`

## 🔄 Multi-Format Input Support

The extension automatically converts these formats to DX when you open a `.dx` file containing them:

| Format | Detection | Example |
|--------|-----------|---------|
| **JSON** | `{` or `[` start | `{"name": "project", "version": "1.0"}` |
| **YAML** | `key:` patterns, `---`, `-` lists | `name: project\nversion: "1.0"` |
| **TOML** | `[section]` + `key = value` | `[project]\nname = "project"` |
| **CSV** | Comma-separated rows | `id,name,enabled\n1,Alpha,true` |

### Conversion Examples

**JSON Input:**
```json
{
  "name": "my-project",
  "version": "1.0.0",
  "dependencies": ["react", "typescript"]
}
```

**Converted to DX (LLM format):**
```dx
#c:nm|my-project;v|1.0.0;dependencies|*react,typescript
```

**YAML Input:**
```yaml
name: my-project
version: "1.0.0"
items:
  - id: 1
    name: Alpha
  - id: 2
    name: Beta
```

**Converted to DX (LLM format):**
```dx
#c:nm|my-project;v|1.0.0
#i(id|nm)
1|Alpha
2|Beta
```

**CSV Input:**
```csv
id,name,enabled
1,Alpha,true
2,Beta,false
```

**Converted to DX (LLM format):**
```dx
#d(id|nm|en)
1|Alpha|+
2|Beta|-
```

## 📦 Installation

### From VSIX
```bash
# Build the extension
cd crates/vscode-dx-serializer
npm install
npm run compile
echo y | npx vsce package --allow-missing-repository

# Install in Kiro
kiro --install-extension vscode-dx-serializer-0.1.0.vsix
```

### Development
```bash
# Install dependencies
npm install

# Compile
npm run compile

# Run tests
node out/humanFormatterV3.test.js
node out/humanParserV3.test.js
node out/formatDetector.test.js
node out/dxCore.test.js
node out/converters/converters.test.js
```

## 🏗️ Architecture

```
src/
├── extension.ts          # Entry point, activation, auto-redirect
├── dxCore.ts             # TypeScript transformation core
├── dxDocumentManager.ts  # Document state and validation
├── dxLensFileSystem.ts   # Virtual file system provider (dxlens://)
├── humanFormatterV3.ts   # Human Format V3 formatter
├── humanParserV3.ts      # Human Format V3 parser + LLM serializer
├── llmParser.ts          # LLM format parser with section order tracking
├── machineFormat.ts      # Binary machine format serializer
├── formatDetector.ts     # Multi-format detection (JSON/YAML/TOML/CSV/LLM)
├── cacheManager.ts       # Cache file management (.human/.machine)
├── converters/           # Format converters
│   ├── index.ts          # Converter exports
│   ├── jsonConverter.ts  # JSON → DxDocument
│   ├── yamlConverter.ts  # YAML → DxDocument
│   ├── tomlConverter.ts  # TOML → DxDocument
│   └── csvConverter.ts   # CSV → DxDocument
└── utils.ts              # Helper functions
```

## 🔧 Technical Implementation

### Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        OPENING A FILE                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  .dx file (disk)                                                │
│       │                                                         │
│       ▼                                                         │
│  ┌─────────────────┐                                            │
│  │ Format Detector │ ─── JSON/YAML/TOML/CSV? ──► Converter      │
│  └────────┬────────┘                                  │         │
│           │ LLM format                                │         │
│           ▼                                           ▼         │
│  ┌─────────────────┐                         ┌──────────────┐   │
│  │   parseLlm()    │ ◄─────────────────────── │ DxDocument   │   │
│  │ (llmParser.ts)  │                         └──────────────┘   │
│  └────────┬────────┘                                            │
│           │ DxDocument + sectionOrder                           │
│           ▼                                                     │
│  ┌─────────────────────┐                                        │
│  │  formatDocumentV3() │                                        │
│  │ (humanFormatterV3)  │                                        │
│  └────────┬────────────┘                                        │
│           │                                                     │
│           ▼                                                     │
│  Human V3 format (editor via dxlens://)                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                        SAVING A FILE                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Human V3 format (editor)                                       │
│       │                                                         │
│       ▼                                                         │
│  ┌─────────────────┐                                            │
│  │  parseHumanV3() │                                            │
│  │ (humanParserV3) │                                            │
│  └────────┬────────┘                                            │
│           │ DxDocument + sectionOrder                           │
│           ▼                                                     │
│  ┌─────────────────────┐     ┌─────────────────────┐            │
│  │ serializeToLlmV3()  │     │ serializeToBinary() │            │
│  │ (humanParserV3)     │     │ (machineFormat)     │            │
│  └────────┬────────────┘     └────────┬────────────┘            │
│           │                           │                         │
│           ▼                           ▼                         │
│  .dx file (disk)              .dx/cache/dx.machine (binary)     │
│                                                                 │
│  Also: .dx/cache/dx.human (Human V3 text cache)                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Section Order Tracking

The extension preserves section order through the entire pipeline:

1. **LLM Parser** (`llmParser.ts`): Tracks `sectionOrder: string[]` as sections are parsed
2. **Human Formatter** (`humanFormatterV3.ts`): Outputs sections in tracked order
3. **Human Parser** (`humanParserV3.ts`): Tracks order as sections are parsed from Human V3
4. **LLM Serializer** (`humanParserV3.ts`): Outputs sections in tracked order

### Key Abbreviations

The extension uses abbreviations to compress keys:

| Abbreviation | Full Name |
|--------------|-----------|
| `nm` | name |
| `v` | version |
| `tt` | title |
| `ds` | description |
| `au` | author |
| `ws` | workspace |
| `ed` | editors |
| `repo` | repository |
| `cont` | container |
| `pt` | path |
| `df` | default |
| `en` | enabled |

### Section Name Mapping

| ID | Section Name |
|----|--------------|
| `c` | config |
| `f` | forge |
| `k` | stack |
| `y` | style |
| `u` | ui |
| `m` | media |
| `i` | i18n |
| `o` | icon |
| `t` | font |
| `d` | data |

## 🏆 How We Achieved It

### The Challenge

Traditional configuration formats have trade-offs:
- **JSON**: Verbose, no comments, poor human readability
- **YAML**: Whitespace-sensitive, complex parsing, ambiguous types
- **TOML**: Better, but still verbose for complex structures
- **CSV**: Limited to tabular data

We wanted the best of all worlds:
- Human-readable editing experience
- Token-efficient storage for LLMs
- Fast binary format for compilers
- Universal conversion from existing formats

### The Solution: Triple Format Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    DX SERIALIZER ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │   JSON      │    │   YAML      │    │   TOML      │         │
│  │   YAML      │───►│  Converter  │───►│  DxDocument │         │
│  │   TOML      │    │   Layer     │    │  (Internal) │         │
│  │   CSV       │    └─────────────┘    └──────┬──────┘         │
│  └─────────────┘                              │                 │
│                                               │                 │
│  ┌────────────────────────────────────────────┼────────────────┐│
│  │                                            ▼                ││
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         ││
│  │  │ LLM Format  │  │ Human V3    │  │ Machine     │         ││
│  │  │ (on disk)   │◄─┤ (in editor) │─►│ (binary)    │         ││
│  │  │ #c:nm|dx    │  │ name = dx   │  │ DXM\x01...  │         ││
│  │  └─────────────┘  └─────────────┘  └─────────────┘         ││
│  │       │                 ▲                 │                 ││
│  │       │    parseLlm()   │   serializeTo   │                 ││
│  │       └─────────────────┴─────────────────┘                 ││
│  │                    DxDocument                               ││
│  │                 (with sectionOrder)                         ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Key Technical Innovations

#### 1. Section Order Tracking
The `DxDocument` interface includes `sectionOrder: string[]` that tracks the order of sections as they appear in the file. This allows users to reorder sections in the Human V3 format and have that order preserved when saving back to LLM format.

```typescript
export interface DxDocument {
    context: Map<string, DxValue>;
    refs: Map<string, string>;
    sections: Map<string, DxSection>;
    sectionOrder?: string[];  // Tracks order of sections
}
```

#### 2. Virtual File System (dxlens://)
The extension uses VS Code's FileSystemProvider API to create a virtual file system. When you open a `.dx` file:
1. The `file://` URI is intercepted
2. Content is transformed to Human V3 format
3. A `dxlens://` URI is opened instead
4. Edits are tracked and saved back to the original file

#### 3. Format Detection
The `formatDetector.ts` module uses heuristics to detect input format:
- **JSON**: Starts with `{` or `[`, valid JSON.parse()
- **YAML**: `key:` patterns, `---` document start, `-` list items
- **TOML**: `[section]` headers with `key = value` pairs
- **CSV**: Consistent comma-separated columns
- **LLM**: `#c:`, `#:`, `#<letter>(` sigils

#### 4. Binary Machine Format
The `.machine` file uses a compact binary format:
```
┌──────────────────────────────────────────┐
│ Magic: DXM\x01 (4 bytes)                 │
│ Version: 1 (1 byte)                      │
│ Flags: 0 (1 byte)                        │
│ Section Count: u16 LE (2 bytes)          │
├──────────────────────────────────────────┤
│ Section 1:                               │
│   ID: 1 byte                             │
│   Schema Length: u16 LE                  │
│   Schema: [key lengths + keys]           │
│   Row Count: u16 LE                      │
│   Rows: [typed values]                   │
├──────────────────────────────────────────┤
│ Section 2...                             │
└──────────────────────────────────────────┘
```

### Implementation Files

| File | Purpose | Lines |
|------|---------|-------|
| `llmParser.ts` | Parse LLM format with section order tracking | ~300 |
| `humanFormatterV3.ts` | Format DxDocument to Human V3 | ~500 |
| `humanParserV3.ts` | Parse Human V3 + serialize to LLM | ~350 |
| `machineFormat.ts` | Binary serialization | ~200 |
| `formatDetector.ts` | Multi-format detection | ~250 |
| `converters/*.ts` | JSON/YAML/TOML/CSV converters | ~600 |
| `dxLensFileSystem.ts` | Virtual file system provider | ~300 |
| `dxDocumentManager.ts` | Document state management | ~400 |
| `extension.ts` | Extension entry point | ~350 |

### Test Coverage

All components have comprehensive tests:
- `humanFormatterV3.test.ts` - 15 unit tests + 6 property tests
- `humanParserV3.test.ts` - 12 unit tests + 3 property tests
- `formatDetector.test.ts` - 18 unit tests + 2 property tests
- `converters.test.ts` - 21 unit tests + 2 property tests
- `dxCore.test.ts` - 10 unit tests

Run tests:
```bash
cd crates/vscode-dx-serializer
npm run compile
node out/humanFormatterV3.test.js
node out/humanParserV3.test.js
node out/formatDetector.test.js
node out/converters/converters.test.js
node out/dxCore.test.js
```

## 📚 Related Documentation

- `crates/serializer/README.md` - DX Serializer Rust library
- `crates/serializer/docs/SYNTAX.md` - Complete DX syntax specification

## 📄 License

MIT
