#!/usr/bin/env bash

# Comprehensive Bundler Benchmark: dx-bundler-v2 vs Bun vs dx-js-bundler
# Tests all three bundlers on the same codebase

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🏁 Comprehensive Bundler Performance Benchmark"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Testing:"
echo "   1️⃣  DX Bundler v2 (New - Binary Everywhere)"
echo "   2️⃣  Bun Bundler"
echo "   3️⃣  DX JS Bundler (Current)"
echo ""

# Paths
PLAYGROUND_DIR="$(pwd)"
TEST_DIR="$PLAYGROUND_DIR/bundler-test"
V2_BINARY="$PLAYGROUND_DIR/../crates/dx-bundler-v2/target/release/dx-bundle"
JS_BUNDLER_DIR="$PLAYGROUND_DIR/../crates/dx-js-bundler"
OUTPUT_DIR="$PLAYGROUND_DIR/output"
CACHE_DIR="$PLAYGROUND_DIR/.dx-cache"

# Create directories
mkdir -p "$OUTPUT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check test files exist
if [ ! -f "$TEST_DIR/index.tsx" ]; then
    echo "❌ Test files not found in $TEST_DIR"
    exit 1
fi

echo "📦 Test Application:"
echo "   ├─ Entry: index.tsx"
echo "   ├─ Utils: utils.ts"  
echo "   └─ Component: components/Component.tsx"
echo ""

# Build dx-bundler-v2 if needed
echo "🔨 Building DX Bundler v2..."
if [ ! -f "$V2_BINARY" ]; then
    cd ../crates/dx-bundler-v2
    cargo build --release
    cd ../../playground
fi

if [ ! -f "$V2_BINARY" ]; then
    echo "❌ Failed to build dx-bundler-v2"
    exit 1
fi

echo "✅ DX Bundler v2 built"
echo ""

# Check Bun availability
if ! command -v bun &> /dev/null; then
    echo "⚠️  Bun not installed. Skipping Bun benchmark..."
    echo "   Install: curl -fsSL https://bun.sh/install | bash"
    BUN_AVAILABLE=0
else
    BUN_VERSION=$(bun --version)
    echo "✅ Bun $BUN_VERSION found"
    BUN_AVAILABLE=1
fi
echo ""

# Check dx-js-bundler availability
if [ -d "$JS_BUNDLER_DIR" ]; then
    echo "✅ DX JS Bundler found"
    JS_AVAILABLE=1
else
    echo "⚠️  DX JS Bundler not found. Skipping..."
    JS_AVAILABLE=0
fi
echo ""

# Benchmark configuration
RUNS=5
echo "🔧 Configuration:"
echo "   Runs per bundler: $RUNS"
echo "   Test file: $TEST_DIR/index.tsx"
echo ""

# Function to calculate median
calculate_median() {
    arr=("$@")
    sorted=($(printf '%s\n' "${arr[@]}" | sort -n))
    len=${#sorted[@]}
    mid=$((len / 2))
    echo "${sorted[$mid]}"
}

# Function to calculate average
calculate_average() {
    arr=("$@")
    sum=0
    for val in "${arr[@]}"; do
        sum=$((sum + val))
    done
    echo $((sum / ${#arr[@]}))
}

#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 1. DX BUNDLER V2 BENCHMARK
#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}🔵 1️⃣  DX Bundler v2 (Binary Everywhere)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

V2_TIMES=()
V2_COLD_TIME=0

for i in $(seq 1 $RUNS); do
    # Clear cache for first run (cold)
    if [ $i -eq 1 ]; then
        rm -rf "$CACHE_DIR"
        echo "Run $i/$RUNS (COLD)..."
    else
        echo "Run $i/$RUNS (WARM)..."
    fi
    
    START=$(date +%s%N)
    "$V2_BINARY" bundle "$TEST_DIR/index.tsx" \
        -o "$OUTPUT_DIR/v2-bundle.js" \
        --format esm \
        --cache \
        --cache-dir "$CACHE_DIR" \
        > /dev/null 2>&1
    END=$(date +%s%N)
    
    TIME=$(( (END - START) / 1000000 ))
    V2_TIMES+=($TIME)
    
    if [ $i -eq 1 ]; then
        V2_COLD_TIME=$TIME
    fi
    
    echo "   Time: ${TIME}ms"
done

V2_AVG=$(calculate_average "${V2_TIMES[@]}")
V2_MEDIAN=$(calculate_median "${V2_TIMES[@]}")
V2_SIZE=$(stat -c%s "$OUTPUT_DIR/v2-bundle.js" 2>/dev/null || stat -f%z "$OUTPUT_DIR/v2-bundle.js")

echo ""
echo "📊 Results:"
echo "   Cold:   ${V2_COLD_TIME}ms"
echo "   Average: ${V2_AVG}ms"
echo "   Median:  ${V2_MEDIAN}ms"
echo "   Size:    $((V2_SIZE / 1024)) KB"
echo ""

#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 2. BUN BUNDLER BENCHMARK
#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if [ $BUN_AVAILABLE -eq 1 ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${YELLOW}🟠 2️⃣  Bun Bundler${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    BUN_TIMES=()
    BUN_COLD_TIME=0
    
    for i in $(seq 1 $RUNS); do
        if [ $i -eq 1 ]; then
            echo "Run $i/$RUNS (COLD)..."
        else
            echo "Run $i/$RUNS (WARM)..."
        fi
        
        START=$(date +%s%N)
        bun build "$TEST_DIR/index.tsx" \
            --outfile "$OUTPUT_DIR/bun-bundle.js" \
            --format esm \
            > /dev/null 2>&1
        END=$(date +%s%N)
        
        TIME=$(( (END - START) / 1000000 ))
        BUN_TIMES+=($TIME)
        
        if [ $i -eq 1 ]; then
            BUN_COLD_TIME=$TIME
        fi
        
        echo "   Time: ${TIME}ms"
    done
    
    BUN_AVG=$(calculate_average "${BUN_TIMES[@]}")
    BUN_MEDIAN=$(calculate_median "${BUN_TIMES[@]}")
    BUN_SIZE=$(stat -c%s "$OUTPUT_DIR/bun-bundle.js" 2>/dev/null || stat -f%z "$OUTPUT_DIR/bun-bundle.js")
    
    echo ""
    echo "📊 Results:"
    echo "   Cold:    ${BUN_COLD_TIME}ms"
    echo "   Average: ${BUN_AVG}ms"
    echo "   Median:  ${BUN_MEDIAN}ms"
    echo "   Size:    $((BUN_SIZE / 1024)) KB"
    echo ""
fi

#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 3. DX JS BUNDLER BENCHMARK
#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if [ $JS_AVAILABLE -eq 1 ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${GREEN}🟢 3️⃣  DX JS Bundler (Current)${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Try to use dx-js-bundler CLI or direct Rust call
    JS_TIMES=()
    JS_COLD_TIME=0
    
    for i in $(seq 1 $RUNS); do
        if [ $i -eq 1 ]; then
            echo "Run $i/$RUNS (COLD)..."
        else
            echo "Run $i/$RUNS (WARM)..."
        fi
        
        # Note: This is a placeholder - adjust based on actual dx-js-bundler API
        START=$(date +%s%N)
        # Assuming there's a bundler binary or we can call via node
        cargo run --release --manifest-path "$JS_BUNDLER_DIR/Cargo.toml" -- \
            "$TEST_DIR/index.tsx" \
            -o "$OUTPUT_DIR/js-bundle.js" \
            > /dev/null 2>&1 || echo "   ⚠️  Benchmark not implemented"
        END=$(date +%s%N)
        
        TIME=$(( (END - START) / 1000000 ))
        JS_TIMES+=($TIME)
        
        if [ $i -eq 1 ]; then
            JS_COLD_TIME=$TIME
        fi
        
        echo "   Time: ${TIME}ms"
    done
    
    if [ -f "$OUTPUT_DIR/js-bundle.js" ]; then
        JS_AVG=$(calculate_average "${JS_TIMES[@]}")
        JS_MEDIAN=$(calculate_median "${JS_TIMES[@]}")
        JS_SIZE=$(stat -c%s "$OUTPUT_DIR/js-bundle.js" 2>/dev/null || stat -f%z "$OUTPUT_DIR/js-bundle.js")
        
        echo ""
        echo "📊 Results:"
        echo "   Cold:    ${JS_COLD_TIME}ms"
        echo "   Average: ${JS_AVG}ms"
        echo "   Median:  ${JS_MEDIAN}ms"
        echo "   Size:    $((JS_SIZE / 1024)) KB"
        echo ""
    fi
fi

#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# FINAL COMPARISON
#━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🏆 FINAL COMPARISON"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "┌────────────────────┬──────────┬──────────┬──────────┬──────────┐"
echo "│ Bundler            │ Cold     │ Median   │ Average  │ Size     │"
echo "├────────────────────┼──────────┼──────────┼──────────┼──────────┤"
printf "│ %-18s │ %7sms │ %7sms │ %7sms │ %7s KB │\n" \
    "DX Bundler v2" "$V2_COLD_TIME" "$V2_MEDIAN" "$V2_AVG" "$((V2_SIZE / 1024))"

if [ $BUN_AVAILABLE -eq 1 ]; then
    printf "│ %-18s │ %7sms │ %7sms │ %7sms │ %7s KB │\n" \
        "Bun" "$BUN_COLD_TIME" "$BUN_MEDIAN" "$BUN_AVG" "$((BUN_SIZE / 1024))"
fi

if [ $JS_AVAILABLE -eq 1 ] && [ -f "$OUTPUT_DIR/js-bundle.js" ]; then
    printf "│ %-18s │ %7sms │ %7sms │ %7sms │ %7s KB │\n" \
        "DX JS Bundler" "$JS_COLD_TIME" "$JS_MEDIAN" "$JS_AVG" "$((JS_SIZE / 1024))"
fi

echo "└────────────────────┴──────────┴──────────┴──────────┴──────────┘"
echo ""

# Calculate speedup vs Bun
if [ $BUN_AVAILABLE -eq 1 ]; then
    SPEEDUP=$(echo "scale=2; $BUN_MEDIAN / $V2_MEDIAN" | bc)
    
    echo "🚀 Performance vs Bun:"
    echo "   DX Bundler v2 is ${SPEEDUP}x"
    
    if (( $(echo "$SPEEDUP >= 3.0" | bc -l) )); then
        echo -e "   ${GREEN}✅ TARGET ACHIEVED! (3x faster)${NC}"
    elif (( $(echo "$SPEEDUP >= 1.0" | bc -l) )); then
        echo -e "   ${YELLOW}⚡ Faster, but below 3x target${NC}"
    else
        echo -e "   ${RED}❌ Slower than Bun${NC}"
    fi
    echo ""
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Benchmark Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
