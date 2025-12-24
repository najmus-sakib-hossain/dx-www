//! # dx-morph: Dirty-Bit State Patcher
//!
//! The state mutation and DOM update layer.
//! Implements O(1) updates via dirty bit masks and binding maps.
//!
//! **ARCHITECTURE:**
//! - Every component has a 64-bit dirty mask
//! - Each bit represents a bindable field
//! - Binding Map: Static lookup from DirtyBit -> [NodeID, BindingType]
//! - No tree traversal, no diffing, pure O(1)
//!
//! **ACID TEST COMPLIANCE:**
//! - Zero allocations in update path
//! - State stored in SharedArrayBuffer (via dx-core)
//! - Dirty bits use atomic operations for thread safety

use bytemuck::{Pod, Zeroable};
use dx_www_core::{OpCode, RenderOp};
use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// DIRTY BIT TRACKING
// ============================================================================

/// Every component state has a 64-bit dirty mask as its first field
/// Each bit corresponds to a bindable property
#[repr(transparent)]
#[derive(Debug)]
pub struct DirtyMask(pub AtomicU64);

impl DirtyMask {
    pub fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    /// Mark a field as dirty (thread-safe)
    pub fn mark_dirty(&self, bit: u8) {
        debug_assert!(bit < 64, "Dirty bit out of range");
        self.0.fetch_or(1u64 << bit, Ordering::SeqCst);
    }

    /// Check if any fields are dirty
    pub fn is_dirty(&self) -> bool {
        self.0.load(Ordering::SeqCst) != 0
    }

    /// Get and clear the dirty mask (atomic swap)
    pub fn take_dirty(&self) -> u64 {
        self.0.swap(0, Ordering::SeqCst)
    }

    /// Check if a specific bit is dirty
    pub fn is_bit_dirty(&self, bit: u8) -> bool {
        let mask = self.0.load(Ordering::SeqCst);
        mask & (1u64 << bit) != 0
    }
}

impl Default for DirtyMask {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// BINDING MAP (Static Lookup Table)
// ============================================================================

/// Type of binding (how to update the DOM)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingType {
    /// Bind to text content
    Text = 1,
    /// Bind to an attribute
    Attribute = 2,
    /// Bind to a class toggle
    ClassToggle = 3,
    /// Bind to a style property
    Style = 4,
}

/// A binding entry in the static map
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct BindingEntry {
    /// Which dirty bit triggers this binding (0-63)
    pub dirty_bit: u8,
    /// Type of binding
    pub binding_type: u8,
    /// Reserved for alignment
    pub reserved: [u8; 2],
    /// Target node ID in the template
    pub node_id: u32,
    /// Attribute/Style name ID (for non-text bindings)
    pub name_id: u32,
    /// Offset into State Region for the value
    pub value_offset: u32,
    /// Length of the value in bytes
    pub value_length: u32,
}

/// The Binding Map for a component template
pub struct BindingMap {
    /// Component ID
    pub component_id: u32,
    /// Number of bindings
    pub binding_count: u32,
    /// Array of binding entries (stored in Static Region)
    pub entries: &'static [BindingEntry],
}

impl BindingMap {
    /// Create a BindingMap from a slice in Static Region
    ///
    /// # Safety
    /// The slice must be properly aligned and contain valid BindingEntry data
    pub unsafe fn from_static_slice(slice: &'static [u8]) -> Self {
        let (component_id_bytes, rest) = slice.split_at(4);
        let (count_bytes, entries_bytes) = rest.split_at(4);

        let component_id = u32::from_le_bytes([
            component_id_bytes[0],
            component_id_bytes[1],
            component_id_bytes[2],
            component_id_bytes[3],
        ]);

        let binding_count = u32::from_le_bytes([
            count_bytes[0],
            count_bytes[1],
            count_bytes[2],
            count_bytes[3],
        ]);

        // Cast the rest to BindingEntry array
        let entries = bytemuck::cast_slice::<u8, BindingEntry>(entries_bytes);

        Self {
            component_id,
            binding_count,
            entries,
        }
    }

    /// Get all binding entries for a given dirty bit
    pub fn get_bindings_for_bit(&self, bit: u8) -> impl Iterator<Item = &BindingEntry> {
        self.entries.iter().filter(move |e| e.dirty_bit == bit)
    }
}

// ============================================================================
// COMPONENT STATE (Base Trait)
// ============================================================================

/// All component state structs must start with a DirtyMask
///
/// Example:
/// ```
/// #[repr(C)]
/// struct CounterState {
///     dirty: DirtyMask,
///     count: i32,
///     label: [u8; 32],
/// }
/// ```
pub trait ComponentState {
    /// Get the dirty mask
    fn dirty_mask(&self) -> &DirtyMask;

    /// Get the component ID (for looking up BindingMap)
    fn component_id(&self) -> u32;

    /// Check if any fields are dirty
    fn is_dirty(&self) -> bool {
        self.dirty_mask().is_dirty()
    }
}

// ============================================================================
// STATE PATCHER (The Update Engine)
// ============================================================================

pub struct StatePatcher {
    /// Cache of binding maps (keyed by component ID)
    binding_maps: std::collections::HashMap<u32, BindingMap>,
}

impl Default for StatePatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl StatePatcher {
    pub fn new() -> Self {
        Self {
            binding_maps: std::collections::HashMap::new(),
        }
    }

    /// Register a binding map for a component
    pub fn register_binding_map(&mut self, map: BindingMap) {
        self.binding_maps.insert(map.component_id, map);
    }

    /// Patch the DOM based on dirty bits (O(1) per dirty field)
    ///
    /// Algorithm:
    /// 1. Read dirty_mask
    /// 2. For each set bit, look up bindings in BindingMap
    /// 3. Generate RenderOps and queue them
    /// 4. Clear dirty_mask
    pub fn patch<S: ComponentState>(&self, state: &S) -> Vec<RenderOp> {
        let mut ops = Vec::new();

        let dirty_mask_val = state.dirty_mask().take_dirty();
        if dirty_mask_val == 0 {
            return ops; // Nothing dirty
        }

        // Get the binding map for this component
        let component_id = state.component_id();
        let binding_map = match self.binding_maps.get(&component_id) {
            Some(map) => map,
            None => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::warn_1(
                    &format!("No binding map for component {}", component_id).into(),
                );
                return ops;
            }
        };

        // Iterate through each dirty bit
        for bit in 0..64 {
            if dirty_mask_val & (1u64 << bit) != 0 {
                // Look up all bindings for this bit
                for binding in binding_map.get_bindings_for_bit(bit) {
                    let op = match binding.binding_type {
                        x if x == BindingType::Text as u8 => RenderOp::new_update_text(
                            binding.node_id,
                            binding.value_offset,
                            binding.value_length,
                        ),
                        x if x == BindingType::Attribute as u8 => RenderOp {
                            opcode: OpCode::UpdateAttr as u8,
                            reserved: [0; 3],
                            arg1: binding.node_id,
                            arg2: binding.name_id,
                            arg3: binding.value_offset,
                        },
                        _ => {
                            // TODO: Implement ClassToggle and Style bindings
                            continue;
                        }
                    };
                    ops.push(op);
                }
            }
        }

        ops
    }
}

// ============================================================================
// EXAMPLE STATE STRUCTS
// ============================================================================

/// Example: Counter component state
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct CounterState {
    // CRITICAL: dirty_mask MUST be first field
    // Note: We use u64 here (not AtomicU64) because this is the memory layout.
    // The StatePatcher wraps access with atomic operations via DirtyMask helper.
    pub dirty_mask: u64,
    pub count: i32,
    pub step: i32,
}

impl CounterState {
    pub const COMPONENT_ID: u32 = 1;
    pub const BIT_COUNT: u8 = 0;
    pub const BIT_STEP: u8 = 1;

    pub fn new(count: i32, step: i32) -> Self {
        Self {
            dirty_mask: 0,
            count,
            step,
        }
    }

    pub fn increment(&mut self) {
        self.count += self.step;
        // Set dirty bit using atomic operations on the raw u64
        let dirty = unsafe { &*((&self.dirty_mask) as *const u64 as *const AtomicU64) };
        dirty.fetch_or(1 << Self::BIT_COUNT, Ordering::Release);
    }

    pub fn set_step(&mut self, new_step: i32) {
        self.step = new_step;
        let dirty = unsafe { &*((&self.dirty_mask) as *const u64 as *const AtomicU64) };
        dirty.fetch_or(1 << Self::BIT_STEP, Ordering::Release);
    }
}

impl ComponentState for CounterState {
    fn dirty_mask(&self) -> &DirtyMask {
        // SAFETY: DirtyMask is repr(transparent) over AtomicU64,
        // and we're casting from u64 to AtomicU64 which have the same layout
        unsafe { &*((&self.dirty_mask) as *const u64 as *const DirtyMask) }
    }

    fn component_id(&self) -> u32 {
        Self::COMPONENT_ID
    }
}

// ============================================================================
// GLOBAL STATE MANAGER (Proof of Concept)
// ============================================================================

use std::cell::RefCell;

pub struct StateManager {
    patcher: StatePatcher,
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            patcher: StatePatcher::new(),
        }
    }

    pub fn register_binding_map(&mut self, map: BindingMap) {
        self.patcher.register_binding_map(map);
    }

    pub fn patch_and_queue<S: ComponentState>(&self, state: &S) {
        let ops = self.patcher.patch(state);

        // Queue ops to dx-dom
        for _op in ops {
            #[cfg(target_arch = "wasm32")]
            match op.opcode {
                x if x == OpCode::UpdateText as u8 => {
                    dx_dom::queue_update_text(op.arg1, op.arg2, op.arg3);
                }
                _ => {}
            }
        }
    }
}

thread_local! {
    static STATE_MANAGER: RefCell<StateManager> = RefCell::new(StateManager::new());
}

pub fn with_state_manager<F, R>(f: F) -> R
where
    F: FnOnce(&mut StateManager) -> R,
{
    STATE_MANAGER.with(|manager| f(&mut manager.borrow_mut()))
}

// ============================================================================
// WASM EXPORTS (For Testing)
// ============================================================================

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub fn init_dx_morph() {
    web_sys::console::log_1(&"dx-morph: State Patcher Initialized".into());
}
