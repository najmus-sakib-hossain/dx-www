# Design Document: Crates Reorganization

## Overview

This design describes the systematic reorganization of the DX workspace from an inconsistent naming scheme to a professional, hierarchical structure. The reorganization involves moving and renaming crates, updating Cargo.toml configurations, and ensuring all documentation is complete and verified.

The approach follows a phased strategy:
1. **Phase 1**: Create new directory structure
2. **Phase 2**: Move and rename crates
3. **Phase 3**: Update all Cargo.toml files
4. **Phase 4**: Verify builds and tests
5. **Phase 5**: Update documentation

## Architecture

### Current Structure (Before)

```
crates/
├── check/
├── debug/
├── driven/
├── dx/
├── dx-cli/              # → cli/
├── dx-core/             # → core/
├── dx-db-teleport/      # → www/db-teleport/
├── dx-js-bundler/       # → javascript/bundler/
├── dx-js-compatibility/ # → javascript/compatibility/
├── dx-js-monorepo/      # → javascript/monorepo/
├── dx-js-package-manager/ # → javascript/package-manager/
├── dx-js-runtime/       # → javascript/runtime/
├── dx-js-test-runner/   # → javascript/test-runner/
├── dx-py/               # → python/
├── dx-reactor/          # → reactor/
├── dx-www/              # → www/core/
├── dx-www-*/            # → www/*/
├── error/
├── font/
├── forge/
├── generator/
├── i18n/
├── icon/
├── media/
├── serializer/
├── stack/               # REMOVE
├── style/
└── workspace/
```

### Target Structure (After)

```
crates/
├── check/
├── cli/                 # renamed from dx-cli
├── core/                # renamed from dx-core
├── debug/
├── driven/
├── dx/
├── error/
├── font/
├── forge/
├── generator/
├── i18n/
├── icon/
├── javascript/
│   ├── bundler/
│   ├── compatibility/
│   ├── monorepo/
│   ├── package-manager/
│   ├── runtime/
│   └── test-runner/
├── media/
├── python/              # renamed from dx-py
├── reactor/             # renamed from dx-reactor
├── serializer/
├── style/
├── workspace/
└── www/
    ├── a11y/
    ├── auth/
    ├── binary/
    ├── cache/
    ├── client/
    ├── client-tiny/
    ├── core/            # main dx-www framework
    ├── db/
    ├── db-teleport/
    ├── dom/
    ├── fallback/
    ├── form/
    ├── framework-core/  # dx-www-core utilities
    ├── guard/
    ├── interaction/
    ├── morph/
    ├── offline/
    ├── packet/
    ├── print/
    ├── query/
    ├── rtl/
    ├── sched/
    ├── server/
    ├── state/
    └── sync/
```

## Components and Interfaces

### Component 1: Directory Structure Manager

Responsible for creating the new directory hierarchy.

```rust
// Pseudocode for directory operations
fn create_directory_structure() {
    create_dir("crates/javascript");
    create_dir("crates/www");
    // Subdirectories created during move operations
}
```

### Component 2: Crate Mover

Handles moving crates to new locations while preserving git history.

```bash
# Git mv preserves history
git mv crates/dx-cli crates/cli
git mv crates/dx-core crates/core
git mv crates/dx-reactor crates/reactor
git mv crates/dx-py crates/python

# JavaScript crates
git mv crates/dx-js-bundler crates/javascript/bundler
git mv crates/dx-js-compatibility crates/javascript/compatibility
git mv crates/dx-js-monorepo crates/javascript/monorepo
git mv crates/dx-js-package-manager crates/javascript/package-manager
git mv crates/dx-js-runtime crates/javascript/runtime
git mv crates/dx-js-test-runner crates/javascript/test-runner

# WWW crates
git mv crates/dx-www crates/www/core
git mv crates/dx-www-a11y crates/www/a11y
# ... etc for all www crates
git mv crates/dx-db-teleport crates/www/db-teleport
```

### Component 3: Cargo.toml Updater

Updates all Cargo.toml files with new paths.

**Root Cargo.toml Changes:**
```toml
[workspace]
members = [
    # Core crates (unchanged paths)
    "crates/check",
    "crates/debug",
    "crates/driven",
    "crates/dx",
    "crates/error",
    "crates/font",
    "crates/forge",
    "crates/generator",
    "crates/i18n",
    "crates/icon",
    "crates/media",
    "crates/serializer",
    "crates/style",
    "crates/workspace",
    
    # Renamed crates
    "crates/cli",
    "crates/core",
    "crates/reactor",
    "crates/python",
    
    # JavaScript crates
    "crates/javascript/bundler",
    "crates/javascript/compatibility",
    "crates/javascript/monorepo",
    "crates/javascript/package-manager",
    "crates/javascript/runtime",
    "crates/javascript/test-runner",
    
    # WWW crates
    "crates/www/core",
    "crates/www/a11y",
    "crates/www/auth",
    # ... all www subcrates
]
```

**Dependency Path Updates:**
```toml
# Before
[dependencies]
dx-core = { path = "../dx-core" }

# After
dx-core = { path = "../core" }
```

### Component 4: Documentation Updater

Updates all documentation references.

**Files to Update:**
- `docs/README.md` - Main navigation
- `docs/api/*.md` - API references
- `docs/crates/*.md` - Crate documentation
- All internal cross-references

## Data Models

### Crate Mapping Model

```rust
struct CrateMapping {
    old_path: String,      // e.g., "crates/dx-cli"
    new_path: String,      // e.g., "crates/cli"
    package_name: String,  // e.g., "dx-cli" (preserved for compatibility)
}

// Complete mapping table
const CRATE_MAPPINGS: &[CrateMapping] = &[
    // Simple renames
    CrateMapping { old: "dx-cli", new: "cli", package: "dx-cli" },
    CrateMapping { old: "dx-core", new: "core", package: "dx-core" },
    CrateMapping { old: "dx-reactor", new: "reactor", package: "dx-reactor" },
    CrateMapping { old: "dx-py", new: "python", package: "dx-py" },
    
    // JavaScript consolidation
    CrateMapping { old: "dx-js-bundler", new: "javascript/bundler", package: "dx-js-bundler" },
    CrateMapping { old: "dx-js-compatibility", new: "javascript/compatibility", package: "dx-js-compatibility" },
    CrateMapping { old: "dx-js-monorepo", new: "javascript/monorepo", package: "dx-js-monorepo" },
    CrateMapping { old: "dx-js-package-manager", new: "javascript/package-manager", package: "dx-js-package-manager" },
    CrateMapping { old: "dx-js-runtime", new: "javascript/runtime", package: "dx-js-runtime" },
    CrateMapping { old: "dx-js-test-runner", new: "javascript/test-runner", package: "dx-js-test-runner" },
    
    // WWW consolidation
    CrateMapping { old: "dx-www", new: "www/core", package: "dx-www" },
    CrateMapping { old: "dx-www-a11y", new: "www/a11y", package: "dx-www-a11y" },
    // ... etc
];
```

### Documentation Audit Model

```rust
struct DocAudit {
    file_path: String,
    has_todos: bool,
    has_broken_links: bool,
    benchmark_claims: Vec<BenchmarkClaim>,
}

struct BenchmarkClaim {
    claim: String,           // e.g., "10x faster than Bun"
    verified: bool,
    test_file: Option<String>,
    methodology_link: Option<String>,
}
```


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

Based on the prework analysis, the following properties can be verified:

### Property 1: Core Crates Remain at Root Level

*For any* crate in the set {check, debug, driven, dx, error, font, forge, generator, i18n, icon, media, serializer, style, workspace}, the crate SHALL exist at `crates/{crate_name}/` after reorganization.

**Validates: Requirements 1.1, 1.3, 10.1, 10.2**

### Property 2: All Path Dependencies Are Valid

*For any* Cargo.toml file in the workspace that contains a path dependency, the referenced path SHALL point to an existing crate directory.

**Validates: Requirements 3.3, 4.3, 5.4, 7.4, 9.1**

### Property 3: Workspace Members Match Directory Structure

*For any* member path listed in the root Cargo.toml, the corresponding directory SHALL exist and contain a valid Cargo.toml file.

**Validates: Requirements 8.1, 8.2**

### Property 4: JavaScript Crates Consolidated

*For any* crate in the set {bundler, compatibility, monorepo, package-manager, runtime, test-runner}, the crate SHALL exist at `crates/javascript/{crate_name}/` after reorganization.

**Validates: Requirements 5.1, 5.2, 5.3**

### Property 5: WWW Crates Consolidated

*For any* crate in the WWW mapping set, the crate SHALL exist at `crates/www/{new_name}/` after reorganization.

**Validates: Requirements 7.1, 7.2, 7.3**

### Property 6: No Incomplete Documentation

*For any* markdown file in `docs/`, the file SHALL NOT contain TODO, WIP, FIXME, or common placeholder patterns.

**Validates: Requirements 12.1, 12.2, 12.5**

### Property 7: No Stale Documentation References

*For any* internal link in documentation files, the link SHALL point to an existing file or anchor, and SHALL NOT reference removed crates (stack) or old crate paths.

**Validates: Requirements 15.1, 15.2, 15.4, 15.5**

### Property 8: Documentation Links Are Valid

*For any* link in `docs/README.md` navigation table, the linked file SHALL exist.

**Validates: Requirements 15.3**

### Property 9: No Temporary Files in Documentation

*For any* file in `docs/`, the file SHALL NOT match common temporary file patterns (*.tmp, *.bak, *~, .DS_Store).

**Validates: Requirements 14.4**

## Error Handling

### Build Failures

If `cargo check` or `cargo build` fails after reorganization:
1. Check for missing path dependencies
2. Verify all Cargo.toml member paths are correct
3. Check for circular dependencies introduced by moves
4. Verify package names are preserved (not changed)

### Documentation Link Failures

If documentation links are broken:
1. Run link checker to identify broken links
2. Update relative paths based on new structure
3. Remove references to deleted crates

### Git History Preservation

Use `git mv` for all moves to preserve history:
```bash
# Correct
git mv crates/dx-cli crates/cli

# Incorrect (loses history)
mv crates/dx-cli crates/cli
git add .
```

## Testing Strategy

### Unit Tests

Unit tests verify specific examples and edge cases:

1. **Directory existence tests** - Verify specific directories exist/don't exist
2. **Cargo.toml parsing tests** - Verify specific fields have expected values
3. **Build verification** - Run `cargo check` and `cargo build`

### Property-Based Tests

Property tests verify universal properties across all inputs:

1. **Path dependency validation** - Scan all Cargo.toml files, verify all path deps are valid
2. **Documentation link validation** - Scan all markdown files, verify all internal links work
3. **Incomplete documentation detection** - Scan all markdown files for TODO/WIP patterns

### Test Configuration

- Use shell scripts for verification (cross-platform with PowerShell/bash)
- Run `cargo check --workspace` as primary build verification
- Use grep/ripgrep for documentation scanning
- Minimum 100 iterations not applicable (deterministic file system checks)

### Verification Commands

```bash
# Verify workspace builds
cargo check --workspace

# Verify all tests pass
cargo test --workspace

# Check for TODO/WIP in docs
rg -i "TODO|WIP|FIXME|TBD" docs/

# Check for old crate references
rg "dx-cli|dx-core|dx-reactor|dx-py|dx-js-|dx-www-" docs/

# Verify no stack references
rg "stack" docs/
```
