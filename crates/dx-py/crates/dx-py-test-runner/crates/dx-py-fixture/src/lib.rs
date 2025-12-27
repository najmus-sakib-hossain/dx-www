//! Memory-mapped fixture cache
//!
//! This crate implements caching for expensive test fixtures
//! using memory-mapped files and Blake3 hashing for invalidation.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use blake3::Hash;
use memmap2::Mmap;
use serde::{de::DeserializeOwned, Serialize};

pub use dx_py_core::{FixtureError, FixtureId};

/// Cached fixture entry with hash for invalidation
#[derive(Debug)]
struct CacheEntry {
    /// Hash of the fixture function source
    source_hash: Hash,
    /// Memory-mapped file containing serialized fixture
    mmap: Option<Mmap>,
    /// Path to the cache file
    path: PathBuf,
}

/// Memory-mapped fixture cache
///
/// Stores serialized fixture values on disk with Blake3 hash-based
/// invalidation. When a fixture's source code changes, the cache
/// is automatically invalidated.
pub struct FixtureCache {
    /// Cache directory
    cache_dir: PathBuf,
    /// In-memory index of cached fixtures
    entries: HashMap<FixtureId, CacheEntry>,
}

impl FixtureCache {
    /// Create a new fixture cache in the given directory
    pub fn new(cache_dir: impl Into<PathBuf>) -> Result<Self, FixtureError> {
        let cache_dir = cache_dir.into();
        fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            cache_dir,
            entries: HashMap::new(),
        })
    }

    /// Get the cache file path for a fixture
    fn cache_path(&self, id: FixtureId) -> PathBuf {
        self.cache_dir.join(format!("{:016x}.fixture", id.0))
    }

    /// Get the hash file path for a fixture
    fn hash_path(&self, id: FixtureId) -> PathBuf {
        self.cache_dir.join(format!("{:016x}.hash", id.0))
    }

    /// Compute Blake3 hash of fixture source code
    pub fn hash_source(source: &str) -> Hash {
        blake3::hash(source.as_bytes())
    }

    /// Check if a cached fixture is valid (source hash matches)
    pub fn is_valid(&self, id: FixtureId, source_hash: Hash) -> bool {
        if let Some(entry) = self.entries.get(&id) {
            return entry.source_hash == source_hash;
        }

        // Check on-disk hash
        if let Ok(stored_hash) = self.load_hash(id) {
            return stored_hash == source_hash;
        }

        false
    }

    /// Load stored hash from disk
    fn load_hash(&self, id: FixtureId) -> Result<Hash, FixtureError> {
        let hash_path = self.hash_path(id);
        let mut file = File::open(&hash_path)?;
        let mut bytes = [0u8; 32];
        file.read_exact(&mut bytes)?;
        Ok(Hash::from_bytes(bytes))
    }

    /// Store hash to disk
    fn store_hash(&self, id: FixtureId, hash: Hash) -> Result<(), FixtureError> {
        let hash_path = self.hash_path(id);
        let mut file = File::create(&hash_path)?;
        file.write_all(hash.as_bytes())?;
        Ok(())
    }

    /// Get a cached fixture value, or create it if not cached/invalid
    ///
    /// The `source` parameter should be the fixture function's source code,
    /// used for cache invalidation.
    pub fn get_or_create<T, F>(
        &mut self,
        id: FixtureId,
        source: &str,
        create: F,
    ) -> Result<T, FixtureError>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> T,
    {
        let source_hash = Self::hash_source(source);

        // Check if we have a valid cached value
        if self.is_valid(id, source_hash) {
            if let Ok(value) = self.load::<T>(id) {
                return Ok(value);
            }
        }

        // Create new value and cache it
        let value = create();
        self.store(id, source_hash, &value)?;
        Ok(value)
    }

    /// Store a fixture value in the cache
    pub fn store<T: Serialize>(
        &mut self,
        id: FixtureId,
        source_hash: Hash,
        value: &T,
    ) -> Result<(), FixtureError> {
        let cache_path = self.cache_path(id);

        // Serialize value
        let bytes = bincode::serialize(value)
            .map_err(|e| FixtureError::SerializationFailed(e.to_string()))?;

        // Write to file
        let mut file = File::create(&cache_path)?;
        file.write_all(&bytes)?;

        // Store hash
        self.store_hash(id, source_hash)?;

        // Update in-memory entry (without mmap for now)
        self.entries.insert(
            id,
            CacheEntry {
                source_hash,
                mmap: None,
                path: cache_path,
            },
        );

        Ok(())
    }

    /// Load a fixture value from the cache using memory mapping
    pub fn load<T: DeserializeOwned>(&mut self, id: FixtureId) -> Result<T, FixtureError> {
        let cache_path = self.cache_path(id);

        if !cache_path.exists() {
            return Err(FixtureError::NotFound(format!("Fixture {:?}", id)));
        }

        // Memory-map the file
        let file = File::open(&cache_path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        // Deserialize from mapped memory
        let value: T = bincode::deserialize(&mmap)
            .map_err(|e| FixtureError::DeserializationFailed(e.to_string()))?;

        // Update entry with mmap
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.mmap = Some(mmap);
        }

        Ok(value)
    }

    /// Invalidate a fixture cache entry
    pub fn invalidate(&mut self, id: FixtureId) -> Result<(), FixtureError> {
        // Remove from memory
        self.entries.remove(&id);

        // Remove from disk
        let cache_path = self.cache_path(id);
        let hash_path = self.hash_path(id);

        if cache_path.exists() {
            fs::remove_file(&cache_path)?;
        }
        if hash_path.exists() {
            fs::remove_file(&hash_path)?;
        }

        Ok(())
    }

    /// Clear all cached fixtures
    pub fn clear(&mut self) -> Result<(), FixtureError> {
        self.entries.clear();

        // Remove all files in cache directory
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }

    /// Get the number of cached fixtures
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Check if a fixture exists in the cache
    pub fn contains(&self, id: FixtureId) -> bool {
        self.entries.contains_key(&id) || self.cache_path(id).exists()
    }
}

#[cfg(test)]
mod tests;
