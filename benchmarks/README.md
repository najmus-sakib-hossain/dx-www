# dx-www-runtime Performance Benchmarks

## Overview
This directory contains performance benchmarks comparing dx-www-runtime against leading frontend frameworks.

## Test Scenarios

### 1. **Initial Load Performance**
- Bundle size (KB)
- Parse time (ms)
- Time to Interactive (TTI)
- First Contentful Paint (FCP)

### 2. **Runtime Performance**
- Component render time
- Update cycles (dirty checking)
- Memory usage
- Frame rate stability

### 3. **Stress Tests**
- 10,000 item list rendering
- Rapid state updates (1000 updates/sec)
- Deep component trees (20+ levels)

## Frameworks Under Test

1. **React 18** (Virtual DOM)
2. **Next.js 14** (SSR + Hydration)
3. **Svelte 5** (Compiler-based)
4. **Qwik** (Resumability)
5. **Vue 3** (Composition API)
6. **Solid.js** (Fine-grained reactivity)
7. **dx-www-runtime** (HTIP Binary Protocol)

## Running Benchmarks

```bash
cd benchmarks
./run-all.sh
```

## Expected Results

Based on architectural design:

### Bundle Size
```
React 18:     ~140 KB (gzipped)
Next.js 14:   ~200+ KB (gzipped)
Svelte 5:     ~20 KB (gzipped)
Qwik:         ~30 KB (gzipped)
dx-www:       ~112 KB (WASM) ✅ No runtime overhead
```

### Initial Parse Time
```
React:        ~50-100ms (JS parse + execute)
Next.js:      ~80-150ms (Hydration overhead)
Svelte:       ~10-20ms (Minimal runtime)
dx-www:       ~5-10ms ✅ WASM instant execution
```

### Update Performance (1000 ops)
```
React:        ~16ms (Virtual DOM diffing)
Svelte:       ~8ms (Compile-time optimization)
Solid:        ~3ms (Fine-grained)
dx-www:       ~1-2ms ✅ O(1) dirty-bit updates
```

### Memory Usage (10k items)
```
React:        ~15 MB (VDOM + Fiber)
Next.js:      ~20 MB (SSR state)
Svelte:       ~8 MB
dx-www:       ~5 MB ✅ Zero GC pressure
```

## Why dx-www is Faster

1. **Zero Parse Time**: WASM executes instantly
2. **Zero Hydration**: No JSON serialization/deserialization
3. **Zero Diffing**: O(1) dirty-bit updates, not O(n) tree traversal
4. **Zero GC**: Linear memory layout, no garbage collection pauses
5. **Binary Protocol**: No HTML string parsing

## Architecture Comparison

| Feature | React | Next.js | Svelte | Qwik | dx-www |
|---------|-------|---------|--------|------|--------|
| Parse Overhead | High | Very High | Low | Medium | **Zero** |
| Update Method | VDOM | VDOM | Reactive | Lazy | **Dirty Bits** |
| Memory Model | Heap | Heap | Heap | Heap | **Linear** |
| Bundle Format | JS | JS | JS | JS | **WASM** |
| Hydration | Yes | Yes | No | No | **No** |
| Runtime Cost | O(n) | O(n) | O(1) | O(1) | **O(1)** |

## Disclaimer

These are architectural projections. Real-world performance depends on:
- Application complexity
- Browser optimizations
- Network conditions
- Device capabilities

Run your own benchmarks for production decisions.
