/**
 * dx-www Client Patcher Example
 * 
 * Demonstrates how to apply binary patches to reconstruct updated binaries.
 * 
 * Performance: < 1ms for typical 20KB patches
 */

import init, {
    init_patcher,
    set_old_binary,
    set_patch_data,
    apply_patch_and_get_length,
    get_patched_binary,
    apply_patch_inplace
} from './pkg/dx_client.js';

/**
 * High-level wrapper for applying patches
 * 
 * @param {Uint8Array} oldBinary - The old binary to patch
 * @param {Uint8Array} patchData - The patch data from server
 * @returns {Promise<Uint8Array>} The patched binary
 */
async function applyPatch(oldBinary, patchData) {
    // Initialize patcher
    init_patcher();
    
    // Set old binary
    set_old_binary(oldBinary);
    
    // Set patch data
    set_patch_data(patchData);
    
    // Apply patch
    const newLength = apply_patch_and_get_length();
    console.log(`Patch applied: ${oldBinary.length} → ${newLength} bytes`);
    
    // Get result
    const newBinary = get_patched_binary();
    
    return newBinary;
}

/**
 * In-place patching (fastest method)
 * 
 * Modifies the buffer directly without allocating new memory.
 * 
 * @param {Uint8Array} buffer - Mutable buffer to patch
 * @param {Uint8Array} patchData - The patch data
 */
function applyPatchInPlace(buffer, patchData) {
    apply_patch_inplace(buffer, patchData);
    console.log('Patch applied in-place');
}

/**
 * Complete update workflow with caching
 * 
 * @param {string} url - URL of the binary
 * @param {string} etag - Current ETag (version)
 */
async function updateBinary(url, etag) {
    console.log('Checking for updates...');
    
    // Fetch with If-None-Match header
    const response = await fetch(url, {
        headers: {
            'If-None-Match': etag
        }
    });
    
    if (response.status === 304) {
        console.log('✅ No updates (304 Not Modified)');
        return null;
    }
    
    const newEtag = response.headers.get('ETag');
    
    // Check if server sent a patch
    const contentType = response.headers.get('Content-Type');
    
    if (contentType === 'application/vnd.dx-patch') {
        console.log('📦 Received patch, applying...');
        
        // Get old binary from cache
        const oldBinary = await getCachedBinary(url);
        if (!oldBinary) {
            throw new Error('No cached binary found for patching');
        }
        
        // Get patch data
        const patchData = new Uint8Array(await response.arrayBuffer());
        
        // Apply patch
        const startTime = performance.now();
        const newBinary = await applyPatch(oldBinary, patchData);
        const duration = performance.now() - startTime;
        
        console.log(`⚡ Patch applied in ${duration.toFixed(2)}ms`);
        
        // Cache new binary
        await cacheBinary(url, newEtag, newBinary);
        
        return newBinary;
    } else {
        console.log('📥 Received full binary');
        
        // Full binary download
        const newBinary = new Uint8Array(await response.arrayBuffer());
        
        // Cache it
        await cacheBinary(url, newEtag, newBinary);
        
        return newBinary;
    }
}

/**
 * IndexedDB helpers (placeholder - implement in Day 14)
 */
async function getCachedBinary(url) {
    // TODO: Implement IndexedDB retrieval
    return null;
}

async function cacheBinary(url, etag, binary) {
    // TODO: Implement IndexedDB storage
    console.log(`Cached: ${url} (${etag})`);
}

/**
 * Example: Create and apply a test patch
 */
async function testPatcher() {
    console.log('=== Patcher Test ===');
    
    await init();
    
    // Create old binary (simulated)
    const oldBinary = new Uint8Array(8192);
    for (let i = 0; i < oldBinary.length; i++) {
        oldBinary[i] = i % 256;
    }
    
    // Create patch data
    // Format: [PatchHeader:17] [BlockCount:4] [Block...]
    const patchData = createTestPatch();
    
    // Apply patch
    const startTime = performance.now();
    const newBinary = await applyPatch(oldBinary, patchData);
    const duration = performance.now() - startTime;
    
    console.log(`✅ Test complete in ${duration.toFixed(3)}ms`);
    console.log(`Old size: ${oldBinary.length} bytes`);
    console.log(`Patch size: ${patchData.length} bytes`);
    console.log(`New size: ${newBinary.length} bytes`);
    
    return { oldBinary, patchData, newBinary, duration };
}

/**
 * Helper: Create a test patch
 */
function createTestPatch() {
    const patch = [];
    
    // PatchHeader (17 bytes)
    // base_version_hash: u64 (0x1111_1111_1111_1111)
    patch.push(0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11);
    // target_version_hash: u64 (0x2222_2222_2222_2222)
    patch.push(0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22);
    // patch_algorithm: u8 (1 = XOR)
    patch.push(0x01);
    
    // Block count: u32 LE (1 block)
    patch.push(0x01, 0x00, 0x00, 0x00);
    
    // Block 0
    // index: u32 LE (block 0)
    patch.push(0x00, 0x00, 0x00, 0x00);
    // length: u16 LE (10 bytes)
    patch.push(0x0A, 0x00);
    // XOR data (10 bytes)
    patch.push(0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, 0x77, 0x66);
    
    return new Uint8Array(patch);
}

// Export functions
export { applyPatch, applyPatchInPlace, updateBinary, testPatcher };

// Auto-run test on load (if running as script)
if (typeof window !== 'undefined') {
    window.testPatcher = testPatcher;
    console.log('Patcher loaded. Run window.testPatcher() to test.');
}
