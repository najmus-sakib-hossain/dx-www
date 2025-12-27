# Architecture

Technical architecture documentation for the DX platform.

## Contents

### Core Architecture
- [Overview](./ARCHITECTURE.md) - High-level system architecture
- [Compiler](./COMPILER.md) - TSX → WASM compiler architecture
- [Compiler Intelligence](./COMPILER_INTELLIGENCE.md) - Auto-selection algorithm
- [Project Structure](./PROJECT_STRUCTURE.md) - Codebase organization
- [Bidirectional System](./BIDIRECTIONAL_SYSTEM.md) - Dual-mode rendering system
- [Binary Dawn Structure](./BINARY_DAWN_FOLDER_STRUCTURE.md) - Binary architecture

### Specifications
- [DX Zero Specification](./DX_ZERO_SPECIFICATION.md) - Zero-cost abstraction spec
- [DXL Lock Spec](./DXL_LOCK_SPEC.md) - Lock file format
- [DXP Format Spec](./DXP_FORMAT_SPEC.md) - Package format
- [DXRP Protocol Spec](./DXRP_PROTOCOL_SPEC.md) - Remote protocol

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
