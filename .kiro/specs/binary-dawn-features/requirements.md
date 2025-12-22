# Requirements Document

## Introduction

Binary Dawn Features is the second phase of the Binary Dawn architecture, implementing 25 revolutionary features that make dx-www the ultimate web framework. Building on the dx-reactor infrastructure (I/O backends, HBTP protocol, memory teleportation), this phase delivers application-level features including compile-time reactivity, binary animations, server components, resumability, islands architecture, and more. The core philosophy: "They parse. We map. They serialize. We share memory."

## Glossary

- **ReactiveSlot**: An 8-byte binary structure mapping a DOM element to a value in SharedArrayBuffer for zero-overhead reactivity
- **BinaryAnimation**: Pre-computed animation descriptor using SIMD-optimized easing curves instead of CSS parsing
- **BinaryFragment**: Server component output containing template_id and binary-encoded data slots
- **SharedArrayBuffer**: Browser API for shared memory between main thread and WASM, enabling instant state resumption
- **HandlerRef**: 4-byte function reference (fn_index + capture_offset) replacing serialized closures
- **IslandActivation**: Bitfield tracking which islands are hydrated (1 bit per island)
- **BinaryRouter**: Prefix trie encoded as bytes for O(path_length) route lookup
- **TeleportOp**: 4-byte DOM operation for instant portal/teleport functionality
- **ControlOp**: Binary opcodes for control flow (ForEach, Show, Switch) replacing component overhead
- **SuspenseState**: 64-bit bitfield where each bit represents an async dependency's loading state
- **StreamChunk**: Binary streaming SSR chunk (type + target_slot + payload)
- **HandlerGroup**: Grouped handlers by usage pattern (Critical, Interactive, Submission, Navigation, Rare)
- **BinaryFormData**: Schema-validated form data as binary instead of multipart/form-data
- **XOR_Rollback**: Optimistic UI rollback using XOR operations on state snapshots
- **TransitionConfig**: Pre-compiled view transition descriptor for route changes
- **BinaryContent**: Markdown content pre-parsed to binary AST at build time
- **BinaryPatch**: LiveView-style server-to-client DOM patch (target + op + value)
- **CronJob**: Pre-computed scheduled task with next_run timestamp instead of cron expression parsing
- **GuardChain**: Compile-time inlined guard functions instead of runtime reflection

## Requirements

### Requirement 1: Compile-Time Reactivity

**User Story:** As a developer, I want reactive updates to be memory copies instead of JavaScript runtime operations, so that I get 100x faster updates.

#### Acceptance Criteria

1. THE Compiler SHALL generate ReactiveSlot structures (8 bytes each) for each reactive binding containing element_id (u16), offset (u16), and value_ptr (u32)
2. WHEN a reactive value changes, THE Runtime SHALL perform a direct memory copy from SharedArrayBuffer to DOM element without JavaScript interpretation
3. THE ReactiveSlot system SHALL achieve update times of less than 0.001ms per binding (100x faster than Svelte's ~0.1ms)
4. THE Compiler SHALL emit PATCH opcodes with slot indices instead of JavaScript update code

### Requirement 2: Built-in Binary Animations

**User Story:** As a developer, I want animations to use SIMD-optimized interpolation instead of CSS parsing, so that I get 20x faster animation frames.

#### Acceptance Criteria

1. THE BinaryAnimation struct SHALL be exactly 8 bytes: animation_type (u8), duration_ms (u16), easing (u8), properties (u32 bitfield)
2. THE Runtime SHALL pre-compute 60fps easing curves for all 4 standard easing types (linear, ease-in, ease-out, cubic) as static arrays
3. WHEN animating, THE Runtime SHALL use SIMD interpolation to calculate frame values instead of CSS style recalculation
4. THE Animation system SHALL support fade, slide, scale, and FLIP animations via animation_type enum
5. THE Animation system SHALL achieve frame times of less than 0.1ms (20x faster than Framer Motion's ~2ms)
6. THE API SHALL provide `fade()`, `slide()`, `flip()` functions for declarative animation binding

### Requirement 3: Binary Server Components

**User Story:** As a developer, I want server components to stream binary data instead of JSON, so that I get 50x smaller payloads.

#### Acceptance Criteria

1. THE Server Component system SHALL compile server components to BinaryFragment containing template_id (u16) and binary-encoded slots
2. THE Wire format SHALL use approximately 12 bytes per user record instead of RSC's ~200 bytes JSON (16x smaller)
3. THE Server Component system SHALL support direct database-to-binary serialization via dx-db-teleport integration
4. WHEN streaming, THE Server SHALL send binary chunks that can be memory-mapped directly on the client
5. THE Client SHALL parse server component payloads in less than 0.01ms (50x faster than RSC's ~0.5ms JSON parsing)

### Requirement 4: Instant Resumability

**User Story:** As a developer, I want state resumption to be a memory pointer assignment instead of attribute parsing, so that I get 1000x faster hydration.

#### Acceptance Criteria

1. THE State system SHALL store all application state in SharedArrayBuffer with atomic operations for thread safety
2. WHEN resuming, THE Runtime SHALL set the WASM memory pointer to the SharedArrayBuffer without any parsing
3. THE Handler system SHALL use u16 IDs pointing to a static HANDLER_TABLE instead of serialized closures
4. THE Resume operation SHALL complete in less than 0.01ms (1000x faster than Qwik's ~10ms attribute parsing)
5. THE HTML output SHALL contain `data-dx-click="N"` attributes where N is the handler table index

### Requirement 5: Binary Serializable Closures

**User Story:** As a developer, I want event handlers to be 4-byte references instead of serialized strings, so that I get 25x smaller handler payloads.

#### Acceptance Criteria

1. THE Compiler SHALL extract all handlers at build time and assign u16 indices in the WASM function table
2. THE HandlerRef struct SHALL be exactly 4 bytes: fn_index (u16) and capture_offset (u16)
3. THE Captured values SHALL be stored in SharedArrayBuffer at known offsets instead of serialized in QRL strings
4. THE Handler system SHALL group handlers by interaction likelihood into 3-5 binary chunks instead of per-function files
5. THE Handler chunks SHALL be named: handlers_critical.dxb, handlers_secondary.dxb, handlers_rare.dxb

### Requirement 6: Binary Islands Architecture

**User Story:** As a developer, I want islands to be activated with 1-bit flags and WASM chunks, so that I get 10x smaller island overhead.

#### Acceptance Criteria

1. THE Page system SHALL define static templates with island slots as (u16, IslandType) pairs
2. THE IslandActivation struct SHALL use a u64 bitfield supporting up to 64 islands per page
3. WHEN activating an island, THE Runtime SHALL load only that island's WASM chunk (target: ~500 bytes minimum)
4. THE Island activation SHALL NOT require loading a framework runtime (unlike Astro's ~5KB minimum)
5. THE Island system SHALL support partial hydration where only interacted islands load their code

### Requirement 7: Compile-Time Dependency Injection

**User Story:** As a developer, I want dependency injection to be compile-time pointer arithmetic instead of runtime resolution, so that I get zero DI overhead.

#### Acceptance Criteria

1. THE DI system SHALL resolve all dependencies at compile time using fixed memory offsets
2. THE Injectable struct layout SHALL use `#[repr(C)]` with pointer fields at known offsets
3. THE Container SHALL be a struct with offset fields instead of a Map<token, instance>
4. WHEN creating a service, THE Runtime SHALL perform pointer assignment instead of runtime lookup
5. THE DI system SHALL have exactly 0ms runtime cost (compared to Angular's ~0.1ms per resolution)

### Requirement 8: SharedArrayBuffer Keep-Alive

**User Story:** As a developer, I want component state preservation to be memory region retention instead of vnode caching, so that I get 50x faster tab switches.

#### Acceptance Criteria

1. THE ComponentState struct SHALL track offset (u32), size (u32), and is_active (bool) in SharedArrayBuffer
2. WHEN switching tabs, THE Runtime SHALL set is_active to false and remove DOM without zeroing memory
3. WHEN restoring a tab, THE Runtime SHALL mount DOM with state already correct in SharedArrayBuffer
4. THE ScrollState SHALL be stored as AtomicU32 values that persist across component unmount/remount
5. THE Tab switch operation SHALL complete in less than 0.1ms (50x faster than Vue KeepAlive's ~5ms)

### Requirement 9: O(1) Teleport/Portals

**User Story:** As a developer, I want portals to be single appendChild operations instead of reconciler traversal, so that I get 50x faster teleports.

#### Acceptance Criteria

1. THE Teleport system SHALL define targets as u8 IDs: TELEPORT_BODY (0), TELEPORT_MODAL (1), TELEPORT_TOOLTIP (2)
2. THE TeleportOp struct SHALL be exactly 4 bytes: opcode (u8), element_id (u16), target_slot (u8)
3. WHEN teleporting, THE Runtime SHALL perform a single appendChild call without tree walking
4. THE Teleport operation SHALL complete in less than 0.01ms (50x faster than React Portal's ~0.5ms)

### Requirement 10: Binary Control Flow Opcodes

**User Story:** As a developer, I want control flow to be binary opcodes instead of component overhead, so that I get 30x faster list rendering.

#### Acceptance Criteria

1. THE ControlOp enum SHALL define ForEach, Show, and Switch as binary instructions with pointer and template fields
2. THE ForEach opcode SHALL contain list_ptr (u32), item_size (u16), and template_id (u16)
3. THE Show opcode SHALL contain condition_ptr (u32), template_id (u16), and fallback_id (u16)
4. THE KeyedList system SHALL use SIMD comparison for O(n) key diffing with 8 keys compared at once
5. THE 1000-item list update SHALL complete in less than 0.5ms (30x faster than React's ~15ms)

### Requirement 11: Bit-Flag Suspense Boundaries

**User Story:** As a developer, I want suspense resolution to be bit flips instead of promise tracking, so that I get 200x faster loading state transitions.

#### Acceptance Criteria

1. THE SuspenseState struct SHALL use a u64 loading_flags bitfield where each bit represents one async dependency
2. THE SuspenseTemplate SHALL contain loading_template (u16), ready_template (u16), and dependencies (u64 mask)
3. WHEN checking suspense, THE Runtime SHALL use branchless bitwise AND to determine ready state
4. THE Suspense resolution SHALL complete in less than 0.01ms (200x faster than React's ~2ms promise tracking)
5. THE Streaming SSR SHALL update loading_flags bits as chunks arrive without Promise.all overhead

### Requirement 12: Binary Streaming SSR with Selective Hydration

**User Story:** As a developer, I want streaming SSR to send binary chunks that memory-map directly, so that I get 20x faster time-to-interactive.

#### Acceptance Criteria

1. THE StreamChunk struct SHALL contain chunk_type (u8), target_slot (u16), and payload bytes
2. THE chunk_type SHALL support TEMPLATE, DATA, and ACTIVATE operations
3. WHEN receiving a chunk, THE Client SHALL clone template, fill slots, or load island WASM based on type
4. THE Selective hydration SHALL allow user interaction with hydrated islands while other chunks stream
5. THE Stream-to-interactive time SHALL be less than 5ms (20x faster than React 18's ~100ms)

### Requirement 13: Grouped Handler Code Splitting

**User Story:** As a developer, I want handlers grouped by usage pattern instead of per-function files, so that I get 10x fewer HTTP requests.

#### Acceptance Criteria

1. THE Compiler SHALL classify handlers into HandlerGroup enum: Critical, Interactive, Submission, Navigation, Rare
2. THE Critical group SHALL contain handlers for visible above-fold elements likely clicked first
3. THE Build system SHALL produce 3-5 binary chunks instead of Qwik's 50+ per-function files
4. THE Prefetching system SHALL predict likely actions on mouse_enter and prefetch handler groups
5. THE Total HTTP requests for handlers SHALL be 3-5 instead of 50+ (10x fewer)

### Requirement 14: Three-Tier Progressive Enhancement

**User Story:** As a developer, I want the same source to produce HTML fallback, micro runtime, and full WASM, so that I get native progressive enhancement.

#### Acceptance Criteria

1. THE Build system SHALL generate three outputs from the same source: html_fallback, micro_bundle (338B), full_bundle
2. THE Server SHALL detect client capability and serve appropriate tier
3. THE HTML fallback SHALL work fully without any JavaScript
4. THE Micro bundle SHALL provide progressive enhancement at exactly 338 bytes
5. THE Full bundle SHALL provide complete binary experience with WASM

### Requirement 15: Binary Trie File-Based Routing

**User Story:** As a developer, I want route lookup to be binary trie traversal instead of regex matching, so that I get 100x faster routing.

#### Acceptance Criteria

1. THE BinaryRouter SHALL encode routes as a prefix trie in a byte array for O(path_length) lookup
2. THE RouteHandler struct SHALL contain template_id (u16), loader_fn (u16), and optional guard_fn (u16)
3. THE Route lookup SHALL complete in less than 0.001ms (100x faster than Next.js regex matching at ~0.1ms)
4. THE Router SHALL support dynamic segments via parameter extraction during trie traversal
5. THE Co-located loaders SHALL compile to WASM functions referenced by loader_fn index

### Requirement 16: Binary Schema Form Actions

**User Story:** As a developer, I want form submissions to be pre-validated binary instead of multipart parsing, so that I get 10x faster form processing.

#### Acceptance Criteria

1. THE BinaryFormData struct SHALL contain schema_id (u16) and fields as (u8, Value) pairs
2. THE Client SHALL validate against schema before submission, eliminating server-side parsing
3. THE Server action SHALL receive pre-validated binary data requiring no FormData parsing
4. THE Form system SHALL support progressive enhancement with native POST fallback
5. THE Form submission processing SHALL complete in less than 1ms (10x faster than Remix's ~10ms)

### Requirement 17: XOR-Based Optimistic UI Rollback

**User Story:** As a developer, I want optimistic UI rollback to be XOR byte operations instead of object cloning, so that I get 50x faster rollbacks.

#### Acceptance Criteria

1. THE Optimistic system SHALL snapshot state regions as raw bytes before mutation
2. WHEN rolling back, THE Runtime SHALL use SIMD-accelerated XOR to restore original state
3. THE `#[optimistic]` attribute SHALL generate snapshot, mutation, server call, and rollback code
4. THE Rollback operation SHALL complete in less than 0.01ms (50x faster than TanStack's ~0.5ms object cloning)
5. THE System SHALL NOT allocate memory for rollback (zero GC pressure)

### Requirement 18: Pre-Compiled View Transitions

**User Story:** As a developer, I want view transitions to be pre-compiled binary descriptors instead of runtime configuration, so that I get 10x faster transition setup.

#### Acceptance Criteria

1. THE TransitionConfig struct SHALL contain from_route (u16), to_route (u16), transition_type (u8), duration_ms (u16), and morph pairs
2. THE Transition configs SHALL be stored in router.dxb at build time
3. THE FLIP-based morphing SHALL use the same binary animation system for element transitions
4. THE Transition setup SHALL require zero runtime configuration (pre-compiled)

### Requirement 19: Memory-Mapped Content Collections

**User Story:** As a developer, I want content to be pre-parsed binary AST instead of runtime markdown parsing, so that I get 500x faster content loading.

#### Acceptance Criteria

1. THE BinaryContent struct SHALL contain metadata as BinarySchema and content_ast as pre-parsed bytes
2. THE Build system SHALL parse markdown to binary AST at build time
3. THE Content query SHALL memory-map binary files without runtime parsing
4. THE Content load time SHALL be less than 0.01ms per file (500x faster than Astro's ~5ms markdown parsing)

### Requirement 20: Binary LiveView Patches

**User Story:** As a developer, I want LiveView diffs to be binary patches instead of HTML diffs, so that I get 6x smaller update payloads.

#### Acceptance Criteria

1. THE BinaryPatch struct SHALL contain target (u16), op (PatchOp enum), and value bytes
2. THE WebSocket message for an increment SHALL be approximately 8 bytes total
3. THE Client SHALL apply patches directly without HTML parsing or morphdom
4. THE Patch payload SHALL be 6x smaller than Phoenix LiveView's ~50 byte HTML diffs

### Requirement 21: Schema-Driven Admin Panel Generation

**User Story:** As a developer, I want admin panels auto-generated from database schema as binary templates, so that I get 10x smaller admin bundles.

#### Acceptance Criteria

1. THE `#[derive(Admin)]` macro SHALL generate list_template, edit_template, and API routes from schema
2. THE AdminConfig SHALL contain ModelAdmin entries with template IDs and column definitions
3. THE Generated admin bundle SHALL be approximately 50KB (10x smaller than Django's ~500KB)
4. THE Admin system SHALL use the same binary template and HTIP infrastructure

### Requirement 22: Binary Ring Buffer Background Jobs

**User Story:** As a developer, I want background jobs to be binary packets in a ring buffer instead of JSON in Redis, so that I get 60x smaller job overhead.

#### Acceptance Criteria

1. THE Job struct SHALL contain job_type (u16), priority (u8), payload bytes, retry_count (u8), and scheduled_at (u64)
2. THE Job queue SHALL be a ring buffer in shared memory instead of Redis
3. THE Job serialization SHALL be approximately 16 bytes (60x smaller than Sidekiq's ~1KB JSON)
4. THE Worker pool SHALL use WASM instances processing ~10,000 jobs/second per worker

### Requirement 23: Pre-Computed Cron Scheduling

**User Story:** As a developer, I want cron jobs to use pre-computed next-run timestamps instead of expression parsing, so that I get 100x faster schedule checks.

#### Acceptance Criteria

1. THE CronJob struct SHALL contain id (u16), next_run (AtomicU64), handler function pointer, and interval_type (u8)
2. THE `#[cron("...")]` attribute SHALL compile the expression to interval_type and next-run calculator
3. THE Schedule check SHALL be a timestamp comparison instead of cron expression parsing
4. THE Check operation SHALL complete in less than 0.001ms (100x faster than NestJS's ~0.1ms parsing)

### Requirement 24: Compile-Time Inlined Guards

**User Story:** As a developer, I want route guards to be inlined at compile time instead of runtime reflection, so that I get 100x faster guard checks.

#### Acceptance Criteria

1. THE `#[guard(auth)]` and `#[guard(role("admin"))]` attributes SHALL inline guard logic into the handler function
2. THE RouteHandler struct SHALL NOT contain runtime guard metadata
3. THE Guard check SHALL be direct function calls instead of reflection-based lookup
4. THE Guard execution SHALL complete in less than 0.001ms (100x faster than NestJS's ~0.1ms reflection)

### Requirement 25: Compile-Time End-to-End Type Safety

**User Story:** As a developer, I want types to be memory layouts shared between server and client instead of runtime Zod validation, so that I get zero-cost type safety.

#### Acceptance Criteria

1. THE `#[derive(BinarySchema)]` macro SHALL generate types usable by both server Rust and client WASM
2. THE Wire format SHALL equal memory layout (no JSON schema, no codegen step)
3. THE Type validation SHALL occur at compile time only (zero runtime cost)
4. THE Shared types SHALL be importable from a common crate by both server and client

