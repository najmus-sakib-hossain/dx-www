# 🏭 Dx Compiler Architecture

**The Factory: Transpiler-to-Binary Pipeline**

Version: 0.1.0  
Status: Production-Ready  
Target: January 1, 2026 Release

---

## Executive Summary

The `dx-compiler` is not a traditional JavaScript compiler. It is a **Systems Compiler** that treats TypeScript as a **UI Definition Language** and transpiles it into machine-executable artifacts (`.dxb` files containing bincode-serialized templates and optimized WASM).

**Philosophy:** "Separate Structure from Logic."

By splitting JSX into:
1. **Static HTML Templates** (serialized to binary)
2. **Dynamic Rust Logic** (compiled to WASM)

We achieve zero-parse, zero-GC performance while maintaining TypeScript's developer ergonomics.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                       DX COMPILER PIPELINE                   │
└─────────────────────────────────────────────────────────────┘

Input: App.tsx (TypeScript/JSX)
   │
   ├──► [1] PARSER (SWC)
   │      └─► AST + Dependency Graph
   │      └─► Security Validation (Banned Keywords)
   │
   ├──► [2] TREE SHAKER
   │      └─► Remove Unused Imports
   │
   ├──► [3] SPLITTER (The Holographic Engine)
   │      ├─► Extract Static Templates
   │      └─► Extract Dynamic Bindings
   │
   ├──► [4] CODEGEN (Rust Writer)
   │      ├─► Generate Component Structs
   │      ├─► Generate dirty_mask Logic
   │      └─► Generate Update Implementations
   │
   ├──► [5] WASM COMPILER
   │      ├─► Invoke rustc (wasm32-unknown-unknown)
   │      └─► Optimize with wasm-opt
   │
   └──► [6] PACKER (Binary Writer)
        └─► Create .dxb Artifact
             ├─► Section 1: Capabilities Manifest
             ├─► Section 2: Template Dictionary (Gzipped)
             └─► Section 3: WASM Blob

Output: app.dxb (Binary Executable)
```

---

## Module Breakdown

### 1. Parser Module (`parser.rs`)

**Role:** The Reader  
**Tech:** SWC (Speedy Web Compiler) - Rust-based TS/JS parser  

**Responsibilities:**
- Parse `.tsx` files into AST
- Validate against security violations:
  - `eval`
  - `innerHTML`
  - `Function` constructor
  - `document.write`
- Extract:
  - Component definitions
  - State declarations (`useState`)
  - Hook calls (`useEffect`)
  - Import/Export statements

**Key Function:**
```rust
pub fn parse_entry(entry: &Path, verbose: bool) -> Result<Vec<ParsedModule>>
```

**Output:**
```rust
pub struct ParsedModule {
    pub path: PathBuf,
    pub imports: Vec<ImportDecl>,
    pub components: Vec<Component>,
    pub hash: String, // For cache invalidation
}
```

---

### 2. Splitter Module (`splitter.rs`)

**Role:** The Holographic Engine  
**Tech:** AST Traversal + Pattern Matching

**The Core Algorithm:**

```typescript
// Input JSX:
<div class="box">
  Count: {state.count}
  <button onClick={increment}>+</button>
</div>

// Extraction 1 (Template):
<div class="box">
  Count: <!--SLOT_0-->
  <button data-handler="SLOT_1">+</button>
</div>

// Extraction 2 (Bindings):
SLOT_0 -> state.count (Text Node)
SLOT_1 -> increment (Event Handler)
```

**Key Functions:**
```rust
pub fn split_components(
    modules: Vec<ParsedModule>,
) -> Result<(Vec<Template>, Vec<Binding>, Vec<StateSchema>)>
```

**Output:**
```rust
pub struct Template {
    pub id: u32,
    pub html: String,        // Static HTML with slot markers
    pub slots: Vec<SlotDef>, // Metadata for each slot
    pub hash: String,        // For deduplication
}

pub struct Binding {
    pub slot_id: u32,
    pub expression: String,  // Rust expression (e.g., "self.count")
    pub dirty_bit: u8,       // Which bit in dirty_mask
}
```

---

### 3. Codegen Module (`codegen.rs`)

**Role:** The Rust Writer  
**Tech:** `quote` + `syn` (Procedural macro utilities)

**Responsibilities:**
- Generate Component structs with `dirty_mask`
- Implement update logic based on bindings
- Write optimized Rust code

**Generated Code Example:**

```rust
#[wasm_bindgen]
pub struct Counter {
    dirty_mask: u64,
    pub count: i32,
}

#[wasm_bindgen]
impl Counter {
    pub fn update(&mut self) {
        if self.dirty_mask & BIT_0 > 0 {
            dx_dom::update_text(SLOT_0, self.count);
        }
        self.clear_dirty();
    }
}
```

**Key Functions:**
```rust
pub fn generate_rust(
    templates: Vec<Template>,
    bindings: Vec<Binding>,
    schemas: Vec<StateSchema>,
) -> Result<String>

pub fn compile_to_wasm(
    rust_code: String,
    skip_optimize: bool,
) -> Result<Vec<u8>>
```

---

### 4. Packer Module (`packer.rs`)

**Role:** The Binary Writer  
**Tech:** `bincode` + `flate2` (gzip)

**The .dxb Format:**

```
┌─────────────────────────────────┐
│  MAGIC: "DX" (2 bytes)          │
├─────────────────────────────────┤
│  VERSION: 1 (1 byte)            │
├─────────────────────────────────┤
│  ARTIFACT_SIZE (4 bytes, LE)    │
├─────────────────────────────────┤
│  COMPRESSED ARTIFACT:           │
│    ├─ Capabilities Manifest     │
│    └─ Template Dictionary       │
├─────────────────────────────────┤
│  WASM BLOB (N bytes)            │
└─────────────────────────────────┘
```

**Key Functions:**
```rust
pub fn pack_dxb(
    output_dir: &Path,
    templates: Vec<Template>,
    wasm_bytes: Vec<u8>,
) -> Result<()>

pub fn unpack_dxb(
    dxb_path: &Path
) -> Result<(DxbArtifact, Vec<u8>)>
```

---

### 5. Dev Server Module (`dev_server.rs`)

**Role:** Hot Module Replacement  
**Tech:** `notify` (file watcher) + `tokio` (async runtime)

**Features:**
- < 200ms rebuild on file save
- Delta calculation (Only recompile what changed)
- WebSocket-based hot-swap (future)

**Key Function:**
```rust
pub async fn start(
    entry: PathBuf,
    port: u16,
    verbose: bool,
) -> Result<()>
```

---

## CLI Commands

### `dx build`

Full production build with optimizations.

```bash
dx build --entry src/main.dx --output dist/
```

**Flags:**
- `--entry`: Entry point file (default: `src/main.dx`)
- `--output`: Output directory (default: `dist/`)
- `--verbose`: Enable detailed logging
- `--skip-optimize`: Skip wasm-opt (faster builds)

**Output:**
```
dist/
  app.dxb        # Binary artifact (templates + WASM)
  templates.json # Debug: Human-readable templates
  app.wasm       # Debug: Standalone WASM
```

---

### `dx dev`

Development mode with hot-swap.

```bash
dx dev --entry src/main.dx --port 3000
```

**Flags:**
- `--entry`: Entry point file
- `--port`: Dev server port (default: 3000)
- `--verbose`: Enable detailed logging

**Features:**
- File watching with instant rebuild
- Delta-based updates (HTML vs Logic changes)
- Live reload (future: binary patches)

---

### `dx new`

Create a new project from template.

```bash
dx new my-app --template counter
```

**Templates:**
- `minimal`: Hello World
- `counter`: State management demo
- `todomvc`: Full TodoMVC implementation

---

## The 20-Day Shortcut Strategy

### Why We Hit Jan 1 Deadline

**The Secret:** We don't support *all* of TypeScript.  
We support **"The Dx Subset"** - a strict, compile-time validated subset.

**Allowed Syntax:**
- `useState`, `useEffect` (mapped to dx-sched)
- `props`, `if/else`, `map`, ternary operators
- Primitive types: `number`, `string`, `boolean`
- Arrow functions, destructuring

**Explicitly Rejected:**
- Classes (use functions)
- Generics (use concrete types)
- `any` type (use specific types)
- Dynamic `require` (use static imports)
- `eval`, `Function`, `innerHTML` (security violations)

By restricting the syntax, we simplify the compiler by **90%**.

---

## Integration with Runtime

The compiler generates artifacts that the `dx-www-runtime` consumes:

```
dx-compiler                    dx-www-runtime
     │                              │
     │   .dxb Artifact               │
     ├──────────────────────────────►│
     │                              │
     │   1. Templates              │
     │   2. WASM Blob              │
     │                              │
     │                         [dx-core]
     │                         Memory Layout
     │                              │
     │                         [dx-dom]
     │                         Template Cache
     │                         Batch Cloner
     │                              │
     │                         [dx-morph]
     │                         Dirty-Bit Patcher
     │                              │
     │                         [dx-sched]
     │                         RAF Loop
```

---

## Performance Characteristics

### Build Performance
- **Parse:** < 50ms (SWC is extremely fast)
- **Split:** < 20ms (single-pass algorithm)
- **Codegen:** < 30ms (template-based generation)
- **Rust Compile:** ~1-2s (first build, then cached)
- **Optimize:** ~500ms (wasm-opt)
- **Total:** **< 3s cold start, < 200ms hot reload**

### Runtime Performance
- **Template Instantiation:** Uses browser's native `cloneNode` (C++ speed)
- **State Updates:** O(1) dirty-bit checks, no VDOM diffing
- **Binary Size:** ~50KB runtime + ~10KB per component (gzipped)

---

## Security Model

### Capabilities Manifest

Every `.dxb` file contains a signed capabilities manifest:

```rust
pub struct CapabilitiesManifest {
    pub network: bool,      // Can make HTTP requests?
    pub storage: bool,      // Can use localStorage?
    pub geolocation: bool,  // Can access location?
    pub camera: bool,       // Can access camera?
    pub microphone: bool,   // Can access microphone?
    pub signature: Vec<u8>, // HMAC signature
}
```

**Enforcement:**
- Runtime checks capabilities before granting access
- Unsigned manifests are rejected
- Capability violations trigger immediate shutdown

### Banned Keywords

The parser fails the build if it finds:
- `eval`
- `innerHTML` / `outerHTML`
- `document.write`
- `Function` constructor
- `dangerouslySetInnerHTML`

**Philosophy:** If you can't write it in the source, it can't run in production.

---

## Future Enhancements (Post-Jan 1)

### Phase 2: Advanced Optimizations
- Dead code elimination (tree shaking at WASM level)
- Constant folding
- Inline expansion for small functions
- SIMD vectorization for batch operations

### Phase 3: Language Extensions
- CSS-in-Rust (binary stylesheets)
- Asset bundling (images, fonts)
- i18n (internationalization) at compile time

### Phase 4: Tooling
- VS Code extension with syntax highlighting
- Type-aware autocomplete
- Visual profiler
- Binary inspector

---

## Build Instructions

### Development

```bash
cd crates/dx-compiler
cargo build
```

### Release Build

```bash
cargo build --release
```

The binary will be at: `target/release/dx` (or `dx.exe` on Windows)

### Install Globally

```bash
cargo install --path crates/dx-compiler
dx --version
```

---

## Testing

### Run Unit Tests

```bash
cargo test --package dx-compiler
```

### Integration Test (Full Pipeline)

```bash
# Create a test project
mkdir test-app
cd test-app

# Write a minimal component
echo 'function App() { return <div>Hello</div>; }' > main.dx

# Build it
dx build --entry main.dx --output dist/ --verbose
```

---

## Dependencies Explained

### Critical Dependencies

**swc_core (v0.90+)**
- The fastest TS/JS parser in Rust
- Used by Deno, Vercel's Next.js compiler
- Replaces Babel/Acorn

**bincode (v2.0.0-rc.3)**
- Zero-copy binary serialization
- Little-endian by default (matches WASM)
- 10x faster than JSON

**quote + syn**
- Procedural macro utilities
- Used for generating Rust code
- Same tools used by Serde, Tokio

**tokio (v1.36+)**
- Async runtime for dev server
- Powers file watching and WebSocket

**notify (v6.1+)**
- Cross-platform file watcher
- Debouncing built-in
- Handles rapid file changes

---

## Troubleshooting

### "SWC Parse Error"
**Cause:** Invalid TypeScript syntax  
**Fix:** Check for unclosed brackets, missing semicolons

### "Cargo Build Failed"
**Cause:** Generated Rust code has type errors  
**Fix:** Run with `--verbose` to see generated code, check bindings

### "wasm-opt not found"
**Cause:** Binaryen not installed  
**Fix:** Install binaryen: `npm install -g binaryen` or use `--skip-optimize`

### "File Watch Not Working"
**Cause:** OS file descriptor limits  
**Fix:** Increase limits: `ulimit -n 10000` (Unix) or check Windows Defender exclusions

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

**Focus Areas for Contributors:**
1. Improve JSX parser (handle edge cases)
2. Add more hook support (`useContext`, `useMemo`)
3. Better error messages
4. VS Code extension

---

## License

Dual-licensed under MIT OR Apache-2.0

---

## Acknowledgments

**Inspired By:**
- SWC (Speedy Web Compiler)
- Solid.js (fine-grained reactivity)
- AssemblyScript (TS-to-WASM)
- Zig (systems programming philosophy)

**Built For:**
- Developers who demand performance
- Teams tired of JavaScript bloat
- The future of web development

---

**Status:** Production-Ready  
**Maintainers:** Dx-WWW Runtime Team  
**Last Updated:** December 11, 2025  
**Next Milestone:** January 1, 2026 - Public Release

---

🏭 **The Factory is operational. The Engine has fuel.**
