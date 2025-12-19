//! Property tests for Node.js buffer module.
//!
//! Feature: dx-js-compatibility, Property 4: Buffer Encoding Round-Trip
//! Validates: Requirements 4.2, 4.7
//!
//! Property: For any string and any supported encoding (utf8, ascii, base64, hex, latin1),
//! creating a Buffer with `Buffer.from(string, encoding)` and then calling
//! `buffer.toString(encoding)` SHALL produce the original string.

use dx_compat_node::buffer::{Buffer, Encoding};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 4: Buffer Encoding Round-Trip - UTF-8
    /// For any valid UTF-8 string, Buffer.from(s, utf8).toString(utf8) SHALL equal s.
    #[test]
    fn buffer_utf8_round_trip(s in "\\PC{0,1000}") {
        let buf = Buffer::from_string(&s, Encoding::Utf8);
        let result = buf.to_string(Encoding::Utf8);

        prop_assert_eq!(
            s,
            result,
            "UTF-8 round-trip should preserve string content"
        );
    }

    /// Property 4: Buffer Encoding Round-Trip - Hex
    /// For any byte sequence, encoding to hex and back should preserve data.
    #[test]
    fn buffer_hex_round_trip(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let buf = Buffer::from_vec(data.clone());
        let hex_str = buf.to_string(Encoding::Hex);
        let buf2 = Buffer::from_string(&hex_str, Encoding::Hex);

        prop_assert_eq!(
            &data[..],
            buf2.as_bytes(),
            "Hex round-trip should preserve byte content"
        );
    }

    /// Property 4: Buffer Encoding Round-Trip - Base64
    /// For any byte sequence, encoding to base64 and back should preserve data.
    #[test]
    fn buffer_base64_round_trip(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let buf = Buffer::from_vec(data.clone());
        let base64_str = buf.to_string(Encoding::Base64);
        let buf2 = Buffer::from_string(&base64_str, Encoding::Base64);

        prop_assert_eq!(
            &data[..],
            buf2.as_bytes(),
            "Base64 round-trip should preserve byte content"
        );
    }

    /// Property 4: Buffer Encoding Round-Trip - ASCII
    /// For any ASCII string, Buffer.from(s, ascii).toString(ascii) SHALL equal s.
    #[test]
    fn buffer_ascii_round_trip(s in "[\\x00-\\x7F]{0,1000}") {
        let buf = Buffer::from_string(&s, Encoding::Ascii);
        let result = buf.to_string(Encoding::Ascii);

        prop_assert_eq!(
            s,
            result,
            "ASCII round-trip should preserve string content"
        );
    }

    /// Property 4: Buffer Encoding Round-Trip - Latin1
    /// For any Latin1 string, Buffer.from(s, latin1).toString(latin1) SHALL equal s.
    #[test]
    fn buffer_latin1_round_trip(s in "[\\x00-\\xFF]{0,1000}") {
        let buf = Buffer::from_string(&s, Encoding::Latin1);
        let result = buf.to_string(Encoding::Latin1);

        prop_assert_eq!(
            s,
            result,
            "Latin1 round-trip should preserve string content"
        );
    }

    /// Property: Buffer.alloc creates zero-filled buffer
    /// Buffer.alloc(n) SHALL create a buffer of length n filled with zeros.
    #[test]
    fn buffer_alloc_creates_zeros(size in 0usize..10000) {
        let buf = Buffer::alloc(size);

        prop_assert_eq!(
            buf.len(),
            size,
            "Buffer.alloc should create buffer of specified size"
        );
        prop_assert!(
            buf.as_bytes().iter().all(|&b| b == 0),
            "Buffer.alloc should create zero-filled buffer"
        );
    }

    /// Property: Buffer.concat preserves all data
    /// Buffer.concat([b1, b2, ...]) SHALL contain all bytes from all buffers in order.
    #[test]
    fn buffer_concat_preserves_data(
        buffers in prop::collection::vec(
            prop::collection::vec(any::<u8>(), 0..100),
            0..10
        )
    ) {
        let bufs: Vec<Buffer> = buffers.iter()
            .map(|data| Buffer::from_vec(data.clone()))
            .collect();
        
        let concatenated = Buffer::concat(&bufs);
        
        // Expected: all bytes in order
        let expected: Vec<u8> = buffers.iter().flatten().cloned().collect();

        prop_assert_eq!(
            concatenated.as_bytes(),
            &expected[..],
            "Buffer.concat should preserve all bytes in order"
        );
    }

    /// Property: Buffer.slice creates view of original data
    /// Buffer.slice(start, end) SHALL return bytes from start to end.
    #[test]
    fn buffer_slice_returns_correct_range(
        data in prop::collection::vec(any::<u8>(), 1..1000),
        start_pct in 0.0f64..0.5,
        len_pct in 0.1f64..0.5
    ) {
        let buf = Buffer::from_vec(data.clone());
        let start = (data.len() as f64 * start_pct) as usize;
        let len = ((data.len() - start) as f64 * len_pct) as usize;
        let end = start + len;

        if end <= data.len() && start < end {
            let slice = buf.slice(start, end);

            prop_assert_eq!(
                slice.as_bytes(),
                &data[start..end],
                "Buffer.slice should return correct range"
            );
        }
    }

    /// Property: Buffer length is consistent
    /// Buffer.len() SHALL equal the number of bytes in the buffer.
    #[test]
    fn buffer_length_is_consistent(data in prop::collection::vec(any::<u8>(), 0..10000)) {
        let buf = Buffer::from_vec(data.clone());

        prop_assert_eq!(
            buf.len(),
            data.len(),
            "Buffer.len() should equal input data length"
        );
        prop_assert_eq!(
            buf.is_empty(),
            data.is_empty(),
            "Buffer.is_empty() should match input data"
        );
    }

    /// Property: Buffer read operations are correct
    /// read_u8, read_u16_be, read_u32_be SHALL return correct values.
    #[test]
    fn buffer_read_operations_correct(
        b0 in any::<u8>(),
        b1 in any::<u8>(),
        b2 in any::<u8>(),
        b3 in any::<u8>()
    ) {
        let data = vec![b0, b1, b2, b3];
        let buf = Buffer::from_vec(data);

        // read_u8
        prop_assert_eq!(buf.read_u8(0), Some(b0));
        prop_assert_eq!(buf.read_u8(1), Some(b1));
        prop_assert_eq!(buf.read_u8(4), None); // Out of bounds

        // read_u16_be
        let expected_u16 = u16::from_be_bytes([b0, b1]);
        prop_assert_eq!(buf.read_u16_be(0), Some(expected_u16));
        prop_assert_eq!(buf.read_u16_be(3), None); // Not enough bytes

        // read_u32_be
        let expected_u32 = u32::from_be_bytes([b0, b1, b2, b3]);
        prop_assert_eq!(buf.read_u32_be(0), Some(expected_u32));
        prop_assert_eq!(buf.read_u32_be(1), None); // Not enough bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_from_slice() {
        let data = b"hello world";
        let buf = Buffer::from_slice(data);
        assert_eq!(buf.as_bytes(), data);
    }

    #[test]
    fn test_buffer_into_bytes() {
        let data = vec![1, 2, 3, 4, 5];
        let buf = Buffer::from_vec(data.clone());
        let bytes = buf.into_bytes();
        assert_eq!(&bytes[..], &data[..]);
    }

    #[test]
    fn test_hex_encoding() {
        let buf = Buffer::from_string("48656c6c6f", Encoding::Hex);
        assert_eq!(buf.to_string(Encoding::Utf8), "Hello");
    }

    #[test]
    fn test_base64_encoding() {
        let buf = Buffer::from_string("SGVsbG8=", Encoding::Base64);
        assert_eq!(buf.to_string(Encoding::Utf8), "Hello");
    }

    #[test]
    fn test_empty_buffer() {
        let buf = Buffer::alloc(0);
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_buffer_from_trait() {
        let buf: Buffer = vec![1, 2, 3].into();
        assert_eq!(buf.len(), 3);

        let buf: Buffer = (&[4, 5, 6][..]).into();
        assert_eq!(buf.len(), 3);
    }
}
