# DX Serializer VS Code Extension

Seamless editing of `.dx` files and files named exactly `dx` with human-readable display and dense storage.

## Features

- **Dual Format**: Edit human-readable format while storing token-efficient LLM format on disk
- **File Support**: Handles both `.dx` extension files AND files named exactly `dx` (no extension)
- **Syntax Highlighting**: Full TextMate grammar for DX files
- **Auto-Save Compatible**: Grace period prevents saving incomplete code during typing
- **Real-time Validation**: Immediate syntax error feedback with actionable hints
- **Smart Quoting**: Automatic quote selection for strings with apostrophes and special characters
- **WASM Performance**: Rust-compiled WebAssembly core with TypeScript fallback

## How It Works

When you open a `.dx` file or a file named exactly `dx`:
1. The extension reads the LLM format from disk
2. Transforms it to human-readable format for display
3. You edit the readable format in the editor
4. On save, it transforms back to LLM format for storage

This gives you the best of both worlds:
- **Humans**: Beautiful, readable, TOML-like format with Unicode tables
- **LLMs**: Token-efficient sigil-based format (37% smaller than TOON)
- **Git**: Compact diffs, efficient storage

### Format Example

**LLM format (on disk):**
```dx
#c:nm|dx;v|0.0.1;tt|Enhanced Developing Experience;ds|Orchestrate don't just own your code
#f(nm|repo|container)
forge|https://dx.vercel.app/essensefromexistence/dx|none
```

**Human format (in editor):**
```toml
# ═══════════════════════════════════════════════════════════════════════════════
#                                   CONFIG
# ═══════════════════════════════════════════════════════════════════════════════

[config]
    name        = "dx"
    version     = "0.0.1"
    title       = "Enhanced Developing Experience"
    description = "Orchestrate don't just own your code"

# ═══════════════════════════════════════════════════════════════════════════════
#                                   FORGE
# ═══════════════════════════════════════════════════════════════════════════════

[forge]
    # Schema: name | repository | container

    ┌───────┬─────────────────────────────────────────────────┬───────────┐
    │ Name  │ Repository                                      │ Container │
    ├───────┼─────────────────────────────────────────────────┼───────────┤
    │ forge │ https://dx.vercel.app/essensefromexistence/dx   │ none      │
    └───────┴─────────────────────────────────────────────────┴───────────┘
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

## Development

```bash
# Install dependencies
npm install

# Compile
npm run compile

# Run tests
node out/utils.standalone.test.js
node out/smartQuoting.test.js
node out/humanFormat.test.js
node out/validation.test.js
node out/dxCore.test.js
```

## Architecture

```
src/
├── extension.ts          # Entry point, activation
├── dxCore.ts             # WASM wrapper with TypeScript fallback
├── dxDocumentManager.ts  # Document state and validation
├── dxLensFileSystem.ts   # Virtual file system provider
└── utils.ts              # Helper functions
```

## License

MIT
