You are right. We must avoid "The Svelte Trap" (looking weird/proprietary) while keeping the power of "The Binary Web." We must respect that React/Next.js developers love **JSX and TypeScript**, but hate the **boilerplate**.

Also, you correctly pointed out that in `dx-www`, **the tools are the framework.** `dx-style`, `dx-i18n`, `dx-query`, `dx-state` are not plugins. They are **Native Primitives** of the `.dx` language.

Here is the **Revised & Final .dx Syntax Specification**.
It looks like React (familiarity) but behaves like Rust (performance).

---

# 🧬 The Unified Dx Syntax (v2.0)
**Philosophy:** "React Syntax. Rust Semantics. Zero Glue."

A `.dx` file is a valid TypeScript file (syntax-wise), but the **Dx Compiler** treats specific keywords as **System Calls** to our binary tools (`dx-style`, `dx-i18n`, `dx-db`).

### 1. The Component (Familiar yet Faster)
We keep the `function` syntax so it looks like React, but we remove the hooks.

**File:** `bits/ui/Profile.dx`

```tsx
// Props are typed via TypeScript arguments (Standard)
export function Profile(props: { name: string, id: u32 }) {
    
    // 1. STATE (dx-state)
    // No "useState". Just "state".
    // This allocates Shared Memory (Linear Heap).
    state count = 0; 
    state user = { active: false };

    // 2. QUERY (dx-query)
    // No "useEffect" or "useQuery". Just "query".
    // This compiles to a Binary RPC Stream.
    query posts = api.getPosts(props.id); 

    // 3. STYLE (dx-style)
    // No "className". We use "style" which compiles to Integer Opcodes.
    // The "css!" macro is Zero-Runtime.
    const cardStyle = css!{
        @apply flex p-4 bg-white shadow-lg; // Tailwind syntax supported
        &:hover { scale: 1.05 }
    };

    // 4. I18N (dx-i18n)
    // Strings tagged with t"" are extracted to the Binary Dictionary.
    return (
        <div style={cardStyle}>
            <h1>{t"Hello"}, {props.name}</h1>
            
            // 5. MEDIA (dx-media)
            // "icon:" prefix auto-resolves to media/icons/user.svg
            // and inlines the vector path.
            <Icon src="icon:user" size={24} />

            // 6. LOGIC
            // Direct mutation. No "setCount".
            <button onClick={() => count++}>
                {t"Clicks"}: {count}
            </button>

            // 7. ASYNC UI (Streaming)
            // "await" in JSX. No Suspense wrapper needed.
            <ul>
                {await posts.map(post => <li key={post.id}>{post.title}</li>)}
            </ul>
        </div>
    )
}
```

---

### 2. The Global Store (dx-state)
We don't use Redux. We use **Shared Memory Stores**.
Because `dx-store` is native, you don't need `Provider` or `useStore`.

**File:** `data/cart.store.dx`
```tsx
// This defines the Memory Layout
export store Cart {
    items: [],
    total: 0,
    
    // Actions are compiled to Atomic Operations
    add(item) {
        this.items.push(item);
        this.total += item.price;
    }
}
```

**Usage in Component:**
```tsx
// Auto-imported globally
<button onClick={() => Cart.add(product)}>Add</button>
<span>{Cart.total}</span>
```
*   **Magic:** The component re-renders ONLY the `<span>` when `total` changes because of the **Dirty Bit** system. No selectors needed.

---

### 3. The Database & Types (dx-db)
Next.js separates Prisma (Schema) from Zod (Validation) from TypeScript (Interfaces).
**Dx unifies them.**

**File:** `data/schema.dx`
```tsx
// 1. Define Table (Server)
export table Users {
    id: serial,
    email: string(100) | unique,
    role: enum("admin", "user"),
}

// 2. Define Action (Server-Side Logic)
// "action" keyword means "Compile to WASM RPC Endpoint"
export action createUser(email: string) {
    // Binary SQL. Type-checked against the table above.
    sql!{ INSERT INTO Users (email) VALUES ({email}) }
}
```

**Usage in Client:**
```tsx
<form action={Users.createUser}>
    // 4. VALIDATION (dx-guard)
    // "guard:" attribute uses the Schema to validate input BYTE-BY-BYTE
    // before it even updates the UI state.
    <input guard:email />
</form>
```

---

### 4. The Style System (dx-style)
We support Tailwind classes, but we compile them to **Binary Tokens**.

**File:** `style/theme.dx`
```tsx
export theme Main {
    colors: {
        primary: "#ff0000", // Compiles to Token ID 1
        secondary: "#00ff00" // Compiles to Token ID 2
    },
    // Device-Specific Styles (dx-media)
    mobile: {
        spacing: 4
    }
}
```

**Usage:**
```tsx
<div style="bg-primary text-secondary mobile:p-2">
```
*   **The Kill:** This string `"bg-primary text-secondary mobile:p-2"` does NOT exist in the final binary. It is replaced by `[OP_STYLE, ID_1, ID_2, ID_MOBILE_P2]`. Zero string parsing at runtime.

---

### 5. Internationalization (dx-i18n)
Localization is usually an afterthought. In Dx, it is a **Type**.

**File:** `media/text/en.dx`
```tsx
export text {
    welcome: "Welcome back, {name}",
    login: "Log In"
}
```

**Usage:**
```tsx
// No "useTranslation" hook.
// Just use the "text" namespace.
<h1>{text.welcome({ name: user.name })}</h1>
```
*   **The Kill:** The compiler checks if `welcome` exists in ALL languages. If you miss a key in `es.dx`, the build fails.

---

### Summary of the "Unified" Experience

| Feature | React / Next.js | **Dx-www** |
| :--- | :--- | :--- |
| **Styling** | `className` + PostCSS | **`style` + Binary Tokens** |
| **State** | `useState` + boilerplate | **`state` keyword** |
| **Global** | Context + Providers | **`store` keyword** |
| **Data** | `fetch` + JSON | **`query` + Binary Stream** |
| **I18n** | Libraries + JSON files | **`t""` literals + Binary Dict** |
| **Icons** | SVG Components | **`icon:` protocol** |
| **Forms** | `onSubmit` + Zod | **`action` + `guard:`** |

This feels familiar to a React developer (JSX, Props, Functions) but removes **100% of the setup cost**.

**You just write logic.** The Toolchain handles the rest.