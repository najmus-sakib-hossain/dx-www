//! Pointer operations.

/// Read a value from a pointer.
///
/// # Safety
/// The pointer must be valid and properly aligned.
pub unsafe fn read<T: Copy>(ptr: *const T) -> T {
    unsafe { *ptr }
}

/// Write a value to a pointer.
///
/// # Safety
/// The pointer must be valid, properly aligned, and writable.
pub unsafe fn write<T>(ptr: *mut T, value: T) {
    unsafe { *ptr = value };
}

/// Convert a pointer and length to a byte vector.
///
/// # Safety
/// The pointer must be valid for `len` bytes.
pub unsafe fn to_array_buffer(ptr: *const u8, len: usize) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(ptr, len).to_vec() }
}
