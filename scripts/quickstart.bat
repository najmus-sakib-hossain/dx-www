@echo off
REM Quick start script for dx-www runtime (Windows)

echo =================================
echo dx-www Runtime - Quick Start
echo =================================
echo.

REM Check prerequisites
echo Checking prerequisites...

where rustc >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo X Rust not found. Install from: https://rustup.rs
    exit /b 1
)

rustup target list | findstr "wasm32-unknown-unknown (installed)" >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Installing wasm32 target...
    rustup target add wasm32-unknown-unknown
)

where wasm-bindgen >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Installing wasm-bindgen-cli...
    cargo install wasm-bindgen-cli
)

echo All prerequisites installed
echo.

REM Build workspace
echo Building workspace...
cargo build --workspace --release

if %ERRORLEVEL% neq 0 (
    echo Build failed!
    exit /b %ERRORLEVEL%
)

echo.
echo Workspace built successfully!
echo.

REM Build hello-world example
echo Building hello-world example...
cd examples\hello-world
call build.bat

echo.
echo =================================
echo Setup Complete!
echo =================================
echo.
echo To run the example:
echo   cd examples\hello-world
echo   python -m http.server 8000
echo   Open http://localhost:8000
echo.
