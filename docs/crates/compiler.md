# Dx Compiler - Quick Reference

## Installation

```bash
cargo install --path crates/dx-compiler
```

## Commands

### Build Production Artifacts
```bash
dx build --entry src/main.dx --output dist/
```

### Start Development Server
```bash
dx dev --entry src/main.dx --port 3000
```

### Create New Project
```bash
dx new my-app --template counter
```

## File Structure

```
my-app/
├── src/
│   ├── main.dx          # Entry point
│   ├── components/
│   │   ├── Counter.dx
│   │   └── Button.dx
│   └── styles/
├── dist/                # Build output
│   ├── app.dxb         # Binary artifact
│   └── index.html
└── dx.config.json      # Optional config
```

## Supported Syntax (The Dx Subset)

### ✅ Allowed
- `useState`, `useEffect`
- `props`, `if/else`, `map`
- Primitives: `number`, `string`, `boolean`
- Arrow functions, destructuring

### ❌ Forbidden
- Classes
- Generics
- `any` type
- `eval`, `innerHTML`, `Function`

## Component Example

```typescript
function Counter() {
  const [count, setCount] = useState(0);

  const increment = () => {
    setCount(count + 1);
  };

  return (
    <div class="counter">
      <span>Count: {count}</span>
      <button onClick={increment}>+</button>
    </div>
  );
}
```

## Output Files

- **app.dxb**: Binary artifact (templates + WASM)
- **templates.json**: Debug templates (with `--verbose`)
- **app.wasm**: Standalone WASM (with `--verbose`)

## Performance

- Build: < 3s cold, < 200ms hot reload
- Runtime: Zero-parse, Zero-GC, Zero-hydration
- Binary size: ~50KB runtime + ~10KB per component

## More Info

See [COMPILER.md](COMPILER.md) for full architecture documentation.
