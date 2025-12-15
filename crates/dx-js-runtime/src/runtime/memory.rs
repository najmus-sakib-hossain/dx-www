//! Arena-based memory management
//!
//! Uses standard Rust allocation for cross-platform support.
//! Can be optimized with platform-specific mmap/VirtualAlloc later.

use crate::error::DxResult;
use std::alloc::{alloc, dealloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Arena allocator for zero-allocation execution
pub struct Arena {
    base: *mut u8,
    size: usize,
    offset: AtomicUsize,
    layout: Layout,
}

// Safety: We control access via atomic operations
unsafe impl Send for Arena {}
unsafe impl Sync for Arena {}

impl Arena {
    /// Create a new arena with the given size
    pub fn new(size: usize) -> DxResult<Self> {
        // Use standard Rust allocation for cross-platform support
        let layout = Layout::from_size_align(size, 4096).unwrap();
        let base = unsafe { alloc(layout) };

        if base.is_null() {
            return Err(crate::error::DxError::RuntimeError(
                "Failed to allocate arena".into(),
            ));
        }

        Ok(Self {
            base,
            size,
            offset: AtomicUsize::new(0),
            layout,
        })
    }

    /// Allocate memory from the arena
    #[inline]
    #[allow(dead_code)]
    pub fn alloc(&self, size: usize, align: usize) -> Option<*mut u8> {
        loop {
            let current = self.offset.load(Ordering::Relaxed);
            let aligned = (current + align - 1) & !(align - 1);
            let new_offset = aligned + size;

            if new_offset > self.size {
                return None;
            }

            if self
                .offset
                .compare_exchange_weak(current, new_offset, Ordering::SeqCst, Ordering::Relaxed)
                .is_ok()
            {
                return Some(unsafe { self.base.add(aligned) });
            }
        }
    }

    /// Reset the arena - O(1) operation
    #[inline]
    pub fn reset(&self) {
        self.offset.store(0, Ordering::SeqCst);
    }

    /// Get current usage
    #[allow(dead_code)]
    pub fn usage(&self) -> usize {
        self.offset.load(Ordering::Relaxed)
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.base, self.layout);
        }
    }
}
