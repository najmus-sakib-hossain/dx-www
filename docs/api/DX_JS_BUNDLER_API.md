# DX JS Bundler - CLI & API Reference

## Command Line Interface

### Basic Usage

```bash
dx-bundle <entry> [options]
```

### Options

#### Input/Output

```bash
--entry <file>          # Entry point (default: index.tsx)
--output <file>         # Output bundle (default: output/test-bundle.js)
--out-dir <directory>   # Output directory (default: ./output)
```

#### Build Modes

```bash
--release              # Production build (minified)
--debug                # Debug build (verbose logging)
--watch                # Watch mode (rebuild on changes)
```

#### Transformation

```bash
--target <version>     # JS version (es5, es6, es2020)
--format <type>        # Module format (commonjs, esm)
--jsx <mode>           # JSX factory (react, preact, custom)
```

#### Optimization

```bash
--tree-shake           # Remove unused code (default: true)
--minify               # Minify output (default: false)
--compress             # Aggressive compression
```

#### Cache

```bash
--cache                # Enable caching (default: true)
--cache-dir <dir>      # Cache directory (default: .dx-cache)
--no-cache             # Disable caching
```

## API Examples

### Basic Bundling

```bash
# Bundle TypeScript entry point
dx-bundle index.ts --output dist/bundle.js

# Bundle React app
dx-bundle src/App.tsx --output public/app.js --jsx react

# Production build
dx-bundle index.tsx --release --minify
```

### Advanced Configuration

```bash
# Custom target & format
dx-bundle src/main.ts \
  --target es2020 \
  --format esm \
  --output dist/modern.js

# Debug build with verbose output
dx-bundle src/index.tsx --debug

# Fast rebuild with caching
dx-bundle src/App.tsx --cache --cache-dir .cache
```

### Watch Mode (Future)

```bash
# Watch for changes and rebuild
dx-bundle src/App.tsx --watch

# Watch with custom debounce
dx-bundle src/App.tsx --watch --debounce 500
```

## Configuration File

### `dx-bundle.toml`

```toml
[project]
entry = "src/index.tsx"
output = "dist/bundle.js"
target = "es2020"
format = "commonjs"

[optimization]
tree_shake = true
minify = true
compress = false

[cache]
enabled = true
directory = ".dx-cache"

[jsx]
factory = "React.createElement"
fragment = "React.Fragment"

[paths]
resolve_extensions = [".ts", ".tsx", ".js", ".jsx"]
module_directories = ["node_modules", "src"]
```

### Loading Config

```bash
# Auto-detect dx-bundle.toml in current directory
dx-bundle

# Use custom config
dx-bundle --config custom.toml

# Override config options
dx-bundle --config dx-bundle.toml --minify
```

## Programmatic API (Future)

### Rust API

```rust
use dx_bundle_core::BundleConfig;
use dx_bundle_cli::Bundle;

// Create config
let config = BundleConfig {
    entry: "src/index.tsx".into(),
    output: "dist/bundle.js".into(),
    minify: true,
    tree_shake: true,
    ..Default::default()
};

// Build bundle
let result = Bundle::build(config)?;

println!("Bundle size: {} bytes", result.size);
println!("Modules: {}", result.modules.len());
```

### Node.js API (via NAPI)

```javascript
const { bundle } = require('@dx/bundler');

// Bundle with options
const result = await bundle({
  entry: 'src/index.tsx',
  output: 'dist/bundle.js',
  minify: true,
  treeShake: true,
});

console.log(`Bundled ${result.modules} modules in ${result.time}ms`);
```

## Output Formats

### CommonJS (Default)

```javascript
(function(){
var __dx_modules={};
var __dx_cache={};
function __dx_require(id){...}
__dx_define(0,function(exports,require,module){
  // Your code here
});
__dx_require(0);
})();
```

**Use Cases:**
- Node.js compatibility
- Maximum browser support
- Predictable module loading

### ES Modules (Future)

```javascript
// Module 0: utils.js
export const add = (a, b) => a + b;

// Module 1: index.js
import { add } from './utils.js';
console.log(add(1, 2));
```

**Use Cases:**
- Modern browsers only
- Smaller output size
- Native module support

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Build error (syntax, type error) |
| `2` | Resolution error (missing module) |
| `3` | Configuration error |
| `4` | File system error |
| `5` | Cache error |

## Environment Variables

```bash
DX_CACHE_DIR=/tmp/.dx-cache   # Override cache directory
DX_LOG_LEVEL=debug             # Set log level (error, warn, info, debug)
DX_NO_COLOR=1                  # Disable colored output
DX_MAX_WORKERS=4               # Parallel processing limit
```

## Output Analysis

### Bundle Statistics

```bash
dx-bundle index.tsx --stats

# Output:
# Bundle: output/test-bundle.js
# Size: 1,240 bytes (1.21 KB)
# Modules: 3
# - index.tsx: 580 bytes
# - utils.ts: 320 bytes
# - Counter.tsx: 340 bytes
# Time: 24ms
```

### Dependency Graph

```bash
dx-bundle index.tsx --graph > deps.json

# Output: JSON dependency tree
# {
#   "entry": "index.tsx",
#   "modules": [
#     { "id": 0, "path": "index.tsx", "deps": [1, 2] },
#     { "id": 1, "path": "utils.ts", "deps": [] },
#     { "id": 2, "path": "Counter.tsx", "deps": [1] }
#   ]
# }
```

## Error Messages

### Syntax Errors

```
Error: Failed to parse module 'src/App.tsx'
  --> src/App.tsx:10:15
   |
10 |   return <div class="app">
   |               ^^^^^ Unexpected token 'class'
   |
   = help: Did you mean 'className'?
```

### Resolution Errors

```
Error: Cannot resolve module './utils'
  --> src/index.tsx:3:21
   |
 3 | import { add } from './utils';
   |                     ^^^^^^^^^ Module not found
   |
   = note: Searched:
     - ./utils.ts
     - ./utils.tsx
     - ./utils.js
     - ./utils/index.ts
```

### Type Errors (Future)

```
Error: Type mismatch in function call
  --> src/App.tsx:15:10
   |
15 |   add("1", "2");
   |       ^^^  ^^^ Expected number, found string
   |
   = help: Did you mean to parse the strings?
```

## Performance Tips

1. **Use Caching**: Enable `--cache` for 10-100x faster rebuilds
2. **Tree Shaking**: Always enable in production (`--tree-shake`)
3. **Target Modern Browsers**: Use `--target es2020` to skip polyfills
4. **Minimize Entry Points**: Fewer entries = faster builds
5. **Parallel Builds** (Future): `--max-workers 4` for large projects

## Integration Examples

### Vite Plugin (Future)

```javascript
// vite.config.js
import dxBundle from '@dx/vite-plugin-bundler';

export default {
  plugins: [
    dxBundle({
      minify: true,
      treeShake: true,
    }),
  ],
};
```

### Webpack Loader (Future)

```javascript
// webpack.config.js
module.exports = {
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: '@dx/webpack-bundler-loader',
      },
    ],
  },
};
```

### GitHub Actions

```yaml
name: Build
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install DX Bundler
        run: cargo install dx-bundle
      - name: Build Bundle
        run: dx-bundle src/index.tsx --release --minify
      - name: Upload Artifact
        uses: actions/upload-artifact@v3
        with:
          name: bundle
          path: output/test-bundle.js
```

## Troubleshooting

### Problem: "Module not found"

**Solution:** Check import paths and file extensions

```bash
# List resolved modules
dx-bundle index.tsx --debug

# Check current resolution
dx-bundle index.tsx --trace-resolution
```

### Problem: "Out of memory"

**Solution:** Reduce parallel workers or enable streaming

```bash
# Limit workers
DX_MAX_WORKERS=2 dx-bundle index.tsx

# Use streaming mode (future)
dx-bundle index.tsx --stream
```

### Problem: "Invalid syntax after transformation"

**Solution:** Report bug with reproducible case

```bash
# Generate debug output
dx-bundle index.tsx --debug --keep-temps

# This preserves intermediate files for inspection
```

---

**Documentation Version**: 1.0.0 (December 17, 2025)

For issues and support, visit: https://github.com/dx-framework/dx-js-bundler
