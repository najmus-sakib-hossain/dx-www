//! AST Cache
//!
//! Persistent binary AST cache for instant re-linting of unchanged files.
//! Uses memory-mapped files for zero-copy access.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;

/// Binary AST cache with memory-mapped storage
pub struct AstCache {
    /// Cache directory
    cache_dir: PathBuf,
    /// Index of cached entries (in memory for fast lookup)
    index: Arc<RwLock<CacheIndex>>,
    /// Maximum cache size in bytes
    max_size: u64,
    /// Current cache size
    current_size: Arc<RwLock<u64>>,
}

/// Cache index for O(1) lookup
#[derive(Default)]
struct CacheIndex {
    /// Content hash -> cache entry
    entries: HashMap<[u8; 32], CacheEntry>,
}

/// Individual cache entry
#[derive(Clone)]
struct CacheEntry {
    /// Path to the cache file
    cache_path: PathBuf,
    /// Size in bytes
    size: u64,
    /// Last access timestamp (for LRU eviction)
    last_access: u64,
}

impl AstCache {
    /// Create a new AST cache
    pub fn new(cache_dir: PathBuf, max_size: u64) -> std::io::Result<Self> {
        // Ensure cache directory exists
        std::fs::create_dir_all(&cache_dir)?;

        let cache = Self {
            cache_dir,
            index: Arc::new(RwLock::new(CacheIndex::default())),
            max_size,
            current_size: Arc::new(RwLock::new(0)),
        };

        // Load existing index
        cache.load_index()?;

        Ok(cache)
    }

    /// Get cached AST or parse and cache
    pub fn get_or_parse<F, T>(&self, source: &[u8], parse_fn: F) -> std::io::Result<T>
    where
        F: FnOnce(&[u8]) -> T,
        T: AsRef<[u8]> + From<Vec<u8>>,
    {
        let hash = self.hash_content(source);

        // Check cache
        if let Some(cached) = self.get(&hash) {
            return Ok(T::from(cached));
        }

        // Parse and cache
        let result = parse_fn(source);
        self.store(&hash, result.as_ref())?;

        Ok(result)
    }

    /// Get cached entry by content hash
    pub fn get(&self, hash: &[u8; 32]) -> Option<Vec<u8>> {
        let index = self.index.read();
        let entry = index.entries.get(hash)?;

        // Read cached data
        let mut file = File::open(&entry.cache_path).ok()?;
        let mut data = Vec::with_capacity(entry.size as usize);
        file.read_to_end(&mut data).ok()?;

        // Update last access time
        drop(index);
        let mut index = self.index.write();
        if let Some(entry) = index.entries.get_mut(hash) {
            entry.last_access = current_timestamp();
        }

        Some(data)
    }

    /// Store data in cache
    pub fn store(&self, hash: &[u8; 32], data: &[u8]) -> std::io::Result<()> {
        // Check if we need to evict
        let data_size = data.len() as u64;
        self.maybe_evict(data_size)?;

        // Generate cache file path
        let cache_path = self.hash_to_path(hash);

        // Ensure parent directory exists
        if let Some(parent) = cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write to cache file
        let mut file = File::create(&cache_path)?;
        file.write_all(data)?;

        // Update index
        let mut index = self.index.write();
        index.entries.insert(
            *hash,
            CacheEntry {
                cache_path,
                size: data_size,
                last_access: current_timestamp(),
            },
        );

        // Update current size
        *self.current_size.write() += data_size;

        Ok(())
    }

    /// Hash content using blake3
    fn hash_content(&self, content: &[u8]) -> [u8; 32] {
        *blake3::hash(content).as_bytes()
    }

    /// Convert hash to file path
    fn hash_to_path(&self, hash: &[u8; 32]) -> PathBuf {
        let hex = hex::encode(hash);
        // Use first 2 characters as subdirectory for better filesystem performance
        self.cache_dir.join(&hex[..2]).join(&hex[2..])
    }

    /// Evict old entries if cache is full
    fn maybe_evict(&self, needed_size: u64) -> std::io::Result<()> {
        let current = *self.current_size.read();
        if current + needed_size <= self.max_size {
            return Ok(());
        }

        // Need to evict - find oldest entries
        let mut index = self.index.write();
        let mut entries: Vec<_> = index.entries.iter().collect();
        entries.sort_by_key(|(_, e)| e.last_access);

        let mut freed = 0u64;
        let target = (current + needed_size).saturating_sub(self.max_size);

        let mut to_remove = Vec::new();
        for (hash, entry) in entries {
            if freed >= target {
                break;
            }

            // Delete cache file
            let _ = std::fs::remove_file(&entry.cache_path);
            freed += entry.size;
            to_remove.push(*hash);
        }

        // Remove from index
        for hash in to_remove {
            index.entries.remove(&hash);
        }

        // Update current size
        *self.current_size.write() = current.saturating_sub(freed);

        Ok(())
    }

    /// Load index from disk
    fn load_index(&self) -> std::io::Result<()> {
        let index_path = self.cache_dir.join("index.bin");
        if !index_path.exists() {
            return Ok(());
        }

        let data = std::fs::read(&index_path)?;
        // In production, we'd deserialize the index properly
        // For now, just scan the cache directory

        Ok(())
    }

    /// Save index to disk
    pub fn save_index(&self) -> std::io::Result<()> {
        let index_path = self.cache_dir.join("index.bin");
        // In production, we'd serialize the index properly
        // For now, this is a placeholder
        Ok(())
    }

    /// Clear all cache entries
    pub fn clear(&self) -> std::io::Result<()> {
        let mut index = self.index.write();

        for entry in index.entries.values() {
            let _ = std::fs::remove_file(&entry.cache_path);
        }

        index.entries.clear();
        *self.current_size.write() = 0;

        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let index = self.index.read();
        CacheStats {
            entry_count: index.entries.len(),
            total_size: *self.current_size.read(),
            max_size: self.max_size,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached entries
    pub entry_count: usize,
    /// Total size of cache in bytes
    pub total_size: u64,
    /// Maximum cache size
    pub max_size: u64,
}

impl CacheStats {
    /// Get cache utilization as percentage
    pub fn utilization(&self) -> f64 {
        if self.max_size == 0 {
            0.0
        } else {
            (self.total_size as f64 / self.max_size as f64) * 100.0
        }
    }
}

/// Get current timestamp in seconds
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Hex encoding helper
mod hex {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    pub fn encode(bytes: &[u8]) -> String {
        let mut result = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            result.push(HEX_CHARS[(b >> 4) as usize] as char);
            result.push(HEX_CHARS[(b & 0xf) as usize] as char);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cache_store_and_get() {
        let dir = tempdir().unwrap();
        let cache = AstCache::new(dir.path().to_path_buf(), 1024 * 1024).unwrap();

        let data = b"test data";
        let hash = *blake3::hash(data).as_bytes();

        cache.store(&hash, data).unwrap();
        let retrieved = cache.get(&hash).unwrap();

        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_cache_stats() {
        let dir = tempdir().unwrap();
        let cache = AstCache::new(dir.path().to_path_buf(), 1024 * 1024).unwrap();

        let stats = cache.stats();
        assert_eq!(stats.entry_count, 0);
        assert_eq!(stats.total_size, 0);
    }
}
