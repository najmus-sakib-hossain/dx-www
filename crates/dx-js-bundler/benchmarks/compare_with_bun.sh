#!/bin/bash
# Benchmark DX bundler vs Bun bundler

set -e

echo "🔥 DX Bundler vs Bun - Performance Comparison"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Create test project
TEST_DIR="./benchmark-test"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/src"

# Generate test files
echo "📝 Generating test files..."
for i in {1..100}; do
  cat > "$TEST_DIR/src/module$i.js" << EOF
export function module$i() {
  return 'Module $i';
}
EOF
done

# Create entry point
cat > "$TEST_DIR/src/index.js" << 'EOF'
// Import all modules
EOF

for i in {1..100}; do
  echo "import { module$i } from './module$i.js';" >> "$TEST_DIR/src/index.js"
done

cat >> "$TEST_DIR/src/index.js" << 'EOF'

// Use all modules
console.log('All modules loaded!');
EOF

# Create Bun config
cat > "$TEST_DIR/bunfig.toml" << EOF
[build]
target = "browser"
EOF

cd "$TEST_DIR"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🟨 Bun Bundler"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Benchmark Bun (3 runs)
BUN_TIMES=()
for i in {1..3}; do
  echo "Run $i/3..."
  START=$(date +%s%N)
  bun build src/index.js --outdir=dist-bun > /dev/null 2>&1
  END=$(date +%s%N)
  TIME=$(( (END - START) / 1000000 ))
  BUN_TIMES+=($TIME)
  echo "  Time: ${TIME}ms"
done

# Calculate average
BUN_AVG=0
for time in "${BUN_TIMES[@]}"; do
  BUN_AVG=$((BUN_AVG + time))
done
BUN_AVG=$((BUN_AVG / 3))

BUN_SIZE=$(du -sk dist-bun/index.js | cut -f1)

echo ""
echo "Average: ${BUN_AVG}ms"
echo "Size: ${BUN_SIZE}KB"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⚡ DX Bundler"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Benchmark DX (3 runs)
DX_TIMES=()
for i in {1..3}; do
  echo "Run $i/3..."
  START=$(date +%s%N)
  ../../../target/release/dx-bundle bundle src/index.js --output dist-dx/bundle.js > /dev/null 2>&1 || true
  END=$(date +%s%N)
  TIME=$(( (END - START) / 1000000 ))
  DX_TIMES+=($TIME)
  echo "  Time: ${TIME}ms"
done

# Calculate average
DX_AVG=0
for time in "${DX_TIMES[@]}"; do
  DX_AVG=$((DX_AVG + time))
done
DX_AVG=$((DX_AVG / 3))

if [ -f "dist-dx/bundle.js" ]; then
  DX_SIZE=$(du -sk dist-dx/bundle.js | cut -f1)
else
  DX_SIZE=0
fi

echo ""
echo "Average: ${DX_AVG}ms"
echo "Size: ${DX_SIZE}KB"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Results"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "| Bundler | Time (avg) | Size |"
echo "|---------|------------|------|"
echo "| Bun     | ${BUN_AVG}ms | ${BUN_SIZE}KB |"
echo "| DX      | ${DX_AVG}ms | ${DX_SIZE}KB |"
echo ""

if [ $DX_AVG -gt 0 ]; then
  SPEEDUP=$(echo "scale=2; $BUN_AVG / $DX_AVG" | bc)
  echo "⚡ Speedup: ${SPEEDUP}x faster"
else
  echo "⚠️  DX bundler not built yet"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cd ..
