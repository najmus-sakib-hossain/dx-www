#!/bin/bash
# Benchmark DX Bundler v2 vs Bun vs DX JS Bundler

echo "=== DX Bundler v2 Performance Test ==="
echo ""

# Test file
TEST_FILE="test-app.js"
OUTPUT_BUN="dist/bundle-bun.js"
OUTPUT_DX="dist/bundle-dx.js"
OUTPUT_V2="dist/bundle-v2.js"

# Create test app if it doesn't exist
if [ ! -f "$TEST_FILE" ]; then
  cat > "$TEST_FILE" << 'EOF'
import React from 'react';
import { useState, useEffect } from 'react';

function Counter() {
  const [count, setCount] = useState(0);
  
  useEffect(() => {
    document.title = `Count: ${count}`;
  }, [count]);
  
  return (
    <div>
      <h1>Counter: {count}</h1>
      <button onClick={() => setCount(count + 1)}>Increment</button>
      <button onClick={() => setCount(count - 1)}>Decrement</button>
    </div>
  );
}

export default Counter;
EOF
fi

# Clean dist
rm -rf dist
mkdir -p dist

echo "Running 5 iterations of each bundler..."
echo ""

# === Bun Benchmark ===
echo "Testing Bun..."
BUN_TOTAL=0
for i in {1..5}; do
  START=$(date +%s%N)
  bun build "$TEST_FILE" --outfile="$OUTPUT_BUN" --target=browser --minify 2>/dev/null
  END=$(date +%s%N)
  DURATION=$(( (END - START) / 1000000 ))
  BUN_TOTAL=$((BUN_TOTAL + DURATION))
  echo "  Run $i: ${DURATION}ms"
done
BUN_AVG=$((BUN_TOTAL / 5))
BUN_SIZE=$(stat -f%z "$OUTPUT_BUN" 2>/dev/null || stat -c%s "$OUTPUT_BUN" 2>/dev/null || echo "0")

# === DX JS Bundler Benchmark ===
echo ""
echo "Testing DX JS Bundler (existing)..."
DX_TOTAL=0
for i in {1..5}; do
  START=$(date +%s%N)
  node ../crates/dx-js-bundler/bundler.js "$TEST_FILE" "$OUTPUT_DX" 2>/dev/null
  END=$(date +%s%N)
  DURATION=$(( (END - START) / 1000000 ))
  DX_TOTAL=$((DX_TOTAL + DURATION))
  echo "  Run $i: ${DURATION}ms"
done
DX_AVG=$((DX_TOTAL / 5))
DX_SIZE=$(stat -f%z "$OUTPUT_DX" 2>/dev/null || stat -c%s "$OUTPUT_DX" 2>/dev/null || echo "0")

# === DX Bundler v2 Benchmark ===
echo ""
echo "Testing DX Bundler v2 (new Rust implementation)..."
V2_TOTAL=0
DX_V2_BIN="../crates/dx-bundler-v2/target/release/dx-bundle.exe"

if [ ! -f "$DX_V2_BIN" ]; then
  echo "ERROR: dx-bundle.exe not found! Build it first with:"
  echo "  cd crates/dx-bundler-v2 && cargo build --release"
  exit 1
fi

for i in {1..5}; do
  START=$(date +%s%N)
  "$DX_V2_BIN" "$TEST_FILE" --output "$OUTPUT_V2" --target browser --minify 2>/dev/null
  END=$(date +%s%N)
  DURATION=$(( (END - START) / 1000000 ))
  V2_TOTAL=$((V2_TOTAL + DURATION))
  echo "  Run $i: ${DURATION}ms"
done
V2_AVG=$((V2_TOTAL / 5))
V2_SIZE=$(stat -f%z "$OUTPUT_V2" 2>/dev/null || stat -c%s "$OUTPUT_V2" 2>/dev/null || echo "0")

# === Results ===
echo ""
echo "======================================"
echo "            RESULTS"
echo "======================================"
echo ""
printf "%-20s %10s %10s\n" "Bundler" "Time (avg)" "Size"
printf "%-20s %10s %10s\n" "--------------------" "----------" "----------"
printf "%-20s %9dms %9db\n" "Bun" "$BUN_AVG" "$BUN_SIZE"
printf "%-20s %9dms %9db\n" "DX JS Bundler" "$DX_AVG" "$DX_SIZE"
printf "%-20s %9dms %9db\n" "DX Bundler v2" "$V2_AVG" "$V2_SIZE"
echo ""

# Calculate speedup
if [ "$BUN_AVG" -gt 0 ]; then
  SPEEDUP=$(awk "BEGIN {printf \"%.2f\", $BUN_AVG / $V2_AVG}")
  echo "DX Bundler v2 is ${SPEEDUP}x faster than Bun"
fi

if [ "$DX_AVG" -gt 0 ]; then
  IMPROVEMENT=$(awk "BEGIN {printf \"%.2f\", $DX_AVG / $V2_AVG}")
  echo "DX Bundler v2 is ${IMPROVEMENT}x faster than DX JS Bundler"
fi

echo ""
echo "======================================"
