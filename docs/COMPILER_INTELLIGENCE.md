# The Intelligent Compiler

**Status:** ✅ Implemented (December 12, 2025)

## The Problem

Developers shouldn't have to choose between "Micro" (338 bytes) and "Macro" (7.5 KB) runtimes manually. That's a compiler's job.

## The Solution: Build Intelligence

The `dx` compiler now **automatically analyzes** your application and selects the optimal runtime variant.

### Decision Matrix

```
┌─────────────┬──────────────┬────────┬───────────────┐
│ Components  │ State        │ Events │ Runtime       │
├─────────────┼──────────────┼────────┼───────────────┤
│ < 10        │ Low          │ < 5    │ Micro (338B)  │
│ < 10        │ Medium       │ < 10   │ Micro (338B)  │
│ >= 10       │ Any          │ Any    │ Macro (7.5KB) │
│ Any         │ High         │ Any    │ Macro (7.5KB) │
│ Any         │ Any          │ >= 10  │ Macro (7.5KB) │
└─────────────┴──────────────┴────────┴───────────────┘
```

## Implementation

### New Module: `analyzer.rs`

Located in: [crates/dx-compiler/src/analyzer.rs](../crates/dx-compiler/src/analyzer.rs)

**Core Features:**

1. **Complexity Metrics Extraction**
   - Component count
   - State variable count and types
   - Event handler count  
   - JSX node count
   - Async logic detection
   - Effect hook usage
   - Component nesting depth

2. **State Complexity Classification**
   - **Low:** 0-2 primitive state variables
   - **Medium:** 3-5 simple state variables
   - **High:** 6+ vars or complex types (arrays, objects)

3. **Runtime Decision Engine**
   ```rust
   pub fn analyze_and_decide(
       modules: &[ParsedModule],
       verbose: bool
   ) -> Result<(ComplexityMetrics, RuntimeVariant)>
   ```

### Updated Build Pipeline

**Before:**
```
Parse → Tree Shake → Split → Generate → Pack
```

**After:**
```
Parse → Analyze & Decide → Tree Shake → Split → Generate → Pack → Copy Runtime
```

### Build Output

```bash
$ dx build

🏭 Dx Compiler - Building...

[00:00:01] ███████████████████████ 7/7 Copying micro runtime...

  📊 Complexity Analysis:
     Components:      3
     State Variables: 2
     Event Handlers:  3
     JSX Nodes:       15
     State:           Low

  🎯 Decision: Micro (338 bytes) - Optimized for simplicity

  🧠 Micro (338B) runtime selected

✓ Built in 1.24s
  Output: dist/
  Runtime: Micro (338 bytes) (auto-selected)
```

## Metadata Output

The compiler now generates `dist/runtime.json`:

```json
{
  "runtime": "micro",
  "metrics": {
    "component_count": 3,
    "total_state_vars": 2,
    "event_handler_count": 3,
    "total_jsx_nodes": 15,
    "state_complexity": "Low",
    "has_async_logic": false,
    "has_effects": false
  },
  "timestamp": "2025-12-12T10:30:45Z"
}
```

## File Structure

```
dist/
├── app.dxb              # HTIP binary (templates + opcodes)
├── runtime.wasm         # Auto-selected runtime (338B or 7.5KB)
└── runtime.json         # Metadata explaining the choice
```

## Decision Rules

The analyzer applies these rules **in order**:

1. **High State Complexity** → Macro
2. **≥10 Components** → Macro
3. **≥10 Event Handlers** → Macro
4. **Async + Many Hooks** → Macro
5. **Effects + Many Hooks** → Macro
6. **Deep Component Trees (>5)** → Macro
7. **Large JSX Trees (>50 nodes)** → Macro
8. **Default** → Micro (for simple apps)

## Testing

Unit tests in `analyzer.rs`:

```rust
#[test]
fn test_micro_decision() { ... }

#[test]
fn test_macro_decision_many_components() { ... }

#[test]
fn test_macro_decision_high_state() { ... }
```

Run tests:
```bash
cd crates/dx-compiler
cargo test analyzer
```

## Benefits

✅ **Zero Decision Fatigue** - Developer writes code, compiler decides  
✅ **Optimal Performance** - Always get the smallest runtime that works  
✅ **Transparent** - Verbose mode shows decision reasoning  
✅ **Predictable** - Same code always gets same runtime  
✅ **Auditable** - `runtime.json` documents the choice  

## Philosophy

> "The developer writes components.  
> The compiler analyzes complexity.  
> The runtime executes efficiently.  
> Nobody thinks about bundle size."

This is **Zero-Configuration Performance**. The assembly line is complete.

## Next Steps

1. **Week 2:** Hot-reload integration (analyze on save)
2. **Week 3:** Source maps (map runtime back to .tsx)
3. **Week 4:** Advanced optimizations (code splitting by route)

---

**The Compiler is now intelligent. The Developer is now free.**
