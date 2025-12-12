//! # dx-client-tiny: < 2 KB WASM Runtime
//!
//! NO_STD Integer DOM Architecture
//! - Zero dependencies
//! - Rust deals only with u32 handles
//! - JS maintains DOM in array
//! - Target: Beat Svelte at < 2 KB

#![no_std]
#![no_main]

mod allocator;

#[global_allocator]
static ALLOC: allocator::BumpAlloc = allocator::BumpAlloc;

#[panic_handler]
#[cfg(target_arch = "wasm32")]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::arch::wasm32::unreachable() }
}

#[panic_handler]
#[cfg(not(target_arch = "wasm32"))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// ============================================================================
// FFI: JavaScript Host Functions (imported by WASM)
// ============================================================================

extern "C" {
    /// Log a u32 value (debugging)
    fn host_log(val: u32);

    /// Clone template by ID, returns node handle
    fn host_clone_template(template_id: u32) -> u32;

    /// Append child to parent
    fn host_append(parent_id: u32, child_id: u32);

    /// Set text content (ptr points to WASM memory)
    fn host_set_text(node_id: u32, ptr: *const u8, len: u32);

    /// Set attribute (key_ptr/val_ptr point to WASM memory)
    fn host_set_attr(
        node_id: u32,
        key_ptr: *const u8,
        key_len: u32,
        val_ptr: *const u8,
        val_len: u32,
    );

    /// Toggle CSS class
    fn host_toggle_class(node_id: u32, class_ptr: *const u8, class_len: u32, enable: u32);

    /// Remove node from DOM
    fn host_remove(node_id: u32);
}

// ============================================================================
// HTIP Opcodes (minimal set)
// ============================================================================

const OP_CLONE: u8 = 1;
const OP_PATCH_TEXT: u8 = 2;
const OP_PATCH_ATTR: u8 = 3;
const OP_CLASS_TOGGLE: u8 = 4;
const OP_REMOVE: u8 = 5;

// ============================================================================
// WASM Exports (called by JavaScript)
// ============================================================================

/// Initialize runtime (no-op for now)
#[no_mangle]
pub extern "C" fn init() -> u32 {
    0 // Success
}

/// Render HTIP stream
///
/// # Format
/// ```
/// [4-byte header: magic + version]
/// [opcode stream...]
/// ```
#[no_mangle]
pub extern "C" fn render_stream(ptr: *const u8, len: u32) -> u32 {
    if len < 4 {
        return 1; // Error: buffer too small
    }

    unsafe {
        // Skip 4-byte header
        let mut offset = 4;

        // Root handle (body) is always 0
        let root_handle = 0u32;

        // Process opcodes
        while offset < len {
            let op = *ptr.add(offset as usize);
            offset += 1;

            match op {
                OP_CLONE => {
                    // Read template ID (1 byte)
                    let template_id = *ptr.add(offset as usize) as u32;
                    offset += 1;

                    // Clone and append to root
                    let node_handle = host_clone_template(template_id);
                    host_append(root_handle, node_handle);
                }

                OP_PATCH_TEXT => {
                    // Read: node_id (2 bytes) + text_len (2 bytes) + text
                    let node_id = read_u16(ptr, offset) as u32;
                    offset += 2;
                    let text_len = read_u16(ptr, offset) as u32;
                    offset += 2;
                    let text_ptr = ptr.add(offset as usize);
                    offset += text_len;

                    host_set_text(node_id, text_ptr, text_len);
                }

                OP_PATCH_ATTR => {
                    // Read: node_id + key_len + key + val_len + val
                    let node_id = read_u16(ptr, offset) as u32;
                    offset += 2;
                    let key_len = read_u16(ptr, offset) as u32;
                    offset += 2;
                    let key_ptr = ptr.add(offset as usize);
                    offset += key_len;
                    let val_len = read_u16(ptr, offset) as u32;
                    offset += 2;
                    let val_ptr = ptr.add(offset as usize);
                    offset += val_len;

                    host_set_attr(node_id, key_ptr, key_len, val_ptr, val_len);
                }

                OP_CLASS_TOGGLE => {
                    // Read: node_id + class_len + class + enable
                    let node_id = read_u16(ptr, offset) as u32;
                    offset += 2;
                    let class_len = read_u16(ptr, offset) as u32;
                    offset += 2;
                    let class_ptr = ptr.add(offset as usize);
                    offset += class_len;
                    let enable = *ptr.add(offset as usize) as u32;
                    offset += 1;

                    host_toggle_class(node_id, class_ptr, class_len, enable);
                }

                OP_REMOVE => {
                    let node_id = read_u16(ptr, offset) as u32;
                    offset += 2;

                    host_remove(node_id);
                }

                _ => {
                    // Unknown opcode, skip
                    break;
                }
            }
        }
    }

    0 // Success
}

/// Get node count (for debugging)
#[no_mangle]
pub extern "C" fn get_node_count() -> u32 {
    0 // Not tracked in tiny version
}

/// Reset runtime
#[no_mangle]
pub extern "C" fn reset() {
    unsafe {
        allocator::reset_heap();
    }
}

// ============================================================================
// Utilities
// ============================================================================

#[inline]
unsafe fn read_u16(ptr: *const u8, offset: u32) -> u16 {
    let b1 = *ptr.add(offset as usize) as u16;
    let b2 = *ptr.add((offset + 1) as usize) as u16;
    b1 | (b2 << 8)
}
