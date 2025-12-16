#!/bin/bash
# =============================================================================
# dx-server Memory Profiling Script
# Target: Beat Fiber's 5-15 MB per instance
# =============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SERVER_PORT=8080
LOAD_DURATION=10
LOAD_CONNECTIONS=100
LOAD_THREADS=4

# Detect script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
RESULTS_FILE="$RESULTS_DIR/memory_$(date +%Y%m%d_%H%M%S).md"

# Create results directory
mkdir -p "$RESULTS_DIR"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║       dx-server Memory Profiling Suite                       ║${NC}"
echo -e "${CYAN}║       Target: Beat Fiber's 5-15 MB per instance              ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════${NC}"
echo ""

# Build server with maximum optimizations
echo -e "${BLUE}Building dx-server with memory optimizations...${NC}"
cd "$PROJECT_ROOT"

RUSTFLAGS="-C target-cpu=native -C lto=fat -C opt-level=z" \
    cargo build --release -p dx-server 2>&1 | tail -3

SERVER_BIN="$PROJECT_ROOT/target/release/dx-server"

if [ ! -f "$SERVER_BIN" ]; then
    echo -e "${RED}Error: dx-server binary not found${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Build complete${NC}"
echo ""

# Function to get memory usage (cross-platform)
get_memory() {
    local pid=$1
    
    if [ "$(uname)" = "Linux" ]; then
        # Linux: read from /proc
        if [ -f "/proc/$pid/status" ]; then
            local rss=$(grep VmRSS /proc/$pid/status | awk '{print $2}')
            local vsz=$(grep VmSize /proc/$pid/status | awk '{print $2}')
            echo "RSS: ${rss} KB, VSZ: ${vsz} KB"
        else
            ps -o pid,rss,vsz -p $pid 2>/dev/null | tail -1
        fi
    elif [ "$(uname)" = "Darwin" ]; then
        # macOS
        ps -o pid,rss,vsz -p $pid 2>/dev/null | tail -1
    else
        # Windows/WSL fallback
        ps -o pid,rss,vsz -p $pid 2>/dev/null | tail -1 || echo "Memory info unavailable"
    fi
}

# Function to measure memory at different stages
measure_memory() {
    local stage=$1
    local pid=$2
    
    echo -e "${YELLOW}[$stage]${NC}"
    get_memory $pid
}

# Initialize results file
cat > "$RESULTS_FILE" << EOF
# dx-server Memory Profiling Results

**Date:** $(date)
**Machine:** $(uname -a)
**Target:** Beat Fiber's 5-15 MB per instance

## Binary Size

EOF

# Get binary size
BINARY_SIZE=$(ls -lh "$SERVER_BIN" | awk '{print $5}')
BINARY_SIZE_BYTES=$(ls -l "$SERVER_BIN" | awk '{print $5}')

echo "Binary size: $BINARY_SIZE ($BINARY_SIZE_BYTES bytes)" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Start server
echo -e "${BLUE}Starting dx-server...${NC}"
"$SERVER_BIN" &
SERVER_PID=$!
sleep 2

# Verify server is running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo -e "${RED}Error: Server failed to start${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Server running (PID: $SERVER_PID)${NC}"
echo ""

# Cleanup trap
cleanup() {
    echo -e "${BLUE}Stopping server...${NC}"
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
}
trap cleanup EXIT

# Measure baseline (idle)
echo "## Memory Measurements" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "### Baseline (Idle)" >> "$RESULTS_FILE"
echo '```' >> "$RESULTS_FILE"

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Stage 1: Baseline Memory (Idle)                              ${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"

measure_memory "Idle" $SERVER_PID | tee -a "$RESULTS_FILE"
echo '```' >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Generate load and measure
echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Stage 2: Memory Under Load                                   ${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"

echo "### Under Load (10K requests)" >> "$RESULTS_FILE"
echo '```' >> "$RESULTS_FILE"

if command -v wrk &> /dev/null; then
    echo "Generating load with wrk..."
    wrk -t$LOAD_THREADS -c$LOAD_CONNECTIONS -d${LOAD_DURATION}s http://127.0.0.1:$SERVER_PORT/ &
    LOAD_PID=$!
    
    sleep 5
    measure_memory "Under Load" $SERVER_PID | tee -a "$RESULTS_FILE"
    
    wait $LOAD_PID 2>/dev/null || true
elif command -v curl &> /dev/null; then
    echo "Generating load with curl (limited)..."
    for i in $(seq 1 1000); do
        curl -s http://127.0.0.1:$SERVER_PORT/ > /dev/null &
    done
    
    sleep 3
    measure_memory "Under Load" $SERVER_PID | tee -a "$RESULTS_FILE"
    
    wait
else
    echo "No load generator available, skipping load test"
fi

echo '```' >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Peak memory (after load)
echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Stage 3: Peak Memory (Post-Load)                             ${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"

echo "### Peak Memory (Post-Load)" >> "$RESULTS_FILE"
echo '```' >> "$RESULTS_FILE"
measure_memory "Peak (after load)" $SERVER_PID | tee -a "$RESULTS_FILE"
echo '```' >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Heaptrack analysis (if available)
echo "" >> "$RESULTS_FILE"
echo "## Advanced Analysis" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

if command -v heaptrack &> /dev/null; then
    echo -e "${BLUE}Running heaptrack analysis...${NC}"
    echo "(This may take a moment)"
    
    # Stop current server
    cleanup
    
    # Run with heaptrack
    heaptrack "$SERVER_BIN" &
    HEAP_PID=$!
    sleep 3
    
    if command -v wrk &> /dev/null; then
        wrk -t2 -c50 -d5s http://127.0.0.1:$SERVER_PORT/ > /dev/null 2>&1 || true
    fi
    
    kill $HEAP_PID 2>/dev/null || true
    wait $HEAP_PID 2>/dev/null || true
    
    echo "Heaptrack data saved. Analyze with: heaptrack_gui heaptrack.dx-server.*.gz" | tee -a "$RESULTS_FILE"
else
    echo "heaptrack not available (install with: apt install heaptrack)" >> "$RESULTS_FILE"
fi

# Summary
echo "" >> "$RESULTS_FILE"
cat >> "$RESULTS_FILE" << EOF

## Comparison with Fiber (Go)

| Metric | Fiber (Go) | dx-server | Status |
|--------|------------|-----------|--------|
| Idle Memory | 5-15 MB | TBD | TBD |
| Under Load | ~20 MB | TBD | TBD |
| Binary Size | ~5 MB | $BINARY_SIZE | ✓ |

## Verdict

TBD - Fill in after reviewing results above.

## Notes

1. Memory measurements use RSS (Resident Set Size)
2. VSZ (Virtual Size) includes shared libraries
3. For accurate comparison, test on Linux with same load profile
EOF

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  PROFILING COMPLETE                                           ${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "Results saved to: ${BLUE}$RESULTS_FILE${NC}"
echo ""
