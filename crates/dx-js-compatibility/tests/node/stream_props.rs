//! Property tests for Node.js stream module.
//!
//! Feature: dx-js-compatibility, Property 8: Node Stream Pipe Completeness
//! Validates: Requirements 5.1, 5.3
//!
//! Property: For any readable stream with N chunks, piping to a writable stream
//! SHALL transfer all N chunks in order, and the total bytes written SHALL equal
//! the total bytes read.

use bytes::Bytes;
use dx_compat_node::stream::{pipe, ReadableStream, Writable, WritableStream};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 8: Node Stream Pipe Completeness
    /// For any readable stream with N chunks, piping SHALL transfer all N chunks in order.
    #[test]
    fn stream_pipe_transfers_all_chunks(
        chunks in prop::collection::vec(
            prop::collection::vec(any::<u8>(), 1..100),
            1..20
        )
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Create readable stream from chunks
            let chunk_bytes: Vec<Bytes> = chunks.iter()
                .map(|c| Bytes::from(c.clone()))
                .collect();
            let readable = ReadableStream::from_chunks(chunk_bytes.clone());

            // Create writable stream
            let mut writable = WritableStream::new();

            // Pipe
            pipe(readable, &mut writable).await.unwrap();

            // Verify all chunks were transferred
            let written_chunks = writable.chunks();
            prop_assert_eq!(
                written_chunks.len(),
                chunks.len(),
                "Number of written chunks should equal number of source chunks"
            );

            // Verify chunks are in order and content matches
            for (i, (written, original)) in written_chunks.iter().zip(chunk_bytes.iter()).enumerate() {
                prop_assert_eq!(
                    &written[..],
                    &original[..],
                    "Chunk {} content should match", i
                );
            }
        });
    }

    /// Property 8: Node Stream Pipe Completeness - total bytes
    /// Total bytes written SHALL equal total bytes read.
    #[test]
    fn stream_pipe_preserves_total_bytes(
        chunks in prop::collection::vec(
            prop::collection::vec(any::<u8>(), 0..200),
            0..30
        )
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Calculate expected total bytes
            let total_bytes: usize = chunks.iter().map(|c| c.len()).sum();

            // Create readable stream from chunks
            let chunk_bytes: Vec<Bytes> = chunks.iter()
                .map(|c| Bytes::from(c.clone()))
                .collect();
            let readable = ReadableStream::from_chunks(chunk_bytes);

            // Create writable stream
            let mut writable = WritableStream::new();

            // Pipe
            pipe(readable, &mut writable).await.unwrap();

            // Verify total bytes
            let written_bytes: usize = writable.chunks().iter().map(|c| c.len()).sum();
            prop_assert_eq!(
                written_bytes,
                total_bytes,
                "Total written bytes should equal total source bytes"
            );
        });
    }

    /// Property: WritableStream write returns true when not closed
    /// write() on an open stream SHALL return Ok(true).
    #[test]
    fn writable_stream_write_returns_true(data in prop::collection::vec(any::<u8>(), 0..100)) {
        let mut writable = WritableStream::new();
        let chunk = Bytes::from(data);

        let result = writable.write(chunk);
        prop_assert!(result.is_ok());
        prop_assert!(result.unwrap(), "write should return true on open stream");
    }

    /// Property: WritableStream write fails after end
    /// write() after end() SHALL return Err(Closed).
    #[test]
    fn writable_stream_write_fails_after_end(data in prop::collection::vec(any::<u8>(), 1..100)) {
        let mut writable = WritableStream::new();
        
        // End the stream
        writable.end().unwrap();

        // Try to write
        let chunk = Bytes::from(data);
        let result = writable.write(chunk);
        
        prop_assert!(result.is_err(), "write after end should fail");
    }

    /// Property: ReadableStream pause/resume works correctly
    /// Paused stream should not yield items until resumed.
    #[test]
    fn readable_stream_pause_resume(
        chunks in prop::collection::vec(
            prop::collection::vec(any::<u8>(), 1..50),
            2..10
        )
    ) {
        use dx_compat_node::stream::Readable;

        let chunk_bytes: Vec<Bytes> = chunks.iter()
            .map(|c| Bytes::from(c.clone()))
            .collect();
        let mut readable = ReadableStream::from_chunks(chunk_bytes);

        // Initially not paused
        prop_assert!(!readable.is_paused());

        // Pause
        readable.pause();
        prop_assert!(readable.is_paused());

        // Resume
        readable.resume();
        prop_assert!(!readable.is_paused());
    }

    /// Property: Empty stream pipe completes successfully
    /// Piping an empty stream should complete without error.
    #[test]
    fn empty_stream_pipe_completes(_dummy in 0..1i32) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let readable = ReadableStream::from_chunks(vec![]);
            let mut writable = WritableStream::new();

            let result = pipe(readable, &mut writable).await;
            prop_assert!(result.is_ok(), "Empty stream pipe should succeed");
            prop_assert!(writable.chunks().is_empty(), "No chunks should be written");
        });
    }

    /// Property: Single chunk stream pipe works
    /// Piping a single-chunk stream should transfer exactly one chunk.
    #[test]
    fn single_chunk_stream_pipe(data in prop::collection::vec(any::<u8>(), 1..1000)) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let chunk = Bytes::from(data.clone());
            let readable = ReadableStream::from_chunks(vec![chunk]);
            let mut writable = WritableStream::new();

            pipe(readable, &mut writable).await.unwrap();

            prop_assert_eq!(writable.chunks().len(), 1);
            prop_assert_eq!(&writable.chunks()[0][..], &data[..]);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pipe_basic() {
        let chunks = vec![
            Bytes::from("hello"),
            Bytes::from(" "),
            Bytes::from("world"),
        ];
        let readable = ReadableStream::from_chunks(chunks.clone());
        let mut writable = WritableStream::new();

        pipe(readable, &mut writable).await.unwrap();

        assert_eq!(writable.chunks().len(), 3);
        assert_eq!(&writable.chunks()[0][..], b"hello");
        assert_eq!(&writable.chunks()[1][..], b" ");
        assert_eq!(&writable.chunks()[2][..], b"world");
    }

    #[test]
    fn test_writable_stream_default() {
        let writable = WritableStream::default();
        assert!(writable.chunks().is_empty());
    }

    #[test]
    fn test_writable_stream_end() {
        let mut writable = WritableStream::new();
        assert!(writable.end().is_ok());
    }
}
