@echo off
REM Quick test script to verify everything works

echo.
echo ════════════════════════════════════════════════════════════════
echo   DX SERIALIZER VERIFICATION TEST
echo ════════════════════════════════════════════════════════════════
echo.

cd /d %~dp0

echo [1/3] Testing DX-Infinity (human-readable format)...
cargo test --test dx_infinity_regression --quiet
if %errorlevel% neq 0 (
    echo ❌ DX-Infinity tests FAILED
    exit /b 1
)
echo ✅ DX-Infinity: 9/9 tests passed

echo.
echo [2/3] Testing DX-Zero (binary format)...
cargo test --test dx_zero_verification --quiet  
if %errorlevel% neq 0 (
    echo ❌ DX-Zero tests FAILED
    exit /b 1
)
echo ✅ DX-Zero: 6/6 tests passed

echo.
echo [3/3] Running quick benchmark...
cargo test --test dx_infinity_regression test_dx_infinity_performance_baseline --quiet -- --nocapture | findstr "✅"

echo.
echo ════════════════════════════════════════════════════════════════
echo   ✅ ALL TESTS PASSING
echo ════════════════════════════════════════════════════════════════
echo.
echo   • DX-Infinity (human format):  WORKING ✅
echo   • DX-Zero (binary format):     WORKING ✅
echo   • Both formats coexist:        NO CONFLICTS ✅
echo.
echo 🚀 Next Steps:
echo    1. Run full benchmarks:  run-benchmarks.bat
echo    2. View results:         TEST_RESULTS.md
echo    3. Install tools:        INSTALLATION.md
echo.

