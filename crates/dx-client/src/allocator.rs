use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;

// 1MB Static Heap. Zero cost startup.
// dx-www design: "Reset the world every frame"
const HEAP_SIZE: usize = 1024 * 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut POINTER: usize = 0;

pub struct BumpAlloc;

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = POINTER;
        let end = start + layout.size();

        // Security: Crash if OOM to prevent memory corruption
        if end > HEAP_SIZE {
            #[cfg(target_arch = "wasm32")]
            core::arch::wasm32::unreachable();
            #[cfg(not(target_arch = "wasm32"))]
            panic!("Heap OOM");
        }

        POINTER = end;
        HEAP.as_mut_ptr().add(start)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-Op. We don't free individual items.
        // We free the WHOLE world at the end of the frame/transaction.
        // This is 1000x faster than malloc/free.
    }
}

/// Reset the heap pointer to 0
///
/// Call this at the start of every HTIP transaction/frame
pub unsafe fn reset_heap() {
    POINTER = 0;
}
