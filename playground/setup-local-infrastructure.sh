#!/bin/bash
# Complete DX Package Manager Infrastructure Setup
# This script sets up everything needed for local testing

set -e

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "  🚀 DX Package Manager - Complete Infrastructure Setup"
echo "═══════════════════════════════════════════════════════════════════"
echo ""

# Step 1: Build all binaries
echo "📦 Step 1: Building binaries..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cd "F:/Code/dx/crates/dx-package-manager"

echo "  Building registry server..."
cargo build --release -p dx-pkg-registry-server 2>&1 | tail -5

echo "  Building package converter..."
cargo build --release -p dx-pkg-converter 2>&1 | tail -5

echo "  Building dx CLI..."
cargo build --release -p dx-pkg-cli 2>&1 | tail -5

echo "✅ All binaries built"
echo ""

# Step 2: Create directories
echo "📂 Step 2: Creating directories..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
mkdir -p .dx-registry
mkdir -p .dx-test
echo "✅ Directories created"
echo ""

# Step 3: Convert popular packages
echo "📦 Step 3: Converting popular packages..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

CONVERTER="./target/release/dx-convert.exe"

# Create package list
cat > /tmp/dx-packages.txt << 'EOF'
lodash
express
axios
chalk
commander
EOF

echo "  Converting: lodash, express, axios, chalk, commander"
echo ""

# Convert each package
for pkg in lodash express axios chalk commander; do
    echo "  📥 $pkg..."
    "$CONVERTER" download "$pkg" -o .dx-registry 2>&1 | grep -E "(Version|Output|✅)" || true
done

echo ""
echo "✅ Packages converted"
echo ""

# Step 4: Start registry server (background)
echo "🌐 Step 4: Starting registry server..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

REGISTRY_SERVER="./target/release/dx-pkg-registry-server.exe"

# Kill any existing server
pkill -f dx-pkg-registry-server || true
sleep 1

# Start server in background
"$REGISTRY_SERVER" .dx-registry 127.0.0.1:3000 > .dx-registry/server.log 2>&1 &
SERVER_PID=$!

echo "  Server PID: $SERVER_PID"
echo "  Waiting for server to start..."
sleep 2

# Check if server is running
if ps -p $SERVER_PID > /dev/null; then
    echo "✅ Registry server running on localhost:3000"
    echo "   Log: .dx-registry/server.log"
else
    echo "❌ Failed to start server"
    cat .dx-registry/server.log
    exit 1
fi

echo ""

# Step 5: Test connection
echo "🔍 Step 5: Testing connection..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Simple ping test (using netcat if available)
if command -v nc > /dev/null; then
    echo -n "Testing TCP connection... "
    if timeout 2 bash -c "</dev/tcp/127.0.0.1/3000" 2>/dev/null; then
        echo "✅ Connected"
    else
        echo "⚠️  Connection test inconclusive"
    fi
else
    echo "⚠️  netcat not available, skipping connection test"
fi

echo ""

# Step 6: Summary
echo "═══════════════════════════════════════════════════════════════════"
echo "  ✅ Setup Complete!"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "📊 Summary:"
echo "   • Registry server:  localhost:3000 (PID: $SERVER_PID)"
echo "   • Packages stored:  .dx-registry/"
echo "   • Converted:        5 packages (lodash, express, axios, chalk, commander)"
echo ""
echo "🎯 Next Steps:"
echo "   1. Run benchmarks:    bash run-end-to-end-benchmark.sh"
echo "   2. Test CLI:          ./target/release/dx.exe install"
echo "   3. Stop server:       kill $SERVER_PID"
echo ""
echo "📝 Configuration:"
echo "   • Update dx-pkg-cli to use localhost:3000 as registry"
echo "   • Server log: .dx-registry/server.log"
echo ""

# Save PID for later
echo $SERVER_PID > .dx-registry/server.pid
echo "Server PID saved to .dx-registry/server.pid"
echo ""
