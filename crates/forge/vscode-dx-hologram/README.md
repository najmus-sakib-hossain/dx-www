# DX Quantum - VS Code Extension

**The World's First Quantum Config File**

Beautiful for Humans. Efficient for LLMs. Blazing for Machines.

## What is this?

The DX Quantum extension implements a revolutionary editing experience. It uses a transparent FileSystemProvider to show you a beautiful, human-readable format while storing an ultra-efficient LLM format on disk - and generating binary `.dxs` files on save.

**ONE file, THREE formats, ZERO friction.**

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         THE DX QUANTUM FILE                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│                           ONE FILE: config/dx                           │
│                                                                         │
│  ┌────────────────┐   ┌────────────────┐   ┌────────────────────┐      │
│  │     EDITOR     │   │     DISK       │   │      RUNTIME       │      │
│  │                │   │                │   │                    │      │
│  │  server        │   │ server#host:   │   │  ┌──────────────┐  │      │
│  │    #host: loc  │──▶│ localhost#     │──▶│  │ BINARY .dxs  │  │      │
│  │    #port: 5432 │   │ port:5432      │   │  │ 0.70ns read  │  │      │
│  │                │   │                │   │  └──────────────┘  │      │
│  └────────────────┘   └────────────────┘   └────────────────────┘      │
│                                                                         │
│        PRETTY              DENSE               BINARY                   │
│     (Human view)        (LLM optimal)      (Machine optimal)            │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## The Three Formats

| Format | Purpose | Location | Size |
|--------|---------|----------|------|
| **Human** | Editor display | Transparent (via FileSystemProvider) | Readable |
| **LLM** | Storage | `.dx` file on disk | 3.5x smaller than JSON |
| **Machine** | Runtime | `.dxs` binary file | 0.70ns access |

## Key Features

- ✅ **Same file, same tab, same path** - No virtual documents
- ✅ **NO dirty indicator** after opening
- ✅ **NO save dialogs** when closing saved files
- ✅ **100% transparent** to user

## Example Transformation

### Human Format (What you see in editor)

```
server
  #host: localhost
  #port: 5432
  #ssl:  true

features
  >authentication
  >caching
  >compression

users @3
  =name ^ role ^ active
  >alice | admin | true
  >bob | user | false
```

### LLM Format (What's stored on disk)

```
server#host:localhost#port:5432#ssl:+
features>authentication|caching|compression
users@3=name^role^active
>alice|admin|+
>bob|user|-
```

### Machine Format (What runs at runtime)

```
Binary: 47 bytes → 0.70ns field access
```

## How It Works

```
USER CLICKS config/dx
        │
        ▼
  ┌─────────────────────────────────────────────┐
  │          DX FILESYSTEM PROVIDER              │
  │                                              │
  │   readFile():              writeFile():      │
  │   ─────────────            ──────────────    │
  │   1. Read dense from disk  1. Receive pretty │
  │   2. Format to pretty      2. Format to dense│
  │   3. Return pretty         3. Write to disk  │
  └─────────────────────────────────────────────┘
        │                         │
        ▼                         ▼
  ┌─────────────┐           ┌─────────────┐
  │  VS CODE    │           │    DISK     │
  │  EDITOR     │           │             │
  │  (Pretty)   │           │  (Dense)    │
  └─────────────┘           └─────────────┘
```

## Features

- **🔮 Quantum Editing**: See human format, save LLM format
- **📦 Auto Binary Build**: Creates `.dxs` on save via dx-forge daemon
- **🎨 Clean Syntax**: Uses # @ > | : ^ operators
- **🔧 JSON Import**: Convert existing JSON to DX format
- **⚡ Status Bar**: Shows editing mode and dx-forge status

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
