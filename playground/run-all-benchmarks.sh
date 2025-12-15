#!/bin/bash
# Run all benchmarks and compare dx-js vs Bun

echo "╔════════════════════════════════════════════════════╗"
echo "║       COMPREHENSIVE BENCHMARK SUITE               ║"
echo "╚════════════════════════════════════════════════════╝"
echo

tests=("simple_test.js" "bench-math-heavy.js" "bench-variables.js" "bench-comparisons.js")

for test in "${tests[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Test: $test"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    # DX-JS (3 runs)
    dx_total=0
    echo "  dx-js:"
    for i in {1..3}; do
        result=$( { time /f/Code/dx/target/release/dx-js.exe "F:/Code/dx/playground/$test" >/dev/null; } 2>&1 | grep real | awk '{print $2}' | sed 's/0m0.0*//;s/s//' )
        echo "    Run $i: ${result}ms"
        dx_total=$((dx_total + result))
    done
    dx_avg=$((dx_total / 3))
    
    # Bun (3 runs)
    bun_total=0
    echo "  bun:"
    for i in {1..3}; do
        result=$( { time bun run "$test" >/dev/null; } 2>&1 | grep real | awk '{print $2}' | sed 's/0m0.0*//;s/s//' )
        echo "    Run $i: ${result}ms"
        bun_total=$((bun_total + result))
    done
    bun_avg=$((bun_total / 3))
    
    # Calculate speedup
    if [ $dx_avg -gt 0 ]; then
        speedup=$(echo "scale=2; $bun_avg / $dx_avg" | bc)
        echo "  ► dx-js avg: ${dx_avg}ms | bun avg: ${bun_avg}ms | speedup: ${speedup}x"
    fi
    echo
done

echo "╚════════════════════════════════════════════════════╝"
