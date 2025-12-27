# dx-cli: The Command-Line Orchestrator

The unified command-line interface for the dx-www Binary Web Runtime.

## Installation

```bash
cargo install dx-cli
```

Or from source:
```bash
git clone https://github.com/yourusername/dx-www-runtime
cd dx-www-runtime
cargo install --path crates/cli
```

## Quick Start

```bash
# Create a new project
dx new my-app

# Start development server
cd my-app
dx dev

# Build for production
dx build --release
```

## Commands

### `dx new <name>`

Create a new dx-www project.

**Options:**
- `--template <template>` - Template to use (counter, dashboard, hackernews)
- `--path <path>` - Target directory

**Example:**
```bash
dx new my-app --template dashboard
```

### `dx dev`

Start development server with hot reload.

**Options:**
- `--port <port>` - Server port (default: 3000)
- `--host <host>` - Bind address (default: localhost)
- `--open` - Open browser automatically

**Example:**
```bash
dx dev --port 8080 --open
```

### `dx build`

Build for production.

**Options:**
- `--release` - Enable full optimizations
- `--output <dir>` - Output directory (default: dist)
- `--skip-optimize` - Skip WASM optimization

**Example:**
```bash
dx build --release
```

### `dx info`

Show project information and build status.

### `dx clean`

Remove build artifacts.

## Configuration

Projects are configured via `dx.toml`:

```toml
[project]
name = "my-app"
version = "0.1.0"

[build]
auto_select = true      # Auto-select Micro vs Macro
sourcemaps = true       # Generate source maps
output = "dist"         # Build output directory

[server]
port = 3000             # Dev server port
host = "localhost"      # Bind address
hmr = true              # Hot module replacement

[optimize]
wasm_opt = "z"          # WASM optimization level
strip = true            # Strip debug symbols
lto = true              # Link-time optimization
```

## Architecture

The CLI orchestrates three main components:

1. **dx-compiler** - Compiles TypeScript to binary
2. **dx-server** - Serves the application over HTTP
3. **File Watcher** - Monitors changes and triggers rebuilds

```
dx-cli
  ├── dx-compiler (TSX → Binary)
  ├── dx-server (Serve HTTP)
  └── notify (File watching)
```

## Development

```bash
# Build the CLI
cargo build -p dx-cli

# Run locally
cargo run -p dx-cli -- new test-app

# Install locally
cargo install --path crates/cli
```

## License

MIT OR Apache-2.0
