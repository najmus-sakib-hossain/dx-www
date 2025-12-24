# Dx Project: Copilot Instructions

## Quick Guidelines
- **Documentation:** All documentation files MUST be in `/docs` folder (except root README.md)
- **Code Quality:** Ensure code is properly formatted (`cargo fmt`) and linted (`cargo clippy`)
- **Dependencies:** Always use latest stable crate versions
- **Structure:** Keep workspace clean - no empty folders or unused files

## Project Context
You are working on **Dx**, a binary-first web framework that replaces React/Next.js with WebAssembly and binary protocols.

**Core Philosophy:** Binary Everywhere - No JSON, no HTML strings, no Virtual DOM diffing.

**Architecture:** Hybrid Template Instantiation Protocol (HTIP) - uses native `cloneNode()` for rendering.

**Target:** January 1, 2026 Public Beta Release

## Cargo Workspace Structure
- **40 specialized crates** in `/crates` directory
- **Core runtime:** core, dom, morph, sched (HTIP engine)
- **Web compiler:** dx-www (TSX → Binary), dx-client (WASM runtime)
- **Developer tools:** dx-cli, dx-forge, dx-debug
- **JavaScript ecosystem:** dx-js-runtime (10x faster than Bun), dx-js-bundler (3.8x faster), dx-js-test-runner (26x faster), dx-js-package-manager (50x target)
- **Data layer:** dx-serializer (world record format), dx-form, dx-query, dx-db
- **Full stack:** dx-server, dx-sync, dx-offline, dx-auth, dx-state, dx-style

## Coding Standards

### Memory & Performance
- **Zero-Copy:** Use `&[u8]` slices, never clone data structures
- **No String Rule:** Forbidden in hot paths - use `u32` indices instead
- **Object Pooling:** Reuse structs per frame (Data-Oriented Design)
- **SIMD:** Use AVX2 where applicable for pattern matching

### Architecture Patterns
- **Linear Memory:** SharedArrayBuffer for zero-copy state
- **Dirty-Bit Tracking:** `u64` bitmask for O(1) change detection
- **Frame Budget:** Max 4ms execution per frame
- **Batch Operations:** Group DOM operations, single flush

### Security & Safety
- **Compile-Time Validation:** Use dx-form, dx-guard, dx-a11y at build time
- **Capability-Based:** Memory-safe with Ed25519 signing
- **Unsafe Blocks:** Only at FFI boundaries with documented safety invariants

### Dependencies (Latest Versions)
```toml
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [...] }
bincode = "2.0.0-rc.3"
bytemuck = { version = "1.14", features = ["derive"] }
oxc_parser = "0.22"
```

## Testing & Quality
- **Format:** Run `cargo fmt --all` before commits
- **Lint:** Run `cargo clippy --all-targets --all-features` and fix warnings
- **Tests:** Write unit tests for all new functionality
- **Docs:** Document all public APIs with examples

## Key Implementation Rules

### Rule 1: No String in Hot Paths
- **Forbidden:** `String` or `Vec<String>` for internal logic
- **Required:** Use `u32` indices, `&[u8]` slices, or `enums`
- **Exception:** Only convert to String when setting DOM `textContent`

### Rule 2: Zero-Copy Memory
- Never clone data structures
- Use `bytemuck` to map `&[u8]` slices onto `#[repr(C)]` structs
- State lives in `SharedArrayBuffer`

### Rule 3: Data-Oriented Design
- Avoid OOP patterns with heavy vtables
- Use Struct of Arrays (SoA) or flat buffers
- Use object pooling - never create/drop per frame

### Rule 4: Frame Budget
- Max 4ms WASM execution per frame
- Yield to browser if exceeded
- Priority: Input events > Render > Network

### Rule 5: Safety Documentation
- `unsafe` blocks only at FFI boundaries
- Document all safety invariants
- WASM panics use `console_error_panic_hook`

## Documentation Reference
- **Architecture:** [docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md)
- **Compiler:** [docs/COMPILER_INTELLIGENCE.md](../docs/COMPILER_INTELLIGENCE.md)
- **HTIP Protocol:** [docs/crates/binary.md](../docs/crates/binary.md)
- **Complete Guide:** [docs/dx.md](../docs/dx.md)

---

**Target:** Professional, production-ready code for January 1, 2026 Beta Release

Assume the target audience is systems engineers.
Start by scaffolding the `Cargo.toml` workspace and the `dx-core` memory layout.
