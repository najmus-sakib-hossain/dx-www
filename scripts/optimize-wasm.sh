#!/bin/bash
# optimize-wasm.sh - Optimize the dx-client runtime WASM binary
# This reduces the bundle size from ~23 KB to ~19 KB

set -e

echo "🔧 dx-www WASM Optimization Script"
echo ""

# Check if wasm-opt is available
if ! command -v wasm-opt &> /dev/null; then
    echo "⚠️  wasm-opt not found. Installing via npm..."
    npm install -g binaryen
    
    # Check again
    if ! npx wasm-opt --version &> /dev/null; then
        echo "❌ Failed to install wasm-opt"
        exit 1
    fi
    echo "✅ Installed binaryen"
fi

# Build the WASM binary first
echo "📦 Building dx-client WASM..."
cargo build --release -p dx-client --target wasm32-unknown-unknown

# Run wasm-bindgen to generate JS bindings
echo "🔗 Generating JS bindings..."
wasm-bindgen \
    target/wasm32-unknown-unknown/release/dx_client.wasm \
    --out-dir target/pkg_optimized \
    --target web \
    --no-typescript

INPUT_FILE="target/pkg_optimized/dx_client_bg.wasm"
OUTPUT_FILE="target/pkg_optimized/dx_client_optimized.wasm"

# Check if input exists
if [ ! -f "$INPUT_FILE" ]; then
    echo "❌ Input file not found: $INPUT_FILE"
    exit 1
fi

# Get original size
ORIGINAL_SIZE=$(stat -f%z "$INPUT_FILE" 2>/dev/null || stat -c%s "$INPUT_FILE")
ORIGINAL_KB=$((ORIGINAL_SIZE / 1024))

echo "📊 Original size: ${ORIGINAL_KB} KB"
echo "🚀 Optimizing with wasm-opt..."

# Run wasm-opt with maximum optimization
npx wasm-opt \
    -Oz \
    --enable-bulk-memory \
    --enable-sign-ext \
    --enable-mutable-globals \
    "$INPUT_FILE" \
    -o "$OUTPUT_FILE"

# Get optimized size
OPTIMIZED_SIZE=$(stat -f%z "$OUTPUT_FILE" 2>/dev/null || stat -c%s "$OUTPUT_FILE")
OPTIMIZED_KB=$((OPTIMIZED_SIZE / 1024))

# Calculate reduction
REDUCTION=$((100 - (OPTIMIZED_SIZE * 100 / ORIGINAL_SIZE)))

echo "✅ Optimized size: ${OPTIMIZED_KB} KB"
echo "📉 Reduction: ${REDUCTION}%"
echo ""
echo "✨ Output: $OUTPUT_FILE"

# Copy to final location
cp "$OUTPUT_FILE" "target/pkg_optimized/dx_client_bg.wasm"
echo "📁 Updated: target/pkg_optimized/dx_client_bg.wasm"

echo ""
echo "🎯 Ready for production!"
echo ""
echo "To use this build:"
echo "  <script type=\"module\">"
echo "    import init from './target/pkg_optimized/dx_client.js';"
echo "    await init();"
echo "  </script>"
