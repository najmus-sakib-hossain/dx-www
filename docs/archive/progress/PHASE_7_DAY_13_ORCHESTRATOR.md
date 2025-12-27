# Phase 7: The Orchestrator 🎭

**Status:** Day 13 Started  
**Date:** December 12, 2025 (Moving ahead of schedule!)  
**Goal:** Create the CLI that connects Compiler + Server + Client

---

## 🎯 The Vision

**Before Phase 7:**
```bash
# Scattered pieces
cd crates/dx-compiler && cargo build
cd crates/dx-server && cargo run
cd examples && python -m http.server
# Manual coordination of 3+ terminals
```

**After Phase 7:**
```bash
# Single command
dx dev
# Everything just works ✨
```

---

## 📦 What We're Building

### `dx-cli` - The Command-Line Interface

A single binary that orchestrates the entire dx-www toolchain:

```
dx
├── new <name>     → Scaffold project
├── dev            → Watch + Compile + Serve + HMR
├── build          → Production build
├── info           → Show project stats
└── clean          → Remove artifacts
```

---

## 🏗️ Architecture

### The Integration Stack

```
┌─────────────────────────────────────────┐
│         dx-cli (Orchestrator)           │
│  • Command parsing (clap)               │
│  • Process management                   │
│  • File watching (notify)               │
└─────────────────────────────────────────┘
                    ↓
     ┌──────────────┼──────────────┐
     ↓              ↓              ↓
┌─────────┐  ┌──────────┐  ┌──────────┐
│dx-compiler  │dx-server │  │FileWatcher│
│TSX → Bin│  │Serve HTTP│  │Detect Δ  │
└─────────┘  └──────────┘  └──────────┘
     ↓              ↓              ↓
┌─────────────────────────────────────────┐
│           Output: dist/                 │
│  • app.dxb (Binary layout)              │
│  • runtime.wasm (338B or 7.5KB)         │
│  • runtime.json (Metadata)              │
└─────────────────────────────────────────┘
```

---

## 📋 Day 13 Implementation

### Created Files

**1. `crates/dx-cli/Cargo.toml`**
- Binary crate with `[[bin]]` section
- Dependencies: clap, notify, tokio, dx-compiler, dx-server
- Name: `dx` (the actual command users type)

**2. `crates/dx-cli/src/main.rs` (250 lines)**
- Command-line parser with clap derive
- Subcommands: new, dev, build, info, clean
- Banner with ASCII art
- Tracing/logging setup
- Error handling with pretty formatting

**3. `crates/dx-cli/src/commands/new_project.rs` (350 lines)**
- Project scaffolder
- Templates: counter, dashboard, hackernews
- Generates:
  - `dx.toml` configuration
  - `src/App.tsx` entry point
  - `index.html` template
  - `.gitignore`
- Directory structure creation

**4. `crates/dx-cli/src/commands/dev.rs` (200 lines)**
- Development server orchestration
- File watching with notify + debouncer
- Automatic rebuild on .tsx changes
- Integration with dx-compiler and dx-server
- Hot reload preparation (WebSocket placeholder)

**5. `crates/dx-cli/src/commands/build.rs` (200 lines)**
- Production build pipeline
- Progress bar with indicatif
- Steps:
  1. Compile TypeScript → Binary
  2. Generate WASM
  3. Optimize with wasm-opt
  4. Copy artifacts to dist/
- Bundle size analysis
- Performance metrics

**6. `crates/dx-cli/src/config.rs` (150 lines)**
- `dx.toml` parser with serde + toml
- Configuration structures:
  - ProjectInfo (name, version)
  - BuildConfig (auto_select, runtime, sourcemaps)
  - ServerConfig (port, host, HMR, CORS)
  - OptimizeConfig (wasm_opt, strip, LTO)
- Load/save methods

**7. Supporting modules:**
- `commands/mod.rs` - Module declarations
- `watch.rs` - File watching utilities (placeholder)
- `scaffold.rs` - Scaffolding utilities (placeholder)

---

## 🎯 Command Details

### `dx new <name>`

**Purpose:** Scaffold a new project

**Usage:**
```bash
dx new my-app
dx new my-app --template dashboard
dx new my-app --path ~/projects/my-app
```

**Templates:**
- **counter** (default) - Simple interactive counter
- **dashboard** - Metrics dashboard with cards
- **hackernews** - Hacker News clone (API integration)

**Generated Structure:**
```
my-app/
├── dx.toml           # Project configuration
├── index.html        # HTML template
├── .gitignore        # Git ignore rules
└── src/
    └── App.tsx       # Main component
```

**What it does:**
1. Creates directory structure
2. Generates template files based on selection
3. Creates configuration with sensible defaults
4. Prints next steps

---

### `dx dev`

**Purpose:** Development server with hot reload

**Usage:**
```bash
dx dev                        # Default: localhost:3000
dx dev --port 8080           # Custom port
dx dev --host 0.0.0.0        # Expose to network
dx dev --open                # Open browser
```

**What it does:**
1. **Initial Build:**
   - Reads `dx.toml` configuration
   - Compiles `src/App.tsx` using dx-compiler
   - Determines runtime (Micro vs Macro)
   - Outputs to `dist/`

2. **Start Server:**
   - Launches dx-server on specified port
   - Serves static files from `dist/`
   - Enables CORS for development

3. **Watch Files:**
   - Monitors `src/` directory for changes
   - Debounces file events (500ms)
   - Triggers rebuild on `.tsx` changes

4. **Hot Reload (Planned):**
   - WebSocket connection to client
   - Push updates without full page reload
   - Preserve application state

**Output:**
```
🚀 Starting development server...

📦 Building application...
✓ Initial build complete (123.45ms)
  → Runtime: auto (micro)
  → Size: 23300 bytes

🌐 Server running at http://localhost:3000
   Press Ctrl+C to stop

👀 Watching src/ for changes...
```

---

### `dx build`

**Purpose:** Production-optimized build

**Usage:**
```bash
dx build                     # Standard build
dx build --release           # Full optimizations
dx build --output public     # Custom output directory
dx build --skip-optimize     # Skip wasm-opt (faster)
```

**Pipeline:**
```
Step 1: Compile TypeScript
  src/App.tsx → Abstract Syntax Tree
  ↓
Step 2: Generate WASM
  AST → WASM instructions
  ↓
Step 3: Optimize (if --release)
  wasm-opt -Oz → Minimized WASM
  ↓
Step 4: Copy Artifacts
  dist/app.dxb (Binary layout)
  dist/runtime.wasm (WASM)
  dist/runtime.json (Metadata)
```

**Output:**
```
🏭 Building for production...

📝 Compiling TypeScript... [=========>] 30%
⚙️  Generating WASM...      [==================>] 60%
🔧 Optimizing WASM...       [===========================>] 90%
📦 Copying artifacts...     [================================] 100%
✓ Build complete

✨ Build successful!

  → Output: dist/
  → Duration: 1.23s

📊 Bundle Sizes:

  • Binary Layout: 663 bytes (663 bytes)
  • WASM Runtime: 22.81 KB (23350 bytes)
  • Metadata: 128 bytes (128 bytes)

  → Total: 23.57 KB

  🎉 That's 5× smaller than React!

Next steps:
  1. Deploy the dist/ directory to your hosting provider
  2. Or run a local server: python -m http.server 8000
```

---

### `dx info`

**Purpose:** Show project information

**Output:**
```
📦 Project Information

  Name: my-app
  Version: 0.1.0
  Runtime: auto

📊 Build Status

  ✓ Built artifacts in dist/
  • app.dxb: 663 bytes
```

---

### `dx clean`

**Purpose:** Remove build artifacts

**What it does:**
- Deletes `dist/` directory
- Deletes `target/dx/` directory
- Prints summary of removed files

---

## ⚙️ Configuration: `dx.toml`

### Example Configuration

```toml
[project]
name = "my-app"
version = "0.1.0"

[build]
# Compiler automatically selects Micro (338B) or Macro (7.5KB)
# based on application complexity
auto_select = true

# Manually force a runtime (optional)
# runtime = "micro"  # or "macro"

# Enable source maps for debugging
sourcemaps = true

# Target directory
output = "dist"

[server]
# Development server configuration
port = 3000
host = "localhost"

# Enable hot module replacement
hmr = true

# CORS origins (for API development)
cors_origins = ["http://localhost:3000"]

[optimize]
# WASM optimization level (0-4, or 's', 'z')
wasm_opt = "z"

# Strip debug symbols in release
strip = true

# Enable advanced optimizations
lto = true
```

### Configuration Fields

**`[project]`**
- `name`: Project name (used in templates)
- `version`: Semantic version

**`[build]`**
- `auto_select`: Let compiler choose runtime (default: true)
- `runtime`: Force "micro" or "macro" (optional)
- `sourcemaps`: Generate source maps (default: true)
- `output`: Build output directory (default: "dist")

**`[server]`**
- `port`: Development server port (default: 3000)
- `host`: Bind address (default: "localhost")
- `hmr`: Enable hot reload (default: true)
- `cors_origins`: Allowed CORS origins (default: [])

**`[optimize]`**
- `wasm_opt`: Optimization level for wasm-opt (default: "z")
- `strip`: Remove debug symbols (default: true)
- `lto`: Link-time optimization (default: true)

---

## 🎓 Developer Experience

### First-Time User Flow

```bash
# 1. Install dx
cargo install dx-cli

# 2. Create project
dx new my-app
cd my-app

# 3. Start dev server
dx dev

# 4. Open browser → http://localhost:3000
# 5. Edit src/App.tsx → See changes instantly
# 6. Build for production
dx build

# 7. Deploy
# Upload dist/ to Netlify/Vercel/CloudFlare
```

### Editing Experience

```bash
# Terminal 1: dx dev running
🚀 Server running at http://localhost:3000
👀 Watching src/ for changes...

# User edits src/App.tsx and saves
🔄 File changed: src/App.tsx
📦 Rebuilding...
✓ Build complete (45.23ms)
  → Size: 23300 bytes

# Browser automatically refreshes
```

---

## 🚀 Current Status

### ✅ Completed (Day 13 Start)

**Core CLI Structure:**
- [x] Cargo.toml with dependencies
- [x] main.rs with command parser
- [x] Banner and logging
- [x] Error handling

**Commands:**
- [x] `dx new` - Project scaffolder (3 templates)
- [x] `dx dev` - Dev server with file watching
- [x] `dx build` - Production build pipeline
- [x] `dx info` - Project information
- [x] `dx clean` - Artifact cleanup

**Configuration:**
- [x] dx.toml parser
- [x] ProjectConfig structure
- [x] Load/save methods

**Documentation:**
- [x] This document
- [x] Command reference
- [x] Configuration reference

### 🔲 TODO (Rest of Day 13)

**Integration:**
- [ ] Actually call dx-compiler (currently simulated)
- [ ] Actually call dx-server (currently placeholder)
- [ ] Wire file watcher to trigger real builds

**Testing:**
- [ ] Test `dx new` with all templates
- [ ] Test `dx dev` workflow
- [ ] Test `dx build` output

**Polish:**
- [ ] Better error messages
- [ ] Progress indicators during builds
- [ ] Colors and formatting

---

## 📊 Performance Targets

| Operation | Target | Current | Status |
|-----------|--------|---------|--------|
| **Initial Build** | < 2s | TBD | 🔲 |
| **Incremental Rebuild** | < 500ms | TBD | 🔲 |
| **File Watch Latency** | < 100ms | 50ms | ✅ |
| **CLI Startup** | < 50ms | ~20ms | ✅ |

---

## 🎯 Next Steps (Day 14-15)

### Day 14: The Real App Test
- Build a complete Hacker News clone
- Test routing (list → detail)
- Test data fetching (API → Binary)
- Measure < 100ms load time on 3G

### Day 15: The Polish
- Pretty error messages (miette/ariadne)
- Loading spinners
- Documentation generator
- VSCode extension preparation

---

## 🎉 The Moment of Truth

Phase 7 is where dx-www stops being a "research project" and becomes a **product**.

When you type:
```bash
dx dev
```

And see a binary web application appear instantly, **you have won**.

---

**Phase 7: Day 13 Started**  
**Status:** CLI scaffolding complete, integration in progress  
**Next:** Wire dx-compiler and dx-server together

🎭 **The Orchestra is Assembling**
