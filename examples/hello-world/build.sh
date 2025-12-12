#!/usr/bin/env bash
# Build script for hello-world example

set -e

# Change to workspace root
cd "$(dirname "$0")/../.." || exit 1

echo "🔨 Building dx-www hello-world example..."

# Build the WASM binary
echo "→ Compiling to wasm32..."
cargo build --target wasm32-unknown-unknown --release

# Generate JS bindings
echo "→ Generating JS bindings..."
wasm-bindgen --target web \
  --out-dir examples/hello-world/pkg \
  target/wasm32-unknown-unknown/release/hello_world.wasm

# Show size
echo ""
echo "📦 Binary size:"
ls -lh examples/hello-world/pkg/hello_world_bg.wasm | awk '{print $5, $9}'

echo ""
echo "✓ Build complete!"
echo ""
echo "To run:"
echo "  cd examples/hello-world"
echo "  python -m http.server 8000"
echo "  Open http://localhost:8000"
