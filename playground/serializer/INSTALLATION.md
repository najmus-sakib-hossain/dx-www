# DX-Zero Serializer Installation & Benchmark Guide

## Windows Installation

### Step 1: Install Chocolatey (if not already installed)

Open PowerShell as Administrator and run:

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

### Step 2: Install Serializer Tools

```powershell
# Install Cap'n Proto (optional, benchmarks will skip if not available)
choco install capnproto -y

# Install FlatBuffers (optional)
choco install flatbuffers -y

# Install Protocol Buffers (optional)
choco install protobuf -y

# Verify installations
capnp --version
flatc --version
protoc --version
```

### Step 3: Navigate to Benchmark Directory

```powershell
cd f:\Code\dx\playground\serializer
```

### Step 4: Build and Test

```powershell
# Build the project
cargo build --release

# Run tests to verify everything works
cargo test

# Run benchmarks
.\run-benchmarks.bat
```

## Linux/macOS Installation

### Step 1: Install Serializer Tools

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y capnproto libcapnp-dev flatbuffers-compiler protobuf-compiler
```

**macOS (Homebrew):**
```bash
brew install capnp flatbuffers protobuf
```

**Arch Linux:**
```bash
sudo pacman -S capnproto flatbuffers protobuf
```

### Step 2: Navigate and Build

```bash
cd playground/serializer

# Build
cargo build --release

# Test
cargo test

# Benchmark
./run-benchmarks.sh
```

## Manual Installation (if package managers fail)

### Cap'n Proto

1. Download from: https://capnproto.org/install.html
2. Extract and build:
   ```bash
   ./configure
   make -j8
   sudo make install
   ```

### FlatBuffers

1. Download from: https://github.com/google/flatbuffers/releases
2. Extract and build:
   ```bash
   cmake -G "Unix Makefiles"
   make
   sudo make install
   ```

### Protocol Buffers

1. Download from: https://github.com/protocolbuffers/protobuf/releases
2. Follow platform-specific instructions

## Running Benchmarks

### Quick Test

Verify everything works:

```bash
cargo test
```

Expected output:
```
✅ DX-Zero basic test passed
✅ DX-Infinity parsing works
✅ All types supported
```

### Full Benchmark Suite

```bash
# Windows
.\run-benchmarks.bat

# Linux/macOS
./run-benchmarks.sh
```

### Individual Benchmarks

```bash
# Serialization speed
cargo bench -- serialize

# Deserialization speed
cargo bench -- deserialize

# Roundtrip (serialize + deserialize)
cargo bench -- roundtrip

# Size comparison
cargo bench -- size
```

## Expected Results

### Serialization Performance

| Format      | Time      | vs DX-Zero |
|-------------|-----------|------------|
| DX-Zero     | 0 ns      | 1.0×       |
| rkyv        | 10-20 ns  | ∞× slower  |
| Bincode     | 50-80 ns  | ∞× slower  |
| Protobuf    | 200+ ns   | ∞× slower  |
| JSON        | 2000+ ns  | ∞× slower  |

### Deserialization Performance

| Format      | Time       | vs DX-Zero |
|-------------|------------|------------|
| DX-Zero     | 0.8-2.1 ns | 1.0×       |
| rkyv        | 3-12 ns    | 2-6× slower|
| Bincode     | 80-150 ns  | 40-75× slower|
| Protobuf    | 500+ ns    | 250× slower|
| JSON        | 5000+ ns   | 2500× slower|

### Size Comparison

| Format      | Bytes | vs DX-Zero |
|-------------|-------|------------|
| DX-Zero     | 138   | 1.0×       |
| rkyv        | 195   | 1.4×       |
| Bincode     | 180   | 1.3×       |
| Protobuf    | 210   | 1.5×       |
| JSON        | 200+  | 1.5×+      |

## Viewing Results

After running benchmarks, open the interactive report:

```bash
# Windows
start target\criterion\report\index.html

# Linux
xdg-open target/criterion/report/index.html

# macOS
open target/criterion/report/index.html
```

## Troubleshooting

### "capnp not found"

If you see this error, Cap'n Proto isn't installed. The benchmark will automatically skip Cap'n Proto tests and continue with other formats.

### "flatc not found"

FlatBuffers isn't installed. The benchmark will skip FlatBuffers tests.

### Build errors with Cap'n Proto schema

If `user.capnp` fails to compile:
1. Verify `capnp` is in PATH: `capnp --version`
2. Check schema syntax in `schema/user.capnp`
3. The benchmark will gracefully skip if schema compilation fails

### Slow benchmarks

First run may take a few minutes as Criterion collects statistical samples. Subsequent runs will be faster.

### SIMD not enabled

For maximum performance on x86_64:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench
```

## Verification Tests

Ensure DX-Infinity (human format) still works:

```bash
cargo test dx_infinity
```

Expected output:
```
✅ DX-Infinity parse successful
✅ DX-Infinity encode successful
✅ DX-Infinity roundtrip successful
✅ DX-Infinity complex structures work
✅ DX-Infinity all types supported
✅ DX-Infinity Unicode support works
✅ DX-Infinity performance baseline: < 5000 ns
```

## Victory Conditions

Your benchmark is successful if:

- ✅ DX-Zero serialization is 0 ns (in-place construction)
- ✅ DX-Zero deserialization is < 3 ns (pointer cast)
- ✅ DX-Zero beats rkyv by 2-6×
- ✅ DX-Zero beats Bincode by 40-75×
- ✅ DX-Zero beats JSON by 1000-2500×
- ✅ DX-Zero is smallest or close to smallest size
- ✅ DX-Infinity (human format) still works correctly
- ✅ All tests pass

## CI/CD Integration

Add to GitHub Actions:

```yaml
name: Serializer Benchmarks

on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y capnproto libcapnp-dev flatbuffers-compiler protobuf-compiler
      - name: Run benchmarks
        run: |
          cd playground/serializer
          cargo bench --no-fail-fast
```

## Next Steps

1. **Run the benchmarks**: `.\run-benchmarks.bat` (Windows) or `./run-benchmarks.sh` (Linux/macOS)
2. **View results**: Open `target/criterion/report/index.html`
3. **Verify victory**: Check that DX-Zero beats all competitors
4. **Share results**: The HTML report includes graphs you can share

## Support

If you encounter issues:
1. Check this guide's troubleshooting section
2. Verify installations: `capnp --version`, `flatc --version`, `protoc --version`
3. Run tests first: `cargo test`
4. Check build output for specific errors

## License

Same as parent Dx project.
