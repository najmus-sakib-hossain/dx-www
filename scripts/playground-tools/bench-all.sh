#!/bin/bash
# Comprehensive benchmark suite for dx-js-runtime

echo "═══════════════════════════════════════════════════════"
echo "  DX-JS RUNTIME - COMPREHENSIVE BENCHMARK SUITE"
echo "═══════════════════════════════════════════════════════"
echo ""

# Array of all test files
tests=(
    "simple_test.js"
    "bench-math-heavy.js"
    "bench-variables.js"
    "bench-comparisons.js"
    "bench-nested-math.js"
    "bench-arithmetic-chains.js"
    "bench-mixed-operations.js"
)

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

total_speedup=0
test_count=0

for test in "${tests[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${BLUE}Testing: $test${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    # Run hyperfine and capture output
    result=$(hyperfine --warmup 1 --runs 10 \
        --prepare "rm -rf /tmp/dx-cache" \
        --export-markdown /tmp/bench-$test.md \
        "/f/Code/dx/target/release/dx-js.exe $test" \
        "bun $test" 2>&1)
    
    # Extract times
    dx_time=$(echo "$result" | grep "Benchmark 1:" -A 1 | grep "Time" | awk '{print $5}')
    bun_time=$(echo "$result" | grep "Benchmark 2:" -A 1 | grep "Time" | awk '{print $5}')
    
    # Extract speedup
    speedup=$(echo "$result" | grep "ran" | grep "times faster" | awk '{print $2}')
    
    echo -e "${GREEN}DX-JS:${NC}  ${dx_time}ms"
    echo -e "${YELLOW}Bun:${NC}    ${bun_time}ms"
    echo -e "${RED}Speedup: ${speedup}x faster${NC}"
    echo ""
    
    # Add to total
    if [ ! -z "$speedup" ]; then
        total_speedup=$(echo "$total_speedup + $speedup" | bc)
        test_count=$((test_count + 1))
    fi
done

echo "═══════════════════════════════════════════════════════"
echo "  FINAL RESULTS"
echo "═══════════════════════════════════════════════════════"
echo ""

if [ $test_count -gt 0 ]; then
    avg_speedup=$(echo "scale=2; $total_speedup / $test_count" | bc)
    echo -e "${GREEN}Tests Completed: $test_count${NC}"
    echo -e "${RED}Average Speedup: ${avg_speedup}x faster than Bun${NC}"
    echo ""
    
    if (( $(echo "$avg_speedup >= 4.0" | bc -l) )); then
        echo -e "${GREEN}✅ TARGET ACHIEVED! (Goal: 4x, Actual: ${avg_speedup}x)${NC}"
    else
        echo -e "${YELLOW}⚠️  Close to target (Goal: 4x, Actual: ${avg_speedup}x)${NC}"
    fi
else
    echo -e "${RED}❌ No valid benchmark results${NC}"
fi

echo ""
echo "═══════════════════════════════════════════════════════"
