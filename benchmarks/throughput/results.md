# dx-server Throughput Benchmark Results

**Date:** 2025-12-17
**Machine:** Windows (Mingw64/Git Bash)
**Target:** Beat Actix Web's ~1,200,000 RPS

## Test Configuration
- Duration: 10s
- Connections: 100
- Threads: 4
- Tool: `rewrk`

## Results (Windows Local)

| Server Type | Requests/Sec | Latency (Avg) | Transfer Rate |
|-------------|--------------|---------------|---------------|
| **dx-server (Axum)** | **104,963** | 0.95ms | 13.01 MB/s |
| Raw Hyper | 102,686 | 0.97ms | 13.51 MB/s |

> **Note:** These results are likely limited by the Windows networking stack or local loopback performance, as both high-level (Axum) and low-level (Hyper) implementations hit the same ceiling (~105k RPS).
> 
> To achieve the >1.2M RPS target, benchmarks must be run on a Linux environment with optimized kernel settings (standard for high-performance Rust benchmarks).

## Raw Output

### Axum
```
Requests:
  Total: 1049423 Req/Sec: 104963.40
Transfer:
  Total: 130.11 MB Transfer Rate: 13.01 MB/Sec
```

### Hyper Raw
```
Requests:
  Total: 1026718 Req/Sec: 102686.29
Transfer:
  Total: 135.12 MB Transfer Rate: 13.51 MB/Sec
```
