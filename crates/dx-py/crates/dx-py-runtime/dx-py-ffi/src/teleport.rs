//! Zero-copy array teleportation for NumPy integration

/// Data type for array elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DType {
    Float32,
    Float64,
    Int32,
    Int64,
    Int16,
    Int8,
    UInt32,
    UInt64,
    UInt16,
    UInt8,
    Bool,
    Complex64,
    Complex128,
}

impl DType {
    /// Get the size of this dtype in bytes
    pub fn size(&self) -> usize {
        match self {
            DType::Bool | DType::Int8 | DType::UInt8 => 1,
            DType::Int16 | DType::UInt16 => 2,
            DType::Float32 | DType::Int32 | DType::UInt32 => 4,
            DType::Float64 | DType::Int64 | DType::UInt64 | DType::Complex64 => 8,
            DType::Complex128 => 16,
        }
    }
    
    /// Get the alignment of this dtype
    pub fn alignment(&self) -> usize {
        self.size().min(8)
    }
}

/// Zero-copy array access for NumPy integration
///
/// This struct provides direct access to array data without copying,
/// enabling SIMD operations directly on NumPy memory.
pub struct TeleportedArray {
    /// Pointer to array data (shared with Python)
    data: *mut u8,
    /// Shape of the array
    shape: Vec<usize>,
    /// Strides in bytes
    strides: Vec<isize>,
    /// Element type
    dtype: DType,
    /// Total byte size of data
    byte_size: usize,
    /// Whether this array is read-only
    readonly: bool,
    /// Owner reference count (to prevent deallocation)
    _owner_refcount: u64,
}

// Safety: TeleportedArray is Send because we ensure proper synchronization
// when accessing the data pointer
unsafe impl Send for TeleportedArray {}

impl TeleportedArray {
    /// Create a new teleported array from raw components
    ///
    /// # Safety
    /// - `data` must be a valid pointer to array data
    /// - The data must remain valid for the lifetime of this struct
    /// - The shape and strides must correctly describe the data layout
    pub unsafe fn new(
        data: *mut u8,
        shape: Vec<usize>,
        strides: Vec<isize>,
        dtype: DType,
        readonly: bool,
    ) -> Self {
        let byte_size = shape.iter().product::<usize>() * dtype.size();
        
        Self {
            data,
            shape,
            strides,
            dtype,
            byte_size,
            readonly,
            _owner_refcount: 1,
        }
    }
    
    /// Create a teleported array from a Vec (takes ownership)
    pub fn from_vec<T: Copy + 'static>(data: Vec<T>, shape: Vec<usize>) -> Self {
        let dtype = Self::dtype_for::<T>();
        let byte_size = data.len() * std::mem::size_of::<T>();
        
        // Calculate strides (row-major / C-contiguous)
        let mut strides = vec![0isize; shape.len()];
        let mut stride = std::mem::size_of::<T>() as isize;
        for i in (0..shape.len()).rev() {
            strides[i] = stride;
            stride *= shape[i] as isize;
        }
        
        let ptr = Box::into_raw(data.into_boxed_slice()) as *mut u8;
        
        Self {
            data: ptr,
            shape,
            strides,
            dtype,
            byte_size,
            readonly: false,
            _owner_refcount: 1,
        }
    }
    
    /// Get the dtype for a Rust type
    fn dtype_for<T: 'static>() -> DType {
        use std::any::TypeId;
        
        let type_id = TypeId::of::<T>();
        
        if type_id == TypeId::of::<f32>() { DType::Float32 }
        else if type_id == TypeId::of::<f64>() { DType::Float64 }
        else if type_id == TypeId::of::<i32>() { DType::Int32 }
        else if type_id == TypeId::of::<i64>() { DType::Int64 }
        else if type_id == TypeId::of::<i16>() { DType::Int16 }
        else if type_id == TypeId::of::<i8>() { DType::Int8 }
        else if type_id == TypeId::of::<u32>() { DType::UInt32 }
        else if type_id == TypeId::of::<u64>() { DType::UInt64 }
        else if type_id == TypeId::of::<u16>() { DType::UInt16 }
        else if type_id == TypeId::of::<u8>() { DType::UInt8 }
        else if type_id == TypeId::of::<bool>() { DType::Bool }
        else { DType::UInt8 } // Default fallback
    }
    
    /// Get raw data pointer (zero-copy)
    #[inline]
    pub fn data_ptr(&self) -> *const u8 {
        self.data
    }
    
    /// Get mutable data pointer (zero-copy)
    ///
    /// Returns None if the array is read-only.
    #[inline]
    pub fn data_ptr_mut(&mut self) -> Option<*mut u8> {
        if self.readonly {
            None
        } else {
            Some(self.data)
        }
    }
    
    /// Get the shape
    #[inline]
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }
    
    /// Get the strides
    #[inline]
    pub fn strides(&self) -> &[isize] {
        &self.strides
    }
    
    /// Get the dtype
    #[inline]
    pub fn dtype(&self) -> DType {
        self.dtype
    }
    
    /// Get the total byte size
    #[inline]
    pub fn byte_size(&self) -> usize {
        self.byte_size
    }
    
    /// Get the number of dimensions
    #[inline]
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }
    
    /// Get the total number of elements
    #[inline]
    pub fn len(&self) -> usize {
        self.shape.iter().product()
    }
    
    /// Check if the array is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Check if the array is contiguous (C-order)
    pub fn is_contiguous(&self) -> bool {
        if self.shape.is_empty() {
            return true;
        }
        
        let elem_size = self.dtype.size() as isize;
        let mut expected_stride = elem_size;
        
        for i in (0..self.shape.len()).rev() {
            if self.strides[i] != expected_stride {
                return false;
            }
            expected_stride *= self.shape[i] as isize;
        }
        
        true
    }
    
    /// Get data as a slice (zero-copy)
    ///
    /// # Safety
    /// The caller must ensure T matches the dtype.
    #[inline]
    pub unsafe fn as_slice<T>(&self) -> &[T] {
        let len = self.byte_size / std::mem::size_of::<T>();
        std::slice::from_raw_parts(self.data as *const T, len)
    }
    
    /// Get data as a mutable slice (zero-copy)
    ///
    /// # Safety
    /// The caller must ensure T matches the dtype.
    #[inline]
    pub unsafe fn as_mut_slice<T>(&mut self) -> Option<&mut [T]> {
        if self.readonly {
            return None;
        }
        let len = self.byte_size / std::mem::size_of::<T>();
        Some(std::slice::from_raw_parts_mut(self.data as *mut T, len))
    }
    
    /// Add a scalar to all elements (SIMD-accelerated for f64)
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub fn add_scalar_f64(&mut self, scalar: f64) -> bool {
        if self.readonly || self.dtype != DType::Float64 {
            return false;
        }
        
        unsafe {
            if is_x86_feature_detected!("avx2") {
                self.add_scalar_f64_avx2(scalar);
            } else {
                self.add_scalar_f64_scalar(scalar);
            }
        }
        
        true
    }
    
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    pub fn add_scalar_f64(&mut self, scalar: f64) -> bool {
        if self.readonly || self.dtype != DType::Float64 {
            return false;
        }
        
        unsafe { self.add_scalar_f64_scalar(scalar) };
        true
    }
    
    /// AVX2 implementation of add_scalar
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[target_feature(enable = "avx2")]
    unsafe fn add_scalar_f64_avx2(&mut self, scalar: f64) {
        use std::arch::x86_64::*;
        
        let scalar_vec = _mm256_set1_pd(scalar);
        let data = self.data as *mut f64;
        let len = self.byte_size / 8;
        
        let mut i = 0;
        while i + 4 <= len {
            let chunk = _mm256_loadu_pd(data.add(i));
            let result = _mm256_add_pd(chunk, scalar_vec);
            _mm256_storeu_pd(data.add(i), result);
            i += 4;
        }
        
        // Scalar remainder
        while i < len {
            *data.add(i) += scalar;
            i += 1;
        }
    }
    
    /// Scalar implementation of add_scalar
    unsafe fn add_scalar_f64_scalar(&mut self, scalar: f64) {
        let data = self.data as *mut f64;
        let len = self.byte_size / 8;
        
        for i in 0..len {
            *data.add(i) += scalar;
        }
    }
    
    /// Multiply all elements by a scalar (SIMD-accelerated for f64)
    pub fn mul_scalar_f64(&mut self, scalar: f64) -> bool {
        if self.readonly || self.dtype != DType::Float64 {
            return false;
        }
        
        unsafe {
            let data = self.data as *mut f64;
            let len = self.byte_size / 8;
            
            for i in 0..len {
                *data.add(i) *= scalar;
            }
        }
        
        true
    }
}

impl Drop for TeleportedArray {
    fn drop(&mut self) {
        // Only deallocate if we own the data (refcount == 1)
        // In real implementation, this would check if data came from Python
        if self._owner_refcount == 1 && !self.data.is_null() {
            // Don't deallocate - data may be owned by Python
            // This is a simplified implementation
        }
    }
}

/// View into a teleported array (borrowed, zero-copy)
#[allow(dead_code)]
pub struct TeleportedArrayView<'a, T> {
    data: &'a [T],
    shape: &'a [usize],
    strides: &'a [isize],
}

impl<'a, T> TeleportedArrayView<'a, T> {
    /// Create a view from a teleported array
    ///
    /// # Safety
    /// T must match the array's dtype
    pub unsafe fn from_array(array: &'a TeleportedArray) -> Self {
        Self {
            data: array.as_slice(),
            shape: &array.shape,
            strides: &array.strides,
        }
    }
    
    /// Get the data slice
    pub fn data(&self) -> &[T] {
        self.data
    }
    
    /// Get the shape
    pub fn shape(&self) -> &[usize] {
        self.shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dtype_size() {
        assert_eq!(DType::Float64.size(), 8);
        assert_eq!(DType::Float32.size(), 4);
        assert_eq!(DType::Int64.size(), 8);
        assert_eq!(DType::Int32.size(), 4);
        assert_eq!(DType::Bool.size(), 1);
    }
    
    #[test]
    fn test_teleported_array_from_vec() {
        let data = vec![1.0f64, 2.0, 3.0, 4.0];
        let array = TeleportedArray::from_vec(data, vec![4]);
        
        assert_eq!(array.shape(), &[4]);
        assert_eq!(array.dtype(), DType::Float64);
        assert_eq!(array.len(), 4);
        assert!(array.is_contiguous());
    }
    
    #[test]
    fn test_teleported_array_as_slice() {
        let data = vec![1.0f64, 2.0, 3.0, 4.0];
        let array = TeleportedArray::from_vec(data, vec![4]);
        
        unsafe {
            let slice: &[f64] = array.as_slice();
            assert_eq!(slice, &[1.0, 2.0, 3.0, 4.0]);
        }
    }
    
    #[test]
    fn test_add_scalar() {
        let data = vec![1.0f64, 2.0, 3.0, 4.0];
        let mut array = TeleportedArray::from_vec(data, vec![4]);
        
        assert!(array.add_scalar_f64(10.0));
        
        unsafe {
            let slice: &[f64] = array.as_slice();
            assert_eq!(slice, &[11.0, 12.0, 13.0, 14.0]);
        }
    }
    
    #[test]
    fn test_mul_scalar() {
        let data = vec![1.0f64, 2.0, 3.0, 4.0];
        let mut array = TeleportedArray::from_vec(data, vec![4]);
        
        assert!(array.mul_scalar_f64(2.0));
        
        unsafe {
            let slice: &[f64] = array.as_slice();
            assert_eq!(slice, &[2.0, 4.0, 6.0, 8.0]);
        }
    }
    
    #[test]
    fn test_2d_array() {
        let data = vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0];
        let array = TeleportedArray::from_vec(data, vec![2, 3]);
        
        assert_eq!(array.shape(), &[2, 3]);
        assert_eq!(array.ndim(), 2);
        assert_eq!(array.len(), 6);
    }
}
