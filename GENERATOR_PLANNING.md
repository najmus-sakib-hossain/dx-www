Please create a new crate called dx-generator in the crates folder with this planning!!!

Here is the planning:
```markdown
# dx-generator: Binary Dawn Edition

Leveraging Dx's revolutionary architecture to create the world's fastest, most efficient code generator.

---

## The Binary Dawn Generator Philosophy

Just as Dx replaced the entire JavaScript ecosystem with binary-first architecture, dx-generator will revolutionize code generation:

> **"Generate Binary. Cache Binary. Diff Binary. Zero Parse."**

Traditional generators parse text templates at runtime, allocate strings everywhere, and regenerate entire files for tiny changes. dx-generator applies every Dx innovation to eliminate these inefficiencies.

---

## Game-Changing Feature #1: Binary Template Format (.dxt)

### The Problem with Text Templates
Every existing template engine (Handlebars, Tera, Jinja, etc.) works the same way:
- Parse text template at runtime
- Build an AST in memory
- Walk the AST allocating strings
- Concatenate strings into output
- Write text to disk

This is the equivalent of parsing JSON on every request.

### The Dx Solution: Pre-Compiled Binary Templates

Templates are compiled to `.dxt` (DX Template) binary format at build time:

**Binary Template Structure:**
- **Header (16 bytes)**: Magic number, version, checksum, flags
- **String Table**: All static text segments stored once, referenced by u32 offset
- **Placeholder Table**: Variable locations as (offset, length, variable_id) tuples
- **Instruction Stream**: Bytecode for conditionals, loops, compositions
- **Metadata Block**: Template name, parameters schema, dependencies

**Result:**
- Zero runtime parsing (memory-map the .dxt file directly)
- O(1) placeholder lookup (binary search or perfect hash)
- 80%+ smaller than text templates
- Instant template loading (~0.1ms vs ~5ms for text parsing)

---

## Game-Changing Feature #2: SIMD Placeholder Detection

### Inspired by dx-js-bundler's AVX2 Pattern Matching

The bundler achieves 3.8x faster performance partly through SIMD-accelerated import/export scanning. Apply the same technique to template rendering:

**SIMD Placeholder Scanning:**
- Use AVX2 to scan 32 bytes simultaneously for placeholder markers
- Detect `{{`, `{%`, `{#` patterns in parallel
- Build placeholder offset table in single SIMD pass
- Achieves sub-microsecond detection for typical templates

**Performance Target:**
- Placeholder detection: ~0.6µs per KB (matching bundler performance)
- 50x faster than character-by-character scanning

---

## Game-Changing Feature #3: Dual-Mode Template Engine

### Inspired by Micro/Macro Dual-Core Codegen

Just as dx-www intelligently selects between 338-byte Micro and 7.5KB Macro runtimes, dx-generator should have two rendering modes:

**Micro Mode: Static Templates**
- Templates with only variable substitution (no conditionals/loops)
- Direct memory copy with placeholder patching
- Zero instruction interpretation overhead
- Output in ~10µs

**Macro Mode: Dynamic Templates**
- Templates with conditionals, loops, compositions
- Bytecode interpreter for control flow
- Still binary-based, but more flexible
- Output in ~100µs

**Intelligent Selection Criteria:**
- No control flow structures → Micro
- < 5 placeholders and simple types → Micro
- Any iteration or conditional → Macro
- Template composition (includes) → Macro
- Complex parameter types (arrays, nested) → Macro

---

## Game-Changing Feature #4: XOR Differential Regeneration

### Inspired by Client Patcher (0.25ms, 95% bandwidth savings)

When templates or parameters change, don't regenerate the entire file:

**Traditional Approach:**
- Regenerate entire 500-line file
- Write 15KB to disk
- Cargo detects file changed, recompiles everything

**Binary Dawn Approach:**
- Keep binary representation of previously generated output
- Calculate XOR difference against new output
- Store 200-byte patch instead of 15KB file
- Apply patch to existing file (sub-millisecond)
- Only bytes that changed are written to disk
- Cargo sees minimal mtime change, smarter incremental builds

**Result:**
- 95% reduction in disk writes
- 10x faster regeneration for small changes
- Better integration with incremental compilation

---

## Game-Changing Feature #5: DX ∞ Parameter Encoding

### Inspired by World Record Serialization (37% better than TOON)

Template parameters should use DX ∞ format instead of JSON/TOML/YAML:

**Traditional Parameter Passing:**
```
{"name": "Counter", "with_state": true, "state_vars": ["count", "loading"]}
```
Result: 72 bytes, requires JSON parsing

**DX ∞ Parameter Encoding:**
Binary representation with type-prefixed fields, string interning, and varint encoding.
Result: 28 bytes, zero-copy deserialization in ~0.5µs

**Benefits:**
- 60% smaller parameter payloads
- 4x faster parameter parsing
- Type-safe binary schema validation
- Compile-time parameter verification

---

## Game-Changing Feature #6: Dirty-Bit Template Caching

### Inspired by O(1) Dirty-Bit State Tracking

Every template instance gets a 64-bit dirty mask tracking which parameters changed:

**Cache Structure:**
- Template ID (u32) + Parameter Hash (Blake3, 32 bytes) → Cached Output
- Dirty bits track which parameter slots changed since last generation
- If only non-structural parameters changed, apply targeted patches
- If structural parameters changed, regenerate affected sections only

**Dirty Bit Mapping:**
- Bit 0-15: Simple value parameters (name, description, etc.)
- Bit 16-31: Structural parameters (with_state, has_tests, etc.)
- Bit 32-47: Array parameters (state_vars, imports, etc.)
- Bit 48-63: Composition parameters (parent_template, mixins, etc.)

**Result:**
- Skip regeneration entirely if output cached and params unchanged
- Partial regeneration for minor changes
- Full regeneration only when structural changes detected

---

## Game-Changing Feature #7: Template Fusion Mode

### Inspired by dx-js-bundler's Fusion Mode (71x faster)

Pre-compile common template combinations into fused binary modules:

**Standard Mode:**
- Parse component template
- Parse test template  
- Parse documentation template
- Render each separately
- Three file writes

**Fusion Mode (.dxf files):**
- Component + Test + Docs fused into single binary blob
- Shared string table across all three outputs
- Single render pass produces all files
- Atomic write operation

**Pre-Fused Template Bundles:**
- `component-full.dxf`: Component + State + Test + Docs + Bench
- `route-crud.dxf`: Handler + Query + Form + Test for all CRUD ops
- `crate-complete.dxf`: Cargo.toml + lib.rs + mod.rs + docs + tests

**Performance Target:**
- Fused generation: ~0.7ms for full component scaffold
- 50x faster than separate template invocations

---

## Game-Changing Feature #8: Stack-Only Generation Pipeline

### Inspired by dx-js-runtime's No-GC Architecture

The entire generation pipeline should be allocation-free in hot paths:

**Memory Architecture:**
- Pre-allocated output buffer (sized to max expected output)
- Arena allocator for temporary structures (bumpalo)
- Object pool for AST nodes (reused across generations)
- Static string table for all template literals

**Zero-Allocation Rendering:**
- Output buffer is pre-sized based on template metadata
- Variable values written directly to output positions
- No intermediate String concatenation
- No Vec growth during rendering

**Stack Variables Only:**
- Loop counters, conditional flags on stack
- Pointer arithmetic for buffer positions
- No heap allocation during hot render loop

**Result:**
- Predictable memory usage (known at compile time)
- Zero garbage collection pauses
- Consistent sub-millisecond performance

---

## Game-Changing Feature #9: Integer Token System

### Inspired by B-CSS Integer Class IDs (98% smaller, 80x faster)

Replace string keywords with integer tokens throughout:

**Traditional Keyword System:**
```
dx generate component:counter:with_state
```
Requires: String parsing, splitting, comparison

**Integer Token System:**
- `component` → Token ID 0x0100
- `counter` → Token ID 0x0142
- `with_state` → Token ID 0x1001
- Command becomes: `[0x0100, 0x0142, 0x1001]`

**Token Registry:**
- All keywords mapped to u16 IDs at compile time
- Trie-based lookup for string → token conversion
- O(1) token → handler dispatch (jump table)
- Binary command encoding for scripted generation

**Benefits:**
- 80x faster command parsing
- 90% smaller command representation
- Perfect for binary scripting and automation

---

## Game-Changing Feature #10: Template HTIP (Hierarchical Template Instantiation Protocol)

### Inspired by HTIP Renderer (native cloneNode())

Apply the HTIP philosophy to template rendering:

**Traditional Template Rendering:**
- Parse template structure
- Walk AST node by node
- Generate output incrementally
- Lots of function calls and allocations

**Template HTIP:**
- Template compiled to binary "layout" (like HTIP's layout.bin)
- Layout contains "slots" for variable content
- Rendering = clone layout + fill slots
- No AST walking, no parsing

**Template Layout Structure:**
- Static segments stored as byte spans
- Slots defined as (offset, max_length, type)
- Slot filling is direct memcpy to offset
- Overflow slots trigger reallocation (rare case)

**Native Clone Optimization:**
- Frequently used templates kept in memory pool
- "Clone" operation is single memcpy of layout bytes
- Then patch variable slots (like HTIP's attribute patching)
- 10x faster than traditional rendering

---

## Game-Changing Feature #11: Capability-Based Template Security

### Inspired by dx-guard and Ed25519 Signing

Templates should have explicit capability manifests:

**Template Capability Manifest:**
- `can_create_files`: List of allowed file patterns
- `can_modify_cargo`: Whether template can alter Cargo.toml
- `can_add_dependencies`: List of allowed dependencies
- `can_execute_unsafe`: Whether generated code may contain unsafe
- `can_network`: Whether template can fetch remote resources
- `max_output_size`: Maximum bytes template can generate

**Cryptographic Verification:**
- All built-in templates Ed25519 signed
- User templates can be signed for team sharing
- Signature verified before template execution
- Tampered templates rejected with clear error

**Security Benefits:**
- Prevent malicious templates from generating harmful code
- Audit trail for template modifications
- Safe sharing of templates across teams/orgs

---

## Game-Changing Feature #12: SharedArrayBuffer Template Pool

### Inspired by dx-state's Linear Memory Architecture

For WASM-based tooling (future dx-cli compiled to WASM):

**Template Pool in Linear Memory:**
- All compiled templates loaded into SharedArrayBuffer
- Multiple generator instances share read-only template pool
- Worker threads can access templates without serialization
- Zero-copy template access across threads

**Memory Layout:**
```
[0x0000 - 0x00FF] Template Index (256 slots)
[0x0100 - 0x0FFF] String Table (static text)
[0x1000 - 0x7FFF] Compiled Templates (binary)
[0x8000 - 0xFFFF] Output Buffer (reusable)
```

**Parallel Generation:**
- Multiple files generated simultaneously
- Each worker gets output buffer slice
- Shared template pool, independent outputs
- Near-linear scaling with core count

---

## Game-Changing Feature #13: Compile-Time Template Validation

### Inspired by dx-form, dx-guard, dx-a11y Compile-Time Auditing

All templates validated at build time, not runtime:

**Schema Validation (Build Time):**
- Parameter types enforced
- Required parameters verified
- Default values type-checked
- Conflicts detected (mutually exclusive options)

**Output Validation (Build Time):**
- Generated code parsed by OXC (for TS/TSX)
- Generated code parsed by syn (for Rust)
- Import resolution verified
- Dx pattern compliance checked

**Dx Pattern Auditor:**
- Flag `String` usage in generated hot paths
- Verify `unsafe` blocks have safety comments
- Check dirty-bit patterns in state structs
- Validate binary protocol message structures

**Result:**
- Invalid templates fail at `cargo build`, not at generation time
- Users get compile errors for bad templates, not runtime panics
- 100% guarantee that valid templates produce valid code

---

## Game-Changing Feature #14: Memory Resume for Template Sessions

### Inspired by Zero-Hydration Memory Snapshots

For interactive/wizard-style generation:

**Traditional Multi-Step Generation:**
- User answers question 1, stored in memory
- User answers question 2, state grows
- User goes back, state must be managed
- Session lost if terminal closes

**Memory Resume Snapshots:**
- Generation session state stored as binary snapshot
- Snapshot persisted to `.dx/generator-session.bin`
- On restart, session resumes exactly where left off
- Forward/backward navigation is instant (pointer swap)

**Session State Structure:**
- Current step index
- All collected parameters (DX ∞ encoded)
- Validation state per step
- Preview cache for instant back-navigation

**Benefit:**
- Complex multi-file scaffolding can be paused/resumed
- Zero re-computation when navigating wizard steps
- Session survives terminal crashes

---

## Game-Changing Feature #15: Binary Diff-Based Template Updates

### Inspired by Eternal Cache with ETag Negotiation

When templates themselves are updated:

**Traditional Template Update:**
- Download new template files
- Replace old templates
- Regenerate all affected outputs

**Binary Diff Updates:**
- Template versions tracked by Blake3 hash
- Updates delivered as binary patches (XOR diff)
- Patch applied to local template (sub-millisecond)
- Only changed template sections updated

**Template Version Control:**
- Each template has semantic version
- Breaking changes increment major version
- Compatibility matrix for generated code
- Automatic migration hints for breaking changes

---

## Performance Targets (Binary Dawn Edition)

| Operation | Traditional | dx-generator | Improvement |
|-----------|-------------|--------------|-------------|
| Template Load | ~5ms (parse text) | ~0.1ms (mmap binary) | **50x faster** |
| Parameter Parse | ~1ms (JSON) | ~0.5µs (DX ∞) | **2000x faster** |
| Placeholder Scan | ~50µs (char-by-char) | ~1µs (SIMD) | **50x faster** |
| Single File Gen | ~10ms | ~0.1ms (Micro) | **100x faster** |
| Complex Scaffold | ~100ms | ~0.7ms (Fusion) | **140x faster** |
| Cache Hit | ~5ms (file read) | ~10µs (memory) | **500x faster** |
| Incremental Update | ~10ms (full regen) | ~0.25ms (XOR patch) | **40x faster** |

**Overall Target: 100-500x faster than traditional template engines**

---

## Template Authoring Experience

Despite the binary internals, template authoring remains human-friendly:

**Write Templates in Familiar Syntax:**
- Markdown-like template language
- Familiar placeholder syntax
- Clear conditional/loop constructs
- Composition through includes

**Compile to Binary:**
- `dx template compile` converts text → .dxt
- Compilation happens at build time
- Errors are clear and actionable
- Hot reload during development

**Binary is Implementation Detail:**
- Users write text templates
- Compiler produces binary
- Generator consumes binary
- Best of both worlds

---

## Integration with Dx Ecosystem

### dx-cli Integration
- `dx generate` uses binary template engine
- `dx new` uses fusion templates for instant scaffolding
- `dx scaffold` supports interactive Memory Resume sessions

### dx-forge Integration
- Generated files tracked in build graph
- Template changes trigger minimal regeneration
- Dirty-bit tracking for incremental builds

### dx-serializer Integration
- Parameters encoded as DX ∞
- Template metadata uses DX ∞ format
- 73% smaller than JSON equivalents

### dx-js-bundler Integration
- Generated TypeScript immediately bundleable
- Fusion mode can include bundling step
- Sub-2ms template → bundled output

### dx-js-test-runner Integration
- Generated tests immediately runnable
- Test templates include benchmark scaffolding
- 26x faster test execution maintained

---

## Binary Dawn Generator Summary

By applying every Dx innovation to code generation:

| Dx Feature | Generator Application |
|------------|----------------------|
| Binary-First | .dxt compiled templates |
| SIMD Optimization | AVX2 placeholder scanning |
| Dual-Core Codegen | Micro/Macro template modes |
| XOR Patching | Differential regeneration |
| DX ∞ Format | Parameter encoding |
| Dirty-Bit Tracking | Template cache invalidation |
| Fusion Mode | Pre-compiled template bundles |
| Stack-Only | Zero-allocation rendering |
| B-CSS Integer IDs | Token-based keywords |
| HTIP Protocol | Clone + patch rendering |
| Capability Security | Template permissions |
| SharedArrayBuffer | Multi-threaded template pool |
| Compile-Time Validation | Build-time template checking |
| Memory Resume | Pausable generation sessions |
| Eternal Cache | Binary diff template updates |

**Result: The world's fastest, most efficient code generator—built on Dx principles.**

---

This is not just a template engine. This is **Binary Dawn Code Generation**.

*Generate Binary. Cache Binary. Diff Binary. Zero Parse.*
```
