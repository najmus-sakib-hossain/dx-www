# DX Serializer VS Code Extension

Seamless editing of `.dx` files and files named exactly `dx` with human-readable display and dense storage.

## Features

- **Dual Format**: Edit human-readable format while storing token-efficient LLM format on disk
- **Human Format V3**: Clean vertical key-value layout with aligned equals signs
- **Multi-Format Input**: Auto-convert JSON, YAML, TOML, CSV to DX format
- **Cache Generation**: Automatic `.dx/cache/{filename}.human` and `.machine` files
- **File Support**: Handles both `.dx` extension files AND files named exactly `dx` (no extension)
- **Syntax Highlighting**: Full TextMate grammar for DX files
- **Auto-Save Compatible**: Grace period prevents saving incomplete code during typing
- **Real-time Validation**: Immediate syntax error feedback with actionable hints

## How It Works

When you open a `.dx` file or a file named exactly `dx`:
1. The extension reads the LLM format from disk
2. Detects the format (JSON, YAML, TOML, CSV, or LLM)
3. Converts non-LLM formats to LLM format automatically
4. Transforms to Human V3 format for display
5. On save, transforms back to LLM format and generates cache files

This gives you the best of both worlds:
- **Humans**: Beautiful, readable vertical key-value format
- **LLMs**: Token-efficient sigil-based format (4.8× better than JSON)
- **Git**: Compact diffs, efficient storage

### Format Example

**LLM format (on disk):**
```dx
#c:nm|dx;v|0.0.1;tt|Enhanced Developing Experience
#:js|javascript/typescript|bun|tsc|vite|bun|react
#f(nm|repo|container)
forge|https://dx.vercel.app/essensefromexistence/dx|none
```

**Human Format V3 (in editor):**
```
name                 = dx
version              = 0.0.1
title                = "Enhanced Developing Experience"

[stack]              = Lang | Runtime | Compiler | Bundler | PM | Framework
js                   = javascript/typescript | bun | tsc | vite | bun | react

[forge]              = Name | Repository | Container
forge                = https://dx.vercel.app/essensefromexistence/dx | none
```

## Human Format V3 Features

### No [config] Header
Config values appear at the top without section header:
```
name                 = dx
version              = 0.0.1
```

### Aligned Equals Signs
Keys are padded to 20 characters for visual alignment:
```
name                 = dx
version              = 0.0.1
title                = "Enhanced Developing Experience"
```

### Pipe Array Separator
Arrays use ` | ` instead of commas:
```
workspace            = frontend/www | frontend/mobile | backend/api
```

### Section Headers with Schema
Data sections show schema in the header:
```
[stack]              = Lang | Runtime | Compiler
javascript           = js | bun | tsc
```

## Multi-Format Input Support

The extension automatically converts these formats to DX:

| Format | Detection | Example |
|--------|-----------|---------|
| JSON | `{` or `[` start | `{"name": "project"}` |
| YAML | `key:` patterns | `name: project` |
| TOML | `[section]` + `key = value` | `[project]\nname = "project"` |
| CSV | Comma-separated rows | `id,name\n1,Alpha` |

## Cache Files

On save, the extension generates cache files in `.dx/cache/`:

```
project/
├── .dx/
│   └── cache/
│       ├── config.human    # Human V3 format
│       └── config.machine  # JSON format
├── config.dx               # LLM format (source)
└── src/
    └── app.dx
```

Cache files preserve subdirectory structure:
- `src/app.dx` → `.dx/cache/src/app.human` and `.dx/cache/src/app.machine`

## Commands

| Command | Description |
|---------|-------------|
| `DX: Refresh from Disk` | Reload file from disk (also in editor title bar) |
| `DX: Force Save` | Save without validation checks |
| `DX: Show Dense View` | Preview the dense format in a read-only view |

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `dx.validateBeforeSave` | `true` | Validate syntax before saving |
| `dx.autoSaveGracePeriod` | `2000` | Grace period (ms) after last keystroke |
| `dx.indentSize` | `2` | Indent size (2 or 4 spaces) |

## Status Bar

The extension shows validation status in the status bar:
- ✓ Green checkmark: File is valid and saveable
- ⚠ Warning: File has syntax errors (click to see details)

## File Type Filtering

The extension processes:
- ✓ `config.dx` - Processed (`.dx` extension)
- ✓ `my-app.dx` - Processed (`.dx` extension)
- ✓ `dx` - Processed (file named exactly "dx")
- ✗ `config.dx.json` - Not processed (compound extension)
- ✗ `mydx` - Not processed (not exactly "dx")

## Requirements

- VS Code 1.85.0 or higher
- Kiro IDE (recommended)

## Development

```bash
# Install dependencies
npm install

# Compile
npm run compile

# Run tests
node out/humanFormatterV3.test.js
node out/humanParserV3.test.js
node out/formatDetector.test.js
node out/converters/converters.test.js
node out/dxCore.test.js
```

## Architecture

```
src/
├── extension.ts          # Entry point, activation
├── dxCore.ts             # TypeScript transformation core
├── dxDocumentManager.ts  # Document state and validation
├── dxLensFileSystem.ts   # Virtual file system provider
├── humanFormatterV3.ts   # Human Format V3 formatter
├── humanParserV3.ts      # Human Format V3 parser
├── formatDetector.ts     # Multi-format detection
├── cacheManager.ts       # Cache file management
├── machineFormat.ts      # Machine format serialization
├── converters/           # Format converters
│   ├── jsonConverter.ts
│   ├── yamlConverter.ts
│   ├── tomlConverter.ts
│   └── csvConverter.ts
├── llmParser.ts          # LLM format parser
└── utils.ts              # Helper functions
```

## Related Documentation

See the project repository for complete documentation:
- `docs/HUMAN.md` - Complete Human Format V3 specification
- `crates/serializer/README.md` - DX Serializer library documentation

## License

MIT
