//! # dx-client: 9KB WASM Runtime
//!
//! The lean, mean, DOM-rendering machine.
//!
//! ## Architecture (Option D: Optimistic Guard)
//!
//! ```text
//! fetch(url) ──► JS Loader ──► SubtleCrypto.verify() ──► dx-client WASM
//!                    │                                        │
//!                    │  (signature verified BEFORE WASM)      │
//!                    ▼                                        ▼
//!              64-byte sig removed                  Zero-copy render
//! ```
//!
//! ## Security Model
//!
//! - Ed25519 signature is verified by JavaScript BEFORE this code runs
//! - The `ptr` passed to `render_stream` is ALREADY VERIFIED
//! - No crypto in WASM = 0 bytes of crypto bloat

// ============================================================================
// ULTRA-TINY ALLOCATOR (~100 bytes vs ~1KB for wee_alloc)
// ============================================================================

mod allocator;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: allocator::BumpAlloc = allocator::BumpAlloc;

use core::ptr;
use dx_packet::*;
use wasm_bindgen::prelude::*;

mod node_registry;
mod patcher;
mod renderer;
mod stream_reader;
mod string_table;
mod template_cache;

pub use node_registry::NodeRegistry;
pub use patcher::{Patcher, PATCH_BLOCK_SIZE};
pub use renderer::Renderer;
pub use stream_reader::{ChunkDispatcher, StreamReader};
pub use string_table::StringTableReader;
pub use template_cache::TemplateCache;

// ============================================================================
// GLOBAL STATE (thread_local for WASM single-thread)
// ============================================================================

use core::cell::RefCell;

thread_local! {
    static RENDERER: RefCell<Option<Renderer>> = RefCell::new(None);
    static PATCHER: RefCell<Option<Patcher>> = RefCell::new(None);
}

// ============================================================================
// WASM EXPORTS
// ============================================================================

/// Initialize the renderer
///
/// Must be called once before render_stream
#[wasm_bindgen]
pub fn init() -> Result<(), u8> {
    RENDERER.with(|r| {
        *r.borrow_mut() = Some(Renderer::new()?);
        Ok(())
    })
}

/// Render a verified HTIP stream to DOM
///
/// # Arguments
/// * `data` - Raw bytes from server (signature already stripped by JS loader)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(error_code)` on failure (see dx_packet::ErrorCode)
///
/// # Safety
/// This function assumes the data has been signature-verified by the JS loader.
/// Passing unverified data is a security vulnerability.
#[wasm_bindgen]
pub fn render_stream(data: &[u8]) -> Result<(), u8> {
    // Validate minimum size
    if data.len() < HtipHeader::SIZE {
        return Err(ErrorCode::BufferTooSmall as u8);
    }

    // Zero-copy header read
    let header = unsafe { ptr::read_unaligned(data.as_ptr() as *const HtipHeader) };

    // Validate header
    if !header.is_valid() {
        return Err(ErrorCode::InvalidMagic as u8);
    }

    RENDERER.with(|r| {
        let mut renderer = r.borrow_mut();
        let renderer = renderer.as_mut().ok_or(ErrorCode::NodeNotFound as u8)?;

        renderer.process_stream(data, &header)
    })
}

/// Get current node count (for debugging)
#[wasm_bindgen]
pub fn get_node_count() -> u32 {
    RENDERER.with(|r| r.borrow().as_ref().map(|r| r.node_count()).unwrap_or(0))
}

/// Clear all state (for hot reload)
#[wasm_bindgen]
pub fn reset() {
    RENDERER.with(|r| {
        *r.borrow_mut() = None;
    });
    // Reset bump allocator to reclaim all memory
    unsafe {
        allocator::reset_heap();
    }
}

// ============================================================================
// CHUNKED STREAMING API (Phase 6: Day 12 & 13)
// ============================================================================

thread_local! {
    static STREAM_READER: RefCell<Option<StreamReader>> = RefCell::new(None);
    static CHUNK_DISPATCHER: RefCell<Option<ChunkDispatcher>> = RefCell::new(None);
}

/// Initialize streaming mode
///
/// Call this before processing chunked streams
#[wasm_bindgen]
pub fn init_streaming() -> Result<(), u8> {
    STREAM_READER.with(|sr| {
        *sr.borrow_mut() = Some(StreamReader::new());
        Ok::<(), u8>(())
    })?;

    CHUNK_DISPATCHER.with(|cd| {
        *cd.borrow_mut() = Some(ChunkDispatcher::new());
        Ok(())
    })
}

/// Feed chunk data to the stream reader
///
/// Returns number of complete chunks ready for processing
/// Call poll_next_chunk() to retrieve them
#[wasm_bindgen]
pub fn feed_chunk_data(data: &[u8]) -> Result<u32, u8> {
    STREAM_READER.with(|sr| {
        let mut reader = sr.borrow_mut();
        let reader = reader.as_mut().ok_or(10u8)?; // ErrorCode::NotInitialized

        let chunks_ready = reader.feed(data)?;
        Ok(chunks_ready as u32)
    })
}

/// Poll for next complete chunk and process it
///
/// Returns true if chunk was processed, false if no chunk available
#[wasm_bindgen]
pub fn poll_and_process_chunk() -> Result<bool, u8> {
    let chunk = STREAM_READER.with(|sr| {
        let mut reader = sr.borrow_mut();
        let reader = reader.as_mut().ok_or(10u8)?; // ErrorCode::NotInitialized
        Ok::<_, u8>(reader.poll_chunk())
    })?;

    if let Some((chunk_type, data)) = chunk {
        CHUNK_DISPATCHER.with(|cd| {
            let mut dispatcher = cd.borrow_mut();
            let dispatcher = dispatcher.as_mut().ok_or(10u8)?; // ErrorCode::NotInitialized
            dispatcher.handle_chunk(chunk_type, data)
        })?;

        return Ok(true);
    }

    Ok(false)
}

/// Check if stream is finished (received EOF)
#[wasm_bindgen]
pub fn is_stream_finished() -> bool {
    STREAM_READER.with(|sr| {
        sr.borrow()
            .as_ref()
            .map(|r| r.is_finished())
            .unwrap_or(false)
    })
}

/// Finalize streaming: Process accumulated chunks and render
///
/// Call this after is_stream_finished() returns true
#[wasm_bindgen]
pub fn finalize_stream() -> Result<(), u8> {
    // Get accumulated data
    let (layout_data, _state_data, _wasm_data) = CHUNK_DISPATCHER.with(|cd| {
        let mut dispatcher = cd.borrow_mut();
        let dispatcher = dispatcher.as_mut().ok_or(10u8)?; // ErrorCode::NotInitialized

        if !dispatcher.is_complete() {
            return Err(11u8); // ErrorCode::IncompleteStream
        }

        Ok::<_, u8>((
            dispatcher.take_layout(),
            dispatcher.take_state(),
            dispatcher.take_wasm(),
        ))
    })?;

    // TODO: Process layout_data to register templates
    // TODO: Process state_data to initialize memory
    // TODO: WASM is handled by browser's WebAssembly.instantiateStreaming

    // For now, just validate we got the data
    if layout_data.is_none() {
        return Err(12u8); // ErrorCode::MissingLayout
    }

    Ok(())
}

// ============================================================================
// PATCHER EXPORTS (Day 13)
// ============================================================================

/// Initialize patcher
#[wasm_bindgen]
pub fn init_patcher() {
    PATCHER.with(|p| {
        *p.borrow_mut() = Some(Patcher::new());
    });
}

/// Set the old binary to patch
///
/// # Arguments
/// * `data` - The old binary data
#[wasm_bindgen]
pub fn set_old_binary(data: &[u8]) -> Result<(), u8> {
    PATCHER.with(|p| {
        let mut patcher = p.borrow_mut();
        let patcher = patcher.as_mut().ok_or(1)?; // ErrorCode::PatcherNotInitialized
        patcher.set_old_binary(data.to_vec());
        Ok(())
    })
}

/// Set patch data
///
/// # Arguments
/// * `data` - The patch data (header + blocks)
#[wasm_bindgen]
pub fn set_patch_data(data: &[u8]) -> Result<(), u8> {
    PATCHER.with(|p| {
        let mut patcher = p.borrow_mut();
        let patcher = patcher.as_mut().ok_or(1)?; // ErrorCode::PatcherNotInitialized
        patcher.set_patch_data(data)
    })
}

/// Apply patch and get new binary length
///
/// Returns the length of the patched binary
#[wasm_bindgen]
pub fn apply_patch_and_get_length() -> Result<u32, u8> {
    PATCHER.with(|p| {
        let mut patcher = p.borrow_mut();
        let patcher = patcher.as_mut().ok_or(1)?; // ErrorCode::PatcherNotInitialized
        let new_binary = patcher.apply_patch()?;
        let len = new_binary.len() as u32;

        // Store the patched binary for retrieval
        patcher.set_old_binary(new_binary);

        Ok(len)
    })
}

/// Get patched binary data
///
/// Returns a JS Uint8Array with the patched binary
/// Call apply_patch_and_get_length() first
#[wasm_bindgen]
pub fn get_patched_binary() -> Result<Vec<u8>, u8> {
    PATCHER.with(|p| {
        let patcher = p.borrow();
        let patcher = patcher.as_ref().ok_or(1)?; // ErrorCode::PatcherNotInitialized

        // Get the old_binary which now contains the patched result
        patcher
            .old_binary
            .clone()
            .ok_or(2) // ErrorCode::NoBinary
    })
}

/// Apply patch in-place to a buffer (fastest method)
///
/// # Arguments
/// * `buffer` - Mutable buffer containing old binary
/// * `patch_data` - The patch data
///
/// # Performance
///
/// Faster than apply_patch() as it modifies in-place without cloning
#[wasm_bindgen]
pub fn apply_patch_inplace(buffer: &mut [u8], patch_data: &[u8]) -> Result<(), u8> {
    Patcher::apply_patch_inplace(buffer, patch_data)
}
