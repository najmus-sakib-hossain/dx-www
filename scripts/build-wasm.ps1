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
#   .\scripts\build-wasm.ps1 [-Release]

param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir
$SerializerDir = Join-Path $RootDir "crates\serializer"
$ExtensionDir = Join-Path $RootDir "crates\vscode-dx-serializer"
$WasmOutDir = Join-Path $ExtensionDir "wasm"

# Determine build mode
if ($Release) {
    $BuildMode = "--release"
    Write-Host "Building in release mode..." -ForegroundColor Green
} else {
    $BuildMode = "--dev"
    Write-Host "Building in dev mode (use -Release for optimized build)..." -ForegroundColor Yellow
}

# Check for wasm-pack
$wasmPack = Get-Command wasm-pack -ErrorAction SilentlyContinue
if (-not $wasmPack) {
    Write-Host "Error: wasm-pack is not installed." -ForegroundColor Red
    Write-Host "Install it with: cargo install wasm-pack"
    exit 1
}

# Check for wasm32 target
$targets = rustup target list --installed
if ($targets -notcontains "wasm32-unknown-unknown") {
    Write-Host "Adding wasm32-unknown-unknown target..." -ForegroundColor Yellow
    rustup target add wasm32-unknown-unknown
}

# Create output directory
if (-not (Test-Path $WasmOutDir)) {
    New-Item -ItemType Directory -Path $WasmOutDir -Force | Out-Null
}

# Build WASM
Write-Host "Building WASM module..." -ForegroundColor Cyan
Push-Location $SerializerDir
try {
    wasm-pack build `
        --target web `
        $BuildMode `
        --out-dir $WasmOutDir `
        --out-name dx_serializer `
        -- --features wasm

    if ($LASTEXITCODE -ne 0) {
        throw "wasm-pack build failed"
    }
} finally {
    Pop-Location
}

# Clean up unnecessary files
Write-Host "Cleaning up..." -ForegroundColor Cyan
$filesToRemove = @(".gitignore", "package.json", "README.md")
foreach ($file in $filesToRemove) {
    $filePath = Join-Path $WasmOutDir $file
    if (Test-Path $filePath) {
        Remove-Item $filePath -Force
    }
}

# Show output
Write-Host ""
Write-Host "WASM build complete!" -ForegroundColor Green
Write-Host "Output directory: $WasmOutDir"
Get-ChildItem $WasmOutDir | Format-Table Name, Length

# Show file sizes for release builds
if ($Release) {
    Write-Host ""
    Write-Host "WASM file sizes:" -ForegroundColor Cyan
    Get-ChildItem $WasmOutDir -Filter "*.wasm" | ForEach-Object {
        $sizeKB = [math]::Round($_.Length / 1024, 2)
        Write-Host "  $($_.Name): $sizeKB KB"
    }
}
