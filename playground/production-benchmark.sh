#!/bin/bash
# DX Package Manager v3.0 - Production Benchmark Suite
# Tests both cold start (3x target) and warm start (50x target)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$SCRIPT_DIR/real-world-test"
DX_BIN="/f/Code/dx/crates/dx-js-package-manager/target/release/dx.exe"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo "╔════════════════════════════════════════════════════════════╗"
echo "║   DX Package Manager v3.0 - Production Benchmark Suite    ║"
echo "╠════════════════════════════════════════════════════════════╣"
echo "║                                                            ║"
echo "║  Target 1: 3x faster than Bun (Cold Start)                ║"
echo "║  Target 2: 50x faster than Bun (Warm Start)               ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

cd "$TEST_DIR"

# Ensure DX binary exists
if [ ! -f "$DX_BIN" ]; then
    echo -e "${RED}Error: DX binary not found at $DX_BIN${NC}"
    echo "Building DX binary..."
    cd /f/Code/dx/crates/dx-js-package-manager
    cargo build --release -p dx-pkg-cli
    cd "$TEST_DIR"
fi

echo -e "${CYAN}Using DX binary: $DX_BIN${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 1: Cold Start Benchmark (3x target)
# ═══════════════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════════════"
echo "  PHASE 1: Cold Start Benchmark (3x Faster Target)"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Clean all caches
echo -e "${YELLOW}Cleaning all caches...${NC}"
rm -rf node_modules bun.lockb dx-lock.json ~/.dx 2>/dev/null || true
echo "✓ All caches cleaned"
echo ""

# Bun baseline (cold)
echo -e "${BLUE}━━━ Bun Cold Install (Baseline) ━━━${NC}"
echo -n "Installing with Bun (run 1/3)... "
BUN_TIME1=$( (time -p bun install 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${BUN_TIME1}s"

rm -rf node_modules bun.lockb
echo -n "Installing with Bun (run 2/3)... "
BUN_TIME2=$( (time -p bun install 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${BUN_TIME2}s"

rm -rf node_modules bun.lockb
echo -n "Installing with Bun (run 3/3)... "
BUN_TIME3=$( (time -p bun install 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${BUN_TIME3}s"

# Calculate Bun average
BUN_COLD_AVG=$(echo "scale=3; ($BUN_TIME1 + $BUN_TIME2 + $BUN_TIME3) / 3" | bc)
echo ""
echo -e "${BLUE}Bun Cold Average: ${BUN_COLD_AVG}s${NC}"
echo ""

# Clean for DX test
rm -rf node_modules bun.lockb ~/.dx 2>/dev/null || true

# DX v3 cold start
echo -e "${GREEN}━━━ DX v3.0 Cold Install ━━━${NC}"
echo -n "Installing with DX v3 (run 1/3)... "
DX_COLD_TIME1=$( (time -p "$DX_BIN" install --v3 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${DX_COLD_TIME1}s"

rm -rf node_modules dx-lock.json ~/.dx
echo -n "Installing with DX v3 (run 2/3)... "
DX_COLD_TIME2=$( (time -p "$DX_BIN" install --v3 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${DX_COLD_TIME2}s"

rm -rf node_modules dx-lock.json ~/.dx
echo -n "Installing with DX v3 (run 3/3)... "
DX_COLD_TIME3=$( (time -p "$DX_BIN" install --v3 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${DX_COLD_TIME3}s"

# Calculate DX cold average
DX_COLD_AVG=$(echo "scale=3; ($DX_COLD_TIME1 + $DX_COLD_TIME2 + $DX_COLD_TIME3) / 3" | bc)
echo ""
echo -e "${GREEN}DX Cold Average: ${DX_COLD_AVG}s${NC}"
echo ""

# Calculate cold speedup
COLD_SPEEDUP=$(echo "scale=2; $BUN_COLD_AVG / $DX_COLD_AVG" | bc)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${CYAN}Cold Start Result: ${COLD_SPEEDUP}x faster than Bun${NC}"
if (( $(echo "$COLD_SPEEDUP >= 3.0" | bc -l) )); then
    echo -e "${GREEN}✅ COLD START TARGET ACHIEVED! (≥3x)${NC}"
else
    echo -e "${YELLOW}⚠️  Cold start: ${COLD_SPEEDUP}x (target: 3x)${NC}"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 2: Warm Start Benchmark (50x target)
# ═══════════════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════════════"
echo "  PHASE 2: Warm Start Benchmark (50x Faster Target)"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Clean node_modules but keep caches
rm -rf node_modules

# Bun warm (with cache)
echo -e "${BLUE}━━━ Bun Warm Install (With Cache) ━━━${NC}"
echo -n "Installing with Bun (run 1/3)... "
BUN_WARM_TIME1=$( (time -p bun install 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${BUN_WARM_TIME1}s"

rm -rf node_modules
echo -n "Installing with Bun (run 2/3)... "
BUN_WARM_TIME2=$( (time -p bun install 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${BUN_WARM_TIME2}s"

rm -rf node_modules
echo -n "Installing with Bun (run 3/3)... "
BUN_WARM_TIME3=$( (time -p bun install 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${BUN_WARM_TIME3}s"

# Calculate Bun warm average
BUN_WARM_AVG=$(echo "scale=3; ($BUN_WARM_TIME1 + $BUN_WARM_TIME2 + $BUN_WARM_TIME3) / 3" | bc)
echo ""
echo -e "${BLUE}Bun Warm Average: ${BUN_WARM_AVG}s${NC}"
echo ""

# DX v3 warm (metadata + tarball cache from v1.5)
rm -rf node_modules
echo -e "${GREEN}━━━ DX v3.0 Warm Install (With Cache) ━━━${NC}"
echo -n "Installing with DX v3 (run 1/3)... "
DX_WARM_TIME1=$( (time -p "$DX_BIN" install --v3 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${DX_WARM_TIME1}s"

rm -rf node_modules
echo -n "Installing with DX v3 (run 2/3)... "
DX_WARM_TIME2=$( (time -p "$DX_BIN" install --v3 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${DX_WARM_TIME2}s"

rm -rf node_modules
echo -n "Installing with DX v3 (run 3/3)... "
DX_WARM_TIME3=$( (time -p "$DX_BIN" install --v3 2>&1) 2>&1 | grep real | awk '{print $2}')
echo "✓ ${DX_WARM_TIME3}s"

# Calculate DX warm average
DX_WARM_AVG=$(echo "scale=3; ($DX_WARM_TIME1 + $DX_WARM_TIME2 + $DX_WARM_TIME3) / 3" | bc)
echo ""
echo -e "${GREEN}DX Warm Average: ${DX_WARM_AVG}s${NC}"
echo ""

# Calculate warm speedup
WARM_SPEEDUP=$(echo "scale=2; $BUN_WARM_AVG / $DX_WARM_AVG" | bc)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${CYAN}Warm Start Result: ${WARM_SPEEDUP}x faster than Bun${NC}"
if (( $(echo "$WARM_SPEEDUP >= 50.0" | bc -l) )); then
    echo -e "${GREEN}✅ WARM START TARGET ACHIEVED! (≥50x)${NC}"
else
    echo -e "${YELLOW}⚠️  Warm start: ${WARM_SPEEDUP}x (target: 50x)${NC}"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# ═══════════════════════════════════════════════════════════════
# FINAL SUMMARY
# ═══════════════════════════════════════════════════════════════

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║              PRODUCTION BENCHMARK RESULTS                  ║"
echo "╠════════════════════════════════════════════════════════════╣"
echo "║                                                            ║"
echo "║  COLD START (3x target):                                  ║"
printf "║    Bun:        %8.3fs                                   ║\n" "$BUN_COLD_AVG"
printf "║    DX v3:      %8.3fs                                   ║\n" "$DX_COLD_AVG"
printf "║    Speedup:    %8.2fx                                   ║\n" "$COLD_SPEEDUP"
if (( $(echo "$COLD_SPEEDUP >= 3.0" | bc -l) )); then
    echo "║    Status:     ✅ ACHIEVED                              ║"
else
    echo "║    Status:     ⚠️  NOT YET                              ║"
fi
echo "║                                                            ║"
echo "║  WARM START (50x target):                                 ║"
printf "║    Bun:        %8.3fs                                   ║\n" "$BUN_WARM_AVG"
printf "║    DX v3:      %8.3fs                                   ║\n" "$DX_WARM_AVG"
printf "║    Speedup:    %8.2fx                                   ║\n" "$WARM_SPEEDUP"
if (( $(echo "$WARM_SPEEDUP >= 50.0" | bc -l) )); then
    echo "║    Status:     ✅ ACHIEVED                              ║"
else
    echo "║    Status:     ⚠️  NOT YET                              ║"
fi
echo "║                                                            ║"
echo "╠════════════════════════════════════════════════════════════╣"
if (( $(echo "$COLD_SPEEDUP >= 3.0" | bc -l) )) && (( $(echo "$WARM_SPEEDUP >= 50.0" | bc -l) )); then
    echo "║                                                            ║"
    echo "║  🎉 ALL TARGETS ACHIEVED! PRODUCTION READY! 🚀            ║"
    echo "║                                                            ║"
else
    echo "║                                                            ║"
    echo "║  📊 Benchmarks complete. Review results above.            ║"
    echo "║                                                            ║"
fi
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Save results to file
RESULTS_FILE="$SCRIPT_DIR/PRODUCTION_BENCHMARK_RESULTS.md"
cat > "$RESULTS_FILE" << EOF
# DX Package Manager v3.0 - Production Benchmark Results

**Date:** $(date)
**Test Project:** Next.js dependencies (5 packages)

## Cold Start Results

| Metric | Bun | DX v3 | Speedup |
|--------|-----|-------|---------|
| Run 1 | ${BUN_TIME1}s | ${DX_COLD_TIME1}s | - |
| Run 2 | ${BUN_TIME2}s | ${DX_COLD_TIME2}s | - |
| Run 3 | ${BUN_TIME3}s | ${DX_COLD_TIME3}s | - |
| **Average** | **${BUN_COLD_AVG}s** | **${DX_COLD_AVG}s** | **${COLD_SPEEDUP}x** |

**Target:** 3x faster  
**Status:** $(if (( $(echo "$COLD_SPEEDUP >= 3.0" | bc -l) )); then echo "✅ ACHIEVED"; else echo "⚠️ ${COLD_SPEEDUP}x"; fi)

## Warm Start Results

| Metric | Bun | DX v3 | Speedup |
|--------|-----|-------|---------|
| Run 1 | ${BUN_WARM_TIME1}s | ${DX_WARM_TIME1}s | - |
| Run 2 | ${BUN_WARM_TIME2}s | ${DX_WARM_TIME2}s | - |
| Run 3 | ${BUN_WARM_TIME3}s | ${DX_WARM_TIME3}s | - |
| **Average** | **${BUN_WARM_AVG}s** | **${DX_WARM_AVG}s** | **${WARM_SPEEDUP}x** |

**Target:** 50x faster  
**Status:** $(if (( $(echo "$WARM_SPEEDUP >= 50.0" | bc -l) )); then echo "✅ ACHIEVED"; else echo "⚠️ ${WARM_SPEEDUP}x"; fi)

## Overall Assessment

$(if (( $(echo "$COLD_SPEEDUP >= 3.0" | bc -l) )) && (( $(echo "$WARM_SPEEDUP >= 50.0" | bc -l) )); then
    echo "**🎉 ALL TARGETS ACHIEVED! PRODUCTION READY! 🚀**"
else
    echo "**Status:** Benchmarks complete. See results above for details."
fi)

## System Information

- **OS:** $(uname -s)
- **DX Binary:** $DX_BIN
- **Test Directory:** $TEST_DIR
EOF

echo -e "${CYAN}Results saved to: $RESULTS_FILE${NC}"
echo ""
