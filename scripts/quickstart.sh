#!/usr/bin/env bash
# Quick start script for dx-www runtime

set -e

echo "================================="
echo "dx-www Runtime - Quick Start"
echo "================================="
echo ""

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Install from: https://rustup.rs"
    exit 1
fi

if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "→ Installing wasm32 target..."
    rustup target add wasm32-unknown-unknown
fi

if ! command -v wasm-bindgen &> /dev/null; then
    echo "→ Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

echo "✓ All prerequisites installed"
echo ""

# Build workspace
echo "Building workspace..."
cargo build --workspace --release

echo ""
echo "✓ Workspace built successfully!"
echo ""

# Build hello-world example
echo "Building hello-world example..."
cd examples/hello-world
chmod +x build.sh
./build.sh

echo ""
echo "================================="
echo "✓ Setup Complete!"
echo "================================="
echo ""
echo "To run the example:"
echo "  cd examples/hello-world"
echo "  python -m http.server 8000"
echo "  Open http://localhost:8000"
echo ""
