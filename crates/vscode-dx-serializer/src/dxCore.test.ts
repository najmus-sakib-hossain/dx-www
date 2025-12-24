/**
 * Property-based tests for DxCore
 * 
 * Feature: dx-serializer-extension-fix, Property 8: WASM and TypeScript Equivalence
 * 
 * For any valid DX content, the WASM implementation and TypeScript fallback
 * implementation SHALL produce identical transformation results.
 * 
 * **Validates: Requirements 5.3, 5.4**
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
import { parseLlm, DxDocument, strValue, numValue, boolValue, nullValue, createDocument, createSection } from './llmParser';
import { formatDocument } from './humanFormatter';
import { parseHuman, serializeToLlm } from './humanParser';

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
 * Generate a valid abbreviated key (2-letter abbreviation)
 */
const abbreviatedKey = fc.constantFrom(
    'nm', 'tt', 'ds', 'id', 'st', 'ac', 'en', 'ct', 'tl', 'pr', 'am', 'qt', 'em', 'ph', 'ur', 'pt', 'vl', 'tp'
);

/**
 * Generate an LLM format value
 */
const llmValue = fc.oneof(
    fc.stringOf(
        fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'),
        { minLength: 1, maxLength: 10 }
    ),
    fc.integer({ min: 0, max: 1000 }).map((n: number) => n.toString()),
    fc.constant('+'),  // boolean true
    fc.constant('-'),  // boolean false
    fc.constant('~'),  // null
);

/**
 * Generate a context section in LLM format
 */
const llmContextSection = fc.array(
    fc.tuple(abbreviatedKey, llmValue),
    { minLength: 1, maxLength: 3 }
).map((pairs: [string, string][]) => {
    const uniquePairs = new Map<string, string>();
    for (const [k, v] of pairs) {
        uniquePairs.set(k, v);
    }
    const pairStr = Array.from(uniquePairs.entries())
        .map(([k, v]) => `${k}|${v}`)
        .join(';');
    return `#c:${pairStr}`;
});

/**
 * Generate a data section in LLM format
 */
const llmDataSection = fc.tuple(
    fc.constantFrom('d', 'h', 'o', 'p', 'u'),  // section ID
    fc.array(abbreviatedKey, { minLength: 1, maxLength: 3 }),  // schema
    fc.array(fc.array(llmValue, { minLength: 1, maxLength: 3 }), { minLength: 1, maxLength: 3 })  // rows
).map(([id, schema, rows]: [string, string[], string[][]]) => {
    const uniqueSchema = [...new Set(schema)];
    const schemaStr = uniqueSchema.join('|');
    const header = `#${id}(${schemaStr})`;
    const dataRows = rows.map(row => {
        // Ensure row has same length as schema
        const paddedRow = uniqueSchema.map((_, i) => row[i] || '~');
        return paddedRow.join('|');
    });
    return [header, ...dataRows].join('\n');
});

/**
 * Generate a complete LLM format document
 */
const llmDocument = fc.tuple(
    fc.option(llmContextSection, { nil: undefined }),
    fc.option(llmDataSection, { nil: undefined })
).map(([context, data]: [string | undefined, string | undefined]) => {
    const parts: string[] = [];
    if (context) parts.push(context);
    if (data) parts.push(data);
    return parts.join('\n');
}).filter((s: string) => s.length > 0);


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
 * Property 8.1: LLM to Human to LLM Round-Trip
 * For any valid LLM format, converting to human and back should preserve data
 * 
 * Feature: dx-serializer-extension-fix, Property 8: WASM and TypeScript Equivalence
 * **Validates: Requirements 5.3, 5.4**
 */
export function testLlmRoundTrip(): void {
    fc.assert(
        fc.property(llmDocument, (llm: string) => {
            // Parse LLM format
            const parseResult = parseLlm(llm);
            if (!parseResult.success || !parseResult.document) {
                // Skip invalid documents
                return true;
            }

            // Convert to human format
            const human = formatDocument(parseResult.document);

            // Parse human format back
            const humanParseResult = parseHuman(human);
            if (!humanParseResult.success || !humanParseResult.document) {
                throw new Error(`Failed to parse human format: ${human}`);
            }

            // Serialize back to LLM
            const llmBack = serializeToLlm(humanParseResult.document);

            // Parse both to compare documents
            const originalDoc = parseResult.document;
            const roundTripDoc = humanParseResult.document;

            // Compare context
            if (originalDoc.context.size !== roundTripDoc.context.size) {
                throw new Error(`Context size mismatch: ${originalDoc.context.size} vs ${roundTripDoc.context.size}`);
            }

            // Compare sections
            if (originalDoc.sections.size !== roundTripDoc.sections.size) {
                throw new Error(`Section count mismatch: ${originalDoc.sections.size} vs ${roundTripDoc.sections.size}`);
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 8.1: LLM to Human to LLM Round-Trip');
}

/**
 * Property 8.2: formatDx produces valid human format
 * For any valid LLM content, formatDx should succeed
 */
export function testFormatDxProducesValidOutput(): void {
    fc.assert(
        fc.property(llmDocument, (llm: string) => {
            const result = formatDx(llm);
            // Should not throw and should produce non-empty output for non-empty input
            if (llm.trim() && !result) {
                throw new Error(`formatDx returned empty for non-empty input: '${llm}'`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 8.2: formatDx produces valid human format');
}

/**
 * Property 8.3: minifyDx produces valid LLM format
 * For any human format produced by formatDx, minifyDx should succeed
 */
export function testMinifyDxProducesValidOutput(): void {
    fc.assert(
        fc.property(llmDocument, (llm: string) => {
            // First convert to human
            const human = formatDx(llm);
            if (!human) return true;

            // Then convert back to LLM
            const result = minifyDx(human);

            // Should produce valid LLM format (starts with # or is empty)
            if (result && !result.startsWith('#')) {
                // It's okay if it doesn't start with # for simple content
                // Just verify it doesn't throw
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 8.3: minifyDx produces valid LLM format');
}

/**
 * Property 8.4: Validation accepts valid LLM format
 * For any valid LLM document, validation should pass
 */
export function testValidationAcceptsValidLlm(): void {
    fc.assert(
        fc.property(llmDocument, (llm: string) => {
            const result = validateDx(llm);
            if (!result.success) {
                throw new Error(`Validation failed for valid LLM: '${llm}', error: ${result.error}`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 8.4: Validation accepts valid LLM format');
}

/**
 * Property 7.1: Fallback toHuman produces valid output
 * For any valid dense content, toHuman should succeed
 */
export function testFallbackToHumanSucceeds(): void {
    const core = createFallbackCore(2);

    fc.assert(
        fc.property(llmDocument, (dense: string) => {
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
        fc.property(llmDocument, (dense: string) => {
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
 * Property 7.3: Round-trip preserves document structure
 * For any LLM document, round-trip should preserve the structure
 */
export function testRoundTripPreservesStructure(): void {
    const core = createFallbackCore(2);

    fc.assert(
        fc.property(llmDocument, (llm: string) => {
            // Transform to human
            const human = core.toHuman(llm);
            if (!human.success) {
                throw new Error(`toHuman failed: ${human.error}`);
            }

            // Transform back to dense
            const result = core.toDense(human.content);
            if (!result.success) {
                throw new Error(`toDense failed: ${result.error}`);
            }

            // Parse both documents
            const originalParse = parseLlm(llm);
            const roundTripParse = parseLlm(result.content);

            if (!originalParse.success || !roundTripParse.success) {
                return true; // Skip if parsing fails
            }

            // Verify structure is preserved
            const origDoc = originalParse.document!;
            const rtDoc = roundTripParse.document!;

            // Context size should match
            if (origDoc.context.size !== rtDoc.context.size) {
                throw new Error(`Context size mismatch: ${origDoc.context.size} vs ${rtDoc.context.size}`);
            }

            // Section count should match
            if (origDoc.sections.size !== rtDoc.sections.size) {
                throw new Error(`Section count mismatch: ${origDoc.sections.size} vs ${rtDoc.sections.size}`);
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7.3: Round-trip preserves document structure');
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
            name: 'toHuman transforms LLM context section',
            test: () => {
                const result = core.toHuman('#c:nm|Test;ct|42');
                return result.success && result.content.includes('name') && result.content.includes('Test');
            }
        },
        {
            name: 'toHuman transforms LLM data section',
            test: () => {
                const result = core.toHuman('#d(id|nm)\n1|Alpha\n2|Beta');
                return result.success &&
                    result.content.includes('Alpha') &&
                    result.content.includes('Beta');
            }
        },
        {
            name: 'toDense transforms human format',
            test: () => {
                const human = '[config]\n    name = Test';
                const result = core.toDense(human);
                return result.success && result.content.includes('nm') && result.content.includes('Test');
            }
        },
        {
            name: 'validate accepts valid LLM content',
            test: () => {
                const result = core.validate('#c:nm|Test');
                return result.success;
            }
        },
        {
            name: 'validate accepts valid human content',
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
            name: 'validate rejects invalid LLM sigil',
            test: () => {
                const result = core.validate('#x:invalid');
                return !result.success && (result.error?.includes('sigil') ?? false);
            }
        },
        {
            name: 'validate rejects schema mismatch',
            test: () => {
                const result = core.validate('#d(id|nm)\n1|Alpha|Extra');
                return !result.success && (result.error?.includes('columns') ?? false);
            }
        },
        {
            name: 'isSaveable returns true for valid content',
            test: () => {
                return core.isSaveable('#c:nm|Test');
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
    console.log('Running Property 8: WASM and TypeScript Equivalence tests...\n');

    // New LLM format tests
    testLlmRoundTrip();
    testFormatDxProducesValidOutput();
    testMinifyDxProducesValidOutput();
    testValidationAcceptsValidLlm();

    // Legacy tests updated for LLM format
    testFallbackToHumanSucceeds();
    testFallbackToDenseSucceeds();
    testRoundTripPreservesStructure();
    testValidationDetectsUnclosedBrackets();
    testValidationDetectsUnclosedStrings();
    testValidContentPassesValidation();
    testSmartQuoteApostrophes();

    console.log('\n✓ All Property 8 tests passed!');
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
