# Hello World Example

This example demonstrates the core HTIP (Hybrid Template Instantiation Protocol) in action.

## What's Being Demonstrated

1. **Template Registration**: HTML templates compiled to binary format
2. **Batch Cloning**: `cloneNode` operations grouped for minimal FFI overhead
3. **Dirty-Bit Patching**: O(1) state updates without VDOM diffing
4. **Frame Scheduling**: RAF loop with 4ms frame budget

## Building & Running

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli
```

### Build

```bash
# From the hello-world directory
cargo build --target wasm32-unknown-unknown --release

# Generate JS bindings
wasm-bindgen --target web --out-dir pkg \
  ../../target/wasm32-unknown-unknown/release/hello_world.wasm
```

### Run

```bash
# Serve with any HTTP server (wasm requires HTTP, not file://)
python -m http.server 8000

# Or use a more advanced server
npx serve .
```

Open http://localhost:8000 in your browser.

## What's Happening Under the Hood

1. **Init Phase**:
   - WASM loads
   - Templates parsed into `HtmlTemplateElement` cache (ONCE)
   - State allocated in memory

2. **User Clicks Button**:
   - Event handler updates state struct
   - Sets dirty bit (atomic operation)
   - Calls `render()`

3. **Render Phase**:
   - Checks dirty mask (O(1))
   - Looks up bindings in static map
   - Queues `RenderOp` (just writes to buffer)
   - Flushes queue (single JS call, batch clones)

4. **No React/VDOM**:
   - No `createElement` calls
   - No VDOM tree construction
   - No recursive diffing
   - Just direct memory writes + cloneNode

## Performance Profile

- **Parse Time**: 0ms (templates pre-parsed)
- **GC Pressure**: 0 (no allocations in hot path)
- **Hydration**: 0ms (no hydration needed)
- **Update Time**: O(1) per dirty field (not O(n) tree traversal)

## The "Binary Everywhere" Philosophy

Notice what's NOT in this code:
- ❌ No JSON.parse()
- ❌ No innerHTML = "..."
- ❌ No Virtual DOM diffing
- ❌ No React.createElement()
- ❌ No string concatenation for HTML

Everything is:
- ✅ Binary blobs (templates)
- ✅ u32 indices (no strings)
- ✅ Dirty bits (no tree walks)
- ✅ cloneNode (native C++ speed)

This is the future. January 1, 2026. 🚀
