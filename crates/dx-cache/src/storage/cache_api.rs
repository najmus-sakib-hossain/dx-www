//! # Cache API Storage
//!
//! HTTP cache for delta updates and assets

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Cache, CacheStorage, Request, Response};

/// Open cache
pub async fn open_cache(cache_name: &str) -> Result<Cache, JsValue> {
    let window = window().ok_or("No window")?;
    let caches: CacheStorage = window.caches()?;

    let promise = caches.open(cache_name);
    let result = JsFuture::from(promise).await?;

    Ok(result.dyn_into()?)
}

/// Store response in cache
pub async fn cache_response(cache: &Cache, url: &str, response: &Response) -> Result<(), JsValue> {
    let promise = cache.put_with_str(url, response);
    JsFuture::from(promise).await?;
    Ok(())
}

/// Get cached response
pub async fn get_cached_response(cache: &Cache, url: &str) -> Result<Option<Response>, JsValue> {
    let request = Request::new_with_str(url)?;
    let promise = cache.match_with_request(&request);
    let result = JsFuture::from(promise).await?;

    if result.is_undefined() {
        return Ok(None);
    }

    Ok(Some(result.dyn_into()?))
}

/// Delete cache
pub async fn delete_cache(cache_name: &str) -> Result<bool, JsValue> {
    let window = window().ok_or("No window")?;
    let caches: CacheStorage = window.caches()?;

    let promise = caches.delete(cache_name);
    let result = JsFuture::from(promise).await?;

    Ok(result.as_bool().unwrap_or(false))
}
