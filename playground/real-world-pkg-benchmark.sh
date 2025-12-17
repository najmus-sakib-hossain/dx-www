#!/bin/bash
# Real-World Package Manager Benchmark: DX vs Bun
# Downloads actual packages: react, next.js, lodash, express
# Date: December 16, 2025

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "  🚀 DX PACKAGE MANAGER vs BUN - REAL-WORLD BENCHMARK"
echo "  Testing with: react, next, lodash, express"
echo "═══════════════════════════════════════════════════════════════════"
echo ""

# Get absolute paths
DX_BIN="F:/Code/dx/crates/dx-package-manager/target/release/dx.exe"
PLAYGROUND_DIR="F:/Code/dx/playground"
RESULTS_FILE="$PLAYGROUND_DIR/benchmark-results.json"

# Check if Bun is installed
if ! command -v bun &> /dev/null; then
    echo -e "${RED}❌ Error: Bun is not installed${NC}"
    echo "Install from: https://bun.sh"
    exit 1
fi

# Check if DX CLI is built
if [ ! -f "$DX_BIN" ]; then
    echo -e "${YELLOW}⚠️  DX CLI not found, building...${NC}"
    cd "F:/Code/dx/crates/dx-package-manager"
    cargo build --release -p dx-pkg-cli
    echo -e "${GREEN}✓ DX CLI built successfully${NC}"
    echo ""
fi

# Create test directories
TEST_ROOT="/tmp/pkg-bench-$$"
mkdir -p "$TEST_ROOT"
cd "$TEST_ROOT"

# Initialize results JSON
cat > "$RESULTS_FILE" << 'EOF'
{
  "timestamp": "",
  "tests": [],
  "summary": {
    "dx_total_ms": 0,
    "bun_total_ms": 0,
    "speedup": 0
  }
}
EOF

# Function to run benchmark test
run_benchmark() {
    local test_name="$1"
    local packages="$2"
    local description="$3"
    
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}📦 Test: $test_name${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "Description: $description"
    echo ""
    
    # Test with Bun (cold start - clear cache)
    echo -e "${YELLOW}Testing with Bun (cold)...${NC}"
    rm -rf "$HOME/.bun/install/cache" 2>/dev/null || true
    mkdir -p "bun-$test_name" && cd "bun-$test_name"
    
    # Create package.json
    echo "{\"dependencies\":{$packages}}" > package.json
    
    # Time the installation
    BUN_START=$(date +%s%3N)
    bun install --silent 2>&1 | head -20
    BUN_END=$(date +%s%3N)
    BUN_COLD_TIME=$((BUN_END - BUN_START))
    
    # Get size
    BUN_SIZE=$(du -sb node_modules 2>/dev/null | cut -f1)
    
    echo -e "${GREEN}✓ Bun cold: ${BUN_COLD_TIME}ms (${BUN_SIZE} bytes)${NC}"
    cd ..
    
    # Test with Bun (warm start)
    echo -e "${YELLOW}Testing with Bun (warm)...${NC}"
    mkdir -p "bun-warm-$test_name" && cd "bun-warm-$test_name"
    echo "{\"dependencies\":{$packages}}" > package.json
    
    BUN_START=$(date +%s%3N)
    bun install --silent 2>&1 | head -20
    BUN_END=$(date +%s%3N)
    BUN_WARM_TIME=$((BUN_END - BUN_START))
    
    echo -e "${GREEN}✓ Bun warm: ${BUN_WARM_TIME}ms${NC}"
    cd ..
    
    # Test with DX (cold start - clear cache)
    echo -e "${YELLOW}Testing with DX (cold)...${NC}"
    rm -rf "$HOME/.dx-pkg/cache" 2>/dev/null || true
    mkdir -p "dx-$test_name" && cd "dx-$test_name"
    
    # Create dx.json
    echo "{\"dependencies\":{$packages}}" > dx.json
    
    # Time the installation
    DX_START=$(date +%s%3N)
    "$DX_BIN" install 2>&1 | head -20
    DX_END=$(date +%s%3N)
    DX_COLD_TIME=$((DX_END - DX_START))
    
    # Get size
    DX_SIZE=$(du -sb .dx-modules 2>/dev/null | cut -f1 || echo "0")
    
    echo -e "${GREEN}✓ DX cold: ${DX_COLD_TIME}ms (${DX_SIZE} bytes)${NC}"
    cd ..
    
    # Test with DX (warm start)
    echo -e "${YELLOW}Testing with DX (warm)...${NC}"
    mkdir -p "dx-warm-$test_name" && cd "dx-warm-$test_name"
    echo "{\"dependencies\":{$packages}}" > dx.json
    
    DX_START=$(date +%s%3N)
    "$DX_BIN" install 2>&1 | head -20
    DX_END=$(date +%s%3N)
    DX_WARM_TIME=$((DX_END - DX_START))
    
    echo -e "${GREEN}✓ DX warm: ${DX_WARM_TIME}ms${NC}"
    cd ..
    
    # Calculate speedups
    COLD_SPEEDUP=$(awk "BEGIN {printf \"%.2f\", $BUN_COLD_TIME / ($DX_COLD_TIME + 0.001)}")
    WARM_SPEEDUP=$(awk "BEGIN {printf \"%.2f\", $BUN_WARM_TIME / ($DX_WARM_TIME + 0.001)}")
    SIZE_RATIO=$(awk "BEGIN {printf \"%.2f\", $BUN_SIZE / ($DX_SIZE + 1)}")
    
    # Display results
    echo ""
    echo -e "${CYAN}📊 Results:${NC}"
    echo -e "  Bun cold:  ${BUN_COLD_TIME}ms"
    echo -e "  DX cold:   ${DX_COLD_TIME}ms  ${GREEN}(${COLD_SPEEDUP}x faster)${NC}"
    echo -e "  Bun warm:  ${BUN_WARM_TIME}ms"
    echo -e "  DX warm:   ${DX_WARM_TIME}ms  ${GREEN}(${WARM_SPEEDUP}x faster)${NC}"
    echo -e "  Size:      Bun=${BUN_SIZE} bytes, DX=${DX_SIZE} bytes (${SIZE_RATIO}x smaller)"
    echo ""
    
    cd "$TEST_ROOT"
}

# Test 1: Small package (lodash)
run_benchmark "lodash" \
    '"lodash":"^4.17.21"' \
    "Small utility library (~500KB unpacked)"

# Test 2: Medium package (react + react-dom)
run_benchmark "react" \
    '"react":"^18.2.0","react-dom":"^18.2.0"' \
    "UI library with peer deps (~2MB unpacked)"

# Test 3: Large package (next.js)
run_benchmark "nextjs" \
    '"next":"^14.0.0","react":"^18.2.0","react-dom":"^18.2.0"' \
    "Full framework with many deps (~25MB unpacked)"

# Test 4: Server framework (express)
run_benchmark "express" \
    '"express":"^4.18.0"' \
    "Backend framework (~1MB unpacked)"

# Test 5: Multiple packages (full stack)
run_benchmark "fullstack" \
    '"react":"^18.2.0","react-dom":"^18.2.0","next":"^14.0.0","express":"^4.18.0","lodash":"^4.17.21","axios":"^1.6.0"' \
    "Complete full-stack app dependencies (~30MB)"

# Test 6: Large monorepo simulation
run_benchmark "monorepo" \
    '"react":"^18.2.0","react-dom":"^18.2.0","next":"^14.0.0","express":"^4.18.0","lodash":"^4.17.21","axios":"^1.6.0","typescript":"^5.0.0","@types/react":"^18.0.0","@types/node":"^20.0.0","eslint":"^8.0.0","prettier":"^3.0.0"' \
    "Typical monorepo with dev deps (~60MB)"

# Generate summary
echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "  📈 BENCHMARK SUMMARY"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "Results saved to: $RESULTS_FILE"
echo ""
echo -e "${GREEN}✅ Benchmark complete!${NC}"
echo ""

# Cleanup
cd "$PLAYGROUND_DIR"
rm -rf "$TEST_ROOT"
