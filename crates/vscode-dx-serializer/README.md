# DX Serializer VS Code Extension

Seamless editing of `.dx` files and files named exactly `dx` with human-readable display and dense storage.

## Features

- **Dual Format**: Edit human-readable format while storing token-efficient LLM format on disk
- **Human Format V2**: Flat TOML-like structure with full key names and Unicode tables
- **File Support**: Handles both `.dx` extension files AND files named exactly `dx` (no extension)
- **Syntax Highlighting**: Full TextMate grammar for DX files
- **Auto-Save Compatible**: Grace period prevents saving incomplete code during typing
- **Real-time Validation**: Immediate syntax error feedback with actionable hints
- **Smart Quoting**: Automatic quote selection for strings with apostrophes and special characters

## How It Works

When you open a `.dx` file or a file named exactly `dx`:
1. The extension reads the LLM format from disk
2. Transforms it to human-readable V2 format for display
3. You edit the readable format in the editor
4. On save, it transforms back to LLM format for storage

This gives you the best of both worlds:
- **Humans**: Beautiful, readable, flat TOML-like format with Unicode tables
- **LLMs**: Token-efficient sigil-based format (4.8× better than JSON)
- **Git**: Compact diffs, efficient storage

### Format Example

**LLM format (on disk):**
```dx
#c:nm|dx;v|0.0.1;tt|Enhanced Developing Experience;ds|Orchestrate don't just own your code
#f(nm|repo|container)
forge|https://dx.vercel.app/essensefromexistence/dx|none
```

**Human Format V2 (in editor):**
```toml
[config]
name        = dx
version     = 0.0.1
title       = Enhanced Developing Experience
description = Orchestrate don't just own your code

[forge]
name   repository                                      container
forge  https://dx.vercel.app/essensefromexistence/dx  none
```

## Human Format V2 Features

### Clean, Simple Structure
```toml
[config]
name    = MyProject
version = 1.0.0
```

### Full Key Name Expansion
| Abbreviated | Full Name   |
|-------------|-------------|
| `nm`        | `name`      |
| `v`         | `version`   |
| `au`        | `author`    |
| `ws`        | `workspace` |
| `ed`        | `editors`   |
| `repo`      | `repository`|
| `cont`      | `container` |
| `ci`        | `ci_cd`     |

### Full Section Names
```toml
[forge]    # instead of [f]
[stack]    # instead of [k]
[style]    # instead of [y]
[media]    # instead of [m]
```

### Comma-Separated Arrays
```toml
workspace = frontend/www, frontend/mobile, backend/api
```

### Simple Indented Data Tables
Data sections use flat rows with column alignment:
```toml
[data]
id  name   active
1   Alpha  true
2   Beta   false
```

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
| `dx.showDensePreview` | `false` | Show dense preview on hover |

## Status Bar

The extension shows validation status in the status bar:
- ✓ Green checkmark: File is valid and saveable
- ⚠ Warning: File has syntax errors (click to see details)

## File Type Filtering

The extension processes:
- ✓ `config.dx` - Processed (`.dx` extension)
- ✓ `my-app.dx` - Processed (`.dx` extension)
- ✓ `dx` - Processed (file named exactly "dx")
- ✓ `/path/to/dx` - Processed (file named exactly "dx" in any directory)
- ✗ `config.dx.json` - Not processed (compound extension)
- ✗ `config.dx.bak` - Not processed (backup file)
- ✗ `mydx` - Not processed (not exactly "dx")
- ✗ `dxconfig` - Not processed (not exactly "dx")
- ✗ `dx.json` - Not processed ("dx" with extension)

## Auto-Save Compatibility

The extension works seamlessly with VS Code's auto-save feature:
- Validates content before saving
- Skips save during active typing (grace period)
- Preserves last valid content on disk if validation fails
- Shows status bar warning when save is skipped

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
node out/humanFormatter.test.js
node out/humanParser.test.js
node out/dxCore.test.js
```

## Architecture

```
src/
├── extension.ts          # Entry point, activation
├── dxCore.ts             # WASM wrapper with TypeScript fallback
├── dxDocumentManager.ts  # Document state and validation
├── dxLensFileSystem.ts   # Virtual file system provider
├── humanFormatter.ts     # Human Format V2 formatter
├── humanParser.ts        # Human Format V2 parser
├── llmParser.ts          # LLM format parser
└── utils.ts              # Helper functions
```

## Related Documentation

See the project repository for complete documentation:
- `HUMAN.md` - Complete Human Format V2 specification
- `crates/serializer/README.md` - DX Serializer library documentation

## License

MIT
