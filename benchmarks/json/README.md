# JSON Serialization Benchmarks

**Goal:** Prove dx-serializer throughput advantage in HTTP context

## Benchmarks Included

1. **Single User** - Small object serialization
2. **10 Users** - Medium list response
3. **100 Users** - Large API response

## Running Benchmarks

```bash
cd f:/Code/dx/benchmarks/json
cargo bench
```

## Metrics Measured

- Serialization throughput (bytes/sec)
- Deserialization throughput (bytes/sec)
- Memory allocations (via pre-allocated buffers)

## Expected Results

dx-serializer should outperform serde_json due to:
1. Zero-copy design
2. Binary format (smaller, faster parsing)
3. Pre-computed layouts

## Results

Run `cargo bench` and results will be saved to `target/criterion/`.
