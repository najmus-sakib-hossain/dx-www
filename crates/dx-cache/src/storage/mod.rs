//! # Storage Module
//!
//! Multi-layer storage strategy for eternal caching

pub mod cache_api;
pub mod indexeddb;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub indexeddb_size: u64,
    pub cache_api_size: u64,
    pub total_entries: u32,
    pub hit_rate: f64,
}

/// Initialize IndexedDB
pub async fn init_indexeddb(db_name: &str, version: u32) -> Result<(), JsValue> {
    indexeddb::open_database(db_name, version).await?;
    Ok(())
}

/// Initialize Cache API
pub async fn init_cache_api() -> Result<(), JsValue> {
    cache_api::open_cache("dx-cache-v1").await?;
    Ok(())
}

/// Get storage statistics
pub async fn get_storage_stats() -> Result<StorageStats, JsValue> {
    // TODO: Implement actual stats collection
    Ok(StorageStats {
        indexeddb_size: 0,
        cache_api_size: 0,
        total_entries: 0,
        hit_rate: 0.0,
    })
}

/// Clear IndexedDB
pub async fn clear_indexeddb() -> Result<(), JsValue> {
    indexeddb::delete_database("dx-cache").await?;
    Ok(())
}

/// Clear Cache API
pub async fn clear_cache_api() -> Result<(), JsValue> {
    cache_api::delete_cache("dx-cache-v1").await?;
    Ok(())
}
