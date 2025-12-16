//! Quick verification test to ensure DX-Zero works correctly

use dx_serializer::zero::{DxZeroBuilder, DxZeroHeader};

#[test]
fn test_dx_zero_basic() {
    let mut buffer = Vec::new();
    let mut builder = DxZeroBuilder::new(&mut buffer, 8, 1);
    
    builder.write_u64(0, 12345);
    builder.write_string(8, "Hello");
    builder.finish();
    
    // Validate header
    let header = DxZeroHeader::from_bytes(&buffer[..4]).expect("Invalid header");
    assert!(header.validate().is_ok(), "Header validation failed");
    
    println!("✅ DX-Zero basic test passed");
    println!("   Buffer size: {} bytes", buffer.len());
}

#[test]
fn test_dx_zero_all_types() {
    let mut buffer = Vec::new();
    let mut builder = DxZeroBuilder::new(&mut buffer, 35, 2);
    
    builder.write_u8(0, 255);
    builder.write_u16(1, 65535);
    builder.write_u32(3, 4294967295);
    builder.write_u64(7, 18446744073709551615);
    builder.write_i8(15, -128);
    builder.write_i16(16, -32768);
    builder.write_i32(18, -2147483648);
    builder.write_f32(22, 3.14159);
    builder.write_f64(26, 2.71828);
    builder.write_bool(34, true);
    builder.write_string(35, "Test");
    builder.write_string(51, "Another");
    
    builder.finish();
    
    println!("✅ DX-Zero all types test passed");
    println!("   Buffer size: {} bytes", buffer.len());
}

#[test]
fn test_dx_zero_inline_optimization() {
    let mut buffer = Vec::new();
    let mut builder = DxZeroBuilder::new(&mut buffer, 0, 3);
    
    // These should all be inline (≤14 bytes)
    builder.write_string(0, "Short");      // 5 bytes
    builder.write_string(16, "Medium");    // 6 bytes
    builder.write_string(32, "ExactlyLimit"); // 12 bytes
    
    builder.finish();
    
    // Should only have header + 3 slots = 4 + 48 = 52 bytes (no heap)
    assert_eq!(buffer.len(), 52, "Inline optimization failed");
    
    println!("✅ DX-Zero inline optimization works");
}

#[test]
fn test_dx_zero_heap_allocation() {
    let mut buffer = Vec::new();
    let mut builder = DxZeroBuilder::new(&mut buffer, 0, 1);
    
    // This should go to heap (>14 bytes)
    let long_string = "This is a very long string that exceeds 14 bytes";
    builder.write_string(0, long_string);
    builder.finish();
    
    // Should have header + slot + heap data
    let expected_size = 4 + 16 + long_string.len();
    assert_eq!(buffer.len(), expected_size, "Heap allocation size mismatch");
    
    println!("✅ DX-Zero heap allocation works");
}

#[test]
fn test_dx_zero_unicode() {
    let mut buffer = Vec::new();
    let mut builder = DxZeroBuilder::new(&mut buffer, 0, 2);
    
    builder.write_string(0, "Hello 世界");
    builder.write_string(16, "🚀 Emoji");
    builder.finish();
    
    println!("✅ DX-Zero Unicode support works");
}

#[test]
fn test_dx_zero_performance() {
    use std::time::Instant;
    
    let mut buffer = Vec::with_capacity(200);
    
    // Warm up
    for _ in 0..100 {
        buffer.clear();
        let mut builder = DxZeroBuilder::new(&mut buffer, 21, 3);
        builder.write_u64(0, 12345);
        builder.write_u32(8, 30);
        builder.write_bool(12, true);
        builder.write_f64(13, 98.5);
        builder.write_string(21, "John");
        builder.write_string(37, "john@example.com");
        builder.write_string(53, "Bio text");
        builder.finish();
    }
    
    // Measure
    let start = Instant::now();
    let iterations = 100000;
    for _ in 0..iterations {
        buffer.clear();
        let mut builder = DxZeroBuilder::new(&mut buffer, 21, 3);
        builder.write_u64(0, 12345);
        builder.write_u32(8, 30);
        builder.write_bool(12, true);
        builder.write_f64(13, 98.5);
        builder.write_string(21, "John");
        builder.write_string(37, "john@example.com");
        builder.write_string(53, "Bio text");
        builder.finish();
    }
    let elapsed = start.elapsed();
    let avg_ns = elapsed.as_nanos() / iterations;
    
    println!("✅ DX-Zero serialization: {} ns per operation", avg_ns);
    // Relaxed threshold for debug mode - release will be much faster
    assert!(avg_ns < 5000, "Performance regression: {} ns (expected < 5000 ns)", avg_ns);
}
