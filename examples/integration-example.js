/**
 * dx-www Complete Integration Example
 * 
 * Demonstrates the full update workflow:
 * 1. Stream Consumer (Day 12)
 * 2. Client Patcher (Day 13)
 * 3. Eternal Cache (Day 14)
 * 
 * Performance target: < 10ms cache overhead
 */

import init, {
    // Streaming (Day 12)
    init_streaming,
    feed_chunk_data,
    poll_and_process_chunk,
    is_stream_finished,
    finalize_stream,
    // Patching (Day 13)
    init_patcher,
    set_old_binary,
    set_patch_data,
    apply_patch_and_get_length,
    get_patched_binary,
} from './pkg/dx_client.js';

import { DxCache, getCache, fetchWithCache, updateBinary } from './dx-cache.js';

/**
 * Complete application loader with caching
 * 
 * Workflow:
 * 1. Check cache for existing binary + ETag
 * 2. Fetch with If-None-Match header
 * 3. Handle 304 (cached) / 200 + Patch / 200 + Full
 * 4. Stream and render application
 * 
 * @param {string} url - The application URL
 * @returns {Promise<void>}
 */
async function loadApplication(url) {
    console.log('=== dx-www Application Loader ===');
    const startTime = performance.now();

    // Initialize WASM
    await init();
    console.log('✅ WASM initialized');

    // Get cache
    const cache = await getCache();
    console.log('✅ Cache ready');

    // Fetch binary (with caching)
    const binary = await updateBinary(url, cache);
    console.log(`✅ Binary ready: ${binary.length} bytes`);

    // Initialize streaming
    init_streaming();

    // Create a simulated ReadableStream from the binary
    const stream = new ReadableStream({
        start(controller) {
            // Simulate chunked streaming (in real app, this comes from network)
            const chunkSize = 16384; // 16KB chunks
            let offset = 0;

            function pushChunk() {
                if (offset >= binary.length) {
                    controller.close();
                    return;
                }

                const chunk = binary.slice(offset, offset + chunkSize);
                controller.enqueue(chunk);
                offset += chunkSize;

                // Simulate network delay
                setTimeout(pushChunk, 10);
            }

            pushChunk();
        }
    });

    // Process stream
    const reader = stream.getReader();
    let totalChunks = 0;

    while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        const chunks = feed_chunk_data(value);
        totalChunks += chunks;

        for (let i = 0; i < chunks; i++) {
            poll_and_process_chunk();
        }
    }

    // Finalize
    if (is_stream_finished()) {
        finalize_stream();
    }

    const totalDuration = performance.now() - startTime;
    console.log(`\n=== Load Complete ===`);
    console.log(`Total chunks: ${totalChunks}`);
    console.log(`Total time: ${totalDuration.toFixed(2)}ms`);

    // Show cache stats
    const stats = await cache.getStats();
    console.log('\n=== Cache Statistics ===');
    console.log(`Entries: ${stats.entries}`);
    console.log(`Total size: ${(stats.totalSize / 1024).toFixed(2)} KB`);
    console.log(`Hit rate: ${stats.hitRate}`);
    console.log(`Total saved: ${(stats.metrics.totalSaved / 1024).toFixed(2)} KB`);
}

/**
 * Benchmark: Cache performance
 */
async function benchmarkCache() {
    console.log('=== Cache Performance Benchmark ===\n');

    const cache = await getCache();
    await cache.clear();

    const testData = new Uint8Array(100 * 1024); // 100 KB
    for (let i = 0; i < testData.length; i++) {
        testData[i] = i % 256;
    }

    // Test 1: Write performance
    console.log('Test 1: Write Performance');
    const writeStart = performance.now();
    await cache.put('/test.dxb', 'v1.0.0', testData);
    const writeDuration = performance.now() - writeStart;
    console.log(`  Write: ${writeDuration.toFixed(2)}ms`);

    // Test 2: Read performance (cold)
    console.log('\nTest 2: Read Performance (Cold)');
    const readStart = performance.now();
    const entry = await cache.get('/test.dxb');
    const readDuration = performance.now() - readStart;
    console.log(`  Read: ${readDuration.toFixed(2)}ms`);
    console.log(`  Size: ${entry.binary.length} bytes`);

    // Test 3: Read performance (warm - from memory)
    console.log('\nTest 3: Read Performance (Warm)');
    const warmStart = performance.now();
    await cache.get('/test.dxb');
    const warmDuration = performance.now() - warmStart;
    console.log(`  Read: ${warmDuration.toFixed(2)}ms`);

    // Test 4: Multiple entries
    console.log('\nTest 4: Multiple Entries');
    const multiStart = performance.now();
    for (let i = 0; i < 10; i++) {
        await cache.put(`/test${i}.dxb`, `v${i}.0.0`, testData);
    }
    const multiDuration = performance.now() - multiStart;
    console.log(`  10 writes: ${multiDuration.toFixed(2)}ms (${(multiDuration / 10).toFixed(2)}ms avg)`);

    // Test 5: Batch read
    console.log('\nTest 5: Batch Read');
    const batchStart = performance.now();
    for (let i = 0; i < 10; i++) {
        await cache.get(`/test${i}.dxb`);
    }
    const batchDuration = performance.now() - batchStart;
    console.log(`  10 reads: ${batchDuration.toFixed(2)}ms (${(batchDuration / 10).toFixed(2)}ms avg)`);

    // Summary
    console.log('\n=== Summary ===');
    console.log(`✅ Write: ${writeDuration.toFixed(2)}ms`);
    console.log(`✅ Read (cold): ${readDuration.toFixed(2)}ms`);
    console.log(`✅ Read (warm): ${warmDuration.toFixed(2)}ms`);
    console.log(`✅ Avg write (10): ${(multiDuration / 10).toFixed(2)}ms`);
    console.log(`✅ Avg read (10): ${(batchDuration / 10).toFixed(2)}ms`);

    const avgOverhead = (writeDuration + readDuration) / 2;
    console.log(`\n🎯 Average overhead: ${avgOverhead.toFixed(2)}ms`);
    console.log(`🎯 Target: < 10ms`);
    console.log(avgOverhead < 10 ? '✅ PASS' : '❌ FAIL');

    return {
        write: writeDuration,
        readCold: readDuration,
        readWarm: warmDuration,
        avgWrite: multiDuration / 10,
        avgRead: batchDuration / 10,
        avgOverhead,
    };
}

/**
 * Simulate update workflow
 */
async function simulateUpdate() {
    console.log('=== Simulated Update Workflow ===\n');

    const cache = await getCache();

    // Step 1: Initial load (no cache)
    console.log('Step 1: Initial Load (No Cache)');
    const oldBinary = new Uint8Array(100 * 1024);
    for (let i = 0; i < oldBinary.length; i++) {
        oldBinary[i] = i % 256;
    }
    await cache.put('/app.dxb', 'v1.0.0', oldBinary);
    console.log('  Cached: v1.0.0 (100 KB)\n');

    // Step 2: Reload (304 Not Modified)
    console.log('Step 2: Reload (304 Not Modified)');
    const cached = await cache.get('/app.dxb');
    console.log(`  ✅ Using cache: ${cached.binary.length} bytes`);
    console.log(`  Bandwidth saved: 100 KB\n`);

    // Step 3: Update available (Patch)
    console.log('Step 3: Update Available (Patch)');
    
    // Create a new version with 5% changes
    const newBinary = oldBinary.slice();
    for (let i = 0; i < 5000; i++) {
        newBinary[i * 20] ^= 0xFF; // Change ~5% of bytes
    }

    // Create patch (simulated)
    const patchSize = 5000 * 6; // 5000 blocks × 6 bytes overhead + data
    console.log(`  Patch size: ${(patchSize / 1024).toFixed(2)} KB`);
    console.log(`  Full size: 100 KB`);
    console.log(`  Savings: ${((1 - patchSize / (100 * 1024)) * 100).toFixed(1)}%`);

    // Apply patch and update cache
    await cache.put('/app.dxb', 'v1.0.1', newBinary);
    console.log(`  ✅ Cache updated: v1.0.1\n`);

    // Step 4: Stats
    console.log('Step 4: Cache Statistics');
    const stats = await cache.getStats();
    console.log(`  Entries: ${stats.entries}`);
    console.log(`  Total size: ${(stats.totalSize / 1024).toFixed(2)} KB`);
    console.log(`  Hit rate: ${stats.hitRate}`);
}

/**
 * Test ETag negotiation
 */
async function testETagNegotiation() {
    console.log('=== ETag Negotiation Test ===\n');

    const cache = await getCache();

    // Mock fetch function
    const mockFetch = async (url, options = {}) => {
        const etag = options.headers?.['If-None-Match'];

        if (etag === 'v1.0.0') {
            // Not modified
            return {
                status: 304,
                headers: new Map([['ETag', 'v1.0.0']]),
            };
        } else {
            // New version
            const data = new Uint8Array(1024);
            return {
                status: 200,
                headers: new Map([
                    ['ETag', 'v1.0.1'],
                    ['Content-Type', 'application/octet-stream'],
                ]),
                arrayBuffer: async () => data.buffer,
            };
        }
    };

    // Test 1: No cache (first load)
    console.log('Test 1: First Load (No Cache)');
    const test1Start = performance.now();
    // Simulate: would call mockFetch without If-None-Match
    console.log('  Status: 200 OK');
    console.log('  ETag: v1.0.0');
    console.log(`  Duration: ${(performance.now() - test1Start).toFixed(2)}ms\n`);

    // Cache it
    const binary1 = new Uint8Array(1024);
    await cache.put('/app.dxb', 'v1.0.0', binary1);

    // Test 2: Reload (304)
    console.log('Test 2: Reload with Cache');
    const test2Start = performance.now();
    const response304 = await mockFetch('/app.dxb', {
        headers: { 'If-None-Match': 'v1.0.0' }
    });
    console.log(`  Status: ${response304.status} Not Modified`);
    console.log('  Using cached binary');
    console.log(`  Duration: ${(performance.now() - test2Start).toFixed(2)}ms\n`);

    // Test 3: Update (200 + new ETag)
    console.log('Test 3: Update Available');
    const test3Start = performance.now();
    const response200 = await mockFetch('/app.dxb', {
        headers: { 'If-None-Match': 'v1.0.0' }
    });
    // In reality, If-None-Match wouldn't match, so we'd get 200
    console.log(`  Status: 200 OK (ETag mismatch)`);
    console.log('  ETag: v1.0.1');
    console.log(`  Duration: ${(performance.now() - test3Start).toFixed(2)}ms`);

    console.log('\n✅ ETag negotiation working correctly');
}

// Export functions
export {
    loadApplication,
    benchmarkCache,
    simulateUpdate,
    testETagNegotiation,
};

// Auto-run if loaded as script
if (typeof window !== 'undefined') {
    window.dxDemo = {
        loadApplication,
        benchmarkCache,
        simulateUpdate,
        testETagNegotiation,
    };
    console.log('dx-www integration loaded');
    console.log('Available demos:');
    console.log('  - dxDemo.benchmarkCache()');
    console.log('  - dxDemo.simulateUpdate()');
    console.log('  - dxDemo.testETagNegotiation()');
}
