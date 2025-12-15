//! Common test utilities.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test fixture for managing temporary files.
pub struct TestFixture {
    pub temp_dir: TempDir,
}

impl TestFixture {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp dir"),
        }
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.temp_dir.path().join(name)
    }

    pub fn create_temp_file(&self, name: &str, content: &[u8]) -> PathBuf {
        let path = self.path(name);
        fs::write(&path, content).expect("Failed to write temp file");
        path
    }

    /// Create a minimal test image (1x1 PGM).
    pub fn create_test_image(&self, name: &str) -> PathBuf {
        // PGM format: simple grayscale
        let pgm = b"P5\n1 1\n255\n\x80";
        self.create_temp_file(name, pgm)
    }

    /// Create a test text file.
    pub fn create_test_text_file(&self, name: &str, content: &str) -> PathBuf {
        self.create_temp_file(name, content.as_bytes())
    }

    /// Create a minimal test audio file placeholder (just bytes, not real audio).
    pub fn create_test_audio(&self, name: &str) -> PathBuf {
        // Note: This won't work with ffmpeg but we're testing function availability
        self.create_temp_file(name, b"fake audio content")
    }

    /// Create a minimal test video file placeholder.
    pub fn create_test_video(&self, name: &str) -> PathBuf {
        self.create_temp_file(name, b"fake video content")
    }
}
