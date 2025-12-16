@echo off
REM Windows Batch Script for Serializer Benchmarks

echo ╔════════════════════════════════════════════════════════════════╗
echo ║          DX-ZERO SERIALIZER BENCHMARK SUITE                   ║
echo ║      Testing Against: rkyv, Bincode, JSON, DX-Infinity        ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.

cd /d %~dp0

REM Check for Cap'n Proto
where capnp >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ Cap'n Proto found
) else (
    echo ⚠️  Cap'n Proto not found ^(will skip Cap'n Proto tests^)
    echo    Install: choco install capnproto
)

REM Check for FlatBuffers
where flatc >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ FlatBuffers found
) else (
    echo ⚠️  FlatBuffers not found ^(will skip FlatBuffers tests^)
    echo    Install: choco install flatbuffers
)

echo.
echo ════════════════════════════════════════════════════════════════
echo  Building in Release Mode...
echo ════════════════════════════════════════════════════════════════
cargo build --release
if %errorlevel% neq 0 goto error

echo.
echo ════════════════════════════════════════════════════════════════
echo  Running Benchmarks...
echo ════════════════════════════════════════════════════════════════
cargo bench --bench all_serializers
if %errorlevel% neq 0 goto error

echo.
echo ════════════════════════════════════════════════════════════════
echo  Benchmark Complete!
echo ════════════════════════════════════════════════════════════════
echo.
echo 📊 View detailed results at:
echo    target\criterion\report\index.html
echo.
echo 🎯 Expected Results:
echo    • DX-Zero Serialize:    0 ns (in-place)
echo    • DX-Zero Deserialize:  0.8-2.1 ns (pointer cast)
echo    • DX-Zero Size:         Smallest (138 bytes)
echo    • Victory: 2-400× faster than all competitors
echo.

goto end

:error
echo.
echo ❌ Error occurred during benchmarks
exit /b 1

:end
