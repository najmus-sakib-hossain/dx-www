//! DXP File Format

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// DXP Package File
#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct DxpFile {
    pub version: u32,
    pub entries: Vec<DxpFileEntry>,
}

/// File entry in DXP package
#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct DxpFileEntry {
    pub path: String,
    pub size: u64,
    pub compressed_size: u64,
    pub hash: String,
    pub data: Vec<u8>,
}

impl DxpFile {
    /// Write DXP file to disk
    pub fn write(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        
        // Write magic
        file.write_all(b"DXPK")?;
        
        // Write version
        file.write_all(&self.version.to_le_bytes())?;
        
        // Write entry count
        file.write_all(&(self.entries.len() as u32).to_le_bytes())?;
        
        // Write entries (using bincode for simplicity)
        let entries_data = bincode::encode_to_vec(&self.entries, bincode::config::standard())?;
        file.write_all(&(entries_data.len() as u64).to_le_bytes())?;
        file.write_all(&entries_data)?;
        
        Ok(())
    }

    /// Read DXP file from disk
    pub fn read(path: &Path) -> Result<Self> {
        use std::io::Read;
        
        let mut file = File::open(path)?;
        
        // Read magic
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;
        if &magic != b"DXPK" {
            anyhow::bail!("Invalid DXP file: bad magic");
        }
        
        // Read version
        let mut version_bytes = [0u8; 4];
        file.read_exact(&mut version_bytes)?;
        let version = u32::from_le_bytes(version_bytes);
        
        // Read entry count
        let mut count_bytes = [0u8; 4];
        file.read_exact(&mut count_bytes)?;
        let _count = u32::from_le_bytes(count_bytes);
        
        // Read entries
        let mut size_bytes = [0u8; 8];
        file.read_exact(&mut size_bytes)?;
        let entries_size = u64::from_le_bytes(size_bytes);
        
        let mut entries_data = vec![0u8; entries_size as usize];
        file.read_exact(&mut entries_data)?;
        
        let (entries, _): (Vec<DxpFileEntry>, _) = 
            bincode::decode_from_slice(&entries_data, bincode::config::standard())?;
        
        Ok(Self { version, entries })
    }
}
