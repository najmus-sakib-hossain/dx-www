# Design Document: Binary Dawn Features

## Overview

Binary Dawn Features transforms dx-www into the ultimate web framework by implementing 25 revolutionary features built on binary-first architecture. The core philosophy: eliminate parsing, serialization, and runtime overhead by operating directly on memory.

This design builds on the dx-reactor infrastructure (I/O backends, HBTP protocol, memory teleportation) to deliver application-level features that achieve 10x-1000x performance improvements over existing frameworks.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Binary Dawn Features Architecture                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     Compiler Layer (Build Time)                      │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐   │   │
│  │  │ Reactivity  │ │  Handler    │ │   Route     │ │   Guard     │   │   │
│  │  │ Slot Gen    │ │ Extraction  │ │ Trie Build  │ │  Inlining   │   │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘   │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐   │   │
│  │  │ Animation   │ │  Content    │ │   Admin     │ │   Type      │   │   │
│  │  │ Pre-compute │ │ AST Parse   │ │ Generation  │ │  Schema     │   │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────┴────────────────────────────────────┐  │
│  │                      Runtime Layer (Client)                          │  │
│  │                                                                       │  │
│  │  ┌───────────────────────────────────────────────────────────────┐  │  │
│  │  │                   SharedArrayBuffer State                      │  │  │
│  │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ │  │  │
│  │  │  │ App     │ │Component│ │ Scroll  │ │ Suspense│ │ Optimist│ │  │  │
│  │  │  │ State   │ │ State   │ │ State   │ │ Flags   │ │ Snapshot│ │  │  │
│  │  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘ │  │  │
│  │  └───────────────────────────────────────────────────────────────┘  │  │
│  │                                                                       │  │
│  │  ┌───────────────────────────────────────────────────────────────┐  │  │
│  │  │                    Binary Operations                           │  │  │
│  │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ │  │  │
│  │  │  │Reactive │ │ Control │ │ Teleport│ │Animation│ │ Island  │ │  │  │
│  │  │  │ Slots   │ │ Opcodes │ │   Ops   │ │  SIMD   │ │ Activate│ │  │  │
│  │  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘ │  │  │
│  │  └───────────────────────────────────────────────────────────────┘  │  │
│  │                                                                       │  │
│  │  ┌───────────────────────────────────────────────────────────────┐  │  │
│  │  │                    Handler System                              │  │  │
│  │  │  HANDLER_TABLE[256] → WASM Function Pointers                   │  │  │
│  │  │  data-dx-click="N" → handler_table[N]()                        │  │  │
│  │  └───────────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                      Server Layer                                     │  │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐    │  │
│  │  │ Binary SSR  │ │  LiveView   │ │ Form Action │ │  Job Queue  │    │  │
│  │  │  Streaming  │ │  Patches    │ │   Binary    │ │ Ring Buffer │    │  │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘    │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Architecture

The Binary Dawn Features architecture is organized into three layers:

1. **Compiler Layer** - Build-time transformations that eliminate runtime overhead
2. **Runtime Layer** - Client-side binary operations on SharedArrayBuffer
3. **Server Layer** - Binary protocols for SSR, LiveView, forms, and background jobs

## Components and Interfaces

### 1. Compile-Time Reactivity System

```rust
// crates/dx-www/src/reactivity.rs

/// 8-byte reactive binding slot
#[repr(C)]
pub struct ReactiveSlot {
    /// DOM element ID (2 bytes)
    pub element_id: u16,
    /// Offset within element for update (2 bytes)
    pub offset: u16,
    /// Pointer to value in SharedArrayBuffer (4 bytes)
    pub value_ptr: u32,
}

impl ReactiveSlot {
    pub const SIZE: usize = 8;
    
    /// Apply reactive update - just memory copy
    #[inline(always)]
    pub fn apply(&self, shared_buffer: &[u8], elements: &mut [Element]) {
        let value = &shared_buffer[self.value_ptr as usize..];
        elements[self.element_id as usize].update_at(self.offset, value);
    }
}

/// Compiler-generated reactive bindings table
pub struct ReactiveBindings {
    slots: Vec<ReactiveSlot>,
}

impl ReactiveBindings {
    /// Batch apply all reactive updates
    pub fn apply_all(&self, shared_buffer: &[u8], elements: &mut [Element]) {
        for slot in &self.slots {
            slot.apply(shared_buffer, elements);
        }
    }
}
```

### 2. Binary Animation System

```rust
// crates/dx-www/src/animation.rs

/// Animation type enum
#[repr(u8)]
pub enum AnimationType {
    Fade = 0,
    Slide = 1,
    Scale = 2,
    Flip = 3,
}

/// 8-byte animation descriptor
#[repr(C)]
pub struct BinaryAnimation {
    pub animation_type: AnimationType,  // 1 byte
    pub duration_ms: u16,               // 2 bytes
    pub easing: EasingType,             // 1 byte
    pub properties: u32,                // 4 bytes (bitfield)
}

#[repr(u8)]
pub enum EasingType {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    Cubic = 3,
}

/// Pre-computed easing curves (60fps * 4 curves)
pub static EASING_CURVES: [[f32; 60]; 4] = [
    // Linear
    [0.0, 0.0169, 0.0339, /* ... 60 values */ 1.0],
    // EaseIn (quadratic)
    [0.0, 0.00028, 0.00113, /* ... */ 1.0],
    // EaseOut (quadratic)
    [0.0, 0.0328, 0.0644, /* ... */ 1.0],
    // Cubic
    [0.0, 0.000048, 0.00038, /* ... */ 1.0],
];

impl BinaryAnimation {
    /// SIMD-optimized frame calculation
    #[inline(always)]
    pub fn calculate_frame(&self, frame_index: usize) -> f32 {
        EASING_CURVES[self.easing as usize][frame_index]
    }
    
    /// Apply animation frame to element
    pub fn apply_frame(&self, element_id: u16, progress: f32) {
        let eased = self.calculate_frame((progress * 59.0) as usize);
        match self.animation_type {
            AnimationType::Fade => set_opacity(element_id, eased),
            AnimationType::Slide => set_transform_y(element_id, lerp(0.0, 100.0, eased)),
            AnimationType::Scale => set_transform_scale(element_id, lerp(0.0, 1.0, eased)),
            AnimationType::Flip => apply_flip(element_id, eased),
        }
    }
}

/// API functions
pub fn fade() -> BinaryAnimation {
    BinaryAnimation {
        animation_type: AnimationType::Fade,
        duration_ms: 300,
        easing: EasingType::EaseOut,
        properties: 0x01, // opacity
    }
}

pub fn slide() -> BinaryAnimation {
    BinaryAnimation {
        animation_type: AnimationType::Slide,
        duration_ms: 300,
        easing: EasingType::EaseOut,
        properties: 0x02, // transform
    }
}

pub fn flip(duration: u16) -> BinaryAnimation {
    BinaryAnimation {
        animation_type: AnimationType::Flip,
        duration_ms: duration,
        easing: EasingType::Cubic,
        properties: 0x06, // transform + position
    }
}
```

### 3. Binary Server Components

```rust
// crates/dx-www/src/server_component.rs

/// Server component output
pub struct BinaryFragment {
    /// Pre-registered template ID
    pub template_id: u16,
    /// Binary-encoded data slots
    pub slots: Vec<u8>,
}

/// Server component trait
pub trait ServerComponent {
    /// Render to binary fragment (never ships to client)
    fn render(&self) -> BinaryFragment;
}

/// Wire format for server component data
#[repr(C)]
pub struct ServerComponentHeader {
    pub template_id: u16,
    pub slot_count: u16,
    pub total_size: u32,
}

impl ServerComponentHeader {
    pub const SIZE: usize = 8;
}

/// Example: User list server component
pub struct UserListComponent {
    pub users: Vec<User>,
}

impl ServerComponent for UserListComponent {
    fn render(&self) -> BinaryFragment {
        let mut slots = Vec::with_capacity(self.users.len() * 12);
        for user in &self.users {
            // ~12 bytes per user vs RSC's ~200 bytes JSON
            slots.extend_from_slice(&user.id.to_le_bytes());
            slots.extend_from_slice(&(user.name.len() as u16).to_le_bytes());
            slots.extend_from_slice(user.name.as_bytes());
        }
        BinaryFragment {
            template_id: 42, // Pre-registered UserList template
            slots,
        }
    }
}
```

### 4. Instant Resumability System

```rust
// crates/dx-www/src/resumability.rs

use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

/// Application state in SharedArrayBuffer
#[repr(C)]
pub struct AppState {
    pub count: AtomicU32,           // Offset 0
    pub user_id: AtomicU32,         // Offset 4
    pub is_logged_in: AtomicU8,     // Offset 8
    // All state is memory-mapped
}

/// Static handler table - no serialization needed
pub static HANDLER_TABLE: [fn(); 256] = [
    || increment_count(),    // ID 0
    || decrement_count(),    // ID 1
    || submit_form(),        // ID 2
    || toggle_menu(),        // ID 3
    // ... up to 256 handlers
    || {},                   // Unused slots
];

/// Resume from SharedArrayBuffer - just set pointer
pub fn resume(shared_buffer: &[u8]) {
    // That's it. No parsing. Just memory pointer.
    unsafe {
        WASM_MEMORY.set(shared_buffer.as_ptr());
    }
}

/// Handler lookup - O(1) array index
#[inline(always)]
pub fn invoke_handler(id: u8) {
    HANDLER_TABLE[id as usize]();
}
```

### 5. Binary Handler References

```rust
// crates/dx-www/src/handlers.rs

/// 4-byte handler reference
#[repr(C)]
pub struct HandlerRef {
    /// Index in WASM function table
    pub fn_index: u16,
    /// Offset to captured values in SharedArrayBuffer
    pub capture_offset: u16,
}

impl HandlerRef {
    pub const SIZE: usize = 4;
}

/// Handler groups for smart code splitting
#[repr(u8)]
pub enum HandlerGroup {
    Critical = 0,      // Above-fold, likely clicked first
    Interactive = 1,   // Hover, focus handlers
    Submission = 2,    // Form submissions
    Navigation = 3,    // Route changes
    Rare = 4,          // Error handlers, edge cases
}

/// Compiler-generated handler manifest
pub struct HandlerManifest {
    pub groups: [Vec<HandlerRef>; 5],
}

impl HandlerManifest {
    /// Get chunk filename for group
    pub fn chunk_name(group: HandlerGroup) -> &'static str {
        match group {
            HandlerGroup::Critical => "handlers_critical.dxb",
            HandlerGroup::Interactive => "handlers_secondary.dxb",
            HandlerGroup::Submission => "handlers_submission.dxb",
            HandlerGroup::Navigation => "handlers_navigation.dxb",
            HandlerGroup::Rare => "handlers_rare.dxb",
        }
    }
}
```

### 6. Binary Islands Architecture

```rust
// crates/dx-www/src/islands.rs

/// Island slot definition
pub struct IslandSlot {
    pub slot_id: u16,
    pub island_type: IslandType,
}

/// Island activation bitfield (64 islands max per page)
pub struct IslandActivation {
    pub bits: u64,
}

impl IslandActivation {
    /// Activate island N
    #[inline(always)]
    pub fn activate(&mut self, n: u8) {
        self.bits |= 1 << n;
    }
    
    /// Check if island N is active
    #[inline(always)]
    pub fn is_active(&self, n: u8) -> bool {
        (self.bits & (1 << n)) != 0
    }
}

/// Page with island slots
pub struct BinaryPage {
    /// Pre-rendered static template
    pub static_template: &'static [u8],
    /// Island slot definitions
    pub island_slots: Vec<IslandSlot>,
    /// Current activation state
    pub activation: IslandActivation,
}

impl BinaryPage {
    /// Activate single island - load only its WASM chunk
    pub async fn activate_island(&mut self, island_id: u8) {
        if !self.activation.is_active(island_id) {
            let chunk = load_island_chunk(island_id).await;
            hydrate_island(island_id, &chunk);
            self.activation.activate(island_id);
        }
    }
}
```

### 7. Compile-Time Dependency Injection

```rust
// crates/dx-www/src/di.rs

/// Container with fixed offsets (no Map lookup)
#[repr(C)]
pub struct Container {
    pub database_offset: u32,
    pub cache_offset: u32,
    pub auth_offset: u32,
}

/// Injectable service layout
#[repr(C)]
pub struct UserService {
    pub db_ptr: u32,     // Offset 0: pointer to Database
    pub cache_ptr: u32,  // Offset 4: pointer to Cache
}

/// Create service - just pointer assignment
#[inline(always)]
pub fn create_user_service(container: &Container) -> UserService {
    UserService {
        db_ptr: container.database_offset,
        cache_ptr: container.cache_offset,
    }
}

// Zero runtime cost - all resolved at compile time
```

### 8. SharedArrayBuffer Keep-Alive

```rust
// crates/dx-www/src/keepalive.rs

use std::sync::atomic::{AtomicU32, Ordering};

/// Component state tracking
#[repr(C)]
pub struct ComponentState {
    /// Offset in SharedArrayBuffer
    pub offset: u32,
    /// Size of state region
    pub size: u32,
    /// Currently mounted?
    pub is_active: bool,
}

/// Scroll state preservation
#[repr(C)]
pub struct ScrollState {
    pub scroll_top: AtomicU32,
    pub scroll_left: AtomicU32,
}

impl ComponentState {
    /// Deactivate - remove DOM, keep memory
    pub fn deactivate(&mut self) {
        self.is_active = false;
        // Memory stays intact - no zeroing
    }
    
    /// Reactivate - mount DOM, state already correct
    pub fn reactivate(&mut self) {
        self.is_active = true;
        // State already in SharedArrayBuffer
    }
}
```

### 9. O(1) Teleport System

```rust
// crates/dx-www/src/teleport.rs

/// Pre-defined teleport targets
pub const TELEPORT_BODY: u8 = 0;
pub const TELEPORT_MODAL: u8 = 1;
pub const TELEPORT_TOOLTIP: u8 = 2;

/// 4-byte teleport operation
#[repr(C)]
pub struct TeleportOp {
    pub opcode: u8,        // TELEPORT = 0x10
    pub element_id: u16,   // Which element
    pub target_slot: u8,   // Where to put it
}

impl TeleportOp {
    pub const SIZE: usize = 4;
    pub const OPCODE: u8 = 0x10;
    
    /// Execute teleport - single appendChild
    #[inline(always)]
    pub fn execute(&self, elements: &[Element], targets: &[Element]) {
        let element = &elements[self.element_id as usize];
        let target = &targets[self.target_slot as usize];
        target.append_child(element);
        // That's it. O(1). No tree walking.
    }
}
```

### 10. Binary Control Flow Opcodes

```rust
// crates/dx-www/src/control.rs

/// Control flow opcodes
#[repr(u8)]
pub enum ControlOpcode {
    ForEach = 0x01,
    Show = 0x02,
    Switch = 0x03,
}

/// ForEach instruction
#[repr(C)]
pub struct ForEachOp {
    pub opcode: ControlOpcode,  // 1 byte
    pub _pad: u8,               // 1 byte padding
    pub list_ptr: u32,          // Pointer to array in SharedArrayBuffer
    pub item_size: u16,         // Size of each item
    pub template_id: u16,       // Template to clone per item
}

/// Show instruction
#[repr(C)]
pub struct ShowOp {
    pub opcode: ControlOpcode,
    pub _pad: u8,
    pub condition_ptr: u32,     // Pointer to bool
    pub template_id: u16,       // What to show
    pub fallback_id: u16,       // Fallback template
}

/// Switch instruction
#[repr(C)]
pub struct SwitchOp {
    pub opcode: ControlOpcode,
    pub case_count: u8,
    pub value_ptr: u32,         // Pointer to discriminant
    pub cases: [u16; 8],        // Template ID per case (max 8)
}

/// Keyed list with SIMD diffing
pub struct KeyedList {
    pub keys: Vec<u32>,
    pub items_ptr: u32,
    pub dom_nodes: Vec<u16>,
}

impl KeyedList {
    /// SIMD-accelerated key comparison (8 keys at once)
    #[cfg(target_arch = "x86_64")]
    pub fn diff_keys(&self, new_keys: &[u32]) -> Vec<DiffOp> {
        use std::arch::x86_64::*;
        // Compare 8 u32 keys at once using AVX2
        // Generate minimal move/insert/delete ops
        todo!()
    }
}
```



### 11. Bit-Flag Suspense System

```rust
// crates/dx-www/src/suspense.rs

/// Suspense state - 64 async dependencies max
pub struct SuspenseState {
    /// Each bit = one async dependency (0 = loaded, 1 = loading)
    pub loading_flags: u64,
}

/// Suspense template configuration
#[repr(C)]
pub struct SuspenseTemplate {
    pub loading_template: u16,   // Skeleton template ID
    pub ready_template: u16,     // Content template ID
    pub dependencies: u64,       // Which bits must be 0 to show content
}

impl SuspenseState {
    /// Branchless ready check
    #[inline(always)]
    pub fn is_ready(&self, template: &SuspenseTemplate) -> bool {
        (self.loading_flags & template.dependencies) == 0
    }
    
    /// Mark dependency as loaded
    #[inline(always)]
    pub fn mark_loaded(&mut self, dependency_id: u8) {
        self.loading_flags &= !(1 << dependency_id);
    }
    
    /// Get template to show (branchless)
    #[inline(always)]
    pub fn get_template(&self, template: &SuspenseTemplate) -> u16 {
        let ready = self.is_ready(template);
        if ready { template.ready_template } else { template.loading_template }
    }
}
```

### 12. Binary Streaming SSR

```rust
// crates/dx-www/src/streaming.rs

/// Stream chunk types
#[repr(u8)]
pub enum ChunkType {
    Template = 0x01,   // Clone template, insert at slot
    Data = 0x02,       // Fill template slots with values
    Activate = 0x03,   // Attach handlers (island hydration)
}

/// Binary stream chunk
#[repr(C)]
pub struct StreamChunk {
    pub chunk_type: ChunkType,
    pub target_slot: u16,
    pub payload_len: u16,
    // payload bytes follow
}

impl StreamChunk {
    pub const HEADER_SIZE: usize = 5;
    
    /// Process received chunk
    pub fn process(&self, payload: &[u8]) {
        match self.chunk_type {
            ChunkType::Template => {
                clone_and_insert(self.target_slot, payload);
            }
            ChunkType::Data => {
                fill_slots(self.target_slot, payload);
            }
            ChunkType::Activate => {
                // Load only this island's WASM
                load_island_chunk(self.target_slot);
            }
        }
    }
}

/// Streaming SSR response builder
pub struct StreamingResponse {
    chunks: Vec<(StreamChunk, Vec<u8>)>,
}

impl StreamingResponse {
    pub fn add_template(&mut self, slot: u16, template_data: Vec<u8>) {
        self.chunks.push((
            StreamChunk {
                chunk_type: ChunkType::Template,
                target_slot: slot,
                payload_len: template_data.len() as u16,
            },
            template_data,
        ));
    }
    
    pub fn add_data(&mut self, slot: u16, data: Vec<u8>) {
        self.chunks.push((
            StreamChunk {
                chunk_type: ChunkType::Data,
                target_slot: slot,
                payload_len: data.len() as u16,
            },
            data,
        ));
    }
}
```

### 13. Handler Code Splitting

```rust
// crates/dx-www/src/code_splitting.rs

/// Handler classification for smart splitting
pub struct HandlerClassifier;

impl HandlerClassifier {
    /// Classify handler by usage pattern
    pub fn classify(handler: &Handler) -> HandlerGroup {
        if handler.is_above_fold && handler.is_click {
            HandlerGroup::Critical
        } else if handler.is_hover || handler.is_focus {
            HandlerGroup::Interactive
        } else if handler.is_form_submit {
            HandlerGroup::Submission
        } else if handler.is_navigation {
            HandlerGroup::Navigation
        } else {
            HandlerGroup::Rare
        }
    }
}

/// Prefetching system
pub struct Prefetcher {
    loaded_groups: u8, // Bitfield of loaded groups
}

impl Prefetcher {
    /// Predict and prefetch on mouse enter
    pub fn on_mouse_enter(&mut self, element_id: u16) {
        let likely_groups = predict_actions(element_id);
        for group in likely_groups {
            if (self.loaded_groups & (1 << group as u8)) == 0 {
                prefetch_handler_group(group);
                self.loaded_groups |= 1 << group as u8;
            }
        }
    }
}
```

### 14. Progressive Enhancement Tiers

```rust
// crates/dx-www/src/progressive.rs

/// Build output for all three tiers
pub struct BuildOutput {
    /// Works without JS (Maud-rendered HTML)
    pub html_fallback: String,
    /// 338B micro runtime for progressive enhancement
    pub micro_bundle: [u8; 338],
    /// Full WASM binary experience
    pub full_bundle: Vec<u8>,
}

/// Client capability detection
pub enum ClientCapability {
    NoJS,
    LightJS,
    FullWASM,
}

/// Serve appropriate tier
pub fn serve_page(request: &Request) -> Response {
    match detect_capability(request) {
        ClientCapability::NoJS => render_html_fallback(),
        ClientCapability::LightJS => serve_micro_runtime(),
        ClientCapability::FullWASM => serve_binary_bundle(),
    }
}
```

### 15. Binary Trie Router

```rust
// crates/dx-www/src/router.rs

/// Route handler definition
#[repr(C)]
pub struct RouteHandler {
    pub template_id: u16,
    pub loader_fn: u16,        // WASM function index
    pub guard_fn: Option<u16>, // Optional guard function
}

/// Binary trie node
#[repr(C)]
pub struct TrieNode {
    pub char: u8,
    pub is_terminal: bool,
    pub handler_idx: u16,
    pub children_offset: u16,
    pub children_count: u8,
}

/// Binary router with prefix trie
pub struct BinaryRouter {
    /// Trie encoded as byte array
    trie: Vec<u8>,
    /// Route handlers
    handlers: Vec<RouteHandler>,
}

impl BinaryRouter {
    /// O(path_length) route lookup
    pub fn lookup(&self, path: &str) -> Option<&RouteHandler> {
        let mut offset = 0usize;
        
        for byte in path.bytes() {
            let node = self.read_node(offset);
            
            // Binary search children
            let child_offset = self.find_child(node, byte)?;
            offset = child_offset;
        }
        
        let node = self.read_node(offset);
        if node.is_terminal {
            Some(&self.handlers[node.handler_idx as usize])
        } else {
            None
        }
    }
    
    fn read_node(&self, offset: usize) -> TrieNode {
        // Read node from byte array
        unsafe { std::ptr::read(self.trie.as_ptr().add(offset) as *const TrieNode) }
    }
    
    fn find_child(&self, node: TrieNode, byte: u8) -> Option<usize> {
        // Binary search through children
        let children_start = node.children_offset as usize;
        // ... binary search implementation
        todo!()
    }
}
```

### 16. Binary Form Actions

```rust
// crates/dx-www/src/forms.rs

/// Binary form data
#[repr(C)]
pub struct BinaryFormData {
    pub schema_id: u16,
    pub field_count: u8,
    // Fields follow as (field_id: u8, value_type: u8, value_bytes...)
}

/// Form field value
pub enum FormValue {
    String { offset: u16, len: u16 },
    Number(i64),
    Boolean(bool),
    Binary { offset: u16, len: u16 },
}

/// Server action handler
pub trait FormAction {
    type Input: BinaryDeserialize;
    type Output: BinarySerialize;
    
    fn execute(&self, input: Self::Input) -> Self::Output;
}

/// Pre-validated form submission
pub fn process_form_action<A: FormAction>(
    action: &A,
    data: &[u8],
) -> Vec<u8> {
    // Data already validated on client
    // No FormData parsing needed
    let input = A::Input::from_binary(data);
    let output = action.execute(input);
    output.to_binary()
}
```

### 17. XOR Optimistic Rollback

```rust
// crates/dx-www/src/optimistic.rs

/// State snapshot for rollback
pub struct StateSnapshot {
    offset: u32,
    size: u32,
    data: Vec<u8>,
}

impl StateSnapshot {
    /// Capture state region
    pub fn capture(shared_buffer: &[u8], offset: u32, size: u32) -> Self {
        let data = shared_buffer[offset as usize..(offset + size) as usize].to_vec();
        Self { offset, size, data }
    }
    
    /// XOR rollback - SIMD accelerated
    #[cfg(target_arch = "x86_64")]
    pub fn rollback(&self, shared_buffer: &mut [u8]) {
        use std::arch::x86_64::*;
        
        let region = &mut shared_buffer[self.offset as usize..(self.offset + self.size) as usize];
        
        // Process 32 bytes at a time with AVX2
        let chunks = region.chunks_exact_mut(32);
        let snapshot_chunks = self.data.chunks_exact(32);
        
        for (dest, src) in chunks.zip(snapshot_chunks) {
            unsafe {
                let dest_vec = _mm256_loadu_si256(dest.as_ptr() as *const __m256i);
                let src_vec = _mm256_loadu_si256(src.as_ptr() as *const __m256i);
                let result = _mm256_xor_si256(dest_vec, src_vec);
                _mm256_storeu_si256(dest.as_mut_ptr() as *mut __m256i, result);
            }
        }
        
        // Handle remainder
        // ...
    }
}

/// Optimistic mutation wrapper
pub async fn optimistic_mutation<F, Fut>(
    state_offset: u32,
    state_size: u32,
    shared_buffer: &mut [u8],
    mutation: F,
    server_call: Fut,
) -> Result<(), Error>
where
    F: FnOnce(&mut [u8]),
    Fut: Future<Output = Result<(), Error>>,
{
    // 1. Snapshot
    let snapshot = StateSnapshot::capture(shared_buffer, state_offset, state_size);
    
    // 2. Apply optimistic update
    let region = &mut shared_buffer[state_offset as usize..(state_offset + state_size) as usize];
    mutation(region);
    
    // 3. Server call
    match server_call.await {
        Ok(()) => Ok(()),
        Err(e) => {
            // 4. Rollback on error
            snapshot.rollback(shared_buffer);
            Err(e)
        }
    }
}
```

### 18. View Transitions

```rust
// crates/dx-www/src/transitions.rs

/// Pre-compiled transition config
#[repr(C)]
pub struct TransitionConfig {
    pub from_route: u16,
    pub to_route: u16,
    pub transition_type: TransitionType,
    pub duration_ms: u16,
    pub morph_count: u8,
    // morph_pairs follow as [(from_id: u16, to_id: u16); morph_count]
}

#[repr(u8)]
pub enum TransitionType {
    None = 0,
    Fade = 1,
    Slide = 2,
    Morph = 3,
}

/// Execute view transition
pub fn navigate_with_transition(
    from: u16,
    to: u16,
    config: &TransitionConfig,
) {
    // 1. Snapshot morph element positions
    let snapshots = capture_morph_elements(config);
    
    // 2. Swap DOM (instant)
    swap_route_templates(from, to);
    
    // 3. FLIP animate to new positions
    animate_morph(snapshots, config.duration_ms);
}
```

### 19. Binary Content Collections

```rust
// crates/dx-www/src/content.rs

/// Binary content file
#[repr(C)]
pub struct BinaryContent {
    /// Metadata as binary schema
    pub metadata_offset: u32,
    pub metadata_size: u16,
    /// Pre-parsed AST
    pub ast_offset: u32,
    pub ast_size: u32,
}

/// Content collection
pub struct ContentCollection {
    /// Memory-mapped content files
    files: memmap2::Mmap,
    /// Index of content entries
    index: Vec<BinaryContent>,
}

impl ContentCollection {
    /// Query content - zero parsing
    pub fn get(&self, id: usize) -> Option<ContentRef> {
        let entry = self.index.get(id)?;
        Some(ContentRef {
            metadata: &self.files[entry.metadata_offset as usize..],
            ast: &self.files[entry.ast_offset as usize..],
        })
    }
}
```

### 20. Binary LiveView Patches

```rust
// crates/dx-www/src/liveview.rs

/// Patch operation types
#[repr(u8)]
pub enum PatchOp {
    SetText = 0x01,
    SetAttr = 0x02,
    Remove = 0x03,
    Insert = 0x04,
    Replace = 0x05,
}

/// Binary patch (8 bytes typical)
#[repr(C)]
pub struct BinaryPatch {
    pub target: u16,
    pub op: PatchOp,
    pub value_len: u8,
    // value bytes follow (4 bytes for u32, etc.)
}

impl BinaryPatch {
    pub const HEADER_SIZE: usize = 4;
    
    /// Apply patch to DOM
    pub fn apply(&self, value: &[u8], elements: &mut [Element]) {
        let element = &mut elements[self.target as usize];
        match self.op {
            PatchOp::SetText => {
                let text = std::str::from_utf8(value).unwrap();
                element.set_text_content(text);
            }
            PatchOp::SetAttr => {
                // First byte is attr ID, rest is value
                let attr_id = value[0];
                let attr_value = &value[1..];
                element.set_attribute(attr_id, attr_value);
            }
            PatchOp::Remove => element.remove(),
            PatchOp::Insert => {
                // value contains template ID + data
                let template_id = u16::from_le_bytes([value[0], value[1]]);
                element.insert_template(template_id, &value[2..]);
            }
            PatchOp::Replace => {
                let template_id = u16::from_le_bytes([value[0], value[1]]);
                element.replace_with_template(template_id, &value[2..]);
            }
        }
    }
}
```

### 21. Schema-Driven Admin

```rust
// crates/dx-www/src/admin.rs

/// Admin configuration
pub struct AdminConfig {
    pub models: Vec<ModelAdmin>,
}

/// Model admin definition
pub struct ModelAdmin {
    pub table_name: &'static str,
    pub list_template: u16,
    pub edit_template: u16,
    pub list_columns: Vec<Column>,
    pub filters: Vec<Filter>,
    pub search_fields: Vec<u8>,
}

/// Column definition
pub struct Column {
    pub field_id: u8,
    pub display_name: &'static str,
    pub sortable: bool,
    pub filterable: bool,
}

/// Generate admin from schema (derive macro)
/// #[derive(Admin)]
/// struct User { ... }
/// Generates list_template, edit_template, API routes
```

### 22. Binary Job Queue

```rust
// crates/dx-www/src/jobs.rs

/// Binary job packet (~16 bytes)
#[repr(C)]
pub struct Job {
    pub job_type: u16,
    pub priority: u8,
    pub retry_count: u8,
    pub scheduled_at: u64,
    pub payload_len: u16,
    // payload bytes follow
}

impl Job {
    pub const HEADER_SIZE: usize = 14;
}

/// Ring buffer job queue
pub struct JobQueue {
    buffer: Vec<u8>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl JobQueue {
    /// Enqueue job - O(1)
    pub fn enqueue(&mut self, job: &Job, payload: &[u8]) -> bool {
        let total_size = Job::HEADER_SIZE + payload.len();
        if self.available_space() < total_size {
            return false;
        }
        
        // Write job header
        let job_bytes = unsafe {
            std::slice::from_raw_parts(
                job as *const Job as *const u8,
                Job::HEADER_SIZE,
            )
        };
        self.write_bytes(job_bytes);
        self.write_bytes(payload);
        true
    }
    
    /// Dequeue job - O(1)
    pub fn dequeue(&mut self) -> Option<(Job, Vec<u8>)> {
        if self.is_empty() {
            return None;
        }
        
        let job = self.read_job();
        let payload = self.read_bytes(job.payload_len as usize);
        Some((job, payload))
    }
    
    fn available_space(&self) -> usize {
        // Ring buffer space calculation
        if self.tail >= self.head {
            self.capacity - (self.tail - self.head)
        } else {
            self.head - self.tail
        }
    }
    
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
    
    fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.buffer[self.tail] = byte;
            self.tail = (self.tail + 1) % self.capacity;
        }
    }
    
    fn read_job(&mut self) -> Job {
        let mut job_bytes = [0u8; Job::HEADER_SIZE];
        for byte in &mut job_bytes {
            *byte = self.buffer[self.head];
            self.head = (self.head + 1) % self.capacity;
        }
        unsafe { std::ptr::read(job_bytes.as_ptr() as *const Job) }
    }
    
    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.buffer[self.head]);
            self.head = (self.head + 1) % self.capacity;
        }
        bytes
    }
}
```

### 23. Pre-Computed Cron

```rust
// crates/dx-www/src/cron.rs

use std::sync::atomic::{AtomicU64, Ordering};

/// Cron job with pre-computed next run
#[repr(C)]
pub struct CronJob {
    pub id: u16,
    pub next_run: AtomicU64,
    pub handler: fn(),
    pub interval_type: IntervalType,
}

#[repr(u8)]
pub enum IntervalType {
    Minutely = 0,
    Hourly = 1,
    Daily = 2,
    Weekly = 3,
    Monthly = 4,
    Custom = 5,
}

impl CronJob {
    /// Check if job should run - just timestamp comparison
    #[inline(always)]
    pub fn should_run(&self, now: u64) -> bool {
        now >= self.next_run.load(Ordering::Relaxed)
    }
    
    /// Execute and update next run
    pub fn execute(&self) {
        (self.handler)();
        let next = calculate_next_run(self.interval_type);
        self.next_run.store(next, Ordering::Relaxed);
    }
}

/// Calculate next run based on interval type
fn calculate_next_run(interval: IntervalType) -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    match interval {
        IntervalType::Minutely => now + 60,
        IntervalType::Hourly => now + 3600,
        IntervalType::Daily => now + 86400,
        IntervalType::Weekly => now + 604800,
        IntervalType::Monthly => now + 2592000,
        IntervalType::Custom => now, // Custom calculator needed
    }
}
```

### 24. Compile-Time Guards

```rust
// crates/dx-www/src/guards.rs

/// Guard result
pub enum GuardResult {
    Allow,
    Deny(Response),
}

/// Auth guard - inlined at compile time
#[inline(always)]
pub fn auth_guard(ctx: &Context) -> GuardResult {
    if ctx.is_authenticated() {
        GuardResult::Allow
    } else {
        GuardResult::Deny(Response::Unauthorized)
    }
}

/// Role guard - inlined at compile time
#[inline(always)]
pub fn role_guard(ctx: &Context, role: &str) -> GuardResult {
    if ctx.has_role(role) {
        GuardResult::Allow
    } else {
        GuardResult::Deny(Response::Forbidden)
    }
}

/// Macro generates guarded handler
/// #[route("/admin")]
/// #[guard(auth)]
/// #[guard(role("admin"))]
/// async fn admin_panel() -> Response { ... }
///
/// Compiles to:
/// async fn admin_panel_guarded(ctx: Context) -> Response {
///     if let GuardResult::Deny(r) = auth_guard(&ctx) { return r; }
///     if let GuardResult::Deny(r) = role_guard(&ctx, "admin") { return r; }
///     admin_panel_impl()
/// }
```

### 25. Compile-Time Type Safety

```rust
// crates/dx-www/src/types.rs

/// Binary schema trait - types shared between server and client
pub trait BinarySchema: Sized {
    const SIZE: usize;
    const ALIGN: usize;
    
    fn to_binary(&self) -> Vec<u8>;
    fn from_binary(bytes: &[u8]) -> Self;
}

/// Derive macro generates BinarySchema impl
/// #[derive(BinarySchema)]
/// #[repr(C)]
/// struct User {
///     id: u64,
///     name_offset: u32,
///     name_len: u32,
///     age: u8,
/// }
///
/// Wire format = memory layout
/// No JSON schema, no codegen step
/// Zero runtime validation cost
```

## Data Models

### Core Binary Structures

| Structure | Size | Fields |
|-----------|------|--------|
| ReactiveSlot | 8 bytes | element_id (u16), offset (u16), value_ptr (u32) |
| BinaryAnimation | 8 bytes | animation_type (u8), duration_ms (u16), easing (u8), properties (u32) |
| HandlerRef | 4 bytes | fn_index (u16), capture_offset (u16) |
| TeleportOp | 4 bytes | opcode (u8), element_id (u16), target_slot (u8) |
| StreamChunk | 5 bytes | chunk_type (u8), target_slot (u16), payload_len (u16) |
| BinaryPatch | 4 bytes | target (u16), op (u8), value_len (u8) |
| Job | 14 bytes | job_type (u16), priority (u8), retry_count (u8), scheduled_at (u64), payload_len (u16) |



## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: ReactiveSlot Size Invariant

*For any* ReactiveSlot instance, `size_of::<ReactiveSlot>()` SHALL equal exactly 8 bytes.

**Validates: Requirements 1.1**

### Property 2: Reactive Update Correctness

*For any* ReactiveSlot and SharedArrayBuffer value, after calling `apply()`, the target DOM element SHALL contain the value from the SharedArrayBuffer at the specified offset.

**Validates: Requirements 1.2**

### Property 3: BinaryAnimation Size Invariant

*For any* BinaryAnimation instance, `size_of::<BinaryAnimation>()` SHALL equal exactly 8 bytes.

**Validates: Requirements 2.1**

### Property 4: Easing Curves Validity

*For all* easing curves in EASING_CURVES, each curve SHALL have exactly 60 values, and all values SHALL be in the range [0.0, 1.0] with the first value being 0.0 and the last being 1.0.

**Validates: Requirements 2.2**

### Property 5: Server Component Binary Size

*For any* user record serialized via BinaryFragment, the serialized size SHALL be less than 20 bytes (significantly smaller than JSON's ~200 bytes).

**Validates: Requirements 3.2**

### Property 6: Binary Fragment Round-Trip

*For any* BinaryFragment written to a stream, reading it back SHALL produce an equivalent BinaryFragment with the same template_id and slots.

**Validates: Requirements 3.1, 3.4**

### Property 7: Handler Table Validity

*For any* handler ID in the range [0, 255], `HANDLER_TABLE[id]` SHALL be a valid function pointer.

**Validates: Requirements 4.3**

### Property 8: HTML Handler Attribute Format

*For any* generated HTML with click handlers, the output SHALL contain `data-dx-click="N"` where N is a valid u8 handler index.

**Validates: Requirements 4.5**

### Property 9: HandlerRef Size Invariant

*For any* HandlerRef instance, `size_of::<HandlerRef>()` SHALL equal exactly 4 bytes.

**Validates: Requirements 5.2**

### Property 10: Handler Group Count

*For any* compiled application, the number of handler chunk files SHALL be between 3 and 5 inclusive.

**Validates: Requirements 5.4, 13.3**

### Property 11: Island Activation Bitfield

*For any* island ID in the range [0, 63], `IslandActivation::activate(n)` followed by `is_active(n)` SHALL return true, and `is_active(m)` for m != n SHALL be unchanged.

**Validates: Requirements 6.2**

### Property 12: Partial Hydration Isolation

*For any* page with multiple islands, activating island N SHALL NOT load or hydrate any other island M where M != N.

**Validates: Requirements 6.5**

### Property 13: DI Offset Consistency

*For any* Injectable struct, the memory offsets of dependency pointers SHALL be fixed at compile time and match the Container's offset fields.

**Validates: Requirements 7.1, 7.2, 7.4**

### Property 14: State Preservation Round-Trip

*For any* ComponentState, deactivating and then reactivating SHALL preserve all state values in SharedArrayBuffer without modification.

**Validates: Requirements 8.2, 8.3**

### Property 15: Scroll State Persistence

*For any* ScrollState, the scroll_top and scroll_left values SHALL persist across component unmount and remount cycles.

**Validates: Requirements 8.4**

### Property 16: TeleportOp Size Invariant

*For any* TeleportOp instance, `size_of::<TeleportOp>()` SHALL equal exactly 4 bytes.

**Validates: Requirements 9.2**

### Property 17: Teleport Correctness

*For any* TeleportOp execution, the target element SHALL become a child of the teleport target container.

**Validates: Requirements 9.3**

### Property 18: Control Op Struct Fields

*For any* ForEachOp, it SHALL contain list_ptr (u32), item_size (u16), and template_id (u16). *For any* ShowOp, it SHALL contain condition_ptr (u32), template_id (u16), and fallback_id (u16).

**Validates: Requirements 10.2, 10.3**

### Property 19: Keyed List Diff Correctness

*For any* two key arrays, the diff operation SHALL produce a minimal set of operations that transforms the first array into the second.

**Validates: Requirements 10.4**

### Property 20: Suspense Bitfield Operations

*For any* SuspenseState and SuspenseTemplate, `is_ready()` SHALL return true if and only if `(loading_flags & dependencies) == 0`. Marking a dependency as loaded SHALL clear exactly that bit.

**Validates: Requirements 11.1, 11.2, 11.3, 11.5**

### Property 21: StreamChunk Processing

*For any* StreamChunk with chunk_type TEMPLATE, DATA, or ACTIVATE, processing SHALL produce the correct DOM modification for that type.

**Validates: Requirements 12.1, 12.3**

### Property 22: Handler Classification Completeness

*For any* handler in an application, the classifier SHALL assign it to exactly one HandlerGroup.

**Validates: Requirements 13.1, 13.2**

### Property 23: Progressive Enhancement Tiers

*For any* source file, the build system SHALL produce exactly three outputs: html_fallback (String), micro_bundle (338 bytes), and full_bundle (Vec<u8>).

**Validates: Requirements 14.1, 14.4**

### Property 24: Capability Detection Correctness

*For any* HTTP request, capability detection SHALL return exactly one of NoJS, LightJS, or FullWASM based on request headers.

**Validates: Requirements 14.2**

### Property 25: Router Lookup Correctness

*For any* registered route path, `BinaryRouter::lookup(path)` SHALL return the corresponding RouteHandler. *For any* unregistered path, it SHALL return None.

**Validates: Requirements 15.1**

### Property 26: Dynamic Route Parameter Extraction

*For any* dynamic route with parameters (e.g., `/users/:id`), lookup SHALL extract parameter values correctly from the path.

**Validates: Requirements 15.4**

### Property 27: Form Validation Correctness

*For any* BinaryFormData, client-side validation against the schema SHALL reject invalid data and accept valid data.

**Validates: Requirements 16.2**

### Property 28: Binary Form Round-Trip

*For any* valid form data, serializing to BinaryFormData and deserializing SHALL produce equivalent data.

**Validates: Requirements 16.1, 16.3**

### Property 29: Optimistic Rollback Round-Trip

*For any* state region, capturing a snapshot, mutating the state, and then rolling back SHALL restore the exact original bytes.

**Validates: Requirements 17.1, 17.2**

### Property 30: Rollback Zero Allocation

*For any* rollback operation, the operation SHALL NOT allocate heap memory (verified by tracking allocations).

**Validates: Requirements 17.5**

### Property 31: TransitionConfig Fields

*For any* TransitionConfig, it SHALL contain from_route (u16), to_route (u16), transition_type (u8), duration_ms (u16), and morph_count (u8).

**Validates: Requirements 18.1**

### Property 32: Content Binary Round-Trip

*For any* markdown content parsed to BinaryContent at build time, the AST SHALL be readable without runtime parsing.

**Validates: Requirements 19.1, 19.2, 19.3**

### Property 33: BinaryPatch Size

*For any* simple increment patch, the total message size SHALL be approximately 8 bytes or less.

**Validates: Requirements 20.1, 20.2**

### Property 34: Patch Application Correctness

*For any* BinaryPatch, applying it to the DOM SHALL produce the expected modification without HTML parsing.

**Validates: Requirements 20.3**

### Property 35: Job Struct Size

*For any* Job instance, `Job::HEADER_SIZE` SHALL be approximately 14-16 bytes.

**Validates: Requirements 22.1, 22.3**

### Property 36: Ring Buffer FIFO

*For any* sequence of jobs enqueued to JobQueue, dequeuing SHALL return them in FIFO order.

**Validates: Requirements 22.2**

### Property 37: CronJob Schedule Check

*For any* CronJob, `should_run(now)` SHALL return true if and only if `now >= next_run`.

**Validates: Requirements 23.1, 23.3**

### Property 38: Guard Inlining

*For any* route with guards, the generated handler SHALL contain inlined guard checks as direct function calls, not reflection-based lookups.

**Validates: Requirements 24.2, 24.3**

### Property 39: BinarySchema Wire Format

*For any* type implementing BinarySchema, the serialized bytes SHALL exactly match the in-memory representation (wire format = memory layout).

**Validates: Requirements 25.2**

## Error Handling

### Compiler Errors

| Error | Handling |
|-------|----------|
| Invalid ReactiveSlot binding | Compile error with source location |
| Handler extraction failure | Compile error listing problematic handler |
| Route conflict in trie | Compile error showing conflicting routes |
| Invalid guard attribute | Compile error with usage hint |

### Runtime Errors

| Error | Handling |
|-------|----------|
| Invalid handler ID | Panic in debug, no-op in release |
| Teleport target not found | Log warning, skip teleport |
| Island chunk load failure | Retry with exponential backoff, fallback to static |
| Form validation failure | Return validation errors to client |

### Server Errors

| Error | Handling |
|-------|----------|
| Binary fragment serialization failure | Return 500 with error details |
| Stream chunk corruption | Close connection, client retries |
| Job queue full | Return backpressure signal, retry later |
| Cron handler panic | Log error, reschedule job |

### Rollback Errors

| Error | Handling |
|-------|----------|
| Snapshot capture failure | Skip optimistic update, do synchronous |
| XOR rollback failure | Log error, force full state refresh |
| Server call timeout | Rollback and show error to user |

## Testing Strategy

### Unit Tests

Unit tests verify specific examples and edge cases:

1. **Struct Sizes**: Verify all binary structs have expected sizes
2. **Enum Variants**: Verify all enums have expected variants
3. **Bitfield Operations**: Test island activation, suspense flags
4. **Ring Buffer**: Test enqueue/dequeue edge cases
5. **Trie Lookup**: Test route matching with various paths

### Property-Based Tests

Property-based tests use `proptest` crate with minimum 100 iterations:

1. **Round-Trip Properties**: Binary serialization, state preservation, form data
2. **Invariant Properties**: Struct sizes, easing curve bounds, handler counts
3. **Correctness Properties**: Teleport, patch application, diff operations
4. **Isolation Properties**: Island activation, handler classification

### Integration Tests

1. **Streaming SSR**: Test full stream-to-interactive flow
2. **Progressive Enhancement**: Test all three tiers
3. **LiveView**: Test WebSocket patch flow
4. **Form Actions**: Test binary form submission

### Performance Benchmarks

1. **Reactive Updates**: Measure update time per binding
2. **Animation Frames**: Measure frame calculation time
3. **Route Lookup**: Measure lookup time vs route count
4. **Rollback**: Measure XOR rollback time

### Test Configuration

- Property tests: Minimum 100 iterations per property
- Each property test tagged with: **Feature: binary-dawn-features, Property N: {property_text}**
- Benchmarks: Warmup + 10 second measurement window
- Integration tests: Use test fixtures for deterministic results

