#!/usr/bin/env bash

# DX vs Bun Bundler Benchmark
# Measures bundling performance on a real React-like TSX application

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 DX vs Bun Bundler Performance Benchmark"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Paths
PLAYGROUND_DIR="$(pwd)/playground/bundler-test"
DX_BINARY="$(pwd)/target/release/dx-bundle"
CACHE_DIR="$(pwd)/playground/.dx-cache"
OUTPUT_DIR="$(pwd)/playground/output"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Check if DX binary exists
if [ ! -f "$DX_BINARY" ]; then
    echo "❌ DX binary not found. Building..."
    cd crates/dx-js-bundler
    cargo build --release -p dx-bundle-cli
    cd ../..
fi

# Check if Bun is installed
if ! command -v bun &> /dev/null; then
    echo "⚠️  Bun not found. Install with: curl -fsSL https://bun.sh/install | bash"
    echo "Skipping Bun benchmark..."
    BUN_AVAILABLE=0
else
    BUN_AVAILABLE=1
fi

echo "📦 Test Application:"
echo "   ├─ Entry: index.tsx"
echo "   ├─ Utils: utils.ts"
echo "   └─ Component: components/Component.tsx"
echo ""

# Warm up (prevent cold cache effects)
echo "🔥 Warming up..."
"$DX_BINARY" "$PLAYGROUND_DIR/index.tsx" -o "$OUTPUT_DIR/dx-warmup.js" --cache "$CACHE_DIR" > /dev/null 2>&1
rm -f "$OUTPUT_DIR/dx-warmup.js"

if [ $BUN_AVAILABLE -eq 1 ]; then
    bun build "$PLAYGROUND_DIR/index.tsx" --outfile "$OUTPUT_DIR/bun-warmup.js" > /dev/null 2>&1
    rm -f "$OUTPUT_DIR/bun-warmup.js"
fi

echo "✅ Warmed up"
echo ""

# Run DX bundler (3 iterations)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔵 DX Bundler (Binary Dawn)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

DX_TIMES=()
for i in {1..3}; do
    echo "Run $i/3..."
    rm -rf "$CACHE_DIR"
    START=$(date +%s%3N)
    "$DX_BINARY" "$PLAYGROUND_DIR/index.tsx" \
        -o "$OUTPUT_DIR/dx-bundle.js" \
        --cache "$CACHE_DIR" \
        --minify \
        --verbose
    END=$(date +%s%3N)
    TIME=$((END - START))
    DX_TIMES+=($TIME)
    echo "   Time: ${TIME}ms"
    echo ""
done

# Calculate average
DX_AVG=0
for time in "${DX_TIMES[@]}"; do
    DX_AVG=$((DX_AVG + time))
done
DX_AVG=$((DX_AVG / 3))

DX_SIZE=$(stat -f%z "$OUTPUT_DIR/dx-bundle.js" 2>/dev/null || stat -c%s "$OUTPUT_DIR/dx-bundle.js")

echo "DX Average: ${DX_AVG}ms"
echo "DX Size: $((DX_SIZE / 1024)) KB"
echo ""

# Run Bun bundler (3 iterations)
if [ $BUN_AVAILABLE -eq 1 ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🟠 Bun Bundler"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    BUN_TIMES=()
    for i in {1..3}; do
        echo "Run $i/3..."
        START=$(date +%s%3N)
        bun build "$PLAYGROUND_DIR/index.tsx" \
            --outfile "$OUTPUT_DIR/bun-bundle.js" \
            --minify
        END=$(date +%s%3N)
        TIME=$((END - START))
        BUN_TIMES+=($TIME)
        echo "   Time: ${TIME}ms"
        echo ""
    done
    
    # Calculate average
    BUN_AVG=0
    for time in "${BUN_TIMES[@]}"; do
        BUN_AVG=$((BUN_AVG + time))
    done
    BUN_AVG=$((BUN_AVG / 3))
    
    BUN_SIZE=$(stat -f%z "$OUTPUT_DIR/bun-bundle.js" 2>/dev/null || stat -c%s "$OUTPUT_DIR/bun-bundle.js")
    
    echo "Bun Average: ${BUN_AVG}ms"
    echo "Bun Size: $((BUN_SIZE / 1024)) KB"
    echo ""
    
    # Calculate speedup
    SPEEDUP=$(echo "scale=2; $BUN_AVG / $DX_AVG" | bc)
    SIZE_REDUCTION=$(echo "scale=2; (1 - $DX_SIZE / $BUN_SIZE) * 100" | bc)
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📊 Results"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "⚡ DX is ${SPEEDUP}x faster than Bun!"
    echo "📦 DX bundle is ${SIZE_REDUCTION}% smaller"
    echo ""
    echo "Detailed Comparison:"
    echo "   ├─ DX:   ${DX_AVG}ms | $((DX_SIZE / 1024)) KB"
    echo "   └─ Bun:  ${BUN_AVG}ms | $((BUN_SIZE / 1024)) KB"
    echo ""
else
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📊 Results (DX Only)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "DX Performance:"
    echo "   ├─ Time: ${DX_AVG}ms"
    echo "   └─ Size: $((DX_SIZE / 1024)) KB"
    echo ""
fi

echo "✅ Benchmark complete!"
echo "   Output: $OUTPUT_DIR"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
