#!/bin/bash
# demo.sh - Complete dx-www demonstration script
# Shows the full pipeline from TSX to optimized production build

set -e

echo "╔════════════════════════════════════════════════════╗"
echo "║      dx-www Complete Pipeline Demonstration      ║"
echo "╚════════════════════════════════════════════════════╝"
echo ""

# Step 1: Create a sample app
echo "📝 Step 1: Creating sample App.tsx..."
mkdir -p demo-app/src
cat > demo-app/src/App.tsx << 'EOF'
function Counter() {
    const [count, setCount] = useState(0);
    
    return (
        <div class="app">
            <h1>dx-www Counter Demo</h1>
            <div class="counter">
                <p>Count: <strong>{count}</strong></p>
                <button onClick={() => setCount(count + 1)}>
                    Increment
                </button>
                <button onClick={() => setCount(count - 1)}>
                    Decrement
                </button>
            </div>
        </div>
    );
}
EOF

echo "✅ Created demo-app/src/App.tsx"
echo ""

# Step 2: Build the compiler if needed
echo "🔨 Step 2: Building dx-compiler..."
if [ ! -f "target/release/dx" ] && [ ! -f "target/release/dx.exe" ]; then
    cargo build --release -p dx-compiler
else
    echo "✅ Compiler already built"
fi
echo ""

# Step 3: Compile the app
echo "🏭 Step 3: Compiling App.tsx → app.dxb..."
if [ -f "target/release/dx.exe" ]; then
    ./target/release/dx.exe build --entry demo-app/src/App.tsx --output demo-app/dist/
else
    ./target/release/dx build --entry demo-app/src/App.tsx --output demo-app/dist/
fi

APP_SIZE=$(stat -f%z demo-app/dist/app.dxb 2>/dev/null || stat -c%s demo-app/dist/app.dxb)
echo "✅ Generated app.dxb: ${APP_SIZE} bytes"
echo ""

# Step 4: Optimize WASM runtime
echo "🚀 Step 4: Optimizing WASM runtime..."
if [ ! -f "target/pkg_optimized/dx_client_optimized.wasm" ]; then
    echo "Running wasm-opt..."
    cargo build --release -p dx-client --target wasm32-unknown-unknown
    
    wasm-bindgen \
        target/wasm32-unknown-unknown/release/dx_client.wasm \
        --out-dir target/pkg_optimized \
        --target web \
        --no-typescript
    
    npx wasm-opt \
        -Oz \
        --enable-bulk-memory \
        target/pkg_optimized/dx_client_bg.wasm \
        -o target/pkg_optimized/dx_client_optimized.wasm
    
    cp target/pkg_optimized/dx_client_optimized.wasm target/pkg_optimized/dx_client_bg.wasm
fi

WASM_SIZE=$(stat -f%z target/pkg_optimized/dx_client_bg.wasm 2>/dev/null || stat -c%s target/pkg_optimized/dx_client_bg.wasm)
WASM_KB=$((WASM_SIZE / 1024))
echo "✅ Optimized runtime: ${WASM_KB} KB"
echo ""

# Step 5: Calculate total bundle
TOTAL_SIZE=$((WASM_SIZE + APP_SIZE))
TOTAL_KB=$((TOTAL_SIZE / 1024))

echo "╔════════════════════════════════════════════════════╗"
echo "║                Bundle Size Report                 ║"
echo "╠════════════════════════════════════════════════════╣"
printf "║ Runtime (dx_client.wasm) │ %18d KB ║\n" "$WASM_KB"
printf "║ App Data (app.dxb)       │ %17d bytes ║\n" "$APP_SIZE"
echo "╠════════════════════════════════════════════════════╣"
printf "║ TOTAL FIRST LOAD         │ %18.2f KB ║\n" "$(echo "scale=2; $TOTAL_SIZE / 1024" | bc)"
echo "╚════════════════════════════════════════════════════╝"
echo ""

# Step 6: Compare with competition
echo "📊 Comparison with React:"
echo "  React + Counter: ~195 KB (140 KB runtime + 55 KB code)"
echo "  dx-www + Counter: ${TOTAL_KB} KB (${WASM_KB} KB runtime + ${APP_SIZE} bytes data)"
SAVINGS=$((195 - TOTAL_KB))
PERCENT=$(echo "scale=1; $SAVINGS * 100 / 195" | bc)
echo "  Savings: ${SAVINGS} KB (${PERCENT}% smaller!)"
echo ""

# Step 7: Create demo HTML
echo "📄 Step 5: Creating demo.html..."
cat > demo-app/dist/index.html << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>dx-www Counter Demo</title>
    <style>
        body {
            font-family: system-ui, -apple-system, sans-serif;
            max-width: 600px;
            margin: 50px auto;
            padding: 20px;
        }
        .app { text-align: center; }
        .counter {
            margin: 30px 0;
            padding: 30px;
            background: #f0f0f0;
            border-radius: 8px;
        }
        button {
            margin: 10px;
            padding: 10px 20px;
            font-size: 16px;
            cursor: pointer;
        }
        strong { color: #0066cc; font-size: 24px; }
    </style>
</head>
<body>
    <div id="root"></div>
    
    <script type="module">
        // Load dx-www runtime
        import init from '../../../target/pkg_optimized/dx_client.js';
        
        // Initialize
        await init();
        
        // Load app binary
        const response = await fetch('./app.dxb');
        const buffer = await response.arrayBuffer();
        const bytes = new Uint8Array(buffer);
        
        // Boot app
        window.dx_runtime.boot(bytes);
    </script>
    
    <div style="position: fixed; bottom: 20px; right: 20px; background: white; padding: 10px; border: 1px solid #ccc; border-radius: 5px; font-size: 12px;">
        Bundle: ${TOTAL_KB} KB<br>
        Runtime: ${WASM_KB} KB (cached)<br>
        App: ${APP_SIZE} bytes
    </div>
</body>
</html>
EOF

echo "✅ Created demo-app/dist/index.html"
echo ""

echo "╔════════════════════════════════════════════════════╗"
echo "║                  SUCCESS! 🎉                      ║"
echo "╚════════════════════════════════════════════════════╝"
echo ""
echo "Your dx-www app is ready!"
echo ""
echo "To view it:"
echo "  cd demo-app/dist"
echo "  python -m http.server 8000"
echo ""
echo "Then open: http://localhost:8000"
echo ""
echo "Files created:"
echo "  demo-app/src/App.tsx          (source code)"
echo "  demo-app/dist/app.dxb         (${APP_SIZE} bytes)"
echo "  demo-app/dist/index.html      (demo page)"
echo ""
echo "Next steps:"
echo "  1. Edit demo-app/src/App.tsx"
echo "  2. Run: dx build --entry demo-app/src/App.tsx --output demo-app/dist/"
echo "  3. Refresh browser to see changes"
echo ""
