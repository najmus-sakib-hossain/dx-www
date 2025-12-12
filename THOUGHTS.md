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