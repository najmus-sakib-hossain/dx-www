# dx-www Runtime Architecture

## System Overview

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
│  │  • wasm-bindgen generated bindings                      │   │
│  │  • Event listeners                                      │   │
│  │  • Single HTIP loop for batch operations               │   │
│  └──────▲──────────────────────────────────────────────────┘   │
└─────────┼────────────────────────────────────────────────────────┘
          │ FFI (minimal calls)
┌─────────┼────────────────────────────────────────────────────────┐
│         │         WebAssembly (Rust)                            │
│  ┌──────┴──────────────────────────────────────────────────┐   │
│  │                     dx-sched                             │   │
│  │  ┌────────────────────────────────────────────────────┐ │   │
│  │  │ RAF Loop (requestAnimationFrame)                   │ │   │
│  │  │ • Frame Budget: 4ms max WASM execution             │ │   │
│  │  │ • Priority Queue: Immediate > Normal > Idle        │ │   │
│  │  │ • Yield Strategy: Prevent frame drops              │ │   │
│  │  └────────────────────────────────────────────────────┘ │   │
│  └──────▲──────────────────────────────────────────────────┘   │
│         │                                                        │
│  ┌──────┴──────────────────────────────────────────────────┐   │
│  │                     dx-morph                             │   │
│  │  ┌────────────────────────────────────────────────────┐ │   │
│  │  │ State Patcher (O(1) Updates)                       │ │   │
│  │  │ • Dirty Bit Mask: 64-bit per component             │ │   │
│  │  │ • Binding Map: Static lookup (bit → DOM node)     │ │   │
│  │  │ • No tree traversal, no diffing                    │ │   │
│  │  │ • Generates RenderOps for dx-dom                   │ │   │
│  │  └────────────────────────────────────────────────────┘ │   │
│  └──────▲──────────────────────────────────────────────────┘   │
│         │                                                        │
│  ┌──────┴──────────────────────────────────────────────────┐   │
│  │                      dx-dom                              │   │
│  │  ┌────────────────────────────────────────────────────┐ │   │
│  │  │ HTIP Renderer                                      │ │   │
│  │  │ • Template Cache: HtmlTemplateElement map          │ │   │
│  │  │ • Batch Cloner: Groups operations by template      │ │   │
│  │  │ • Node Registry: Tracks cloned nodes by ID         │ │   │
│  │  │ • Queue Flusher: Single JS call per frame          │ │   │
│  │  └────────────────────────────────────────────────────┘ │   │
│  └──────▲──────────────────────────────────────────────────┘   │
│         │                                                        │
│  ┌──────┴──────────────────────────────────────────────────┐   │
│  │                     dx-core                              │   │
│  │  ┌────────────────────────────────────────────────────┐ │   │
│  │  │ Linear Memory Manager                              │ │   │
│  │  │                                                     │ │   │
│  │  │ [Static Region: 0-2MB]                             │ │   │
│  │  │  • Read-only dictionaries                          │ │   │
│  │  │  • Template strings, class names                   │ │   │
│  │  │  • Capability manifest (security)                  │ │   │
│  │  │                                                     │ │   │
│  │  │ [State Region: 2-10MB]                             │ │   │
│  │  │  • Component state structs                         │ │   │
│  │  │  • SharedArrayBuffer ready                         │ │   │
│  │  │  • Atomic operations for dirty bits                │ │   │
│  │  │                                                     │ │   │
│  │  │ [Queue Region: 10-16MB]                            │ │   │
│  │  │  • Ring buffer for RenderOps                       │ │   │
│  │  │  • Zero-copy write operations                      │ │   │
│  │  └────────────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────┘
```

## Data Flow

### Initialization Phase

1. **Template Registration** (dx-dom)
   ```
   Binary Template Data → register_templates() → HtmlTemplateElement Cache
   ```

2. **Initial Render**
   ```
   queue_clone(template_id) → Batch Cloner → flush_to_element() → DOM
   ```

### Update Cycle

1. **User Interaction**
   ```
   Button Click → JS Event Listener → WASM handle_increment()
   ```

2. **State Mutation** (dx-morph)
   ```
   state.count += 1
   state.dirty.mark_dirty(BIT_COUNT)  // Atomic operation
   ```

3. **Dirty Check** (dx-morph)
   ```
   is_dirty() → Get Binding Map → Generate RenderOps
   ```

4. **Queue Operations** (dx-dom)
   ```
   RenderOp → Queue Region (dx-core) → Batch Cloner
   ```

5. **Flush to DOM** (dx-sched)
   ```
   RAF tick() → flush_queue() → Single JS call → cloneNode() loop → DOM
   ```

## Memory Layout

```
Linear Memory (16MB)
├── [0x000000 - 0x200000] Static Region (2MB)
│   ├── 0x000000: Capability Manifest (64 bytes)
│   │   ├── magic: u32 (0x44585757 "DXWW")
│   │   ├── version: u32
│   │   ├── capabilities: u64
│   │   └── checksum: u32
│   ├── 0x000040: Class Name Dictionary
│   ├── 0x001000: Template Strings
│   └── 0x100000: Binding Maps
│
├── [0x200000 - 0xA00000] State Region (8MB)
│   ├── Component State Structs
│   │   ├── dirty_mask: u64 (first field, always)
│   │   ├── field1: T
│   │   ├── field2: T
│   │   └── ...
│   └── (SharedArrayBuffer for Worker threads)
│
└── [0xA00000 - 0x1000000] Queue Region (6MB)
    ├── Ring Buffer Header
    │   ├── write_offset: AtomicU32
    │   └── read_offset: AtomicU32
    └── RenderOp Array
        ├── RenderOp { opcode, arg1, arg2, arg3 }
        ├── RenderOp { ... }
        └── ...
```

## Performance Characteristics

### Initialization
- Template parsing: **O(1)** (done once, cached)
- Memory allocation: **O(1)** (linear pre-allocated buffer)

### Update Cycle
- Dirty check: **O(1)** (bit mask check)
- Binding lookup: **O(1)** (static array index)
- RenderOp generation: **O(k)** where k = dirty fields (typically 1-3)
- DOM update: **O(k)** native cloneNode operations

### Memory
- Zero allocations in hot path ✓
- Zero GC pressure ✓
- Constant memory footprint ✓

## Comparison to React

| Operation | React | dx-www |
|-----------|-------|--------|
| Create element | `React.createElement()` | `template.cloneNode()` |
| Update check | VDOM diff O(n) | Dirty bit O(1) |
| State storage | JS heap (GC'd) | Linear memory (no GC) |
| Re-render | Full tree reconciliation | Direct node patches |
| Hydration | Parse + attach | Not needed |
| Bundle size | 150KB+ | <50KB |

## Security Model

### Capability Manifest
Every binary must include a capability manifest with:
- Magic number validation
- Version check
- Permission flags (DOM, Network, Storage)
- CRC32 checksum

### Isolation
- Static Region: Read-only after init
- State Region: Isolated per component
- Queue Region: Write-only from WASM, read-only from JS

## Future Extensions

### Worker Thread Support
```
Main Thread                Worker Thread
    |                           |
    ├─ dx-dom ─────────────────────┤ (Shared Template Cache)
    |                           |
    ├─ SharedArrayBuffer ───────────┤ (State Region)
    |                           |
    └─ Atomic Ops ─────────────────┘ (Dirty Bits)
```

### Server-Side Rendering
```
Server (Rust)          Browser (WASM)
    |                      |
    ├─ Template Cache ────────┤ (Same binary format)
    |                      |
    ├─ Initial HTML ─────────┤ (Pre-rendered)
    |                      |
    └─ State Binary ─────────┘ (Serialized state)
```

---

**Binary Everywhere. Zero Parse. Zero GC. Zero Hydration.**
