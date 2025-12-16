//! Package Converter Implementation

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Archive;

use crate::format::{DxpFile, DxpFileEntry};

#[derive(Clone)]
pub struct PackageConverter {
    // Converter configuration
}

impl PackageConverter {
    pub fn new() -> Self {
        Self {}
    }

    /// Convert a .tgz file to .dxp
    pub async fn convert_file(&self, input: &Path, output: Option<&PathBuf>) -> Result<PathBuf> {
        // Read .tgz
        let tgz_data = std::fs::read(input)?;
        
        // Extract package name and version from path or tar contents
        let name = input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        // Generate output path
        let output_path = output
            .cloned()
            .unwrap_or_else(|| PathBuf::from(format!("{}.dxp", name)));
        
        // Convert
        self.convert_tgz(&tgz_data, &output_path).await?;
        
        Ok(output_path)
    }

    /// Convert package bytes to .dxp
    pub async fn convert_bytes(
        &self,
        name: &str,
        version: &str,
        tgz_data: &[u8],
        output_dir: &Path,
    ) -> Result<PathBuf> {
        let output_path = output_dir.join(format!("{}@{}.dxp", name, version));
        std::fs::create_dir_all(output_dir)?;
        
        self.convert_tgz(tgz_data, &output_path).await?;
        
        Ok(output_path)
    }

    /// Core conversion logic
    async fn convert_tgz(&self, tgz_data: &[u8], output: &Path) -> Result<()> {
        // Decompress gzip
        let gz = GzDecoder::new(tgz_data);
        let mut archive = Archive::new(gz);
        
        // Extract all files
        let mut entries = Vec::new();
        
        for entry_result in archive.entries()? {
            let mut entry = entry_result?;
            let path = entry.path()?.to_path_buf();
            
            // Skip directories
            if entry.header().entry_type().is_dir() {
                continue;
            }
            
            // Read file contents
            let mut contents = Vec::new();
            entry.read_to_end(&mut contents)?;
            
            // Compress with lz4
            let compressed = lz4_flex::compress_prepend_size(&contents);
            
            // Calculate hash
            let hash = blake3::hash(&contents);
            let hash_hex = format!("{}", hash.to_hex());
            
            entries.push(DxpFileEntry {
                path: path.to_string_lossy().to_string(),
                size: contents.len() as u64,
                compressed_size: compressed.len() as u64,
                hash: hash_hex,
                data: compressed,
            });
        }
        
        // Create .dxp file
        let dxp = DxpFile {
            version: 1,
            entries,
        };
        
        // Write to disk
        dxp.write(output)?;
        
        Ok(())
    }
}
