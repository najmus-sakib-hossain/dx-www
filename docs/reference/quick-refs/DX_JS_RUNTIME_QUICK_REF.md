# Dx-JS-Runtime Quick Reference

## 📦 Module Structure

```
dx-js-runtime/src/
├── compiler/
│   ├── ast_lowering.rs      # AST → MIR conversion
│   ├── expressions.rs       # Expression lowering (590 lines)
│   ├── statements.rs        # Statement lowering (383 lines)
│   ├── functions.rs         # Functions & classes (240 lines)
│   ├── typescript.rs        # TypeScript types (265 lines)
│   ├── builtins_registry.rs # Built-in objects (460 lines)
│   ├── modules.rs           # Module resolution (220 lines)
│   ├── optimizations.rs     # Optimizations (370 lines)
│   ├── mir.rs               # Typed MIR definition
│   ├── codegen.rs           # Cranelift codegen
│   └── parser.rs            # OXC parser wrapper
├── runtime/
│   ├── async_runtime.rs     # Event loop & Promises (240 lines)
│   ├── nodejs.rs            # Node.js APIs (498 lines)
│   ├── builtins.rs          # Runtime built-ins
│   └── memory.rs            # Memory management
├── cache/
│   └── ...                  # Blake3 caching
└── lib.rs                   # Public API
```

## 🔧 Key Components

### Expression Lowering
**File:** `compiler/expressions.rs`  
**Purpose:** Convert all JavaScript expressions to Typed MIR

**Handles:**
- Literals (string, number, boolean, null, undefined)
- Binary operations (+, -, *, /, %, ==, !=, <, >, etc.)
- Unary operations (!, -, +, typeof, void, delete)
- Member access (dot, bracket, private fields)
- Function calls with spread arguments
- Array/object literals
- Template strings
- Arrow functions
- Assignments and updates

**Usage:**
```rust
use crate::compiler::expressions::ExprContext;

let mut ctx = ExprContext::new(builder, &var_map);
ctx.lower_expression(&expr)?;
```

### Statement Lowering
**File:** `compiler/statements.rs`  
**Purpose:** Convert all JavaScript statements to Typed MIR

**Handles:**
- Variable declarations (var, let, const)
- If/else statements
- Switch statements
- Loops (for, while, do-while, for-in, for-of)
- Try/catch/finally
- Break/continue/return
- Block statements

**Usage:**
```rust
use crate::compiler::statements::StatementLowerer;

let mut lowerer = StatementLowerer::new(builder);
lowerer.lower_statement(&stmt)?;
```

### TypeScript Type System
**File:** `compiler/typescript.rs`  
**Purpose:** Analyze TypeScript types for optimization

**Features:**
- Convert TS type annotations to MIR types
- Type inference from expressions
- Assignability checking
- Optimization hint generation

**Usage:**
```rust
use crate::compiler::typescript::TypeScriptAnalyzer;

let mut analyzer = TypeScriptAnalyzer::new();
let mir_type = analyzer.convert_ts_type(&ts_type)?;
let is_assignable = analyzer.is_assignable(&value_type, &target_type);
```

### Built-in Objects
**File:** `compiler/builtins_registry.rs`  
**Purpose:** Native implementations of JavaScript built-ins

**Includes:**
- **Math:** floor, ceil, round, sqrt, sin, cos, tan, random, etc.
- **console:** log, warn, error, time, timeEnd
- **Object:** keys, values, entries, assign, freeze
- **Array:** isArray, from
- **String:** fromCharCode
- **Number:** isNaN, isFinite, parseInt, parseFloat
- **JSON:** parse, stringify

**Usage:**
```rust
use crate::compiler::builtins_registry::BuiltinRegistry;

let registry = BuiltinRegistry::new();
if let Some(func) = registry.get_builtin("Math.floor") {
    // Use the built-in function
}
```

### Module System
**File:** `compiler/modules.rs`  
**Purpose:** ES6 and CommonJS module resolution

**Features:**
- ES6 import/export parsing
- CommonJS require support
- package.json parsing
- Node.js module resolution algorithm

**Usage:**
```rust
use crate::compiler::modules::ModuleResolver;

let resolver = ModuleResolver::new("./src");
let resolved = resolver.resolve("./utils")?;
```

### Async Runtime
**File:** `runtime/async_runtime.rs`  
**Purpose:** Event loop and asynchronous operations

**Components:**
- **EventLoop:** Main event loop with micro/macro tasks
- **Promise:** Promise implementation
- **TimerAPI:** setTimeout/setInterval/setImmediate

**Usage:**
```rust
use crate::runtime::async_runtime::{EventLoop, Promise};

let mut event_loop = EventLoop::new();
event_loop.queue_microtask(|| {
    println!("Microtask executed");
});
event_loop.run();
```

### Node.js APIs
**File:** `runtime/nodejs.rs`  
**Purpose:** Node.js built-in module implementations

**Modules:**
- **fs:** File system operations (read, write, mkdir, etc.)
- **path:** Path manipulation (join, dirname, basename, etc.)
- **process:** Process control (env, argv, cwd, exit)
- **buffer:** Binary data handling

**Usage:**
```rust
use crate::runtime::nodejs::NodeAPIs;

let apis = NodeAPIs::new();
let data = apis.fs.read_file_sync("file.txt")?;
let joined = apis.path.join(&["dir", "file.txt"]);
```

### Optimizations
**File:** `compiler/optimizations.rs`  
**Purpose:** Advanced performance optimizations

**Techniques:**
- **Inline Caching:** Hot method lookup caching
- **Escape Analysis:** Stack vs heap allocation decision
- **SIMD:** Vectorization for array operations
- **Monomorphization:** Generic specialization
- **Constant Folding:** Compile-time evaluation
- **Loop Unrolling:** Reduce loop overhead

**Usage:**
```rust
use crate::compiler::optimizations::OptimizationPipeline;

let mut pipeline = OptimizationPipeline::new();
let optimized_mir = pipeline.optimize(mir)?;
```

## 🎯 Common Patterns

### Lowering a JavaScript Expression
```rust
// 1. Create context
let mut ctx = ExprContext::new(builder, &var_map);

// 2. Lower expression
let result = ctx.lower_expression(&expr)?;

// 3. Get the result local
let dest = result.unwrap(); // LocalId
```

### Compiling a Function
```rust
// 1. Create function builder
let mut builder = FunctionBuilder::new(func_name, return_type);

// 2. Add parameters
for param in params {
    builder.add_param(param.name, param.ty);
}

// 3. Lower function body
let mut lowerer = StatementLowerer::new(builder);
for stmt in &body.statements {
    lowerer.lower_statement(stmt)?;
}

// 4. Build the function
let typed_function = lowerer.finish().build();
```

### Applying Optimizations
```rust
// 1. Build MIR
let mir = compile_to_mir(source_code)?;

// 2. Run optimization pipeline
let mut pipeline = OptimizationPipeline::new();
let optimized = pipeline.optimize(mir)?;

// 3. Generate native code
let native_code = codegen.compile_mir(optimized)?;
```

## 📊 Type System

### MIR Types
```rust
pub enum Type {
    Primitive(PrimitiveType),
    Object(TypeId),
    Array(Box<Type>),
    Function(FunctionSignature),
    Any,
    Never,
}

pub enum PrimitiveType {
    I32,      // 32-bit integer
    I64,      // 64-bit integer
    F64,      // 64-bit float
    Bool,     // Boolean
    String,   // String reference
    Null,     // Null value
    Undefined,// Undefined value
}
```

### Type Conversions
```rust
// TypeScript → MIR
let ts_type = &node.type_annotation;
let mir_type = analyzer.convert_ts_type(ts_type)?;

// Inference from expression
let inferred_type = analyzer.infer_type(&expr);

// Type checking
let is_valid = analyzer.is_assignable(&value_type, &target_type);
```

## 🚀 Performance Tips

### 1. Enable Optimizations
```rust
let mut pipeline = OptimizationPipeline::new();
let optimized = pipeline.optimize(mir)?;
```

### 2. Use Type Annotations
```typescript
// Enables better optimization
function add(a: number, b: number): number {
    return a + b; // Can generate optimized i32/f64 code
}
```

### 3. Leverage SIMD
```javascript
// Array operations automatically vectorized
const result = array.map(x => x * 2); // Uses SIMD if possible
```

### 4. Minimize Escaping Allocations
```javascript
// Stack allocated (fast)
function local() {
    const x = { a: 1 };
    return x.a;
}

// Heap allocated (slower)
function escaping() {
    const x = { a: 1 };
    return x; // x escapes
}
```

## 🔍 Debugging

### Enable Debug Output
```bash
RUST_LOG=debug cargo build -p dx-js-runtime
```

### Check Compilation Errors
```bash
cargo build -p dx-js-runtime 2>&1 | grep error
```

### Run Tests
```bash
cargo test -p dx-js-runtime
```

## 📚 Further Reading

- [DX_JS_RUNTIME_PROGRESS.md](./DX_JS_RUNTIME_PROGRESS.md) - Full progress report
- [COMPILER_INTELLIGENCE.md](./COMPILER_INTELLIGENCE.md) - Compiler optimization decisions
- OXC Documentation: https://github.com/oxc-project/oxc
- Cranelift Guide: https://cranelift.dev/

## 🎯 Next Steps

1. **Add Tests:** Write comprehensive unit and integration tests
2. **Benchmarks:** Measure performance vs Bun/Node.js
3. **Cache:** Implement persistent code cache (Phase 10)
4. **Docs:** Write detailed API documentation
5. **Examples:** Create example applications

---

**Last Updated:** December 12, 2025  
**Status:** Phases 1-9 Complete (40%)
