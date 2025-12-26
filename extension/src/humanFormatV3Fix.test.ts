/**
 * Human Format V3 Fix Test
 * 
 * Verifies that the toHuman() output is in TOML-like format (Human Format V3)
 * and NOT the old format with decorative comments and Unicode tables.
 */

import { formatDx, validateDx } from './dxCore';
import { formatDocumentV3, containsTableFormatting } from './humanFormatterV3';
import { parseLlm } from './llmParser';

// Test LLM input (from the dx file in the workspace) - using new format
const TEST_LLM_INPUT = `au|essensefromexistence
ds|Orchestrate don't just own your code
ed|*neovim,zed,vscode,cursor,antigravity,replit,firebase-studio
nm|dx
tt|Enhanced Developing Experience
vr|0.0.1
ws|*@/www,@/backend
#:js|javascript/typescript|bun|tsc|vite|bun|react
#:rust|rs|rust|native|rustc|cargo|actix-web
#:python|py|python|python|uv|pip|django
#:A|en-US
#:B|latest
#d(pt)
@/driven
#i(locales_pt|locales_df|locales_dv|locales_pd|ttses_pt|ttses_df|ttses_dv|ttses_pd)
@/locales|^A|^A|all|@/media/sounds|^A|^A|bn-BD
#o(pt|pk|vr)
@/icons|Lucide|Hugeicons
#p(dependencies_django|dependencies_numpy)
^B|^B
#s(pt|th)
@/style|dx
#t(pt|df)
@/fonts|Inter
#u(pt|cp)
@/components/ui|button`;

// Old format markers that should NOT appear
const OLD_FORMAT_MARKERS = [
    '# ═',           // Decorative comment header
    '┌',             // Table top-left corner
    '┐',             // Table top-right corner
    '└',             // Table bottom-left corner
    '┘',             // Table bottom-right corner
    '─',             // Horizontal line
    '│',             // Vertical line
    '├',             // Left T-junction
    '┤',             // Right T-junction
    '┬',             // Top T-junction
    '┴',             // Bottom T-junction
    '┼',             // Cross junction
    'Total:',        // Row count footer
    'CONFIGURATION', // Old section header
    'REFERENCES',    // Old section header
];

// Human Format V3 markers that SHOULD appear
const V3_FORMAT_MARKERS = [
    'author',        // Expanded key
    'name',          // Expanded key
    'variant',       // Expanded key (vr -> variant in abbreviations)
    '[stack]',       // Section header
    '[driven]',      // Section header
    ' = ',           // Key-value separator
    ' | ',           // Array separator
];

function runTests(): void {
    console.log('=== Human Format V3 Fix Tests ===\n');

    let passed = 0;
    let failed = 0;

    // Test 1: formatDx produces Human Format V3
    console.log('Test 1: formatDx produces Human Format V3');
    try {
        const output = formatDx(TEST_LLM_INPUT);

        // Check for old format markers
        let hasOldMarkers = false;
        for (const marker of OLD_FORMAT_MARKERS) {
            if (output.includes(marker)) {
                console.log(`  ✗ FAIL: Output contains old format marker: "${marker}"`);
                hasOldMarkers = true;
            }
        }

        if (!hasOldMarkers) {
            console.log('  ✓ PASS: No old format markers found');
            passed++;
        } else {
            failed++;
        }
    } catch (error) {
        console.log(`  ✗ FAIL: ${error}`);
        failed++;
    }

    // Test 2: formatDx produces V3 format markers
    console.log('\nTest 2: formatDx produces V3 format markers');
    try {
        const output = formatDx(TEST_LLM_INPUT);

        let hasV3Markers = true;
        for (const marker of V3_FORMAT_MARKERS) {
            if (!output.includes(marker)) {
                console.log(`  ✗ FAIL: Output missing V3 format marker: "${marker}"`);
                hasV3Markers = false;
            }
        }

        if (hasV3Markers) {
            console.log('  ✓ PASS: All V3 format markers found');
            passed++;
        } else {
            failed++;
        }
    } catch (error) {
        console.log(`  ✗ FAIL: ${error}`);
        failed++;
    }

    // Test 3: containsTableFormatting returns false
    console.log('\nTest 3: containsTableFormatting returns false for V3 output');
    try {
        const output = formatDx(TEST_LLM_INPUT);
        const hasTableFormatting = containsTableFormatting(output);

        if (!hasTableFormatting) {
            console.log('  ✓ PASS: No table formatting detected');
            passed++;
        } else {
            console.log('  ✗ FAIL: Table formatting detected in output');
            failed++;
        }
    } catch (error) {
        console.log(`  ✗ FAIL: ${error}`);
        failed++;
    }

    // Test 4: Output has aligned equals signs
    console.log('\nTest 4: Output has aligned equals signs');
    try {
        const output = formatDx(TEST_LLM_INPUT);
        const lines = output.split('\n').filter(line => line.includes('=') && !line.startsWith('['));

        // Check that all = signs are at position >= 20 (key padding)
        let allAligned = true;
        for (const line of lines) {
            const eqPos = line.indexOf('=');
            if (eqPos < 18) { // Allow some flexibility
                console.log(`  ✗ FAIL: Line not aligned: "${line.substring(0, 40)}..."`);
                allAligned = false;
                break;
            }
        }

        if (allAligned) {
            console.log('  ✓ PASS: All lines have aligned equals signs');
            passed++;
        } else {
            failed++;
        }
    } catch (error) {
        console.log(`  ✗ FAIL: ${error}`);
        failed++;
    }

    // Test 5: Output uses pipe separator for arrays
    console.log('\nTest 5: Output uses pipe separator for arrays');
    try {
        const output = formatDx(TEST_LLM_INPUT);

        // Check that editors line uses pipe separator
        const editorsLine = output.split('\n').find(line => line.includes('editors'));
        if (editorsLine && editorsLine.includes(' | ')) {
            console.log('  ✓ PASS: Arrays use pipe separator');
            passed++;
        } else {
            console.log('  ✗ FAIL: Arrays do not use pipe separator');
            failed++;
        }
    } catch (error) {
        console.log(`  ✗ FAIL: ${error}`);
        failed++;
    }

    // Print sample output
    console.log('\n=== Sample Output (first 30 lines) ===\n');
    try {
        const output = formatDx(TEST_LLM_INPUT);
        const lines = output.split('\n').slice(0, 30);
        console.log(lines.join('\n'));
    } catch (error) {
        console.log(`Error: ${error}`);
    }

    // Summary
    console.log('\n=== Summary ===');
    console.log(`Passed: ${passed}`);
    console.log(`Failed: ${failed}`);

    if (failed > 0) {
        process.exit(1);
    }
}

// Run tests
runTests();
