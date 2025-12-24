import { defineConfig } from 'vitest/config';

export default defineConfig({
    test: {
        // Only include vitest-compatible test files
        // The existing test files use a custom test runner, not vitest
        include: [
            'src/**/*.vitest.test.ts',
        ],
        // Exclude everything else
        exclude: [
            'out/**',
            'node_modules/**',
            'src/**/*.test.ts',  // Exclude all .test.ts files (they use custom runner)
        ],
        globals: true,
        environment: 'node',
    },
});
