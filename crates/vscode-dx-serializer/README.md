# DX Serializer VS Code Extension

Seamless editing of `.dx` files with human-readable display and dense storage.

## Features

- **Dual Format**: Edit human-readable format while storing token-efficient dense format on disk
- **Syntax Highlighting**: Full TextMate grammar for `.dx` files
- **Auto-Save Compatible**: Grace period prevents saving incomplete code during typing
- **Real-time Validation**: Immediate syntax error feedback with actionable hints
- **WASM Performance**: Rust-compiled WebAssembly core with TypeScript fallback

## How It Works

When you open a `.dx` file:
1. The extension reads the dense format from disk
2. Transforms it to human-readable format for display
3. You edit the readable format in the editor
4. On save, it transforms back to dense format for storage

This gives you the best of both worlds:
- **Humans**: Beautiful, readable, indented code
- **LLMs**: Token-efficient, minimal bytes
- **Git**: Compact diffs, efficient storage

## Commands

- `DX: Refresh from Disk` - Reload file from disk
- `DX: Force Save` - Save without validation
- `DX: Show Dense View` - Preview the dense format

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `dx.validateBeforeSave` | `true` | Validate syntax before saving |
| `dx.autoSaveGracePeriod` | `2000` | Grace period (ms) after last keystroke |
| `dx.indentSize` | `2` | Indent size (2 or 4 spaces) |
| `dx.showDensePreview` | `false` | Show dense preview on hover |

## Requirements

- VS Code 1.85.0 or higher

## License

MIT
