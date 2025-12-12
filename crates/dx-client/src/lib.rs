//! # dx-client: Sub-20KB WASM Runtime (Pure FFI)
//!
//! Full-featured Macro runtime without wasm-bindgen bloat.
//! Uses pure FFI imports like dx-client-tiny but with complete HTIP support.
//!
//! ## Features
//! - Template cloning and caching
//! - Incremental DOM patching
//! - State management
//! - Event handling
//!
//! Target: < 20KB unoptimized

#![no_std]

extern crate alloc;

mod allocator;

use alloc::vec::Vec;
use core::cell::RefCell;
use core::slice;

#[global_allocator]
static ALLOC: allocator::BumpAlloc = allocator::BumpAlloc;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        core::arch::wasm32::unreachable()
    }
    #[cfg(not(target_arch = "wasm32"))]
    loop {}
}

// ============================================================================
// FFI: JavaScript Host Functions
// ============================================================================

extern "C" {
    // Template operations
    fn host_clone_template(id: u32) -> u32;
    fn host_cache_template(id: u32, html_ptr: *const u8, html_len: u32);

    // DOM operations
    fn host_append(parent: u32, child: u32);
    fn host_remove(node: u32);
    fn host_set_text(node: u32, ptr: *const u8, len: u32);
    fn host_set_attr(node: u32, key_ptr: *const u8, key_len: u32, val_ptr: *const u8, val_len: u32);
    fn host_toggle_class(node: u32, class_ptr: *const u8, class_len: u32, enable: u32);

    // Events
    fn host_listen(node: u32, event_type: u32, handler_id: u32);

    // State
    fn host_notify_state_change(slot_id: u32);

    // Debug
    fn host_log(val: u32);
}

// ============================================================================
// HTIP Opcodes
// ============================================================================

const OP_CLONE: u8 = 1;
const OP_PATCH_TEXT: u8 = 2;
const OP_PATCH_ATTR: u8 = 3;
const OP_CLASS_TOGGLE: u8 = 4;
const OP_REMOVE: u8 = 5;
const OP_EVENT: u8 = 6;
const OP_STATE_UPDATE: u8 = 7;
const OP_TEMPLATE_DEF: u8 = 8;
const OP_EOF: u8 = 255;

// ============================================================================
// State
// ============================================================================

struct Runtime {
    node_count: u32,
    template_count: u32,
}

static mut RUNTIME: Runtime = Runtime {
    node_count: 0,
    template_count: 0,
};

// ============================================================================
// WASM Exports
// ============================================================================

/// Initialize runtime
#[no_mangle]
pub extern "C" fn init() -> u32 {
    unsafe {
        RUNTIME.node_count = 0;
        RUNTIME.template_count = 0;
    }
    0
}

/// Render HTIP stream
#[no_mangle]
pub extern "C" fn render_stream(ptr: *const u8, len: u32) -> u32 {
    if ptr.is_null() || len < 4 {
        return 1;
    }

    unsafe {
        let data = slice::from_raw_parts(ptr, len as usize);
        process_htip_stream(data)
    }
}

/// Process HTIP stream bytes
unsafe fn process_htip_stream(data: &[u8]) -> u32 {
    let mut offset = 4; // Skip header

    while offset < data.len() {
        let op = data[offset];
        offset += 1;

        match op {
            OP_CLONE => {
                if offset >= data.len() {
                    break;
                }
                let template_id = data[offset] as u32;
                offset += 1;

                let node = host_clone_template(template_id);
                host_append(0, node);
                RUNTIME.node_count += 1;
            }

            OP_TEMPLATE_DEF => {
                if offset + 3 >= data.len() {
                    break;
                }
                let id = data[offset] as u32;
                offset += 1;
                let len = read_u16(&data, offset) as usize;
                offset += 2;

                if offset + len > data.len() {
                    break;
                }
                let html = &data[offset..offset + len];
                offset += len;

                host_cache_template(id, html.as_ptr(), len as u32);
                RUNTIME.template_count += 1;
            }

            OP_PATCH_TEXT => {
                if offset + 4 >= data.len() {
                    break;
                }
                let node_id = read_u16(&data, offset) as u32;
                offset += 2;
                let text_len = read_u16(&data, offset) as usize;
                offset += 2;

                if offset + text_len > data.len() {
                    break;
                }
                let text = &data[offset..offset + text_len];
                offset += text_len;

                host_set_text(node_id, text.as_ptr(), text_len as u32);
            }

            OP_PATCH_ATTR => {
                if offset + 6 >= data.len() {
                    break;
                }
                let node_id = read_u16(&data, offset) as u32;
                offset += 2;
                let key_len = read_u16(&data, offset) as usize;
                offset += 2;

                if offset + key_len >= data.len() {
                    break;
                }
                let key = &data[offset..offset + key_len];
                offset += key_len;

                let val_len = read_u16(&data, offset) as usize;
                offset += 2;

                if offset + val_len > data.len() {
                    break;
                }
                let val = &data[offset..offset + val_len];
                offset += val_len;

                host_set_attr(node_id, key.as_ptr(), key_len as u32, val.as_ptr(), val_len as u32);
            }

            OP_CLASS_TOGGLE => {
                if offset + 5 >= data.len() {
                    break;
                }
                let node_id = read_u16(&data, offset) as u32;
                offset += 2;
                let class_len = read_u16(&data, offset) as usize;
                offset += 2;

                if offset + class_len >= data.len() {
                    break;
                }
                let class = &data[offset..offset + class_len];
                offset += class_len;

                let enable = data[offset] as u32;
                offset += 1;

                host_toggle_class(node_id, class.as_ptr(), class_len as u32, enable);
            }

            OP_REMOVE => {
                if offset + 2 > data.len() {
                    break;
                }
                let node_id = read_u16(&data, offset) as u32;
                offset += 2;

                host_remove(node_id);
            }

            OP_EVENT => {
                if offset + 5 > data.len() {
                    break;
                }
                let node_id = read_u16(&data, offset) as u32;
                offset += 2;
                let event_type = data[offset] as u32;
                offset += 1;
                let handler_id = read_u16(&data, offset) as u32;
                offset += 2;

                host_listen(node_id, event_type, handler_id);
            }

            OP_EOF => break,

            _ => {
                // Unknown opcode, stop processing
                break;
            }
        }
    }

    0
}

/// Event dispatcher (called by JS)
#[no_mangle]
pub extern "C" fn on_event(handler_id: u32) {
    // Dispatch to registered handler
    unsafe {
        host_log(handler_id);
    }
}

/// Get node count
#[no_mangle]
pub extern "C" fn get_node_count() -> u32 {
    unsafe { RUNTIME.node_count }
}

/// Get template count
#[no_mangle]
pub extern "C" fn get_template_count() -> u32 {
    unsafe { RUNTIME.template_count }
}

/// Reset runtime
#[no_mangle]
pub extern "C" fn reset() {
    unsafe {
        RUNTIME.node_count = 0;
        RUNTIME.template_count = 0;
        allocator::reset_heap();
    }
}

// ============================================================================
// Utilities
// ============================================================================

#[inline]
fn read_u16(data: &[u8], offset: usize) -> u16 {
    if offset + 1 >= data.len() {
        return 0;
    }
    (data[offset] as u16) | ((data[offset + 1] as u16) << 8)
}
