# DX Hologram - VS Code Extension

**Binary-first holographic editing for DX configuration files**

## What is this?

The DX Hologram extension provides a revolutionary editing experience for DX configuration files. It implements "Quantum Superposition" for config files - showing you a beautiful, human-readable format while storing an ultra-efficient LLM format, and generating binary `.dxb` files on save.

## The Three Formats

| Format | Purpose | Location | Size |
|--------|---------|----------|------|
| **Human** | Editor display | Virtual (in-memory) | Readable |
| **LLM** | Storage | `.dx` file on disk | 5x smaller |
| **Machine** | Runtime | `.dxb` binary file | 0.70ns access |

## Example Transformation

### Human Format (What you see in editor)

```
▼ server
    host: localhost
    port: 5432
    ssl:  ✓

▼ features (3 items)
    • authentication
    • caching
    • compression

▼ users (3 columns × 2 rows)
    ┌──────────┬─────────┬────────┐
    │ name     │ role    │ active │
    ├──────────┼─────────┼────────┤
    │ alice    │ admin   │ ✓      │
    │ bob      │ user    │ ✗      │
    └──────────┴─────────┴────────┘
```

### LLM Format (What's stored on disk)

```
server#host:localhost#port:5432#ssl:1
features@3>authentication|caching|compression
users@2=name^role^active
>alice|admin|1
>bob|user|0
```

### Machine Format (What runs at runtime)

```
Binary: 47 bytes → 0.70ns field access
```

## Features

- **🔮 Holographic Editing**: See human format, save LLM format
- **📦 Auto Binary Build**: Creates `.dxb` on save via dx-forge daemon
- **🎨 Beautiful Tables**: Box-drawing characters for tabular data
- **✓✗ Unicode Symbols**: Booleans as ✓/✗, nulls as —
- **🔧 JSON Import**: Convert existing JSON to DX format
- **⚡ Status Bar**: Shows dx-forge daemon status

## Commands

| Command | Description |
|---------|-------------|
| `DX: Open in Hologram Mode` | Open file in human-readable format |
| `DX: Show LLM Format` | View the underlying LLM-dense format |
| `DX: Build Binary` | Manually trigger binary build |
| `DX: Start Forge Daemon` | Start the dx-forge background process |
| `DX: Stop Forge Daemon` | Stop the dx-forge background process |
| `DX: Convert JSON to DX` | Import JSON file as DX format |

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `dx-hologram.autoInflate` | `true` | Auto-show human format when opening |
| `dx-hologram.indentSize` | `4` | Spaces per indent level |
| `dx-hologram.useUnicode` | `true` | Use ✓/✗ instead of true/false |
| `dx-hologram.useBoxDrawing` | `true` | Use box characters for tables |
| `dx-hologram.forge.autoStart` | `true` | Auto-start forge daemon |
| `dx-hologram.forge.autoBuild` | `true` | Auto-build binary on save |

## File Icon

Files named `dx` (no extension) get the DX logo as their icon.

## Requirements

- dx-forge binary (from the DX project)
- VS Code 1.85.0 or higher

## Installation

1. Build the extension:
   ```bash
   cd vscode-dx-hologram
   npm install
   npm run compile
   npx vsce package
   ```

2. Install the `.vsix` file:
   - Open VS Code
   - Press `Ctrl+Shift+P` → "Extensions: Install from VSIX..."
   - Select the generated `.vsix` file

## License

MIT OR Apache-2.0

---

**The future is binary. The present is beautiful.**
