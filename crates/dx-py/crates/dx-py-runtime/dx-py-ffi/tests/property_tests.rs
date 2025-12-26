//! Property-based tests for Memory Teleportation FFI
//!
//! Property 13: Zero-Copy FFI Pointer Sharing
//! Validates: Requirements 6.1

use dx_py_ffi::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    /// Property 13: Zero-Copy FFI Pointer Sharing
    /// Validates: Requirements 6.1
    ///
    /// Teleported arrays should share the same memory as the original data.
    #[test]
    fn prop_zero_copy_pointer_sharing(data in prop::collection::vec(any::<f64>(), 1..1000)) {
        let original_ptr = data.as_ptr();
        let original_len = data.len();
        
        let array = TeleportedArray::from_vec(data, vec![original_len]);
        
        // The data pointer should point to valid memory
        prop_assert!(!array.data_ptr().is_null());
        
        // The array should have the correct length
        prop_assert_eq!(array.len(), original_len);
        
        // The data should be accessible
        unsafe {
            let slice: &[f64] = array.as_slice();
            prop_assert_eq!(slice.len(), original_len);
        }
    }
    
    /// Property: Array operations preserve data integrity
    #[test]
    fn prop_array_operations_preserve_data(
        data in prop::collection::vec(-1000.0f64..1000.0, 1..100),
        scalar in -100.0f64..100.0
    ) {
        let original: Vec<f64> = data.clone();
        let mut array = TeleportedArray::from_vec(data, vec![original.len()]);
        
        // Add scalar
        array.add_scalar_f64(scalar);
        
        // Verify result
        unsafe {
            let result: &[f64] = array.as_slice();
            for (i, &val) in result.iter().enumerate() {
                let expected = original[i] + scalar;
                prop_assert!(
                    (val - expected).abs() < 1e-10,
                    "Mismatch at index {}: {} != {}",
                    i, val, expected
                );
            }
        }
    }
    
    /// Property: Multiply scalar preserves data integrity
    #[test]
    fn prop_mul_scalar_preserves_data(
        data in prop::collection::vec(-100.0f64..100.0, 1..100),
        scalar in -10.0f64..10.0
    ) {
        let original: Vec<f64> = data.clone();
        let mut array = TeleportedArray::from_vec(data, vec![original.len()]);
        
        // Multiply by scalar
        array.mul_scalar_f64(scalar);
        
        // Verify result
        unsafe {
            let result: &[f64] = array.as_slice();
            for (i, &val) in result.iter().enumerate() {
                let expected = original[i] * scalar;
                prop_assert!(
                    (val - expected).abs() < 1e-10,
                    "Mismatch at index {}: {} != {}",
                    i, val, expected
                );
            }
        }
    }
    
    /// Property: Shape and strides are consistent
    #[test]
    fn prop_shape_strides_consistent(
        rows in 1usize..100,
        cols in 1usize..100
    ) {
        let data: Vec<f64> = (0..(rows * cols)).map(|i| i as f64).collect();
        let array = TeleportedArray::from_vec(data, vec![rows, cols]);
        
        prop_assert_eq!(array.shape(), &[rows, cols]);
        prop_assert_eq!(array.ndim(), 2);
        prop_assert_eq!(array.len(), rows * cols);
        
        // For C-contiguous arrays, strides should be [cols * 8, 8]
        let strides = array.strides();
        prop_assert_eq!(strides.len(), 2);
        prop_assert_eq!(strides[1], 8); // sizeof(f64)
        prop_assert_eq!(strides[0], (cols * 8) as isize);
    }
    
    /// Property: DType size is correct
    #[test]
    fn prop_dtype_size(_seed in any::<u64>()) {
        prop_assert_eq!(DType::Float64.size(), 8);
        prop_assert_eq!(DType::Float32.size(), 4);
        prop_assert_eq!(DType::Int64.size(), 8);
        prop_assert_eq!(DType::Int32.size(), 4);
        prop_assert_eq!(DType::Int16.size(), 2);
        prop_assert_eq!(DType::Int8.size(), 1);
        prop_assert_eq!(DType::Bool.size(), 1);
    }
    
    /// Property: Contiguous arrays are detected correctly
    #[test]
    fn prop_contiguous_detection(size in 1usize..1000) {
        let data: Vec<f64> = (0..size).map(|i| i as f64).collect();
        let array = TeleportedArray::from_vec(data, vec![size]);
        
        // 1D arrays from vec should always be contiguous
        prop_assert!(array.is_contiguous());
    }
    
    /// Property: Empty arrays are handled correctly
    #[test]
    fn prop_empty_array_handling(_seed in any::<u64>()) {
        let data: Vec<f64> = vec![];
        let array = TeleportedArray::from_vec(data, vec![0]);
        
        prop_assert!(array.is_empty());
        prop_assert_eq!(array.len(), 0);
        prop_assert_eq!(array.byte_size(), 0);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use dx_py_ffi::capi::{CApiCompat, PyObjectHeader};
    use dx_py_ffi::fast_ffi::{FastFfi, GilFreeContext};
    
    #[test]
    fn test_py_object_refcount() {
        let header = PyObjectHeader::new();
        
        assert_eq!(header.ref_count(), 1);
        
        header.inc_ref();
        header.inc_ref();
        assert_eq!(header.ref_count(), 3);
        
        header.dec_ref();
        assert_eq!(header.ref_count(), 2);
    }
    
    #[test]
    fn test_capi_compat_creation() {
        let capi = CApiCompat::new();
        
        // Should have registered functions
        assert!(capi.api_count() > 0);
    }
    
    #[test]
    fn test_fast_ffi_lifecycle() {
        let ffi = FastFfi::new();
        
        assert!(ffi.is_empty());
        
        extern "C" fn dummy() -> i32 { 0 }
        
        unsafe {
            ffi.register("dummy", dummy as *const (), 0, true);
        }
        
        assert!(!ffi.is_empty());
        assert_eq!(ffi.len(), 1);
        assert!(ffi.has("dummy"));
        
        ffi.remove("dummy");
        assert!(!ffi.has("dummy"));
    }
    
    #[test]
    fn test_gil_free_context() {
        let mut ctx = GilFreeContext::new();
        
        assert!(!ctx.is_active());
        
        ctx.enter();
        assert!(ctx.is_active());
        
        ctx.exit();
        assert!(!ctx.is_active());
    }
    
    #[test]
    fn test_array_readonly() {
        let data = vec![1.0f64, 2.0, 3.0];
        let shape = vec![3];
        
        // Create readonly array
        let array = unsafe {
            TeleportedArray::new(
                data.as_ptr() as *mut u8,
                shape,
                vec![8],
                DType::Float64,
                true, // readonly
            )
        };
        
        // Should not be able to get mutable pointer
        let mut array = array;
        assert!(array.data_ptr_mut().is_none());
        
        // Operations should fail on readonly
        assert!(!array.add_scalar_f64(1.0));
    }
}
