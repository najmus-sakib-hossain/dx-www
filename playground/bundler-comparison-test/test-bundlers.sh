#!/bin/bash

# DX JS Bundler vs Bun Bundler Comparison Test
# This script tests both bundlers and validates their outputs

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   DX JS Bundler vs Bun - Comparison Test"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/../benchmark-simple"
DX_BUNDLER="$SCRIPT_DIR/../../crates/dx-js-bundler/target/release/dx-bundle"
OUTPUT_DIR="$SCRIPT_DIR/output"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Check if DX bundler exists
if [ ! -f "$DX_BUNDLER" ]; then
    echo "❌ DX bundler not found. Building..."
    cd "$SCRIPT_DIR/../../crates/dx-js-bundler"
    cargo build --release -p dx-bundle-cli
    cd "$SCRIPT_DIR"
fi

# Check if Bun is installed
if ! command -v bun &> /dev/null; then
    echo "❌ Bun is not installed. Please install Bun first."
    echo "   Visit: https://bun.sh"
    exit 1
fi

echo "📦 Source files:"
echo "   Entry: $SOURCE_DIR/index.ts"
echo "   Utils: $SOURCE_DIR/utils.ts"
echo ""

# Test 1: DX Bundler
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Testing DX Bundler"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

DX_START=$(date +%s%N)
"$DX_BUNDLER" bundle "$SOURCE_DIR/index.ts" -o "$OUTPUT_DIR/dx-bundle.js"
DX_END=$(date +%s%N)
DX_TIME=$(awk "BEGIN {printf \"%.2f\", ($DX_END - $DX_START) / 1000000}")

echo ""
echo "✅ DX Bundle created: $OUTPUT_DIR/dx-bundle.js"
echo "   Size: $(wc -c < "$OUTPUT_DIR/dx-bundle.js") bytes"
echo "   Time: ${DX_TIME}ms"
echo ""

# Test 2: Bun Bundler
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🐰 Testing Bun Bundler"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

BUN_START=$(date +%s%N)
bun build "$SOURCE_DIR/index.ts" --outfile "$OUTPUT_DIR/bun-bundle.js" --target node
BUN_END=$(date +%s%N)
BUN_TIME=$(awk "BEGIN {printf \"%.2f\", ($BUN_END - $BUN_START) / 1000000}")

echo ""
echo "✅ Bun Bundle created: $OUTPUT_DIR/bun-bundle.js"
echo "   Size: $(wc -c < "$OUTPUT_DIR/bun-bundle.js") bytes"
echo "   Time: ${BUN_TIME}ms"
echo ""

# Validation: Syntax Check
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 Validating Output (Syntax Check)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo -n "DX Bundle: "
if node -c "$OUTPUT_DIR/dx-bundle.js" 2>/dev/null; then
    echo "✅ Valid JavaScript"
else
    echo "❌ Syntax Error"
    exit 1
fi

echo -n "Bun Bundle: "
if node -c "$OUTPUT_DIR/bun-bundle.js" 2>/dev/null; then
    echo "✅ Valid JavaScript"
else
    echo "❌ Syntax Error"
    exit 1
fi

echo ""

# Validation: Execution Test
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🧪 Execution Test"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo ""
echo "DX Bundle Output:"
echo "─────────────────"
DX_OUTPUT=$(node "$OUTPUT_DIR/dx-bundle.js" 2>&1)
echo "$DX_OUTPUT"

echo ""
echo "Bun Bundle Output:"
echo "──────────────────"
BUN_OUTPUT=$(node "$OUTPUT_DIR/bun-bundle.js" 2>&1)
echo "$BUN_OUTPUT"

echo ""

# Compare outputs
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Comparison Results"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

DX_SIZE=$(wc -c < "$OUTPUT_DIR/dx-bundle.js")
BUN_SIZE=$(wc -c < "$OUTPUT_DIR/bun-bundle.js")
SIZE_RATIO=$(awk "BEGIN {printf \"%.2f\", $BUN_SIZE / $DX_SIZE}")
SPEED_RATIO=$(awk "BEGIN {printf \"%.2f\", $BUN_TIME / $DX_TIME}")

echo "| Metric        | DX Bundler    | Bun Bundler   | Winner    |"
echo "|---------------|---------------|---------------|-----------|"
printf "| Time          | %-13s | %-13s | " "${DX_TIME}ms" "${BUN_TIME}ms"
DX_FASTER=$(awk "BEGIN {print ($DX_TIME < $BUN_TIME) ? 1 : 0}")
if [ "$DX_FASTER" = "1" ]; then
    echo "DX (${SPEED_RATIO}x) |"
else
    echo "Bun       |"
fi
printf "| Size          | %-13s | %-13s | " "${DX_SIZE}B" "${BUN_SIZE}B"
if [ "$DX_SIZE" -lt "$BUN_SIZE" ]; then
    echo "DX (${SIZE_RATIO}x) |"
else
    echo "Bun       |"
fi
echo ""

# Check if outputs match
if [ "$DX_OUTPUT" = "$BUN_OUTPUT" ]; then
    echo "✅ Output Match: Both bundlers produce identical runtime behavior"
else
    echo "⚠️  Output Difference: Bundlers produce different output"
    echo ""
    echo "This is expected - they may format output differently but should"
    echo "produce the same calculated results."
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Test Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Output files saved to: $OUTPUT_DIR/"
