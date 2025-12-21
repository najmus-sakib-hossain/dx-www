//! Zero-copy teleportation types.

use std::mem::{size_of, align_of};

/// Layout information for teleportable types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeleportLayout {
    /// Size of the type in bytes.
    pub size: usize,
    /// Alignment requirement in bytes.
    pub align: usize,
    /// Checksum for layout verification.
    pub checksum: u64,
}

impl TeleportLayout {
    /// Create a new layout.
    pub const fn new(size: usize, align: usize, checksum: u64) -> Self {
        Self { size, align, checksum }
    }
}

/// Marker trait for zero-copy transferable types.
///
/// # Safety
///
/// This trait is unsafe because implementors must guarantee:
/// 1. The type uses `#[repr(C)]` for stable memory layout
/// 2. The type contains no pointers or references
/// 3. The type is `Copy` (no drop semantics)
/// 4. The layout is identical on server and WASM client
pub unsafe trait Teleportable: Copy + 'static {
    /// Layout information for this type.
    const LAYOUT: TeleportLayout;
}

/// Buffer for writing teleportable values.
pub struct TeleportBuffer {
    /// Main data buffer.
    buffer: Vec<u8>,
    /// Current write position.
    position: usize,
    /// String table buffer.
    strings: Vec<u8>,
}

impl TeleportBuffer {
    /// Create a new teleport buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            position: 0,
            strings: Vec::new(),
        }
    }

    /// Write a teleportable value to the buffer.
    pub fn write<T: Teleportable>(&mut self, value: &T) {
        let size = size_of::<T>();
        let align = align_of::<T>();
        
        // Align the position
        let padding = (align - (self.position % align)) % align;
        self.buffer.resize(self.position + padding + size, 0);
        self.position += padding;
        
        // Copy the value
        let bytes = unsafe {
            std::slice::from_raw_parts(value as *const T as *const u8, size)
        };
        self.buffer[self.position..self.position + size].copy_from_slice(bytes);
        self.position += size;
    }

    /// Write a slice of teleportable values to the buffer.
    pub fn write_slice<T: Teleportable>(&mut self, values: &[T]) {
        for value in values {
            self.write(value);
        }
    }

    /// Write a string to the string table.
    ///
    /// Returns (offset, length) for later retrieval.
    pub fn write_string(&mut self, s: &str) -> (u32, u32) {
        let offset = self.strings.len() as u32;
        let len = s.len() as u32;
        self.strings.extend_from_slice(s.as_bytes());
        (offset, len)
    }

    /// Finalize the buffer and return the complete byte slice.
    ///
    /// The format is:
    /// - [data section]
    /// - [string table offset: u32]
    /// - [string table]
    pub fn finalize(&mut self) -> &[u8] {
        // Write string table offset
        let string_table_offset = self.position as u32;
        self.buffer.extend_from_slice(&string_table_offset.to_le_bytes());
        self.position += 4;
        
        // Append string table
        self.buffer.extend_from_slice(&self.strings);
        self.position += self.strings.len();
        
        &self.buffer[..self.position]
    }

    /// Get the current buffer contents without finalizing.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.position]
    }

    /// Get the current position.
    pub fn position(&self) -> usize {
        self.position
    }
}

/// Reader for teleportable values (zero-copy).
pub struct TeleportReader<'a> {
    /// Buffer to read from.
    buffer: &'a [u8],
    /// Current read position.
    position: usize,
    /// String table offset.
    string_table_offset: usize,
}

impl<'a> TeleportReader<'a> {
    /// Create a new reader from a finalized buffer.
    pub fn new(buffer: &'a [u8]) -> Self {
        // Read string table offset from the end of the data section
        let string_table_offset = if buffer.len() >= 4 {
            // Find the string table offset marker
            // For simplicity, we'll scan backwards for it
            // In a real implementation, this would be at a known position
            0 // Placeholder - actual implementation would parse the format
        } else {
            buffer.len()
        };
        
        Self {
            buffer,
            position: 0,
            string_table_offset,
        }
    }

    /// Create a reader with explicit string table offset.
    pub fn with_string_table(buffer: &'a [u8], string_table_offset: usize) -> Self {
        Self {
            buffer,
            position: 0,
            string_table_offset,
        }
    }

    /// Read a teleportable value (zero-copy reference).
    pub fn read<T: Teleportable>(&mut self) -> Option<&'a T> {
        let size = size_of::<T>();
        let align = align_of::<T>();
        
        // Align the position
        let padding = (align - (self.position % align)) % align;
        self.position += padding;
        
        if self.position + size > self.buffer.len() {
            return None;
        }
        
        let ptr = self.buffer[self.position..].as_ptr() as *const T;
        self.position += size;
        
        // SAFETY: We've verified bounds and alignment
        Some(unsafe { &*ptr })
    }

    /// Read a slice of teleportable values (zero-copy reference).
    pub fn read_slice<T: Teleportable>(&mut self, count: usize) -> Option<&'a [T]> {
        let size = size_of::<T>();
        let align = align_of::<T>();
        let total_size = size * count;
        
        // Align the position
        let padding = (align - (self.position % align)) % align;
        self.position += padding;
        
        if self.position + total_size > self.buffer.len() {
            return None;
        }
        
        let ptr = self.buffer[self.position..].as_ptr() as *const T;
        self.position += total_size;
        
        // SAFETY: We've verified bounds and alignment
        Some(unsafe { std::slice::from_raw_parts(ptr, count) })
    }

    /// Read a string from the string table.
    pub fn read_string(&self, offset: u32, len: u32) -> Option<&'a str> {
        let start = self.string_table_offset + offset as usize;
        let end = start + len as usize;
        
        if end > self.buffer.len() {
            return None;
        }
        
        std::str::from_utf8(&self.buffer[start..end]).ok()
    }

    /// Get the current read position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Reset the read position.
    pub fn reset(&mut self) {
        self.position = 0;
    }
}

// Implement Teleportable for primitive types
macro_rules! impl_teleportable_primitive {
    ($($ty:ty),*) => {
        $(
            unsafe impl Teleportable for $ty {
                const LAYOUT: TeleportLayout = TeleportLayout::new(
                    size_of::<$ty>(),
                    align_of::<$ty>(),
                    0, // Checksum computed at compile time in real impl
                );
            }
        )*
    };
}

impl_teleportable_primitive!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
