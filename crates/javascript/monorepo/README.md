# dx-js-monorepo

Binary-first monorepo management system achieving 30-100x performance improvements over traditional JSON-based tools.

## Features

- **Binary Workspace Manifest (BWM)**: Memory-mapped workspace structure with pre-computed dependency graphs
- **Binary Task Graph (BTG)**: Pre-compiled task pipelines with u32 indices and parallel execution maps
- **DXC Cache Format**: Zero-copy task output caching with XOR differential updates
- **DXL-Workspace Lockfile**: O(1) dependency resolution with CRDT merge support
- **SIMD Change Detection**: Blake3 hashing with AVX2 pattern matching for instant file analysis

## Performance Targets

| Operation | Target | Traditional |
|-----------|--------|-------------|
| Workspace load (500 packages) | <5ms | ~500ms |
| Task graph load (1000 nodes) | <2ms | ~200ms |
| Cache lookup | <0.5ms | ~50ms |
| Affected detection | <5ms | ~500ms |
| File hashing (10k files) | <200ms | ~6s |

## Usage

```rust
use dx_js_monorepo::{WorkspaceManager, TaskExecutor};

// Load workspace
let mut workspace = WorkspaceManager::new();
workspace.load("./dx.workspace.bwm")?;

// Execute tasks
let mut executor = TaskExecutor::new();
executor.load("./dx.tasks.btg")?;
executor.run("build")?;
```

## License

MIT
