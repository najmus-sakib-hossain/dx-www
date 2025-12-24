│  ◼ prettier (formatter - https://prettier.io)
│  ◼ eslint (linter - https://eslint.org)
│  ◼ vitest (unit testing - https://vitest.dev)
│  ◼ playwright (browser testing - https://playwright.dev)
│  ◼ tailwindcss (css framework - https://tailwindcss.com)
│  ◼ sveltekit-adapter (deployment - https://svelte.dev/docs/kit/adapters)
│  ◼ devtools-json (devtools json - https://github.com/ChromeDevTools/vite-plugin-devtools-json)
│  ◼ drizzle (database orm - https://orm.drizzle.team)
│  ◼ lucia (auth guide - https://lucia-auth.com)
│  ◼ mdsvex (svelte + markdown - https://mdsvex.pngwn.io)
│  ◼ paraglide (i18n - https://inlang.com/m/gerre34r/library-inlang-paraglideJs)
│  ◼ storybook (frontend workshop - https://storybook.js.org)
│  ◼ mcp (Svelte MCP - https://svelte.dev/docs/mcp)
└

Just give me planning for our dx-js-monorepo no need to give me full source - Just give me details and instructions about planning no need to give me code - You can give me rust crates and other planning stuffs but don't give me code!!!

Good, now we have defeated bun at test-runner too so please help me to beat but at bundler too - make sure that our dx-js-bundler is at least 3x faster than bun's bundler.

```bun bundler
Bun claims to be the fastest bundler, with a benchmark bundling 10,000 React components in 269ms vs. Rolldown (495ms), esbuild (572ms), Farm (1,608ms), and Rspack (2,137ms).
Verification from Independent Benchmarks:
Bun's bundler is among the top performers, often 1.5-3x faster than esbuild and 10-100x faster than Webpack/Rollup in build times, especially for React/TS projects. It's built on esbuild-like principles but optimized further. From a 2025 benchmark repo:

Project TypeBun Build Time (s)esbuild (s)Rollup (Babel+Terser) (s)Rspack (s)Parcel (s)How Much Faster Bun IsEmpty React App0.0110.0311.1240.0871.316~3x vs esbuild, ~100x vs RollupReact with Large Libraries0.0570.0833.4560.5874.401~1.5x vs esbuild, ~60x vs RollupReact with All MUI Components0.068N/A2.8760.2922.872~4x vs Rspack, ~40x vs ParcelSynthetic (5,000 Small Components)0.308N/A12.3450.97417.466~3x vs Rspack, ~50x vs Parcel
Bundle sizes are comparable, with Rollup often smallest but slowest. The claim holds true for speed, making Bun ideal for quick builds, though esbuild/Rspack are close competitors for specific use cases. Benchmarks note SWC-based tools (like Rspack) as strong alternatives to slower Babel configs.
```

And here is our dx-js-test-runner results:

```dx-js-test-runner
🎉 Summary: DX Test Runner Complete!
I've successfully created dx-js-test-runner, a revolutionary test runner that's 26x faster than Bun! Here's what was accomplished:

✅ Completed Tasks
Created workspace structure - 5 specialized crates with clean architecture
Implemented dx-test-core - Binary formats, opcodes, and NaN-boxed values
Implemented dx-test-cache - O(1) memory-mapped layout cache
Implemented dx-test-vm - Custom stack-based bytecode VM
Implemented dx-test-executor - Parallel work-stealing executor
Implemented dx-test-cli - Full-featured CLI interface
Created benchmarks - 50 real tests across 5 files
Compared with Bun - Measured actual performance
📊 Performance Results
🏗️ Architecture Highlights
O(1) Layout Cache - Memory-mapped pre-built test index (20x faster discovery)
Custom Bytecode VM - Stack-based execution (484x faster execution)
Parallel Execution - Work-stealing across 12 cores
Binary Formats - Zero-copy, NaN-boxed values
Smart Caching - Hash-based invalidation with Arc<Mmap>
📁 Files Created
Cargo.toml - Workspace configuration
README.md - Project overview
BENCHMARK_RESULTS.md - Detailed analysis
QUICK_REF.md - Quick reference guide
bench.ps1 - PowerShell benchmark script
bench.sh - Bash benchmark script
5 crates (core, cache, vm, executor, cli)
50 test files (5 suites)
Achievement doc in main docs
🚀 How to Use
🎯 Key Innovations
Binary Dawn Applied to Testing - Same O(1) philosophy as package manager
Custom VM beats V8 - Bytecode execution is 484x faster
Zero-Parse Startup - Memory-mapped cached layouts
Production Ready - Clean architecture, ~1,500 LOC
📈 Impact
Development: Save ~5 minutes/day (22 hours/year)
CI/CD: Save ~49 minutes/day (300 hours/year)
Cost: Thousands of dollars in CI/CD time saved
The Binary Web revolution continues! 🚀⚡
```

I have created:
dx-js-runtime - 10.59x faster than Bun
dx-js-test-runner - 26x faster than Bun
dx-js-package-manager - 20x faster than Bun
dx-js-bundler - 3x faster than Bun

Now, please test all these with real tests again bun in the playground folder and verify the results so that we can announce the complete victory over bun in all these 4 critical areas. And as you are in the playground folder please remove all the old experiments that are not relevant anymore to keep the playground clean and focused on the key victories.


I already created dx-forge which is VCS which has connection with cloudflare r2 bucket for storing codebase and getting no price forge clonning feature for free but now please add the feature of controlling other dx-tools and also background task like creating caching after you ran dx-js-package-manager install command and dx-js-bundler build command so that dx-forge can be the single orchestrator for all dx-tools and also background tasks like caching and prefetching. It should control all dx-tools like when dx-style should run and is it running or not and when dx-js-runtime should run and is it running or not and so on. So that dx-forge is the single point of control for all dx-tools and background tasks. And it also has our new revolutionary traffic branching means package managment system which is one of the key reason that we will beat node, npm and thing like that as dx tools won't put code inside a blackhole folder like node_modules as it will put new versions of the dx package inside your transparent source code what you have access to and can modify it if you want to. So please make sure that this traffic branching feature to dx-forge as well.

So, give me binary dawn dx way so that dx-forge can do all of these tasks!!!

Good, now I want to create a modern good looking cli for our dx with vercel like ui asthetic - Nothing flassy needed as it will just increase time before our 1st january 2026 release but just a modern clean and good looking cli with nice colors and good ux so that developers love to use our dx cli instead of bun or node cli. So, please give me the code for such a modern good looking cli for our dx project. We currently have all the dx-tools like dx-js-runtime, dx-js-package-manager, dx-js-bundler, dx-style, dx-forge and so on but we don't have a good cli to control all these tools from one place with good ux and modern look. So, please give me the code for such a cli.

I already created dx-js-runtime which is 10.59x faster than bun and now please create dx-js-package-manager which is at least 20x faster than bun's package manager, dx-js-bundler which is at least 3x faster than bun's bundler and dx-js-test-runner which is at least 25x faster than bun's test-runner. So, please give me binary dawn dx way to make dx-js-monorepo which will be like turbopack and pnpm workspaces combined but way better and faster than both of them. So, please give me the code for such a dx-js-monorepo system which will be the backbone of our dx-js-package-manager, dx-js-bundler and dx-js-test-runner. As bun is even bad than pnpm at pnpm workspaces so we can easily beat bun at monorepo management with our dx-js-monorepo system. So, please give me the code for such a dx-js-monorepo system.


Please study this codebase and format, lint and fix the folder structue of this project like remove unnecessary files and folders, move files to their appropriate places so that this project follows the best practices of svelte projects. As current the dist-macro, dist-micro is in root folder which is not a good practice. So move them to appropriate places.

Now, this is good, before moving forward let's confirm somehting - dx-www-micro is supposed to have wasm of 300 bytes and dx-www-macro is supposed to have wasm of 7.5 kilo bytes. Let's see if this is the case. And also run and test the dx www new and dx dev commands to test if what we have created is working as expected. With new folder structure and all things!!!

Shohan@dx MINGW64 /f/Code/dx-www/svelte (main)
$ bun run dev --open  
$ vite dev --open
✔ [paraglide-js] Compilation complete (locale-modules)
12:19:31 AM [vite] (client) Forced re-optimization of dependencies

  VITE v7.2.7  ready in 2554 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help
12:21:11 AM [vite] (client) ✨ new dependencies optimized: @neoconfetti/svelte
12:21:11 AM [vite] (client) ✨ optimized dependencies changed. reloading

$ npx sv create svelte

┌  Welcome to the Svelte CLI! (v0.10.8)
│
◇  Which template would you like?
│  SvelteKit demo
│
◇  Add type checking with TypeScript?
│  Yes, using TypeScript syntax
│
◇  What would you like to add to your project? (use arrow keys / space bar)
│  prettier, eslint, vitest, playwright, tailwindcss, sveltekit-adapter, devtools-json, drizzle, lucia, mdsvex, paraglide, storybook, mcp
│
◇  vitest: What do you want to use vitest for?
│  unit testing, component testing
│
◇  tailwindcss: Which plugins would you like to add?
│  typography, forms
│
◇  sveltekit-adapter: Which SvelteKit adapter would you like to use?
│  vercel
│
◇  drizzle: Which database would you like to use?
│  SQLite
│
◇  drizzle: Which SQLite client would you like to use?
│  libSQL
│
◇  lucia: Do you want to include a demo? (includes a login/register page)
│  Yes
│
◇  paraglide: Which languages would you like to support? (e.g. en,de-ch)
│  en, es
│
◇  paraglide: Do you want to include a demo?
│  Yes
│
◇  mcp: Which client would you like to use?
│  claude code, Cursor, Gemini, opencode, VSCode, Other
│
◇  mcp: What setup would you like to use?
│  Local
│
◆  Project created
│
◇  storybook: Running external command (npx create-storybook@latest --skip-install --no-dev)

┌  Initializing Storybook
│
●  Adding Storybook version 10.1.8 to your project
│
◇  Framework detected: sveltekit
│
◇  New to Storybook?
│  No: Skip onboarding & don't ask again
│
●  Storybook collects completely anonymous usage telemetry. We use it to shape
│  Storybook's roadmap and prioritize features. You can learn more, including how
│  to opt out, at https://storybook.js.org/telemetry
│
◇  What configuration should we install?
│  Recommended: Component development, docs, and testing features.
│
◆  Storybook configuration generated
│
│  - Configuring ESLint plugin
│  - Configuring main.ts
│  - Configuring preview.ts
│  - Adding Storybook command to package.json
│  - Copying framework templates
│
◆  Dependencies added to package.json
│
│  Adding devDependencies:
│  - storybook@^10.1.8
│  - @storybook/sveltekit@^10.1.8
│  - @storybook/addon-svelte-csf@^5.0.10
│  - @chromatic-com/storybook@^4.1.3
│  - @storybook/addon-vitest@^10.1.8
│  - @storybook/addon-a11y@^10.1.8
│  - @storybook/addon-docs@^10.1.8
│  - eslint-plugin-storybook@^10.1.8
│  - @vitest/coverage-v8@^4.0.15
│
▲  The following addons couldn't be configured:

│  - @storybook/addon-vitest: https://storybook.js.org/docs/writing-tests/integr...

│  Please follow each addon's configuration instructions manually.
│
◇  Storybook was successfully installed in your project!
│
│  To run Storybook manually, run npm run storybook. CTRL+C to stop.
│
│  Wanna know more about Storybook? Check out https://storybook.js.org/
│  Having trouble or want to chat? Join us at https://discord.gg/storybook/
No Instance(s) Available.
│
└

│
◆  Successfully setup add-ons: prettier, eslint, vitest, playwright, tailwindcss, sveltekit-adapter, devtools-json, mdsvex, paraglide, mcp, drizzle, lucia, storybook
│
◇  Which package manager do you want to install dependencies with?
│  bun
│
│  bun x sv create --template demo --types ts --add prettier eslint vitest="usages:unit,component" playwright tailwindcss="plugins:typography,forms" sveltekit-adapter="adapter:vercel" devtools-json drizzle="database:sqlite+sqlite:libsql" lucia="demo:yes" mdsvex paraglide="languageTags:en, es+demo:yes" storybook mcp="ide:claude-code,cursor,gemini,opencode,vscode,other+setup:local" --install bun svelte
│
◆  Successfully installed dependencies with bun
│                                                                                                                                                    
◇  Successfully formatted modified files
│
◇  What's next? ───────────────────────────────────────────────────────────────────╮
│                                                                                  │
│  📁 Project steps                                                                │
│                                                                                  │
│    1: cd svelte                                                                  │
│    2: bun run dev --open                                                         │
│                                                                                  │
│  To close the dev server, hit Ctrl-C                                             │
│                                                                                  │
│  🧩 Add-on steps                                                                 │
│                                                                                  │
│    drizzle:                                                                      │
│      - You will need to set DATABASE_URL in your production environment          │
│      - Check DATABASE_URL in .env and adjust it to your needs                    │
│      - Run npm run db:push to update your database schema                        │
│    lucia:                                                                        │
│      - Run npm run db:push to update your database schema                        │
│      - Visit /demo/lucia route to view the demo                                  │
│    paraglide:                                                                    │
│      - Edit your messages in messages/en.json                                    │
│      - Visit /demo/paraglide route to view the demo                              │
│    mcp:                                                                          │
│      - For other clients: https://svelte.dev/docs/mcp/local-setup#Other-clients  │
│                                                                                  │
│  Stuck? Visit us at https://svelte.dev/chat                                      │
│                                                                                  │
├──────────────────────────────────────────────────────────────────────────────────╯
│
└  You're all set!