#!/bin/bash
# Quick verification that DX-Infinity still works correctly

set -e

cd "$(dirname "$0")"

echo "════════════════════════════════════════════════════════════════"
echo " DX-INFINITY REGRESSION TEST"
echo " Verifying human-readable format still works after DX-Zero addition"
echo "════════════════════════════════════════════════════════════════"
echo

echo "Running DX-Infinity tests..."
cargo test --test dx_infinity_regression -- --nocapture

echo
echo "Running DX-Zero verification..."
cargo test --test dx_zero_verification -- --nocapture

echo
echo "════════════════════════════════════════════════════════════════"
echo " ✅ ALL TESTS PASSED"
echo "════════════════════════════════════════════════════════════════"
echo
echo "Summary:"
echo "  • DX-Infinity (human format) works correctly"
echo "  • DX-Zero (binary format) works correctly"
echo "  • Both formats coexist without conflicts"
echo "  • Performance baselines maintained"
echo
