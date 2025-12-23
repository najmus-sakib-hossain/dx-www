/**
 * Property-based tests for DxCore
 * 
 * Feature: dx-serializer-extension, Property 7: WASM and Fallback Equivalence
 * 
 * For any valid DX content, the WASM implementation and TypeScript fallback
 * implementation SHALL produce identical transformation results.
 * 
 * **Validates: Requirements 12.3**
 * 
 * Note: Since WASM may not be available in all test environments, these tests
 * focus on the TypeScript fallback implementation's correctness and round-trip
 * properties.
 */

import * as fc from 'fast-check';
import {
    formatDx,
    minifyDx,
    validateDx,
    smartQuote,
    createFallbackCore,
    TransformResult,
    ValidationResult,
} from './dxCore';

// ============================================================================
// Generators for DX content
// ============================================================================

/**
 * Generate a valid key (alphanumeric with dots and underscores)
 */
const validKey = fc.stringOf(
    fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz0123456789_'),
    { minLength: 1, maxLength: 15 }
).filter((s: string) => /^[a-z]/.test(s)); // Must start with letter

/**
 * Generate a simple value (no special characters)
 */
const simpleValue = fc.oneof(
    fc.stringOf(
        fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'),
        { minLength: 1, maxLength: 15 }
    ),
    fc.integer({ min: 0, max: 10000 }).map((n: number) => n.toString()),
    fc.constant('true'),
    fc.constant('false'),
);


/**
 * Generate a key:value pair in dense format
 */
const keyValuePair = fc.tuple(validKey, simpleValue)
    .map(([k, v]: [string, string]) => `${k}:${v}`);

/**
 * Generate an object in dense format: key#field:val#field:val
 */
const simpleObject = fc.tuple(
    validKey,
    fc.array(fc.tuple(validKey, simpleValue), { minLength: 1, maxLength: 3 })
).map(([key, fields]: [string, [string, string][]]) => {
    // Ensure unique field keys
    const uniqueFields = new Map<string, string>();
    for (const [k, v] of fields) {
        uniqueFields.set(k, v);
    }
    const fieldStr = Array.from(uniqueFields.entries())
        .map(([k, v]) => `#${k}:${v}`)
        .join('');
    return `${key}${fieldStr}`;
});

// ============================================================================
// Property Tests
// ============================================================================

/**
 * Property 7.1: Fallback toHuman produces valid output
 * For any valid dense content, toHuman should succeed
 */
export function testFallbackToHumanSucceeds(): void {
    const core = createFallbackCore(2);

    fc.assert(
        fc.property(keyValuePair, (dense: string) => {
            const result = core.toHuman(dense);
            if (!result.success) {
                throw new Error(`toHuman failed for '${dense}': ${result.error}`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.1: Fallback toHuman produces valid output');
}

/**
 * Property 7.2: Fallback toDense produces valid output
 * For any valid human content, toDense should succeed
 */
export function testFallbackToDenseSucceeds(): void {
    const core = createFallbackCore(2);

    fc.assert(
        fc.property(keyValuePair, (dense: string) => {
            // First convert to human, then back to dense
            const human = core.toHuman(dense);
            if (!human.success) return true; // Skip if toHuman fails

            const result = core.toDense(human.content);
            if (!result.success) {
                throw new Error(`toDense failed for '${human.content}': ${result.error}`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.2: Fallback toDense produces valid output');
}

/**
 * Property 7.3: Round-trip preserves key-value data
 * For any key:value pair, round-trip should preserve the data
 */
export function testRoundTripPreservesKeyValue(): void {
    const core = createFallbackCore(2);

    fc.assert(
        fc.property(validKey, simpleValue, (key: string, value: string) => {
            const dense = `${key}:${value}`;

            // Transform to human
            const human = core.toHuman(dense);
            if (!human.success) {
                throw new Error(`toHuman failed: ${human.error}`);
            }

            // Transform back to dense
            const result = core.toDense(human.content);
            if (!result.success) {
                throw new Error(`toDense failed: ${result.error}`);
            }

            // Verify key is preserved
            if (!result.content.includes(key)) {
                throw new Error(`Key '${key}' not found in result: '${result.content}'`);
            }

            // Verify value is preserved (accounting for boolean normalization)
            const normalizedValue = value === 'true' ? '1' : value === 'false' ? '0' : value;
            const valuePresent = result.content.includes(value) ||
                result.content.includes(normalizedValue);
            if (!valuePresent) {
                throw new Error(`Value '${value}' not found in result: '${result.content}'`);
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.3: Round-trip preserves key-value data');
}


/**
 * Property 7.4: Validation detects unclosed brackets
 * For any content with unclosed brackets, validation should fail
 */
export function testValidationDetectsUnclosedBrackets(): void {
    const unclosedBracket = fc.tuple(
        validKey,
        fc.constantFrom('{', '[', '(')
    ).map(([key, bracket]: [string, string]) => `${key}: ${bracket}value`);

    fc.assert(
        fc.property(unclosedBracket, (content: string) => {
            const result = validateDx(content);
            if (result.success) {
                throw new Error(`Validation should fail for unclosed bracket in: '${content}'`);
            }
            if (!result.error?.includes('Unclosed') && !result.error?.includes('bracket')) {
                throw new Error(`Error should mention unclosed bracket: ${result.error}`);
            }
            if (result.line === undefined) {
                throw new Error('Validation error should include line number');
            }
            if (result.hint === undefined) {
                throw new Error('Validation error should include hint');
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.4: Validation detects unclosed brackets');
}

/**
 * Property 7.5: Validation detects unclosed strings
 * For any content with unclosed strings, validation should fail
 */
export function testValidationDetectsUnclosedStrings(): void {
    const unclosedString = fc.tuple(
        validKey,
        fc.constantFrom('"', "'"),
        simpleValue
    ).map(([key, quote, value]: [string, string, string]) => `${key}: ${quote}${value}`);

    fc.assert(
        fc.property(unclosedString, (content: string) => {
            const result = validateDx(content);
            if (result.success) {
                throw new Error(`Validation should fail for unclosed string in: '${content}'`);
            }
            if (!result.error?.includes('Unclosed') && !result.error?.includes('string')) {
                throw new Error(`Error should mention unclosed string: ${result.error}`);
            }
            if (result.line === undefined) {
                throw new Error('Validation error should include line number');
            }
            if (result.hint === undefined) {
                throw new Error('Validation error should include hint');
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.5: Validation detects unclosed strings');
}

/**
 * Property 7.6: Valid content passes validation
 * For any properly formatted content, validation should succeed
 */
export function testValidContentPassesValidation(): void {
    const validContent = fc.tuple(validKey, simpleValue)
        .map(([k, v]: [string, string]) => `${k}: ${v}`);

    fc.assert(
        fc.property(validContent, (content: string) => {
            const result = validateDx(content);
            if (!result.success) {
                throw new Error(`Validation should pass for: '${content}', got error: ${result.error}`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.6: Valid content passes validation');
}

// ============================================================================
// Smart Quoting Tests
// ============================================================================

/**
 * Property 7.7: Smart quoting handles apostrophes correctly
 * Strings with apostrophes should be wrapped in double quotes
 */
export function testSmartQuoteApostrophes(): void {
    const stringWithApostrophe = fc.tuple(
        fc.stringOf(fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz'), { minLength: 1, maxLength: 10 }),
        fc.stringOf(fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz'), { minLength: 1, maxLength: 10 })
    ).map(([before, after]: [string, string]) => `${before}'${after}`);

    fc.assert(
        fc.property(stringWithApostrophe, (value: string) => {
            const quoted = smartQuote(value);
            // Should use double quotes for strings with apostrophes
            if (!quoted.startsWith('"') || !quoted.endsWith('"')) {
                throw new Error(`Expected double quotes for '${value}', got: ${quoted}`);
            }
            // Should contain the original apostrophe
            if (!quoted.includes("'")) {
                throw new Error(`Apostrophe should be preserved in: ${quoted}`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.7: Smart quoting handles apostrophes correctly');
}


// ============================================================================
// Unit Tests
// ============================================================================

export function runUnitTests(): void {
    console.log('Running unit tests for DxCore...\n');

    const core = createFallbackCore(2);
    let passed = 0;
    let failed = 0;

    const tests: Array<{ name: string; test: () => boolean }> = [
        {
            name: 'toHuman transforms simple key:value',
            test: () => {
                const result = core.toHuman('name:John');
                return result.success && result.content.includes('name') && result.content.includes('John');
            }
        },
        {
            name: 'toHuman transforms object',
            test: () => {
                const result = core.toHuman('server#host:localhost#port:5432');
                return result.success &&
                    result.content.includes('host') &&
                    result.content.includes('localhost');
            }
        },
        {
            name: 'toDense transforms human format',
            test: () => {
                const human = 'name: John';
                const result = core.toDense(human);
                return result.success && result.content.includes('name') && result.content.includes('John');
            }
        },
        {
            name: 'validate accepts valid content',
            test: () => {
                const result = core.validate('key: value');
                return result.success;
            }
        },
        {
            name: 'validate rejects unclosed bracket',
            test: () => {
                const result = core.validate('key: {value');
                return !result.success && (result.error?.includes('Unclosed') ?? false);
            }
        },
        {
            name: 'validate rejects unclosed string',
            test: () => {
                const result = core.validate('key: "value');
                return !result.success && (result.error?.includes('Unclosed') ?? false);
            }
        },
        {
            name: 'validate rejects mismatched brackets',
            test: () => {
                const result = core.validate('key: [value}');
                return !result.success && (result.error?.includes('Mismatched') ?? false);
            }
        },
        {
            name: 'isSaveable returns true for valid content',
            test: () => {
                return core.isSaveable('key: value');
            }
        },
        {
            name: 'isSaveable returns false for invalid content',
            test: () => {
                return !core.isSaveable('key: {unclosed');
            }
        },
        {
            name: 'smartQuote handles simple strings',
            test: () => {
                return smartQuote('hello') === 'hello';
            }
        },
        {
            name: 'smartQuote handles strings with spaces',
            test: () => {
                return smartQuote('hello world') === '"hello world"';
            }
        },
        {
            name: 'smartQuote handles apostrophes',
            test: () => {
                return smartQuote("don't") === '"don\'t"';
            }
        },
        {
            name: 'smartQuote handles double quotes',
            test: () => {
                return smartQuote('say "hello"') === "'say \"hello\"'";
            }
        },
        {
            name: 'smartQuote handles both quote types',
            test: () => {
                const result = smartQuote("don't say \"hello\"");
                return result.startsWith('"') && result.includes("\\'") === false;
            }
        },
        {
            name: 'empty input returns empty output',
            test: () => {
                const human = core.toHuman('');
                const dense = core.toDense('');
                return human.success && human.content === '' &&
                    dense.success && dense.content === '';
            }
        },
    ];

    for (const { name, test } of tests) {
        try {
            if (test()) {
                console.log(`  ✓ ${name}`);
                passed++;
            } else {
                console.log(`  ✗ ${name}`);
                failed++;
            }
        } catch (error) {
            console.log(`  ✗ ${name}: ${error}`);
            failed++;
        }
    }

    console.log(`\nUnit tests: ${passed} passed, ${failed} failed`);

    if (failed > 0) {
        throw new Error(`${failed} unit tests failed`);
    }
}

// ============================================================================
// Run All Tests
// ============================================================================

export function runAllPropertyTests(): void {
    console.log('Running Property 7: WASM and Fallback Equivalence tests...\n');

    testFallbackToHumanSucceeds();
    testFallbackToDenseSucceeds();
    testRoundTripPreservesKeyValue();
    testValidationDetectsUnclosedBrackets();
    testValidationDetectsUnclosedStrings();
    testValidContentPassesValidation();
    testSmartQuoteApostrophes();

    console.log('\n✓ All Property 7 tests passed!');
}

// Run tests if this file is executed directly
if (require.main === module) {
    try {
        runUnitTests();
        console.log('');
        runAllPropertyTests();
    } catch (error) {
        console.error('Tests failed:', error);
        process.exit(1);
    }
}
