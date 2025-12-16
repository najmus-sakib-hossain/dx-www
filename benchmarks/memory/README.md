# dx-server Memory Profiling

**Target:** Beat Fiber's 5-15 MB per instance

## Memory Goals

| Metric | Target | Must Have |
|--------|--------|-----------|
| Idle Memory | < 5 MB | < 15 MB |
| Under Load | < 10 MB | < 30 MB |
| Per Connection | < 1 KB | < 5 KB |

## Running the Profiler

```bash
chmod +x profile.sh
./profile.sh
```

## What's Measured

1. **Baseline (Idle)**: Memory usage with no active connections
2. **Under Load**: Memory during active request handling
3. **Peak Memory**: Maximum memory observed
4. **Binary Size**: Compiled executable size

## Tools Used

- **ps**: Basic RSS/VSZ measurement
- **heaptrack**: Detailed heap analysis (Linux)
- **/proc/PID/status**: Linux-specific memory stats

## Memory Optimization Techniques Applied

1. **System allocator** - Predictable memory behavior
2. **Zero-copy responses** - No per-request allocations
3. **Pre-allocated buffers** - Fixed-size buffer pools
4. **LTO + opt-level=z** - Smaller binary = smaller memory footprint

## Results

See `results/` directory for timestamped profiling results.
