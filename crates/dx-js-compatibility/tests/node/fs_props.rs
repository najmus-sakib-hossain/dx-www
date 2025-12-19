//! Property tests for Node.js fs module.
//!
//! Feature: dx-js-compatibility, Property 1: File System Read/Write Round-Trip
//! Validates: Requirements 2.1, 2.2
//!
//! Property: For any valid file path and any byte sequence, writing the data with
//! `fs.writeFile()` and then reading it with `fs.readFile()` SHALL produce the
//! original byte sequence.

use dx_compat_node::fs;
use proptest::prelude::*;
use tempfile::tempdir;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 1: File System Read/Write Round-Trip
    /// For any byte sequence, write then read SHALL produce the original data.
    #[test]
    fn fs_read_write_round_trip(data in prop::collection::vec(any::<u8>(), 0..10000)) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = tempdir().unwrap();
            let path = dir.path().join("test_file.bin");

            // Write data
            fs::write_file(&path, &data).await.unwrap();

            // Read data back
            let read_data = fs::read_file(&path).await.unwrap();

            prop_assert_eq!(
                &data[..],
                &read_data[..],
                "Read data should match written data"
            );
        });
    }

    /// Property 1: File System Read/Write Round-Trip with text content
    /// For any valid UTF-8 string, write then read SHALL produce the original string.
    #[test]
    fn fs_read_write_round_trip_text(content in "\\PC{0,5000}") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = tempdir().unwrap();
            let path = dir.path().join("test_file.txt");

            // Write text content
            fs::write_file(&path, content.as_bytes()).await.unwrap();

            // Read data back
            let read_data = fs::read_file(&path).await.unwrap();
            let read_content = String::from_utf8_lossy(&read_data);

            prop_assert_eq!(
                content,
                read_content,
                "Read text should match written text"
            );
        });
    }

    /// Property 1: File System Read/Write Round-Trip - sync variant
    /// For any byte sequence, sync write then read SHALL produce the original data.
    #[test]
    fn fs_read_write_round_trip_sync(data in prop::collection::vec(any::<u8>(), 0..10000)) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_file_sync.bin");

        // Write data synchronously
        fs::sync::write_file_sync(&path, &data).unwrap();

        // Read data back synchronously
        let read_data = fs::sync::read_file_sync(&path).unwrap();

        prop_assert_eq!(
            &data[..],
            &read_data[..],
            "Sync read data should match written data"
        );
    }

    /// Property 1: File System Read/Write Round-Trip - large files (mmap path)
    /// For large byte sequences (>1MB), write then read SHALL produce the original data.
    #[test]
    fn fs_read_write_round_trip_large(
        // Generate data larger than MMAP_THRESHOLD (1MB)
        base in prop::collection::vec(any::<u8>(), 100..1000),
        repeat in 1100usize..1200
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = tempdir().unwrap();
            let path = dir.path().join("large_file.bin");

            // Create data larger than 1MB to trigger mmap path
            let data: Vec<u8> = base.iter().cycle().take(repeat * base.len()).cloned().collect();
            
            // Only test if we actually have > 1MB
            if data.len() > 1_048_576 {
                // Write data
                fs::write_file(&path, &data).await.unwrap();

                // Read data back (should use mmap)
                let read_data = fs::read_file(&path).await.unwrap();

                prop_assert_eq!(
                    data.len(),
                    read_data.len(),
                    "Large file read length should match written length"
                );
                prop_assert_eq!(
                    &data[..],
                    &read_data[..],
                    "Large file read data should match written data"
                );
            }
        });
    }

    /// Property: File stats are consistent after write
    /// After writing N bytes, stat().size SHALL equal N.
    #[test]
    fn fs_stat_size_matches_written_data(data in prop::collection::vec(any::<u8>(), 0..10000)) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = tempdir().unwrap();
            let path = dir.path().join("stat_test.bin");

            // Write data
            fs::write_file(&path, &data).await.unwrap();

            // Get stats
            let stats = fs::stat(&path).await.unwrap();

            prop_assert_eq!(
                stats.size as usize,
                data.len(),
                "File size should match written data length"
            );
            prop_assert!(stats.is_file, "Should be a file");
            prop_assert!(!stats.is_directory, "Should not be a directory");
        });
    }

    /// Property: Directory operations are consistent
    /// After mkdir, stat().is_directory SHALL be true.
    #[test]
    fn fs_mkdir_creates_directory(dirname in "[a-zA-Z][a-zA-Z0-9_]{0,20}") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = tempdir().unwrap();
            let path = dir.path().join(&dirname);

            // Create directory
            fs::mkdir(&path, false).await.unwrap();

            // Get stats
            let stats = fs::stat(&path).await.unwrap();

            prop_assert!(stats.is_directory, "Should be a directory");
            prop_assert!(!stats.is_file, "Should not be a file");
        });
    }

    /// Property: readdir returns created files
    /// After creating N files in a directory, readdir SHALL return N entries.
    #[test]
    fn fs_readdir_returns_created_files(
        filenames in prop::collection::vec("[a-zA-Z][a-zA-Z0-9_]{0,10}\\.txt", 1..10)
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dir = tempdir().unwrap();
            
            // Create unique filenames
            let unique_names: std::collections::HashSet<_> = filenames.iter().collect();
            
            // Create files
            for name in &unique_names {
                let path = dir.path().join(name);
                fs::write_file(&path, b"test").await.unwrap();
            }

            // Read directory
            let entries = fs::read_dir(dir.path()).await.unwrap();

            prop_assert_eq!(
                entries.len(),
                unique_names.len(),
                "Directory should contain {} files, found {}",
                unique_names.len(),
                entries.len()
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_file_round_trip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.txt");

        fs::write_file(&path, b"").await.unwrap();
        let data = fs::read_file(&path).await.unwrap();

        assert!(data.is_empty());
    }

    #[tokio::test]
    async fn test_rename_preserves_content() {
        let dir = tempdir().unwrap();
        let from = dir.path().join("from.txt");
        let to = dir.path().join("to.txt");

        let content = b"test content";
        fs::write_file(&from, content).await.unwrap();
        fs::rename(&from, &to).await.unwrap();

        let data = fs::read_file(&to).await.unwrap();
        assert_eq!(&data[..], content);
    }

    #[tokio::test]
    async fn test_unlink_removes_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("to_delete.txt");

        fs::write_file(&path, b"delete me").await.unwrap();
        assert!(path.exists());

        fs::unlink(&path).await.unwrap();
        assert!(!path.exists());
    }
}
