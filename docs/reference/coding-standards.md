# DX Coding Standards

The DX Coding Standard defines the syntax and conventions for writing DX applications.

## Core Principles

1. **Explicit over Implicit** - Code should clearly indicate what is reactive vs constant
2. **Compile-Time Safety** - Catch errors at build time, not runtime
3. **Binary-First** - Everything compiles to efficient binary representations
4. **Zero Boilerplate** - Common patterns should be concise

## Variable Declaration

DX uses three distinct keywords that map to memory types:

```tsx
<Counter>
    // 1. CONST (Immutable) -> Compiles to Hardcoded Value
    const MAX_CLICKS = 10 

    // 2. STATE (Reactive) -> Compiles to SharedArrayBuffer
    state count = 0
    state status: "idle" | "active" = "idle"

    // 3. DERIVED (Computed) -> Compiles to Dependency Graph
    derived double = count * 2 

    <button onClick={count++} disabled={count >= MAX_CLICKS}>
        {count} / {double}
    </button>
</Counter>
```

## Asset References

Use the `@/` prefix for compile-time verified assets:

```tsx
// Verified at Compile Time
<img src="@/logo.png" />

// SVGs are inlined automatically
<img src="@/icons/user.svg" />
```

## Event Modifiers

Pipe modifiers eliminate boilerplate:

```tsx
// Prevent Default & Stop Propagation
<form onSubmit|prevent|stop={submitForm}>

// Keyboard Shortcuts
<div onKeyDown|meta+s={save} tabindex="0">

// Debounce
<input onInput|debounce:300ms={search} />

// Once - Removes listener after trigger
<button onClick|once={showConfetti}>Explode</button>
```

## Async Blocks

Handle loading/error states declaratively:

```tsx
#async users
    :loading
        <Skeleton count={3} />
    :error(err)
        <Alert>~"Failed: {err.message}"</Alert>
    :data(list)
        <ul>
            #each list as item
                <li>{item.name}</li>
        </ul>
```

## Scoped Styling

Apply styles only to the current component:

```tsx
<div style:scope>
    <h1>Hello</h1>
</div>

style {
    h1 { color: red; }
}
```

## Time Primitives

Safe interval handling with automatic cleanup:

```tsx
state seconds = 0

// Automatically cleans up when component unmounts
every 1s {
    seconds++
}

<div>Time: {seconds}</div>
```

## Context Sharing

Share state with descendants without prop drilling:

**Parent.dx:**
```tsx
state theme = "dark"
expose theme

<Child />
```

**Child.dx:**
```tsx
consume theme 

<div class={theme}>...</div>
```

## Enums (Finite State Machines)

Type-safe state management:

```tsx
enum Status { Idle, Fetching, Success(u32), Error(string) }

state current = Status.Idle

#match current
    | .Idle       => <button onClick={load}>Start</button>
    | .Success(t) => <span>Done in {t}ms</span>
```

## Portals

Render elements anywhere in the DOM:

```tsx
// Renders inside document.body
<Modal portal:body>
    <h1>Overlay</h1>
</Modal>

// Renders into a specific ID
<Tooltip portal="#header-slot">
    Info
</Tooltip>
```

## Related

- [Architecture](../architecture/README.md)
- [API Reference](../api/README.md)
