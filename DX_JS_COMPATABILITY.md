At root crates folder please create a new crates for dx called dx-js-compatibility and there please do these:

```markdown

```

Builtin Core Features
Essential runtime capabilities

Web Standard APIs
Support for web standard APIs like fetch, URL, EventTarget, Headers, etc.

Powered by WebCore (from WebKit/Safari)
Native Addons
Call C-compatible native code from JavaScript
Bun.ffi, NAPI, partial V8 C++ API
TypeScript
First-class support, including "paths" enum namespace
JSX
First-class support without configuration
Module loader plugins
Plugin API for importing/requiring custom file types
`Bun.plugin` works in browsers & Bun
3 different loader APIs. Server-side only


PostgreSQL, MySQL, and SQLite drivers
Connect to any SQL database with one fast, unified API
Fastest available, with query pipelining
S3 Cloud Storage driver
Upload and download from S3-compatible storage, built-in
Fastest available
Redis client
Redis client built into Bun with Pub/Sub support
WebSocket server (including pub/sub)
WebSocket server built into Bun.serve() with backpressure handling
`Bun.serve()`
HTTP server
Lightning-fast HTTP server built into Bun
Bun.serve()
HTTP router
Route HTTP requests with dynamic paths and wildcards, built into Bun.serve()
Bun.serve({routes: {'/api/:path': (req) => { ... }}}})
Single-file executables
Compile your app to a standalone executable that runs anywhere
bun build --compile with cross-compilation & code signing
No native addons, embedded files, cross-compilation or bytecode. Multi-step process.
No native addons, no cross-compilation
YAML
YAML is a first-class citizen in Bun, just like JSON
Bun.YAML & import from .yaml files
Cookies API
Parse and set cookies with zero overhead using a Map-like API
request.cookies Map-like API
Encrypted Secrets Storage
Store secrets securely using your OS's native keychain
Bun.secrets (Keychain/libsecret/Windows Credential Manager)

npm package management
Install, manage, and publish npm-compatible dependencies
With catalogs, isolated installs, bun audit, bun why
Limited features
Bundler
Build production-ready code for frontend & backend
Bun.build
Cross-platform $ shell API
Native bash-like shell for cross-platform shell scripting
`Bun.$`
Requires 'dax'
Jest-compatible test runner
Testing library compatible with the most popular testing framework
bun test with VS Code integration & concurrent execution
Hot reloading (server)
Reload your backend without disconnecting connections, preserving state
bun --hot
Monorepo support
Install workspaces packages and run commands across workspaces
bun run --filter=package-glob ...
Frontend Development Server
Run modern frontend apps with a fully-featured dev server
bun ./index.html
Built-in formatter and linter

Password & Hashing APIs
bcrypt, argon2, and non-cryptographic hash functions
`Bun.password` & `Bun.hash`
String Width API
Calculate the width of a string as displayed in the terminal
Bun.stringWidth
Glob API
Glob patterns for file matching
Bun.Glob
fs.promises.glob
Semver API
Compare and sort semver strings
Bun.semver
CSS color conversion API
Convert between CSS color formats
Bun.color
CSRF API
Generate and verify CSRF tokens
Bun.CSRF
