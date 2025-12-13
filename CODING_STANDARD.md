This is the **Dx Coding Standard (v4.0 - The Paradigm Shift)**.

You are right. `state` is better than `let` because it is **Explicit**. Code should tell you what is reactive (expensive memory) and what is constant (cheap memory).
And regarding assets: **Next.js is right about simplicity**, but **Wrong about implementation**. We will use simple strings, but the **Compiler** will intercept them.

Here are **10 New Game-Changing Features** integrated into the final coding standard.

---

# 💎 The Dx Standard v4.0: "Explicit Magic"

### 1. The Trinity of Variable Declaration
We replace ambiguous `let` with three distinct keywords that map to memory types.

```tsx
<Counter>
    // 1. CONST (Immutable) -> Compiles to Hardcoded Value
    const MAX_CLICKS = 10 

    // 2. STATE (Reactive) -> Compiles to SharedArrayBuffer
    state count = 0
    state status: "idle" | "active" = "idle"

    // 3. DERIVED (Computed) -> Compiles to Dependency Graph
    // Updates ONLY when 'count' changes.
    derived double = count * 2 

    <button onClick={count++} disabled={count >= MAX_CLICKS}>
        {count} / {double}
    </button>
</Counter>
```

---

### 2. The "Smart String" Assets
You asked for `src="logo.png"`. We give you that, but with **Compiler Verification**.

*   **Syntax:** Use the `@/` prefix to trigger the Binary Pipeline.
*   **The Magic:** The compiler checks `media/` folder.
    *   If `logo.png` exists: It compiles to a Binary ID.
    *   If missing: **Build Error.** (No more 404s in production).

```tsx
// Verified at Compile Time.
// If you rename the file in 'media/', this line errors instantly.
<img src="@/logo.png" />

// SVGs are inlined automatically.
<img src="@/icons/user.svg" />
```

---

### 3. Event Modifiers (The "Vim" of Coding)
We remove the need for `e.preventDefault()` or `lodash.debounce`.
We use **Pipe Modifiers** on events.

```tsx
// 1. Prevent Default & Stop Propagation
<form onSubmit|prevent|stop={submitForm}>

// 2. Keyboard Shortcuts (Game Changer)
// No global event listener setup required.
<div onKeyDown|meta+s={save} tabindex="0">

// 3. Performance Modifiers
// "debounce:300ms" compiles to a native timer logic in WASM.
<input onInput|debounce:300ms={search} />

// 4. "Once" - Removes listener after trigger
<button onClick|once={showConfetti}>Explode</button>
```

---

### 4. The `#async` Block (Streaming UI)
Handling Loading/Error states is usually boilerplate hell.
Dx introduces the **Promise Block**.

```tsx
// "users" is a Promise/Stream
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

---

### 5. "Scope" Styling (Shadow DOM without Shadow DOM)
Sometimes you want styles that apply *only* to this file, without a global class.

```tsx
<div style:scope>
    <h1>Hello</h1>
</div>

// This CSS block is extracted, hashed, and scoped to this component ID.
// It compiles to binary CSS tokens.
style {
    h1 { color: red; }
    @media (mobile) { h1 { size: 12px; } }
}
```

---

### 6. The "Every" Loop (Temporal Logic)
`setInterval` is dangerous (memory leaks).
Dx adds a **Time Primitive**.

```tsx
state seconds = 0

// Automatically cleans up when component unmounts.
// Runs on the Frame Budget (won't lag the UI).
every 1s {
    seconds++
}

<div>Time: {seconds}</div>
```

---

### 7. "Prop Drilling" Killer: The `expose` keyword
If you have a nested component tree, passing props down is painful.
`expose` makes a state variable available to **all children** (Implicit Context).

**Parent.dx:**
```tsx
state theme = "dark"
expose theme // Now available to all descendants

<Child />
```

**Child.dx:**
```tsx
// No import needed. It finds the nearest ancestor exposing 'theme'.
consume theme 

<div class={theme}>...</div>
```

---

### 8. Binary "Enums" (Finite State Machines)
Strings like `"loading"` are fragile.
Dx allows inline Enum definitions for robust state logic.

```tsx
// Define valid states
enum Status { Idle, Fetching, Success(u32), Error(string) }

state current = Status.Idle

fn load() {
    current = Status.Success(200)
}

#match current
    | .Idle       => <button onClick={load}>Start</button>
    | .Success(t) => <span>Done in {t}ms</span>
```

---

### 9. The "Portal" Attribute (Teleportation)
Move an element anywhere in the DOM tree using one attribute.

```tsx
// Renders inside document.body, regardless of where this component is.
<Modal portal:body>
    <h1>Overlay</h1>
</Modal>

// Renders into a specific ID
<Tooltip portal="#header-slot">
    Info
</Tooltip>
```

---

### 10. "Auto-Form" (The 10x Feature)
Since `dx-guard` knows your database schema, why write form HTML manually?

```tsx
// "Users" is imported from schema.dx
import { Users } from "@/data/schema"

// The Compiler generates the <input>, labels, and validation 
// based on the DB Schema types (Email, Password, etc).
<AutoForm 
    schema={Users} 
    action={api.createUser}
    class="flex col gap-4"
/>
```

---

### 📜 The Final "God Mode" Example

Here is what it looks like when you combine everything.

**File:** `pages/signup.dx`

```tsx
<SignupPage>
    // 1. LOGIC
    state step = 0
    state formData = {}
    
    // Derived State
    derived isLastStep = step == 3

    // 2. SERVER ACTION
    action submit() {
        // Runs on Server
        db.users.insert(formData)
    }

    // 3. VIEW
    <div class="layout-center" motion:layout>
        
        // Asset from media/
        <img src="@/logo.svg" class="w-16 mb-4" />

        <h1>~"Join the Revolution"</h1>

        // Step Wizard using Match
        #match step
            | 0 => <AccountForm bind:data={formData} />
            | 1 => <ProfileForm bind:data={formData} />
            | 2 => <PlanSelection bind:data={formData} />
            | 3 => 
                <div class="text-center">
                    <Spinner />
                    <p>~"Creating account..."</p>
                </div>

        // Navigation
        <div class="flex justify-between mt-6">
            <button 
                disabled={step == 0} 
                onClick={step--}
                class="btn-ghost"
            >
                ~"Back"
            </button>

            <button 
                onClick={isLastStep ? submit() : step++}
                class="btn-primary"
                // Keyboard Shortcut
                onKeyDown|meta+enter={submit}
            >
                {isLastStep ? ~"Finish" : ~"Next"}
                <Icon.ArrowRight />
            </button>
        </div>
    </div>

    // 4. SCOPED STYLES
    style {
        .layout-center { @apply max-w-md mx-auto min-h-screen flex col justify-center; }
        h1 { @apply text-2xl font-bold text-center; }
    }
</SignupPage>
```

### Why this changes the game:
1.  **Readability:** It reads like a user story.
2.  **Safety:** Assets, I18n, and Styles are compiler-verified.
3.  **Speed:** `state`, `derived`, and `onKeyDown` compile to raw binary opcodes.

This is **Dx**.