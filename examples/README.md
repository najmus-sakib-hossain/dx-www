# Dx-WWW Examples & Demos

This folder contains working examples and interactive demos for the dx-www runtime.

---

## 🚀 Quick Start

### 1. Build WASM Runtime

```bash
# From project root
cd crates/dx-client
wasm-pack build --target web --out-dir ../../examples/pkg
```

### 2. Start Local Server

```bash
cd examples
python -m http.server 8080
```

### 3. Open Demo

Navigate to: http://localhost:8080/integration-demo.html

---

## 📁 File Structure

```
examples/
├── 📘 Integration (Phase 6 Complete Demo)
│   ├── integration-demo.html       # Interactive UI
│   ├── integration-example.js      # Complete workflow demos
│   └── dx-cache.js                 # IndexedDB + ETag logic
│
├── 📗 Individual Components
│   ├── streaming-example.js        # Day 12: Stream Consumer
│   └── patcher-example.js          # Day 13: XOR Patcher
│
├── 📕 Sample Apps
│   ├── counter-simple.tsx          # Simple counter (Micro runtime)
│   └── dashboard-complex.tsx       # Complex dashboard (Macro runtime)
│
└── 🏗️ Hello World Project
    └── hello-world/
        ├── src/lib.rs              # Basic WASM example
        ├── demo.html               # Minimal demo
        └── build.sh                # Build script
```

---

## 🎯 Demos & Examples

### 1. Integration Demo (Recommended)

**File:** [integration-demo.html](./integration-demo.html)

Complete demonstration of Phase 6 (Stream + Patch + Cache).

**Features:**
- ✅ Cache performance benchmarks
- ✅ Update workflow simulation
- ✅ ETag negotiation testing
- ✅ Live metrics dashboard
- ✅ Interactive console

**How to Use:**
1. Open `http://localhost:8080/integration-demo.html`
2. Click "Run Benchmark" to test cache speed
3. Click "Run Simulation" to see update workflow
4. Click "Test ETag" to validate negotiation logic

**Expected Results:**
- Cache overhead: ~5ms (target < 10ms) ✅
- Bandwidth savings: 95% with patches
- 304 responses: 0 bytes downloaded

---

### 2. Streaming Example

**File:** [streaming-example.js](./streaming-example.js)

Demonstrates incremental binary streaming (Day 12).

**API:**
```javascript
import { streamBinary } from './streaming-example.js';

// Stream from URL
const binary = await streamBinary('/app.dxb', {
    onProgress: (loaded, total) => {
        console.log(`Progress: ${loaded}/${total} bytes`);
    }
});
```

**Features:**
- Zero-copy chunk parsing
- Incremental processing
- Progress callbacks
- Network resumption

**Test:**
```javascript
// In browser console
await streamBinary('/test.dxb');
```

---

### 3. Patcher Example

**File:** [patcher-example.js](./patcher-example.js)

Demonstrates XOR-based binary patching (Day 13).

**API:**
```javascript
import { applyPatch } from './patcher-example.js';

// Apply patch to old binary
const newBinary = applyPatch(oldBinary, patchData);
```

**Features:**
- 4KB block-aligned XOR
- In-place modification
- < 1ms execution
- 95% bandwidth reduction

**Test:**
```javascript
// In browser console
const patch = createTestPatch();
const result = applyPatch(oldBinary, patch);
```

---

### 4. Cache Example

**File:** [dx-cache.js](./dx-cache.js)

Complete caching solution with IndexedDB (Day 14).

**API:**
```javascript
import { getCache, fetchWithCache } from './dx-cache.js';

// Get cache instance
const cache = await getCache();

// Fetch with automatic caching
const binary = await fetchWithCache('/app.dxb', cache);

// Manual operations
await cache.put(url, etag, binary);
const entry = await cache.get(url);
await cache.delete(url);
await cache.clear();

// Statistics
const stats = await cache.getStats();
```

**Features:**
- ETag-based versioning
- If-None-Match negotiation
- 304 vs 200 handling
- Quota enforcement (50 MB, 100 entries, 7 days)
- LRU eviction
- Metrics tracking

**Test:**
```javascript
// In browser console
const cache = await getCache();
await cache.put('/test.dxb', 'v1.0.0', new Uint8Array(1024));
const entry = await cache.get('/test.dxb');
console.log(entry);
```

---

## 🧪 Testing

### Run All Tests

Open [integration-demo.html](./integration-demo.html) and:

1. **Cache Benchmark** - Validates < 10ms target
2. **Update Simulation** - Tests complete workflow
3. **ETag Negotiation** - Verifies 304/200 logic

### Manual Testing

```javascript
// Open browser console at http://localhost:8080/integration-demo.html

// Test 1: Benchmark cache
await dxDemo.benchmarkCache();

// Test 2: Simulate updates
await dxDemo.simulateUpdate();

// Test 3: Test ETag
await dxDemo.testETagNegotiation();
```

---

## 📊 Performance Benchmarks

### Expected Results

| Test | Target | Typical Result | Status |
|------|--------|----------------|--------|
| **Cache Write** | < 10ms | 4-5ms | ✅ |
| **Cache Read (Cold)** | < 10ms | 2-3ms | ✅ |
| **Cache Read (Warm)** | < 1ms | 0.1-0.2ms | ✅ |
| **Patch Apply** | < 1ms | 0.2-0.3ms | ✅ |
| **Stream TTFB** | < 50ms | 30ms | ✅ |

### Bandwidth Savings

| Scenario | Download | Savings |
|----------|----------|---------|
| 304 Not Modified | ~200 bytes | 99.8% |
| 200 + Patch | ~5 KB | 95% |
| 200 + Full | ~100 KB | 0% |

---

## 🔧 Building from Source

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Build WASM Client

```bash
cd crates/dx-client
wasm-pack build --target web --out-dir ../../examples/pkg
```

### Build Hello World Example

```bash
cd examples/hello-world
./build.sh  # or build.bat on Windows
```

---

## 📚 Documentation

- **Quick Reference:** [docs/PHASE_6_QUICK_REFERENCE.md](../docs/PHASE_6_QUICK_REFERENCE.md)
- **Complete Guide:** [docs/PHASE_6_VICTORY.md](../docs/PHASE_6_VICTORY.md)
- **Day 12 (Streaming):** [docs/DAY_12_STREAM_CONSUMER.md](../docs/DAY_12_STREAM_CONSUMER.md)
- **Day 13 (Patching):** [docs/DAY_13_CLIENT_PATCHER.md](../docs/DAY_13_CLIENT_PATCHER.md)
- **Day 14 (Caching):** [docs/DAY_14_ETERNAL_CACHE.md](../docs/DAY_14_ETERNAL_CACHE.md)

---

## 🐛 Troubleshooting

### WASM Not Loading

**Symptom:** `Failed to fetch` or `WebAssembly.instantiate` errors

**Solution:**
1. Check that WASM file exists in `pkg/` folder
2. Ensure local server is running (not `file://`)
3. Check browser console for CORS errors

```bash
# Rebuild WASM
cd crates/dx-client
wasm-pack build --target web --out-dir ../../examples/pkg
```

### Cache Not Working

**Symptom:** Cache always misses, no persistence

**Solution:**
1. Check IndexedDB is enabled in browser
2. Check browser storage quota
3. Open DevTools → Application → IndexedDB → `dx-cache`

```javascript
// Check cache status
const cache = await getCache();
const stats = await cache.getStats();
console.log(stats);
```

### Patch Errors

**Symptom:** `Patch failed` or incorrect binary output

**Solution:**
1. Ensure old binary matches patch expectations
2. Check patch format (PatchHeader + PatchBlocks)
3. Verify block alignment (4096 bytes)

```javascript
// Debug patch
console.log('Old size:', oldBinary.length);
console.log('Patch size:', patchData.length);
console.log('Expected blocks:', Math.ceil(oldBinary.length / 4096));
```

### Performance Issues

**Symptom:** Cache operations taking > 10ms

**Solution:**
1. Check cache size (may need cleanup)
2. Clear old entries
3. Check browser storage quota

```javascript
// Clear cache
const cache = await getCache();
await cache.clear();

// Check quota
const estimate = await navigator.storage.estimate();
console.log('Used:', estimate.usage);
console.log('Quota:', estimate.quota);
```

---

## 🔗 Related Files

### WASM Source Code

- [crates/dx-client/src/streaming.rs](../crates/dx-client/src/streaming.rs)
- [crates/dx-client/src/patcher.rs](../crates/dx-client/src/patcher.rs)
- [crates/dx-client/src/lib.rs](../crates/dx-client/src/lib.rs)

### Build Configuration

- [crates/dx-client/Cargo.toml](../crates/dx-client/Cargo.toml)
- [Cargo.toml](../Cargo.toml) (workspace root)

---

## 🎓 Learning Path

### Beginner

1. Start with [integration-demo.html](./integration-demo.html)
2. Run benchmarks to see performance
3. Read [PHASE_6_QUICK_REFERENCE.md](../docs/PHASE_6_QUICK_REFERENCE.md)

### Intermediate

1. Study [dx-cache.js](./dx-cache.js) implementation
2. Understand ETag negotiation logic
3. Read individual day docs (Day 12, 13, 14)

### Advanced

1. Read WASM source code in `crates/dx-client/src/`
2. Modify examples to test new features
3. Build custom applications using the APIs

---

## 💡 Usage Examples

### Example 1: Basic App Loader

```javascript
import init from './pkg/dx_client.js';
import { getCache, updateBinary } from './dx-cache.js';

async function loadApp() {
    // Initialize
    await init();
    const cache = await getCache();
    
    // Load binary (with automatic caching)
    const binary = await updateBinary('/app.dxb', cache);
    
    // Use binary...
    console.log('App loaded:', binary.length, 'bytes');
}

loadApp();
```

### Example 2: Progress Tracking

```javascript
import { streamBinary } from './streaming-example.js';

async function loadWithProgress() {
    const binary = await streamBinary('/app.dxb', {
        onProgress: (loaded, total) => {
            const percent = (loaded / total * 100).toFixed(1);
            console.log(`Loading: ${percent}%`);
        }
    });
    console.log('Complete!');
}
```

### Example 3: Manual Cache Control

```javascript
import { getCache } from './dx-cache.js';

async function manualCache() {
    const cache = await getCache();
    
    // Check cache
    const entry = await cache.get('/app.dxb');
    
    if (entry) {
        // Use cached version
        console.log('Using cache:', entry.etag);
        return entry.binary;
    } else {
        // Download and cache
        const response = await fetch('/app.dxb');
        const etag = response.headers.get('ETag');
        const binary = new Uint8Array(await response.arrayBuffer());
        
        await cache.put('/app.dxb', etag, binary);
        return binary;
    }
}
```

---

## 🚀 Next Steps

**After exploring the examples:**

1. **Read Documentation**
   - Start: [PHASE_6_VICTORY.md](../docs/PHASE_6_VICTORY.md)
   - Reference: [PHASE_6_QUICK_REFERENCE.md](../docs/PHASE_6_QUICK_REFERENCE.md)

2. **Build Your App**
   - Use the provided APIs
   - Customize cache configuration
   - Monitor performance metrics

3. **Contribute**
   - Report issues
   - Submit improvements
   - Share your use cases

---

**Questions?** Check the [docs](../docs/) folder or open an issue.

**Phase 6: Stream + Patch + Cache = Binary Web 🎯**

*Last Updated: December 14, 2025*
