//! Direct tarball extraction - fast path for cold installs
//! No binary conversion, just extract like Bun does

use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::path::Path;
use tar::Archive;

/// Direct extractor - extracts tarball without binary conversion
pub struct DirectExtractor;

impl DirectExtractor {
    /// Extract tarball to node_modules - FAST!
    pub fn extract(tgz_path: &Path, target_dir: &Path) -> io::Result<()> {
        let file = File::open(tgz_path)?;
        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);

        // Extract all entries
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;

            // Skip "package/" prefix that npm tarballs have
            let path_str = path.to_string_lossy();
            let clean_path = path_str.strip_prefix("package/").unwrap_or(&path_str);

            let target_path = target_dir.join(clean_path);

            // Create parent directories
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Extract file
            if entry.header().entry_type().is_file() {
                entry.unpack(&target_path)?;
            } else if entry.header().entry_type().is_dir() {
                std::fs::create_dir_all(&target_path)?;
            }
        }

        Ok(())
    }

    /// Parallel extraction of multiple packages (single-threaded for now)
    pub fn extract_many(packages: &[(std::path::PathBuf, std::path::PathBuf)]) -> io::Result<()> {
        for (tgz_path, target_dir) in packages {
            Self::extract(tgz_path, target_dir)?;
        }
        Ok(())
    }
}
