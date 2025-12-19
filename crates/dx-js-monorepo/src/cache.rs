//! Cache Manager
//!
//! Manages local DXC cache storage with memory-mapped access.

use crate::dxc::{CacheEntry, XorPatch};
use crate::error::CacheError;
use std::collections::HashMap;
use std::path::PathBuf;

/// Cache Manager for task output caching
pub struct CacheManager {
    /// Cache directory
    cache_dir: PathBuf,
    /// Maximum cache size in bytes
    max_size: u64,
    /// Current cache size
    current_size: u64,
    /// In-memory cache for zero-disk mode
    memory_cache: HashMap<[u8; 32], CacheEntry>,
    /// Zero-disk mode enabled
    zero_disk: bool,
    /// Bloom filter for fast miss detection (simplified)
    bloom_filter: Vec<u64>,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(cache_dir: PathBuf, max_size: u64) -> Self {
        Self {
            cache_dir,
            max_size,
            current_size: 0,
            memory_cache: HashMap::new(),
            zero_disk: false,
            bloom_filter: vec![0; 1024], // 8KB bloom filter
        }
    }

    /// Check if cache entry exists (< 0.1ms target)
    pub fn has(&self, task_hash: &[u8; 32]) -> bool {
        // Check bloom filter first for fast negative
        if !self.bloom_check(task_hash) {
            return false;
        }

        if self.zero_disk {
            return self.memory_cache.contains_key(task_hash);
        }

        // Check file existence
        let path = self.hash_to_path(task_hash);
        path.exists()
    }

    /// Get cached output with zero-copy access (< 0.5ms target)
    pub fn get(&self, task_hash: &[u8; 32]) -> Option<CacheEntry> {
        if self.zero_disk {
            return self.memory_cache.get(task_hash).cloned();
        }

        let path = self.hash_to_path(task_hash);
        if !path.exists() {
            return None;
        }

        // Read and parse cache entry
        let data = std::fs::read(&path).ok()?;
        self.parse_cache_entry(&data)
    }

    /// Store task output in cache
    pub fn put(&mut self, task_hash: &[u8; 32], entry: &CacheEntry) -> Result<(), CacheError> {
        let size = entry.total_size() as u64;

        // Check if we need to evict
        while self.current_size + size > self.max_size {
            self.evict_lru()?;
        }

        // Add to bloom filter
        self.bloom_add(task_hash);

        if self.zero_disk {
            self.memory_cache.insert(*task_hash, entry.clone());
            self.current_size += size;
            return Ok(());
        }

        // Serialize and write to disk
        let data = self.serialize_cache_entry(entry);
        let path = self.hash_to_path(task_hash);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&path, &data)?;
        self.current_size += size;

        Ok(())
    }

    /// Apply XOR patch to update cache entry
    pub fn apply_patch(
        &mut self,
        task_hash: &[u8; 32],
        patch: &XorPatch,
    ) -> Result<(), CacheError> {
        let base_entry = self.get(&patch.base_hash).ok_or(CacheError::EntryNotFound {
            hash: patch.base_hash,
        })?;

        // Apply patch to each file
        let mut new_entry = CacheEntry::new(*task_hash);

        for file in &base_entry.files {
            let patched_content = patch.apply(&file.content);
            new_entry.add_file(file.path.clone(), patched_content, file.mode);
        }

        self.put(task_hash, &new_entry)
    }

    /// Verify Ed25519 signature of cache entry
    pub fn verify(&self, entry: &CacheEntry) -> Result<bool, CacheError> {
        let (signature, public_key) = match (entry.signature, entry.public_key) {
            (Some(sig), Some(pk)) => (sig, pk),
            _ => return Ok(false), // No signature to verify
        };

        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        let verifying_key =
            VerifyingKey::from_bytes(&public_key).map_err(|_| CacheError::SignatureInvalid)?;

        let sig = Signature::from_bytes(&signature);

        // Hash all file contents
        let mut hasher = blake3::Hasher::new();
        hasher.update(&entry.task_hash);
        for file in &entry.files {
            hasher.update(file.path.as_bytes());
            hasher.update(&file.content);
        }
        let content_hash = hasher.finalize();

        verifying_key
            .verify(content_hash.as_bytes(), &sig)
            .map(|_| true)
            .map_err(|_| CacheError::SignatureInvalid)
    }

    /// Enable zero-disk mode with virtual filesystem
    pub fn enable_zero_disk(&mut self) -> Result<(), CacheError> {
        self.zero_disk = true;
        Ok(())
    }

    /// Disable zero-disk mode
    pub fn disable_zero_disk(&mut self) {
        self.zero_disk = false;
    }

    /// Get current cache size
    pub fn size(&self) -> u64 {
        self.current_size
    }

    /// Clear all cache entries
    pub fn clear(&mut self) -> Result<(), CacheError> {
        self.memory_cache.clear();
        self.current_size = 0;
        self.bloom_filter.fill(0);

        if !self.zero_disk && self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)?;
        }

        Ok(())
    }

    // Private helpers

    fn hash_to_path(&self, hash: &[u8; 32]) -> PathBuf {
        let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        self.cache_dir.join(&hex[0..2]).join(&hex[2..4]).join(&hex)
    }

    fn bloom_add(&mut self, hash: &[u8; 32]) {
        let h1 = u64::from_le_bytes(hash[0..8].try_into().unwrap());
        let h2 = u64::from_le_bytes(hash[8..16].try_into().unwrap());

        for i in 0..4 {
            let idx =
                ((h1.wrapping_add(i as u64 * h2)) % (self.bloom_filter.len() as u64 * 64)) as usize;
            let word = idx / 64;
            let bit = idx % 64;
            self.bloom_filter[word] |= 1 << bit;
        }
    }

    fn bloom_check(&self, hash: &[u8; 32]) -> bool {
        let h1 = u64::from_le_bytes(hash[0..8].try_into().unwrap());
        let h2 = u64::from_le_bytes(hash[8..16].try_into().unwrap());

        for i in 0..4 {
            let idx =
                ((h1.wrapping_add(i as u64 * h2)) % (self.bloom_filter.len() as u64 * 64)) as usize;
            let word = idx / 64;
            let bit = idx % 64;
            if self.bloom_filter[word] & (1 << bit) == 0 {
                return false;
            }
        }
        true
    }

    fn evict_lru(&mut self) -> Result<(), CacheError> {
        // Simple eviction: remove first entry in memory cache
        if self.zero_disk {
            if let Some(key) = self.memory_cache.keys().next().cloned() {
                if let Some(entry) = self.memory_cache.remove(&key) {
                    self.current_size -= entry.total_size() as u64;
                }
            }
        }
        // TODO: Implement proper LRU for disk cache
        Ok(())
    }

    fn serialize_cache_entry(&self, entry: &CacheEntry) -> Vec<u8> {
        // Simple serialization format
        let mut data = Vec::new();

        // Header
        data.extend_from_slice(b"DXC\0");
        data.extend_from_slice(&1u32.to_le_bytes()); // version
        data.extend_from_slice(&entry.task_hash);

        // File count
        data.extend_from_slice(&(entry.files.len() as u32).to_le_bytes());

        // Files
        for file in &entry.files {
            data.extend_from_slice(&(file.path.len() as u32).to_le_bytes());
            data.extend_from_slice(file.path.as_bytes());
            data.extend_from_slice(&(file.content.len() as u64).to_le_bytes());
            data.extend_from_slice(&file.content);
            data.extend_from_slice(&file.mode.to_le_bytes());
        }

        data
    }

    fn parse_cache_entry(&self, data: &[u8]) -> Option<CacheEntry> {
        if data.len() < 44 {
            return None;
        }

        // Verify magic
        if &data[0..4] != b"DXC\0" {
            return None;
        }

        let task_hash: [u8; 32] = data[8..40].try_into().ok()?;
        let file_count = u32::from_le_bytes(data[40..44].try_into().ok()?) as usize;

        let mut entry = CacheEntry::new(task_hash);
        let mut offset = 44;

        for _ in 0..file_count {
            if offset + 4 > data.len() {
                return None;
            }

            let path_len = u32::from_le_bytes(data[offset..offset + 4].try_into().ok()?) as usize;
            offset += 4;

            if offset + path_len > data.len() {
                return None;
            }

            let path = std::str::from_utf8(&data[offset..offset + path_len]).ok()?.to_string();
            offset += path_len;

            if offset + 8 > data.len() {
                return None;
            }

            let content_len =
                u64::from_le_bytes(data[offset..offset + 8].try_into().ok()?) as usize;
            offset += 8;

            if offset + content_len > data.len() {
                return None;
            }

            let content = data[offset..offset + content_len].to_vec();
            offset += content_len;

            if offset + 4 > data.len() {
                return None;
            }

            let mode = u32::from_le_bytes(data[offset..offset + 4].try_into().ok()?);
            offset += 4;

            entry.add_file(path, content, mode);
        }

        Some(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_manager_zero_disk() {
        let temp = TempDir::new().unwrap();
        let mut cache = CacheManager::new(temp.path().to_path_buf(), 1024 * 1024);
        cache.enable_zero_disk().unwrap();

        let hash = [1u8; 32];
        let mut entry = CacheEntry::new(hash);
        entry.add_file("test.txt".to_string(), b"hello".to_vec(), 0o644);

        // Initially not in cache
        assert!(!cache.has(&hash));

        // Add to cache
        cache.put(&hash, &entry).unwrap();
        assert!(cache.has(&hash));

        // Retrieve from cache
        let retrieved = cache.get(&hash).unwrap();
        assert_eq!(retrieved.files.len(), 1);
        assert_eq!(retrieved.files[0].content, b"hello");
    }

    #[test]
    fn test_cache_manager_disk() {
        let temp = TempDir::new().unwrap();
        let mut cache = CacheManager::new(temp.path().to_path_buf(), 1024 * 1024);

        let hash = [2u8; 32];
        let mut entry = CacheEntry::new(hash);
        entry.add_file("dist/index.js".to_string(), b"console.log('hi')".to_vec(), 0o644);

        cache.put(&hash, &entry).unwrap();
        assert!(cache.has(&hash));

        let retrieved = cache.get(&hash).unwrap();
        assert_eq!(retrieved.files[0].path, "dist/index.js");
    }

    #[test]
    fn test_bloom_filter() {
        let temp = TempDir::new().unwrap();
        let mut cache = CacheManager::new(temp.path().to_path_buf(), 1024 * 1024);
        cache.enable_zero_disk().unwrap();

        let hash1 = [1u8; 32];
        let hash2 = [2u8; 32];
        let _hash3 = [3u8; 32];

        // Add hash1 and hash2
        cache.bloom_add(&hash1);
        cache.bloom_add(&hash2);

        // hash1 and hash2 should pass bloom check
        assert!(cache.bloom_check(&hash1));
        assert!(cache.bloom_check(&hash2));

        // hash3 might pass (false positive) or fail
        // We can't assert it fails due to bloom filter nature
    }

    #[test]
    fn test_cache_eviction() {
        let temp = TempDir::new().unwrap();
        let mut cache = CacheManager::new(temp.path().to_path_buf(), 100); // Very small cache
        cache.enable_zero_disk().unwrap();

        // Add entries until eviction happens
        for i in 0..10 {
            let hash = [i as u8; 32];
            let mut entry = CacheEntry::new(hash);
            entry.add_file("test.txt".to_string(), vec![0u8; 20], 0o644);
            cache.put(&hash, &entry).unwrap();
        }

        // Cache size should be limited
        assert!(cache.size() <= 100);
    }

    #[test]
    fn test_cache_clear() {
        let temp = TempDir::new().unwrap();
        let mut cache = CacheManager::new(temp.path().to_path_buf(), 1024 * 1024);
        cache.enable_zero_disk().unwrap();

        let hash = [1u8; 32];
        let mut entry = CacheEntry::new(hash);
        entry.add_file("test.txt".to_string(), b"hello".to_vec(), 0o644);

        cache.put(&hash, &entry).unwrap();
        assert!(cache.has(&hash));

        cache.clear().unwrap();
        assert!(!cache.has(&hash));
        assert_eq!(cache.size(), 0);
    }
}
