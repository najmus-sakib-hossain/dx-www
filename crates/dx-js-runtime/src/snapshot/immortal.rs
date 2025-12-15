//! Persistent code cache

use crate::compiler::CompiledModule;
use crate::error::DxResult;
use crate::CacheStats;
use blake3::Hasher;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Hash of source code
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceHash([u8; 32]);

impl SourceHash {
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

/// Persistent code cache - compiled modules survive restarts
pub struct ImmortalCache {
    cache_dir: PathBuf,
    /// In-memory index of cached modules
    index: HashMap<SourceHash, CacheEntry>,
    /// Statistics
    hits: AtomicU64,
    misses: AtomicU64,
}

#[derive(Clone)]
struct CacheEntry {
    path: PathBuf,
    size: u64,
}

impl ImmortalCache {
    /// Open or create the cache
    pub fn open_or_create(cache_dir: &Path) -> DxResult<Self> {
        fs::create_dir_all(cache_dir)?;

        // Scan existing cache entries
        let mut index = HashMap::new();
        if let Ok(entries) = fs::read_dir(cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "dxc") {
                    if let Some(stem) = path.file_stem() {
                        let hex_str = stem.to_string_lossy();
                        if let Ok(bytes) = hex::decode(&*hex_str) {
                            if bytes.len() == 32 {
                                let mut hash = [0u8; 32];
                                hash.copy_from_slice(&bytes);
                                if let Ok(meta) = entry.metadata() {
                                    index.insert(
                                        SourceHash(hash),
                                        CacheEntry {
                                            path: path.clone(),
                                            size: meta.len(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            cache_dir: cache_dir.to_path_buf(),
            index,
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        })
    }

    /// Hash source code with version and architecture info
    pub fn hash_source(&self, source: &str) -> SourceHash {
        let mut hasher = Hasher::new();
        hasher.update(source.as_bytes());
        hasher.update(env!("CARGO_PKG_VERSION").as_bytes());
        hasher.update(std::env::consts::ARCH.as_bytes());
        SourceHash(hasher.finalize().into())
    }

    /// Get cached module (currently returns None - full impl would load from disk)
    pub fn get(&self, hash: &SourceHash) -> DxResult<Option<CompiledModule>> {
        if self.index.contains_key(hash) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            // TODO: Load from disk and return
            // For now, we always recompile
            Ok(None)
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            Ok(None)
        }
    }

    /// Store compiled module to cache
    pub fn store(&mut self, hash: &SourceHash, _module: &CompiledModule) -> DxResult<()> {
        let path = self.cache_path(hash);
        
        // Write a cache marker file
        let mut file = File::create(&path)?;
        file.write_all(b"DXCACHE\x00")?;
        file.write_all(env!("CARGO_PKG_VERSION").as_bytes())?;
        // TODO: Serialize the actual compiled code

        let size = file.metadata().map(|m| m.len()).unwrap_or(0);
        self.index.insert(
            *hash,
            CacheEntry {
                path,
                size,
            },
        );

        Ok(())
    }

    fn cache_path(&self, hash: &SourceHash) -> PathBuf {
        self.cache_dir.join(format!("{}.dxc", hash.to_hex()))
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let total_size: u64 = self.index.values().map(|e| e.size).sum();
        CacheStats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            modules_cached: self.index.len(),
            total_size_bytes: total_size,
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) -> DxResult<()> {
        for entry in self.index.values() {
            let _ = fs::remove_file(&entry.path);
        }
        self.index.clear();
        Ok(())
    }
}
