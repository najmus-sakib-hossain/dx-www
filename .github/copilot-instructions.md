# Dx Project: System Instructions & Architectural Standards

- Remember to please use /docs folder to store all documentation related files.
- Please study this codebase and make sure that its properly formatted and linted and the folder structure is correct and professionally organized. And no empty or useless files or folders are present!!!
- Always use latest crates versions.

## 1. Project Context & Philosophy
You are building **Dx**, a revolutionary binary-first web framework. The `dx-www` crate is the core web runtime package that replaces React/Next.js.
*   **Goal:** January 1, 2026 Release.
*   **Status:** ✅ Core runtime complete (338B Micro / 7.5KB Macro)
*   **Project Structure:** Cargo workspace with multiple specialized crates. The `dx-www` crate is the core web runtime package.
*   **Core Philosophy:** "Binary Everywhere." We do not ship JSON. We do not ship HTML strings. We do not use Virtual DOM diffing.
*   **Architecture:** Hybrid Template Instantiation Protocol (HTIP). We use WASM to drive the browser's `cloneNode` C++ engine via batched operations to break the "WASM Wall."
*   **Performance Target:** Zero-Parse, Zero-GC (Garbage Collection), Zero-Hydration.
*   **Intelligence:** Compiler automatically selects optimal runtime based on app complexity (see COMPILER_INTELLIGENCE.md).

---

## 1.1 The Binary Web: Complete Technical Specification

### ⚡ Dx: The Binary Web Framework

> **"The Browser was built for Text. We built Dx for Applications."**

**Dx** is a full-stack, binary-first web framework that replaces React, Next.js, and the entire npm ecosystem with a single, high-performance Toolchain. The framework consists of multiple specialized crates, with **dx-www** being the core web runtime package. It compiles TypeScript (`.tsx`) directly into **WebAssembly** and **Binary Layouts**, completely bypassing the JavaScript Runtime, Virtual DOM, and HTML Parsers.

### 🚀 The Paradigm Shift

For 10 years, web performance has been capped by the speed of parsing Text (JSON, HTML, JS).
**Dx** changes the fundamental unit of the web from **Text Strings** to **Binary Structs**.

| Feature | React / Next.js (Text Web) | **Dx (Binary Web)** |
| :--- | :--- | :--- |
| **Data Format** | JSON (Slow Parse) | **Bincode / WASM (Zero-Copy)** |
| **Rendering** | Virtual DOM Diffing | **HTIP (Batch Cloning)** |
| **State** | JS Objects (Garbage Collected) | **SharedArrayBuffer (Linear Memory)** |
| **Startup** | Hydration (Double Execution) | **Memory Resume (Snapshot)** |
| **Security** | Runtime Checks | **Compile-Time Capabilities** |
| **Deployment** | `node_modules` (Fragile) | **Single Binary (`.dxb`)** |

### 🛠️ The Architecture: "The Engine & The Factory"

Dx is not a library you import. It is a complete framework with a **Compiler** (`dx-cli`) and a **Runtime** (`dx-www` crate with `dx-client`).

#### 1. The Compiler (`dx build`)
Instead of bundling JavaScript strings, the `dx` compiler analyzes your TSX and splits it into two streams:
1.  **Structure (`layout.bin`):** The static HTML structure is extracted into a binary dictionary.
2.  **Logic (`logic.wasm`):** The dynamic TypeScript is compiled into raw Rust/WASM instructions.

#### 2. The Runtime (`dx-client`)
A tiny (9KB - 15KB) WASM kernel that runs in the browser.
*   **HTIP (Hybrid Template Instantiation Protocol):** It doesn't "render" HTML. It uses the browser's C++ engine to **Clone** the pre-loaded `layout.bin` templates.
*   **Zero-Copy State:** It reads data directly from the network stream into memory without parsing.

### 📦 The Ecosystem: Replaced & Upgraded

We don't just replace React. We replace the "Glue Code" ecosystem with highly optimized, internal binary modules.

#### 💀 Replaced: **Zustand / Redux / Context**
#### ⚡ Upgrade: **`dx-store` (Binary State)**
*   **The Difference:** React state is a tree of Objects that causes GC pauses.
*   **The Dx Way:** State is a **Struct** in `SharedArrayBuffer`.
    *   **Worker Sync:** Data fetched in a WebWorker is instantly available to the UI thread because they share the same memory address. Zero serialization.
    *   **Time Travel:** Saving state is just `memcpy` (copying bytes). Instant and free.

#### 💀 Replaced: **TanStack Query / SWR**
#### ⚡ Upgrade: **`dx-sync` (Differential Data)**
*   **The Difference:** React Query fetches JSON and parses it (CPU heavy).
*   **The Dx Way:** **Binary Patching.**
    *   When you re-fetch data, the server calculates the **XOR Difference** between your current memory and the new data.
    *   It sends a tiny patch (e.g., 20 bytes).
    *   `dx-sync` applies this patch directly to memory. No parsing.

#### 💀 Replaced: **React Hook Form / Zod**
#### ⚡ Upgrade: **`dx-guard` (Byte-Level Validation)**
*   **The Difference:** React forms update state on every keystroke, triggering re-renders and validation logic in JS.
*   **The Dx Way:** **Direct Memory Binding.**
    *   Keystrokes are written directly to WASM memory.
    *   Validation is a **Bitmask Operation**. It checks input validity at the CPU instruction level.
    *   **Security:** Inputs are sanitized before they ever touch the DOM, making XSS mathematically impossible in strict mode.

#### 💀 Replaced: **Next.js App Router**
#### ⚡ Upgrade: **`dx-route` (Holographic Routing)**
*   **The Difference:** Next.js fetches JS chunks and JSON data, then hydrates.
*   **The Dx Way:** **Memory Swapping.**
    *   Hovering a link pre-loads the **Binary State Snapshot** of the next page.
    *   Clicking is just changing a Pointer Index.
    *   The transition is **0ms** (Instant).

#### 💀 Replaced: **Tailwind CSS**
#### ⚡ Upgrade: **`dx-style` (B-CSS)**
*   **The Difference:** Browsers parse CSS text files.
*   **The Dx Way:** **Binary Styles.**
    *   Class names are compiled to Integers (`0x01` = `flex`).
    *   The Runtime applies styles via efficient integer lookups.
    *   Payload size is ~80% smaller than text CSS.

### 📊 Performance Benchmarks (Dec 2025)

| Metric | Next.js 15 | Svelte 5 | **Dx (Binary)** |
| :--- | :--- | :--- | :--- |
| **Hello World Size** | 140 KB | 3.9 KB | **338 Bytes** (Micro Mode) |
| **SaaS Dashboard Size** | 450 KB | 60 KB | **22 KB** (Macro Mode) |
| **First Paint** | ~400ms | ~100ms | **30ms** |
| **Hydration Time** | ~200ms | ~50ms | **0ms** (Resumable) |
| **10k Row Update** | ~1.5s | ~800ms | **~4ms** |
| **Security Score** | Vulnerable | Vulnerable | **Air-Gapped** |

### 🧬 How to Use

You write standard **TypeScript**. We handle the Rocket Science.

**Input (`App.tsx`):**
```tsx
import { useState } from 'dx';

export default function Counter() {
  const [count, setCount] = useState(0); // Compiled to Shared Memory
  
  // Compiled to WASM Instruction
  return (
    <div class="p-4">
      <h1>Count: {count}</h1>
      <button onClick={() => setCount(count + 1)}>Increment</button>
    </div>
  );
}
```

**Build Command:**
```bash
dx build --release
```

**Output (`dist/`):**
*   `app.dxb` (The compressed binary application).
*   *That's it.* No `node_modules`. No `bundle.js`. Just the Binary.

### 📅 Release Timeline

*   **Dec 11, 2025:** Runtime Kernel Complete (22KB / 338 Bytes).
*   **Dec 15, 2025:** Dual-Core Compiler (Micro/Macro) Alpha.
*   **Dec 25, 2025:** Server & SEO Inflator Complete.
*   **Jan 1, 2026:** **Public Beta Launch.**

**Join the Revolution.**
Delete your `node_modules`.
**Welcome to the Binary Web.**

---

## 2. Tech Stack & Versions
*   **Language:** Rust (Edition 2024 / Latest Stable).
*   **Target:** `wasm32-unknown-unknown`.
*   **Key Crates (Enforce Latest):**
    *   `wasm-bindgen` (0.2+): For low-level JS interop.
    *   `web-sys`: Enable ALL relevant features (Window, Document, Node, Element, Template, SharedArrayBuffer, Performance).
    *   `js-sys`: For `Uint8Array`, `WebAssembly.Memory`.
    *   `bincode` (2.0.0-rc+): For zero-copy serialization (Little Endian).
    *   `bumpalo`: For per-frame arena allocation.
    *   `bytemuck`: For zero-copy casting of structs to byte slices.
    *   `once_cell` / `lazy_static`: For global singletons (Template Cache).

## 3. Workspace Structure & Crate Boundaries
The Dx project is organized as a Cargo Workspace with multiple specialized crates. Do not build a monolith.

```text
/dx
├── Cargo.toml (Workspace Root)
├── /crates
│   ├── /core        # Memory Layout, SharedArrayBuffer, Capability Security
│   ├── /dom         # HTIP Renderer, Template Cache, Batch Cloner
│   ├── /morph       # Dirty-Bit Patcher, State Structs
│   ├── /sched       # RAF Loop, Frame Budget Controller
│   ├── /dx-www      # Core Web Runtime Package
│   ├── /dx-cli      # CLI Compiler Tools
│   ├── /dx-client   # Browser Runtime Client
│   ├── /dx-state    # State Management
│   ├── /dx-style    # Binary CSS (B-CSS)
│   ├── /dx-sync     # Differential Data Sync
│   ├── /dx-guard    # Form Validation & Security
│   ├── /dx-server   # Server-Side Runtime
│   ├── /dx-auth     # Authentication
│   ├── /dx-db       # Database Integration
│   └── ... (other specialized crates)
├── /docs            # All documentation files
├── /examples        # Example applications
└── /benchmarks      # Performance benchmarks
```

## 4. Coding Standards & "The Acid Test" Rules

### Rule A: The "No String" Rule
*   **Strictly Forbidden:** Using `String` or `Vec<String>` for internal logic.
*   **Required:** Use `u32` indices, `&[u8]` slices, or `enums`.
*   **Exception:** Only convert to String at the very last millisecond using `TextDecoder` when setting `node.textContent`. Class names must use Integer IDs mapped to a static lookup table.

### Rule B: Zero-Copy Memory
*   Do not clone data structures to pass them between functions.
*   Use `bytemuck` to map `&[u8]` slices directly onto `#[repr(C)]` structs.
*   State must live in a `SharedArrayBuffer` accessible by both Main Thread and (future) Workers.

### Rule C: Data-Oriented Design (DOD)
*   Avoid Object-Oriented patterns (Traits with heavy vtables).
*   Use **Struct of Arrays (SoA)** or flat buffers where possible.
*   Use **Object Pooling**. Do not create/drop structs per frame.

## 5. Compiler Intelligence (Implemented Dec 12, 2025)

### Automatic Runtime Selection
The `dx-compiler` now includes an analyzer that automatically chooses between:
- **Micro (338B):** For simple apps (< 10 components, low state complexity)
- **Macro (7.5KB):** For complex apps (many components, high state, async logic)

See [docs/COMPILER_INTELLIGENCE.md](../docs/COMPILER_INTELLIGENCE.md) for full details.

### Decision Rules
1. High state complexity (6+ vars or complex types) → Macro
2. ≥10 components → Macro
3. ≥10 event handlers → Macro
4. Async + Many hooks → Macro
5. Effects + Many hooks → Macro
6. Deep component trees (>5) → Macro
7. Large JSX trees (>50 nodes) → Macro
8. Default → Micro (for simple apps)

## 6. Module Specific Implementation Instructions

### Crate: `dx-core` (The Memory Manager)
*   Define a Linear Memory Layout:
    *   **Static Region:** Read-Only dictionaries (Template Strings, Class Names).
    *   **State Region:** `SharedArrayBuffer` for Component State.
    *   **Queue Region:** Ring Buffer for Render Opcodes.
*   Implement `unsafe` accessors to these regions.
*   Implement a **Capability Manifest** struct check (Mock this for now, but create the structure for security capabilities).

### Crate: `dx-dom` (The HTIP Engine)
*   **Template Cache:** Use a `HashMap<u32, web_sys::HtmlTemplateElement>`.
*   **Initialization:** Create a function `register_templates(binary_map: &[u8])` that parses the layout binary ONCE and creates the DOM templates.
*   **The Batch Cloner:**
    *   Implement a Rust function `flush_queue()` that groups operations by `TemplateID`.
    *   **CRITICAL:** It must minimize JS calls. It should prepare a pointers array and call a *single* JavaScript shim (which uses `cloneNode` in a loop).
    *   Use `web_sys::DocumentFragment` to batch the appends before flushing to the real DOM.

### Crate: `dx-morph` (The State Patcher)
*   **Dirty Bits:** Every Component State struct must have a `u64` header called `dirty_mask`.
*   **Update Logic:**
    1.  Check `dirty_mask`.
    2.  If dirty, look up the **Binding Map** (static binary).
    3.  Generate `OP_UPDATE` opcodes.
    4.  Reset `dirty_mask`.
*   **Performance:** This must be O(1). No tree traversal.

### Crate: `dx-sched` (The Heartbeat)
*   Implement a loop using `request_animation_frame`.
*   **Frame Budget:** Check `performance.now()`. If WASM execution exceeds 4ms, yield to browser to prevent frame drops.
*   **Priority Queue:** Handle Input events (Keyboard) immediately. Handle Network events in `requestIdleCallback`.

## 6. JavaScript Bridge (Glue Code)
*   Keep the JS glue file minimal.
*   It should only contain the **HTIP Loop**:
    ```javascript
    // Instructions for the JS side (to be generated/included)
    // iterate count
    // cloneNode(true)
    // read direct memory offsets for text/attributes
    // append to fragment
    ```

## 7. Error Handling
*   **WASM Panic:** Use `console_error_panic_hook`.
*   **Fatal Errors:** If a binary checksum fails or capability check fails, PANIC immediately. Do not try to recover. (Security first).

---
**Instruction to Copilot:**
When generating code, prioritize raw execution speed and memory compactness over readability. Use `unsafe` blocks where necessary for FFI and casting, but document the safety invariant clearly.
Assume the target audience is systems engineers.
Start by scaffolding the `Cargo.toml` workspace and the `dx-core` memory layout.
