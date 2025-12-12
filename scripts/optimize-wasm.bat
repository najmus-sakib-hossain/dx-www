@echo off
REM optimize-wasm.bat - Optimize the dx-client runtime WASM binary for Windows
REM This reduces the bundle size from ~23 KB to ~19 KB

setlocal enabledelayedexpansion

echo 🔧 dx-www WASM Optimization Script
echo.

REM Check if wasm-opt is available
where wasm-opt >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ⚠️  wasm-opt not found. Installing via npm...
    call npm install -g binaryen
    
    REM Check again using npx
    npx wasm-opt --version >nul 2>nul
    if !ERRORLEVEL! NEQ 0 (
        echo ❌ Failed to install wasm-opt
        exit /b 1
    )
    echo ✅ Installed binaryen
)

REM Build the WASM binary first
echo 📦 Building dx-client WASM...
cargo build --release -p dx-client --target wasm32-unknown-unknown
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Build failed
    exit /b 1
)

REM Run wasm-bindgen to generate JS bindings
echo 🔗 Generating JS bindings...
wasm-bindgen target\wasm32-unknown-unknown\release\dx_client.wasm --out-dir target\pkg_optimized --target web --no-typescript
if %ERRORLEVEL% NEQ 0 (
    echo ❌ wasm-bindgen failed
    exit /b 1
)

set INPUT_FILE=target\pkg_optimized\dx_client_bg.wasm
set OUTPUT_FILE=target\pkg_optimized\dx_client_optimized.wasm

REM Check if input exists
if not exist "%INPUT_FILE%" (
    echo ❌ Input file not found: %INPUT_FILE%
    exit /b 1
)

REM Get original size
for %%A in ("%INPUT_FILE%") do set ORIGINAL_SIZE=%%~zA
set /a ORIGINAL_KB=ORIGINAL_SIZE / 1024

echo 📊 Original size: %ORIGINAL_KB% KB
echo 🚀 Optimizing with wasm-opt...

REM Run wasm-opt with MAXIMUM optimization (enhanced for sub-15KB target)
npx wasm-opt -Oz --enable-bulk-memory --enable-sign-ext --enable-mutable-globals --dce --duplicate-function-elimination --coalesce-locals --code-folding "%INPUT_FILE%" -o "%OUTPUT_FILE%"
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Optimization failed
    exit /b 1
)

REM Get optimized size
for %%A in ("%OUTPUT_FILE%") do set OPTIMIZED_SIZE=%%~zA
set /a OPTIMIZED_KB=OPTIMIZED_SIZE / 1024

REM Calculate reduction
set /a REDUCTION=100 - (OPTIMIZED_SIZE * 100 / ORIGINAL_SIZE)

echo ✅ Optimized size: %OPTIMIZED_KB% KB
echo 📉 Reduction: %REDUCTION%%%
echo.
echo ✨ Output: %OUTPUT_FILE%

REM Copy to final location
copy /Y "%OUTPUT_FILE%" "target\pkg_optimized\dx_client_bg.wasm" >nul
echo 📁 Updated: target\pkg_optimized\dx_client_bg.wasm

echo.
echo 🎯 Ready for production!
echo.
echo To use this build:
echo   ^<script type="module"^>
echo     import init from './target/pkg_optimized/dx_client.js';
echo     await init();
echo   ^</script^>

endlocal
