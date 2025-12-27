# Design Document: Crates Cleanup

## Overview

This design describes moving the `error` and `debug` crates from `crates/` root level into `crates/www/` where they logically belong. Both crates are www-specific:
- `error` - Binary error boundaries for WASM component isolation
- `debug` - DevTools bridge for debugging binary protocols

## Architecture

### Current Structure
```
crates/
├── debug/          # DevTools bridge (www-specific)
├── error/          # Error boundaries (www-specific)
├── www/
│   ├── client/     # Uses error & debug as optional deps
│   └── ...
└── ...
```

### Target Structure
```
crates/
├── www/
│   ├── client/     # Uses error & debug as optional deps
│   ├── debug/      # DevTools bridge (moved)
│   ├── error/      # Error boundaries (moved)
│   └── ...
└── ...
```

## Components and Interfaces

### Error Crate (`dx-error`)
- **Purpose**: Binary error boundaries for component isolation
- **Features**: WASM panic hooks, auto-retry, binary error reporting
- **Dependencies**: serde, bincode
- **Used by**: `crates/www/client` (optional feature)

### Debug Crate (`dx-debug`)
- **Purpose**: DevTools bridge for debugging binary protocols
- **Features**: Binary message decoding, performance profiling
- **Dependencies**: serde, serde_json, chrono
- **Used by**: `crates/www/client` (optional feature)

## Data Models

No data model changes required. The crates maintain their existing APIs.

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system.*

### Property 1: Workspace Compilation
*For any* valid workspace state after the move, running `cargo check --workspace` SHALL succeed without errors.
**Validates: Requirements 1.5, 2.5, 3.1**

### Property 2: Path Dependencies Valid
*For any* Cargo.toml file referencing the moved crates, the path dependency SHALL resolve to the correct new location.
**Validates: Requirements 1.3, 1.4, 2.3, 2.4, 3.2**

### Property 3: Crate Functionality Preserved
*For any* test in the error or debug crates, the test SHALL pass after the move.
**Validates: Requirements 1.5, 2.5**

## Error Handling

- If `git mv` fails, the operation should be aborted
- If `cargo check` fails after a move, paths should be verified and corrected
- All changes should be atomic (complete all moves before verification)

## Testing Strategy

### Unit Tests
- Run existing tests in error crate: `cargo test -p dx-error`
- Run existing tests in debug crate: `cargo test -p dx-debug`

### Integration Tests
- Verify workspace compiles: `cargo check --workspace`
- Verify www/client can use optional features: `cargo check -p dx-www-client --features error,debug`

### Property Tests
- Property 1 verified by `cargo check --workspace`
- Property 2 verified by `cargo metadata` showing correct paths
- Property 3 verified by `cargo test -p dx-error -p dx-debug`
