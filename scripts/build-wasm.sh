#!/bin/bash
# Build WASM for DX Serializer VS Code Extension
#
# This script builds the serializer crate as a WASM module using wasm-pack
# and copies the output to the VS Code extension directory.
#
# Prerequisites:
#   - Rust toolchain with wasm32-unknown-unknown target
#   - wasm-pack (install with: cargo install wasm-pack)
#
# Usage:
#   ./scripts/build-wasm.sh [--release]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SERIALIZER_DIR="$ROOT_DIR/crates/serializer"
EXTENSION_DIR="$ROOT_DIR/crates/vscode-dx-serializer"
WASM_OUT_DIR="$EXTENSION_DIR/wasm"

# Parse arguments
BUILD_MODE="--dev"
if [ "$1" = "--release" ]; then
    BUILD_MODE="--release"
    echo "Building in release mode..."
else
    echo "Building in dev mode (use --release for optimized build)..."
fi

# Check for wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Install it with: cargo install wasm-pack"
    exit 1
fi

# Check for wasm32 target
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Create output directory
mkdir -p "$WASM_OUT_DIR"

# Build WASM
echo "Building WASM module..."
cd "$SERIALIZER_DIR"
wasm-pack build \
    --target web \
    $BUILD_MODE \
    --out-dir "$WASM_OUT_DIR" \
    --out-name dx_serializer \
    -- --features wasm

# Clean up unnecessary files
echo "Cleaning up..."
rm -f "$WASM_OUT_DIR/.gitignore"
rm -f "$WASM_OUT_DIR/package.json"
rm -f "$WASM_OUT_DIR/README.md"

# Show output
echo ""
echo "WASM build complete!"
echo "Output directory: $WASM_OUT_DIR"
ls -la "$WASM_OUT_DIR"

# Show file sizes
echo ""
echo "File sizes:"
if [ "$BUILD_MODE" = "--release" ]; then
    wc -c "$WASM_OUT_DIR"/*.wasm
fi
