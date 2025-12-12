# dx-www Quickstart Guide

## Installation

```bash
# Clone the repository
git clone https://github.com/your-org/dx-www-runtime.git
cd dx-www-runtime

# Build the compiler
cargo build --release -p dx-compiler

# Add to PATH (optional)
export PATH="$PATH:$(pwd)/target/release"
```

## Create Your First App

### 1. Create App.tsx

```tsx
// src/App.tsx
function HelloWorld() {
    return (
        <div class="container">
            <h1>Hello from dx-www!</h1>
            <p>This is a 19 KB runtime.</p>
        </div>
    );
}
```

### 2. Compile

```bash
dx build --entry src/App.tsx --output dist/
```

**Output:**
```
🏭 Dx Compiler - Building...
✓ Parsed 1 modules
✓ Split 1 components  
✓ Packed to: dist/app.dxb (70 bytes)
✓ Built in 0.03s
```

### 3. View Bundle Size

```bash
ls -lh dist/
# app.dxb: 70 bytes
```

Plus the universal runtime:
```bash
ls -lh target/pkg_minimal/dx_client_optimized.wasm
# 19 KB (downloaded once, cached forever)
```

**Total First Load: 19.07 KB**

---

## Project Structure

```
my-dx-app/
├── src/
│   ├── App.tsx          # Your components
│   ├── Counter.tsx      # More components
│   └── styles.css       # Optional CSS
├── dist/
│   └── app.dxb          # Compiled binary (tiny!)
└── index.html           # Entry point
```

---

## Development Mode

```bash
dx dev --entry src/App.tsx --port 3000
```

Features:
- 🔥 **Hot Reload:** Edit → Save → See changes instantly
- 🐛 **Source Maps:** Debug original TSX in DevTools
- 📊 **Bundle Analyzer:** See what's in your .dxb

---

## Advanced: Counter Example

```tsx
// src/Counter.tsx
function Counter() {
    const [count, setCount] = useState(0);
    
    return (
        <div>
            <h2>Count: {count}</h2>
            <button onClick={() => setCount(count + 1)}>
                Increment
            </button>
        </div>
    );
}
```

Compile:
```bash
dx build --entry src/Counter.tsx --output dist/
```

**Result:**
- `dist/app.dxb`: ~120 bytes (50 bytes more for state logic)
- Runtime: Still 19 KB (unchanged!)

---

## Bundle Size Comparison (Brotli Compressed)

| App | dx-www | React | Svelte |
|-----|--------|-------|--------|
| Hello World | **400 bytes** 🏆 | 50 KB | 2 KB |
| Counter | **450 bytes** 🏆 | 55 KB | 4 KB |
| TodoMVC | **9 KB** 🏆 | 100 KB | 15 KB |
| Dashboard (100 comps) | **13 KB** 🏆 | 1.5 MB | 150 KB |

**dx-www uses auto-switching:**
- Small sites (< 10 components) → dx-client-tiny (338 bytes)
- Large apps (10+ components) → dx-client (7.5 KB)

---

## Commands Reference

### Build
```bash
dx build [OPTIONS]
  --entry <FILE>     Entry point (default: src/main.dx)
  --output <DIR>     Output directory (default: dist/)
  --verbose          Show detailed logs
  --skip-optimize    Skip WASM optimization (faster builds)
```

### Dev Server
```bash
dx dev [OPTIONS]
  --entry <FILE>     Entry point
  --port <PORT>      Dev server port (default: 3000)
  --verbose          Show detailed logs
```

### New Project
```bash
dx new <NAME> [OPTIONS]
  --template <TMPL>  Template: minimal, counter, todomvc
```

---

## Understanding .dxb Files

A `.dxb` file contains:
1. **Magic Bytes:** "DX" identifier
2. **Version:** Format version (currently 1)
3. **Capabilities:** Security permissions
4. **Templates:** Gzipped HTML structures
5. **WASM:** Compiled logic (if any)

**Why it's tiny:**
- No JavaScript code
- No Virtual DOM
- No runtime overhead
- Just pure data + references

---

## Performance Tips

### 1. Component Splitting
Split large apps into multiple `.dxb` files:
```bash
dx build --entry src/App.tsx --output dist/app.dxb
dx build --entry src/Admin.tsx --output dist/admin.dxb
```

Load on-demand:
```tsx
const AdminPanel = lazy(() => import('./admin.dxb'));
```

### 2. WASM Optimization
Already applied by default:
```bash
wasm-opt -Oz --enable-bulk-memory runtime.wasm -o optimized.wasm
```

### 3. Template Deduplication
The compiler automatically deduplicates identical templates:
```tsx
// These share ONE template in the .dxb
<div class="card">A</div>
<div class="card">B</div>
<div class="card">C</div>
```

---

## Troubleshooting

### Error: "File not found"
```bash
# Make sure entry file exists
ls src/App.tsx

# Use absolute path if needed
dx build --entry $(pwd)/src/App.tsx
```

### Error: "SECURITY VIOLATION: banned keyword"
The compiler blocks unsafe patterns:
- `eval()`
- `innerHTML`
- `dangerouslySetInnerHTML`

**Solution:** Remove unsafe code. Use safe alternatives.

### Build is slow
```bash
# Skip WASM optimization during development
dx build --skip-optimize
```

---

## What's Next?

- 📖 Read the [Architecture Guide](./ARCHITECTURE.md)
- 🧪 Try the [TodoMVC Example](../examples/todomvc/)
- 🎨 Explore [Templates](../examples/)
- 🐛 Report issues on [GitHub](https://github.com/your-org/dx-www-runtime/issues)

---

**Welcome to dx-www - where apps are data, not code.** 🚀
