@echo off
REM Build script for hello-world example (Windows)

echo Building dx-www hello-world example...

REM Build the WASM binary
echo Compiling to wasm32...
cargo build --target wasm32-unknown-unknown --release

if %ERRORLEVEL% neq 0 (
    echo Build failed!
    exit /b %ERRORLEVEL%
)

REM Generate JS bindings
echo Generating JS bindings...
wasm-bindgen --target web --out-dir examples\hello-world\pkg target\wasm32-unknown-unknown\release\hello_world.wasm

if %ERRORLEVEL% neq 0 (
    echo wasm-bindgen failed!
    exit /b %ERRORLEVEL%
)

echo.
echo Build complete!
echo.
echo To run:
echo   cd examples\hello-world
echo   python -m http.server 8000
echo   Open http://localhost:8000
