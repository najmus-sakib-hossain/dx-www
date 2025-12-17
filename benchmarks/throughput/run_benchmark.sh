#!/bin/bash
# =============================================================================
# dx-server Throughput Benchmark Script
# Target: Beat Actix Web's ~1,200,000 RPS
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
BENCHMARK_DURATION=30s
CONNECTIONS=512
THREADS=12
SERVER_PORT=8080
WARMUP_DURATION=5s

# Detect script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
RESULTS_FILE="$RESULTS_DIR/results_$(date +%Y%m%d_%H%M%S).md"

# Create results directory
mkdir -p "$RESULTS_DIR"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║       dx-server Throughput Benchmark Suite                   ║${NC}"
echo -e "${CYAN}║       Target: Beat Actix Web's 1,200,000 RPS                 ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check for required tools
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${YELLOW}Warning: $1 not found. Install with: $2${NC}"
        return 1
    fi
    return 0
}

echo -e "${BLUE}Checking benchmark tools...${NC}"
HAS_WRK=false
HAS_REWRK=false
HAS_DRILL=false

if check_tool "wrk" "apt install wrk / brew install wrk"; then
    HAS_WRK=true
fi

if check_tool "rewrk" "cargo install rewrk"; then
    HAS_REWRK=true
fi

if check_tool "drill" "cargo install drill"; then
    HAS_DRILL=true
fi

if [ "$HAS_WRK" = false ] && [ "$HAS_REWRK" = false ]; then
    echo -e "${RED}Error: Neither wrk nor rewrk found. Install at least one.${NC}"
    echo "  wrk:   apt install wrk (Linux) / brew install wrk (macOS)"
    echo "  rewrk: cargo install rewrk"
    exit 1
fi

# Function to build release binary
build_server() {
    local server_type=$1
    echo -e "${BLUE}Building $server_type server (release)...${NC}"
    cd "$SCRIPT_DIR"
    
    RUSTFLAGS="-C target-cpu=native" cargo build --release --bin "$server_type" 2>&1 | tail -3
    
    echo -e "${GREEN}✓ Build complete${NC}"
}

# Function to start server
start_server() {
    local server_type=$1
    local binary_path="$SCRIPT_DIR/target/release/$server_type"
    
    if [ ! -f "$binary_path" ]; then
        binary_path="$PROJECT_ROOT/target/release/$server_type"
    fi
    
    echo -e "${BLUE}Starting $server_type server...${NC}"
    "$binary_path" &
    SERVER_PID=$!
    sleep 2
    
    # Verify server is running
    if ! curl -s http://127.0.0.1:$SERVER_PORT/plaintext > /dev/null 2>&1; then
        if ! curl -s http://127.0.0.1:$SERVER_PORT/ > /dev/null 2>&1; then
            echo -e "${RED}Error: Server failed to start${NC}"
            exit 1
        fi
    fi
    
    echo -e "${GREEN}✓ Server running (PID: $SERVER_PID)${NC}"
}

# Function to stop server
stop_server() {
    if [ -n "$SERVER_PID" ]; then
        echo -e "${BLUE}Stopping server...${NC}"
        kill $SERVER_PID 2>/dev/null || true
        wait $SERVER_PID 2>/dev/null || true
        SERVER_PID=""
    fi
}

# Trap for cleanup
trap stop_server EXIT

# Function to run wrk benchmark
run_wrk() {
    local endpoint=$1
    local label=$2
    
    echo -e "${YELLOW}Running wrk: $label${NC}"
    
    # Warmup
    echo "  Warming up for $WARMUP_DURATION..."
    wrk -t4 -c100 -d$WARMUP_DURATION http://127.0.0.1:$SERVER_PORT$endpoint > /dev/null 2>&1
    
    # Actual benchmark
    echo "  Running benchmark for $BENCHMARK_DURATION with $CONNECTIONS connections..."
    wrk -t$THREADS -c$CONNECTIONS -d$BENCHMARK_DURATION http://127.0.0.1:$SERVER_PORT$endpoint
}

# Function to run rewrk benchmark
run_rewrk() {
    local endpoint=$1
    local label=$2
    
    echo -e "${YELLOW}Running rewrk: $label${NC}"
    
    # Warmup
    echo "  Warming up..."
    rewrk -t4 -c100 -d 5s -h http://127.0.0.1:$SERVER_PORT$endpoint > /dev/null 2>&1 || true
    
    # Actual benchmark
    echo "  Running benchmark for $BENCHMARK_DURATION with $CONNECTIONS connections..."
    rewrk -t$THREADS -c$CONNECTIONS -d 30s -h http://127.0.0.1:$SERVER_PORT$endpoint || echo "(rewrk may have issues on Windows)"
}

# Initialize results file
cat > "$RESULTS_FILE" << EOF
# dx-server Throughput Benchmark Results

**Date:** $(date)
**Machine:** $(uname -a)
**Target:** Beat Actix Web's ~1,200,000 RPS

## Test Configuration

| Parameter | Value |
|-----------|-------|
| Duration | $BENCHMARK_DURATION |
| Connections | $CONNECTIONS |
| Threads | $THREADS |

## Results

EOF

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  BENCHMARK 1: Axum Plaintext Server                           ${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

build_server "plaintext_server"
start_server "plaintext_server"

echo "" >> "$RESULTS_FILE"
echo "### Axum Plaintext Server" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo '```' >> "$RESULTS_FILE"

if [ "$HAS_WRK" = true ]; then
    run_wrk "/plaintext" "Axum Plaintext" | tee -a "$RESULTS_FILE"
fi

echo '```' >> "$RESULTS_FILE"

stop_server

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  BENCHMARK 2: Raw Hyper Server (Maximum Throughput)           ${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

build_server "hyper_raw_server"
start_server "hyper_raw_server"

echo "" >> "$RESULTS_FILE"
echo "### Raw Hyper Server (Maximum Throughput)" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo '```' >> "$RESULTS_FILE"

if [ "$HAS_WRK" = true ]; then
    run_wrk "/" "Raw Hyper" | tee -a "$RESULTS_FILE"
fi

echo '```' >> "$RESULTS_FILE"

stop_server

# Summary
echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  BENCHMARK COMPLETE                                           ${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "Results saved to: ${BLUE}$RESULTS_FILE${NC}"
echo ""

cat >> "$RESULTS_FILE" << EOF

## Comparison with Actix Web

| Framework | Expected RPS | Notes |
|-----------|--------------|-------|
| Actix Web | ~1,200,000 | Industry standard Rust framework |
| dx-server (Axum) | TBD | Our Axum-based implementation |
| dx-server (Raw Hyper) | TBD | Maximum throughput baseline |

## Verdict

TBD - Fill in after reviewing results above.
EOF

echo -e "${YELLOW}To compare with Actix Web:${NC}"
echo "  1. Review results above"
echo "  2. Compare with TechEmpower benchmarks: https://www.techempower.com/benchmarks/"
echo "  3. Run on Linux for accurate results (Windows/WSL may have lower numbers)"
echo ""
