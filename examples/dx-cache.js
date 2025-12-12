/**
 * dx-www Cache Manager
 * 
 * IndexedDB-based caching layer for dx-www binaries with ETag versioning.
 * 
 * Features:
 * - Binary storage with ETags
 * - If-None-Match negotiation
 * - 304 vs 200 + Patch handling
 * - Cache invalidation (TTL, size limits)
 * - Performance: < 10ms overhead
 * 
 * @module dx-cache
 */

const DB_NAME = 'dx-cache';
const DB_VERSION = 1;
const STORE_NAME = 'binaries';

// Cache configuration
const CONFIG = {
    maxAge: 7 * 24 * 60 * 60 * 1000, // 7 days in ms
    maxSize: 50 * 1024 * 1024, // 50 MB total cache size
    maxEntries: 100, // Maximum number of cached binaries
};

/**
 * Cache entry structure
 * 
 * @typedef {Object} CacheEntry
 * @property {string} url - The URL of the binary
 * @property {string} etag - The ETag version identifier
 * @property {Uint8Array} binary - The binary data
 * @property {number} timestamp - When cached (ms since epoch)
 * @property {number} size - Binary size in bytes
 * @property {number} hits - Access count
 */

/**
 * DxCache - IndexedDB wrapper for binary caching
 */
class DxCache {
    constructor() {
        this.db = null;
        this.metrics = {
            hits: 0,
            misses: 0,
            updates: 0,
            patches: 0,
            totalSaved: 0, // Bytes saved via caching
        };
    }

    /**
     * Initialize the IndexedDB database
     * 
     * @returns {Promise<void>}
     */
    async init() {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(DB_NAME, DB_VERSION);

            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                this.db = request.result;
                resolve();
            };

            request.onupgradeneeded = (event) => {
                const db = event.target.result;

                // Create object store
                if (!db.objectStoreNames.contains(STORE_NAME)) {
                    const store = db.createObjectStore(STORE_NAME, { keyPath: 'url' });
                    
                    // Create indexes
                    store.createIndex('etag', 'etag', { unique: false });
                    store.createIndex('timestamp', 'timestamp', { unique: false });
                    store.createIndex('size', 'size', { unique: false });
                }
            };
        });
    }

    /**
     * Get cached binary by URL
     * 
     * @param {string} url - The URL to look up
     * @returns {Promise<CacheEntry|null>}
     */
    async get(url) {
        const startTime = performance.now();

        return new Promise((resolve, reject) => {
            const transaction = this.db.transaction([STORE_NAME], 'readonly');
            const store = transaction.objectStore(STORE_NAME);
            const request = store.get(url);

            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                const entry = request.result;
                const duration = performance.now() - startTime;

                if (entry) {
                    // Check if expired
                    const age = Date.now() - entry.timestamp;
                    if (age > CONFIG.maxAge) {
                        console.log(`Cache expired: ${url} (${age}ms old)`);
                        this.delete(url); // Fire and forget
                        this.metrics.misses++;
                        resolve(null);
                        return;
                    }

                    // Update hit counter
                    entry.hits = (entry.hits || 0) + 1;
                    this.put(entry); // Fire and forget

                    this.metrics.hits++;
                    console.log(`Cache hit: ${url} (${duration.toFixed(2)}ms)`);
                    resolve(entry);
                } else {
                    this.metrics.misses++;
                    console.log(`Cache miss: ${url} (${duration.toFixed(2)}ms)`);
                    resolve(null);
                }
            };
        });
    }

    /**
     * Store binary in cache
     * 
     * @param {string} url - The URL
     * @param {string} etag - The ETag
     * @param {Uint8Array} binary - The binary data
     * @returns {Promise<void>}
     */
    async put(url, etag, binary) {
        const startTime = performance.now();

        // Handle entry object form (for updates)
        if (typeof url === 'object') {
            const entry = url; // Overloaded parameter
            return new Promise((resolve, reject) => {
                const transaction = this.db.transaction([STORE_NAME], 'readwrite');
                const store = transaction.objectStore(STORE_NAME);
                const request = store.put(entry);

                request.onerror = () => reject(request.error);
                request.onsuccess = () => resolve();
            });
        }

        // Check cache size before adding
        await this.enforceQuota();

        const entry = {
            url,
            etag,
            binary,
            timestamp: Date.now(),
            size: binary.length,
            hits: 0,
        };

        return new Promise((resolve, reject) => {
            const transaction = this.db.transaction([STORE_NAME], 'readwrite');
            const store = transaction.objectStore(STORE_NAME);
            const request = store.put(entry);

            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                const duration = performance.now() - startTime;
                console.log(`Cached: ${url} (${etag}) ${binary.length} bytes in ${duration.toFixed(2)}ms`);
                resolve();
            };
        });
    }

    /**
     * Delete cached entry
     * 
     * @param {string} url - The URL to delete
     * @returns {Promise<void>}
     */
    async delete(url) {
        return new Promise((resolve, reject) => {
            const transaction = this.db.transaction([STORE_NAME], 'readwrite');
            const store = transaction.objectStore(STORE_NAME);
            const request = store.delete(url);

            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                console.log(`Cache deleted: ${url}`);
                resolve();
            };
        });
    }

    /**
     * Get all cached entries
     * 
     * @returns {Promise<CacheEntry[]>}
     */
    async getAll() {
        return new Promise((resolve, reject) => {
            const transaction = this.db.transaction([STORE_NAME], 'readonly');
            const store = transaction.objectStore(STORE_NAME);
            const request = store.getAll();

            request.onerror = () => reject(request.error);
            request.onsuccess = () => resolve(request.result);
        });
    }

    /**
     * Clear all cache entries
     * 
     * @returns {Promise<void>}
     */
    async clear() {
        return new Promise((resolve, reject) => {
            const transaction = this.db.transaction([STORE_NAME], 'readwrite');
            const store = transaction.objectStore(STORE_NAME);
            const request = store.clear();

            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                console.log('Cache cleared');
                resolve();
            };
        });
    }

    /**
     * Enforce cache quota (size and entry limits)
     * 
     * @returns {Promise<void>}
     */
    async enforceQuota() {
        const entries = await this.getAll();

        // Check total size
        const totalSize = entries.reduce((sum, e) => sum + e.size, 0);
        
        if (totalSize > CONFIG.maxSize || entries.length > CONFIG.maxEntries) {
            console.log(`Cache quota exceeded: ${totalSize} bytes, ${entries.length} entries`);

            // Sort by last access (hits) and age
            entries.sort((a, b) => {
                const scoreA = (a.hits || 0) / (Date.now() - a.timestamp);
                const scoreB = (b.hits || 0) / (Date.now() - b.timestamp);
                return scoreA - scoreB; // Ascending (worst first)
            });

            // Remove worst 20% of entries
            const toRemove = Math.ceil(entries.length * 0.2);
            for (let i = 0; i < toRemove; i++) {
                await this.delete(entries[i].url);
            }

            console.log(`Evicted ${toRemove} cache entries`);
        }
    }

    /**
     * Get cache statistics
     * 
     * @returns {Promise<Object>}
     */
    async getStats() {
        const entries = await this.getAll();
        const totalSize = entries.reduce((sum, e) => sum + e.size, 0);
        const hitRate = this.metrics.hits / (this.metrics.hits + this.metrics.misses) || 0;

        return {
            entries: entries.length,
            totalSize,
            hitRate: (hitRate * 100).toFixed(1) + '%',
            metrics: this.metrics,
            oldestEntry: entries.length > 0 
                ? new Date(Math.min(...entries.map(e => e.timestamp)))
                : null,
        };
    }
}

/**
 * Fetch with cache and ETag support
 * 
 * Handles:
 * - 304 Not Modified (use cache)
 * - 200 + Patch (apply patch)
 * - 200 + Full Binary (update cache)
 * 
 * @param {string} url - The URL to fetch
 * @param {DxCache} cache - The cache instance
 * @returns {Promise<{binary: Uint8Array, etag: string, source: string}>}
 */
async function fetchWithCache(url, cache) {
    const startTime = performance.now();

    // Check cache
    const cached = await cache.get(url);
    const headers = {};

    if (cached) {
        // Send If-None-Match header
        headers['If-None-Match'] = cached.etag;
        console.log(`Fetching with If-None-Match: ${cached.etag}`);
    }

    // Fetch from server
    const response = await fetch(url, { headers });

    // 304 Not Modified - use cache
    if (response.status === 304) {
        const duration = performance.now() - startTime;
        console.log(`✅ 304 Not Modified - using cache (${duration.toFixed(2)}ms)`);
        
        cache.metrics.totalSaved += cached.size;
        
        return {
            binary: cached.binary,
            etag: cached.etag,
            source: 'cache',
            duration,
        };
    }

    const newEtag = response.headers.get('ETag');
    const contentType = response.headers.get('Content-Type');

    // Patch response
    if (contentType === 'application/vnd.dx-patch' || contentType === 'application/octet-stream-patch') {
        if (!cached) {
            throw new Error('Received patch but no cached binary found');
        }

        console.log('📦 Applying patch...');

        // Import patcher
        const { applyPatch } = await import('./patcher-example.js');
        const patchData = new Uint8Array(await response.arrayBuffer());

        const patchStart = performance.now();
        const newBinary = await applyPatch(cached.binary, patchData);
        const patchDuration = performance.now() - patchStart;

        console.log(`⚡ Patched in ${patchDuration.toFixed(2)}ms`);

        // Cache the new binary
        await cache.put(url, newEtag, newBinary);

        const totalDuration = performance.now() - startTime;
        cache.metrics.patches++;
        cache.metrics.totalSaved += (cached.size - patchData.length);

        return {
            binary: newBinary,
            etag: newEtag,
            source: 'patch',
            duration: totalDuration,
            patchSize: patchData.length,
        };
    }

    // Full binary download
    console.log('📥 Downloading full binary...');
    const newBinary = new Uint8Array(await response.arrayBuffer());

    // Cache it
    await cache.put(url, newEtag, newBinary);

    const duration = performance.now() - startTime;
    cache.metrics.updates++;

    return {
        binary: newBinary,
        etag: newEtag,
        source: 'download',
        duration,
    };
}

/**
 * Complete update workflow
 * 
 * @param {string} url - The URL to fetch
 * @param {DxCache} cache - The cache instance
 * @returns {Promise<Uint8Array>} The binary data
 */
async function updateBinary(url, cache) {
    console.log('=== Update Binary ===');
    console.log(`URL: ${url}`);

    const result = await fetchWithCache(url, cache);

    console.log(`Source: ${result.source}`);
    console.log(`ETag: ${result.etag}`);
    console.log(`Size: ${result.binary.length} bytes`);
    console.log(`Duration: ${result.duration.toFixed(2)}ms`);

    if (result.patchSize) {
        const savings = ((1 - result.patchSize / result.binary.length) * 100).toFixed(1);
        console.log(`Patch savings: ${savings}%`);
    }

    return result.binary;
}

// Export
export { DxCache, fetchWithCache, updateBinary, CONFIG };

// Global instance (singleton)
let globalCache = null;

/**
 * Get or create global cache instance
 * 
 * @returns {Promise<DxCache>}
 */
export async function getCache() {
    if (!globalCache) {
        globalCache = new DxCache();
        await globalCache.init();
    }
    return globalCache;
}

// Auto-initialize on module load
if (typeof window !== 'undefined') {
    console.log('dx-cache loaded');
}
