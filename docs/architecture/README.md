# Architecture

Technical architecture documentation for the DX platform.

## Contents

- [Overview](./overview.md) - High-level system architecture
- [Compiler](./compiler.md) - TSX → WASM compiler architecture
- [Compiler Intelligence](./compiler-intelligence.md) - Auto-selection algorithm
- [Project Structure](./project-structure.md) - Codebase organization
- [Binary Protocol](./binary-protocol.md) - Binary communication protocol
- [Bidirectional System](./bidirectional-system.md) - Dual-mode rendering system

## Quick Overview

DX is built on a **binary-first architecture** that eliminates traditional web development bottlenecks:

```
┌─────────────────────────────────────────────────────────────────┐
│                       Browser Environment                        │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                     HTML Document                          │ │
│  │  ┌──────────────────────────────────────────────────────┐  │ │
│  │  │            <div id="app">                            │  │ │
│  │  │              <!-- HTIP renders here -->              │  │ │
│  │  │            </div>                                     │  │ │
│  │  └──────────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────┘ │
│         ▲                                                        │
│         │ cloneNode (batched)                                   │
│         │                                                        │
│  ┌──────┴──────────────────────────────────────────────────┐   │
│  │              JavaScript Glue Layer (Minimal)            │   │
│  └──────▲──────────────────────────────────────────────────┘   │
└─────────┼────────────────────────────────────────────────────────┘
          │ FFI (minimal calls)
┌─────────┼────────────────────────────────────────────────────────┐
│         │         WebAssembly (Rust)                            │
│  ┌──────┴──────────────────────────────────────────────────┐   │
│  │  dx-sched → dx-morph → dx-dom → dx-core                 │   │
│  └─────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────┘
```

## Core Principles

| Traditional Approach | DX Approach |
|---------------------|-------------|
| Parse JSON at runtime | Binary formats, zero parsing |
| Garbage collection | Stack-only allocation |
| Virtual DOM diffing | Direct DOM manipulation via HTIP |
| Hydration overhead | Resumable state snapshots |
| Text-based CSS | Binary CSS with integer class IDs |

## Performance Characteristics

| Operation | React | DX |
|-----------|-------|-----|
| Create element | `React.createElement()` | `template.cloneNode()` |
| Update check | VDOM diff O(n) | Dirty bit O(1) |
| State storage | JS heap (GC'd) | Linear memory (no GC) |
| Re-render | Full tree reconciliation | Direct node patches |
| Bundle size | 150KB+ | <50KB |

## Related

- [API Reference](../api/README.md)
- [Benchmarks](../reference/benchmarks/README.md)
- [Getting Started](../getting-started/README.md)
