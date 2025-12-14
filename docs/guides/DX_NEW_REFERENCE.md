# dx new - Project Creation Quick Reference

## Command

```bash
dx new <project-name>
```

## Generated Structure

```
<project-name>/
├── .dx/                     # System files (gitignored)
│   ├── cache/              # Build cache
│   ├── build/              # Build artifacts
│   └── temp/               # Temporary files
│
├── app/                     # Application pages and routes
│   ├── pages/              # Page components
│   │   └── index.tsx       # Home page (generated)
│   ├── layouts/            # Layout components
│   │   └── MainLayout.tsx  # Main layout (generated)
│   └── api/                # API routes
│
├── auth/                    # Authentication
│   ├── providers/          # OAuth providers, etc.
│   └── middleware/         # Auth middleware
│
├── component/               # Reusable components
│   ├── ui/                 # UI components
│   │   └── Button.tsx      # Button component (generated)
│   ├── forms/              # Form components
│   └── layout/             # Layout components
│
├── db/                      # Database
│   ├── schema/             # Database schemas
│   ├── migrations/         # Schema migrations
│   └── seeds/              # Seed data
│
├── media/                   # Static assets
│   ├── images/             # Images (PNG, JPG, SVG)
│   ├── video/              # Video files
│   ├── audio/              # Audio files
│   └── documents/          # PDFs, docs, etc.
│
├── icon/                    # Icons
│   ├── svg/                # SVG icons
│   └── sprite/             # Icon sprites
│
├── feature/                 # Feature modules
│   ├── analytics/          # Analytics feature
│   ├── billing/            # Billing feature
│   └── notifications/      # Notifications feature
│
├── font/                    # Custom fonts
│   ├── woff2/              # WOFF2 fonts
│   └── variable/           # Variable fonts
│
├── i18n/                    # Internationalization
│   └── locales/            # Translation files
│       ├── en/             # English
│       │   └── common.json # Common translations (generated)
│       ├── es/             # Spanish
│       └── fr/             # French
│
├── style/                   # Styles
│   ├── main.css            # Global styles (generated)
│   ├── themes/             # Theme files
│   ├── components/         # Component styles
│   └── utilities/          # Utility classes
│
├── dx                       # Configuration file (TOML)
├── README.md                # Project documentation (generated)
└── .gitignore               # Git ignore patterns (generated)
```

## Configuration File (dx)

The `dx` configuration file uses TOML format:

```toml
[project]
name = "my-app"
version = "1.0.0"
description = "A dx-www application"

[build]
target = "wasm32-unknown-unknown"
mode = "auto"  # auto-select micro (338B) or macro (7.5KB) runtime
optimize = "size"

[dev]
port = 3000
hot_reload = true
open_browser = true

[server]
compression = "brotli"
cache = true

[style]
framework = "dx-style"  # Binary CSS (B-CSS)
autoprefixer = true

[i18n]
default_locale = "en"
fallback = "en"
```

## Generated Files

### 1. app/pages/index.tsx
Home page with a counter example demonstrating state management.

### 2. app/layouts/MainLayout.tsx
HTML layout wrapper with head and body structure.

### 3. component/ui/Button.tsx
Reusable button component example.

### 4. style/main.css
Global CSS styles with CSS variables and component styles.

### 5. i18n/locales/en/common.json
English translations for common strings.

### 6. README.md
Project-specific documentation with quick start guide.

### 7. .gitignore
Git ignore patterns for dx-www projects.

## Next Steps After Creation

```bash
# Navigate to project
cd my-app

# Start development server
dx dev

# Build for production
dx build --release

# Preview production build
dx preview
```

## Folder Usage Guidelines

| Folder | Purpose | Example Files |
|--------|---------|--------------|
| `app/pages/` | Page routes | `index.tsx`, `about.tsx`, `blog/[slug].tsx` |
| `app/layouts/` | Layout wrappers | `MainLayout.tsx`, `DashboardLayout.tsx` |
| `app/api/` | API endpoints | `users.ts`, `auth/login.ts` |
| `auth/` | Auth logic | `AuthProvider.tsx`, `useAuth.ts` |
| `component/ui/` | UI components | `Button.tsx`, `Input.tsx`, `Modal.tsx` |
| `db/schema/` | DB schemas | `users.ts`, `posts.ts` |
| `feature/` | Feature modules | `analytics/`, `payments/`, `admin/` |
| `i18n/locales/` | Translations | `en/common.json`, `es/errors.json` |
| `style/` | Styles | `main.css`, `themes/dark.css` |

## Best Practices

1. **Keep pages simple:** Use components for complex logic
2. **Organize by feature:** Group related files in `feature/` folders
3. **Reusable components:** Put in `component/` for sharing across features
4. **Translations:** Use i18n for all user-facing text
5. **Type safety:** Use TypeScript for all `.tsx` files
6. **Styles:** Prefer component-scoped styles, use `style/` for globals

## Examples

### Creating a Blog Feature

```
feature/
└── blog/
    ├── BlogList.tsx
    ├── BlogPost.tsx
    ├── BlogEditor.tsx
    ├── useBlogPosts.ts
    └── types.ts
```

### Adding Authentication

```
auth/
├── providers/
│   ├── GoogleAuth.ts
│   └── GitHubAuth.ts
├── middleware/
│   └── requireAuth.ts
├── AuthProvider.tsx
└── useAuth.ts
```

### Internationalization

```
i18n/
└── locales/
    ├── en/
    │   ├── common.json
    │   ├── errors.json
    │   └── blog.json
    └── es/
        ├── common.json
        ├── errors.json
        └── blog.json
```

## Additional Commands

```bash
# Create with specific template
dx new my-app --template=dashboard

# Create in specific directory
dx new my-app --path=./projects

# Skip git initialization
dx new my-app --no-git

# Use specific runtime mode
dx new my-app --mode=micro  # or --mode=macro
```

---

For more information, see:
- [Full Documentation](https://dx-www.dev/docs)
- [Examples](https://dx-www.dev/examples)
- [API Reference](https://dx-www.dev/api)
