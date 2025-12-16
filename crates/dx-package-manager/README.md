# dx-package-manager

**50x faster package manager using binary-first architecture**

## Status

**Phase 1: Foundation** (In Progress)

- ✅ Workspace structure created
- ✅ dx-pkg-core implemented (memory layout, types, hashing, versioning)
- 🔄 dx-pkg-format implemented (DXP reader, compression, index)
- ⏳ dx-pkg-store (next)
- ⏳ dx-pkg-lock (next)

## Architecture

```
dx-package-manager/
├── dx-pkg-core/      # Core types & memory layouts ✅
├── dx-pkg-format/    # DXP binary packages 🔄
├── dx-pkg-store/     # Content-addressed storage ⏳
├── dx-pkg-lock/      # Binary lock files ⏳
├── dx-pkg-registry/  # DXRP protocol client ⏳
├── dx-pkg-fetch/     # Speculative fetcher ⏳
├── dx-pkg-verify/    # SIMD verification ⏳
├── dx-pkg-resolve/   # Dependency resolver ⏳
├── dx-pkg-link/      # Instant linking ⏳
├── dx-pkg-audit/     # Security scanner ⏳
├── dx-pkg-workspace/ # Monorepo support ⏳
├── dx-pkg-compat/    # npm compatibility ⏳
└── dx-pkg-cli/       # CLI interface ⏳
```

## Performance Targets

| Operation | Bun | dx-pkg | Target |
|-----------|-----|--------|--------|
| Cold Install | 10.5s | 0.53s | **20x** |
| Warm Install | 0.3s | 0.011s | **27x** |
| Parse Lock | 100ms | 0.0001ms | **1000x** |

## Documentation

See [/docs/DX_PACKAGE_MANAGER_VISION.md](../../docs/DX_PACKAGE_MANAGER_VISION.md) for complete vision.

## Building

```bash
cd crates/dx-package-manager
cargo build --release
```

## Testing

```bash
cargo test --workspace
```

---

**Target:** January 1, 2026 Beta Launch
