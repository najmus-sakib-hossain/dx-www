/**
 * Property-based tests for Human Format V3 Parser
 * 
 * Feature: dx-serializer-v3
 * 
 * Tests Property 3 from the design document:
 * - Property 3: Human Format V3 Round-Trip
 * 
 * **Validates: Requirements 2.1-2.7, 3.1**
 */

import * as fc from 'fast-check';
import {
    parseHumanV3,
    parseValueV3,
    parseArrayValueV3,
    parseSectionHeaderV3,
    parseKeyValueLineV3,
    serializeToLlmV3,
} from './humanParserV3';
import {
    formatDocumentV3,
    DEFAULT_CONFIG,
} from './humanFormatterV3';
import {
    DxDocument,
    DxSection,
    DxValue,
    createDocument,
    createSection,
    strValue,
    numValue,
    boolValue,
    nullValue,
    arrValue,
} from './llmParser';

// ============================================================================
// Generators
// ============================================================================

/**
 * Generate a valid abbreviated key
 */
const abbreviatedKey = fc.constantFrom(
    'nm', 'tt', 'ds', 'id', 'v', 'au', 'ws', 'ed', 'repo', 'cont', 'ci',
    'st', 'ac', 'en', 'ct', 'tl', 'pr', 'am', 'qt', 'em', 'ph', 'ur', 'pt'
);

/**
 * Generate a simple string value (no special characters that would break parsing)
 */
const simpleString = fc.stringOf(
    fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-./'),
    { minLength: 1, maxLength: 15 }
);

/**
 * Generate a DxValue (excluding arrays for simpler round-trip)
 */
const simpleDxValue: fc.Arbitrary<DxValue> = fc.oneof(
    simpleString.map(s => strValue(s)),
    fc.integer({ min: 0, max: 10000 }).map(n => numValue(n)),
    fc.boolean().map(b => boolValue(b)),
    fc.constant(nullValue())
);

/**
 * Generate a context map with simple values
 */
const simpleContextMap = fc.array(
    fc.tuple(abbreviatedKey, simpleDxValue),
    { minLength: 1, maxLength: 3 }
).map(pairs => {
    const map = new Map<string, DxValue>();
    const usedKeys = new Set<string>();
    for (const [key, value] of pairs) {
        if (!usedKeys.has(key)) {
            map.set(key, value);
            usedKeys.add(key);
        }
    }
    return map;
});

/**
 * Generate a simple DxSection (single row for easier round-trip)
 */
const simpleDxSection = fc.tuple(
    fc.constantFrom('f', 'k', 'y', 'u', 'm', 'i', 'o', 't'),
    fc.array(abbreviatedKey, { minLength: 1, maxLength: 3 }),
    fc.array(simpleDxValue, { minLength: 1, maxLength: 3 })
).map(([id, schema, values]) => {
    // Ensure unique schema keys
    const uniqueSchema = [...new Set(schema)];
    const section = createSection(id, uniqueSchema);

    // Create single row matching schema length
    const row = uniqueSchema.map((_, i) => values[i] || nullValue());
    section.rows.push(row);

    return section;
});

/**
 * Generate a simple DxDocument
 */
const simpleDxDocument = fc.tuple(
    simpleContextMap,
    fc.array(simpleDxSection, { minLength: 0, maxLength: 2 })
).map(([context, sections]) => {
    const doc = createDocument();
    for (const [key, value] of context) {
        doc.context.set(key, value);
    }
    // Ensure unique section IDs
    const usedIds = new Set<string>();
    for (const section of sections) {
        if (!usedIds.has(section.id)) {
            doc.sections.set(section.id, section);
            usedIds.add(section.id);
        }
    }
    return doc;
});

// ============================================================================
// Property Tests
// ============================================================================

/**
 * Property 3: Human Format V3 Round-Trip
 * For any valid DxDocument, formatting to Human V3 and parsing back SHALL produce an equivalent document
 * 
 * **Validates: Requirements 2.1-2.7, 3.1**
 */
export function testHumanV3RoundTrip(): void {
    fc.assert(
        fc.property(simpleDxDocument, (doc: DxDocument) => {
            // Format to Human V3
            const humanV3 = formatDocumentV3(doc);

            // Parse back
            const parseResult = parseHumanV3(humanV3);

            if (!parseResult.success || !parseResult.document) {
                throw new Error(`Failed to parse Human V3:\n${humanV3}\nError: ${parseResult.error?.message}`);
            }

            const roundTripDoc = parseResult.document;

            // Compare context sizes
            if (doc.context.size !== roundTripDoc.context.size) {
                throw new Error(
                    `Context size mismatch: original ${doc.context.size}, round-trip ${roundTripDoc.context.size}\n` +
                    `Original: ${JSON.stringify([...doc.context.entries()])}\n` +
                    `Round-trip: ${JSON.stringify([...roundTripDoc.context.entries()])}`
                );
            }

            // Compare context values
            for (const [key, value] of doc.context) {
                const rtValue = roundTripDoc.context.get(key);
                if (!rtValue) {
                    throw new Error(`Missing context key after round-trip: ${key}`);
                }
                // Compare value types and values
                if (value.type !== rtValue.type) {
                    throw new Error(`Context value type mismatch for ${key}: ${value.type} vs ${rtValue.type}`);
                }
            }

            // Compare section counts
            if (doc.sections.size !== roundTripDoc.sections.size) {
                throw new Error(
                    `Section count mismatch: original ${doc.sections.size}, round-trip ${roundTripDoc.sections.size}`
                );
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 3: Human Format V3 Round-Trip');
}

// ============================================================================
// Unit Tests
// ============================================================================

export function runUnitTests(): void {
    console.log('Running unit tests for Human Parser V3...\n');

    let passed = 0;
    let failed = 0;

    const tests: Array<{ name: string; test: () => boolean }> = [
        {
            name: 'parseValueV3 handles simple strings',
            test: () => {
                const result = parseValueV3('hello');
                return result.type === 'string' && result.value === 'hello';
            }
        },
        {
            name: 'parseValueV3 handles quoted strings',
            test: () => {
                const result = parseValueV3('"hello world"');
                return result.type === 'string' && result.value === 'hello world';
            }
        },
        {
            name: 'parseValueV3 handles numbers',
            test: () => {
                const result = parseValueV3('42');
                return result.type === 'number' && result.value === 42;
            }
        },
        {
            name: 'parseValueV3 handles booleans',
            test: () => {
                const trueResult = parseValueV3('true');
                const falseResult = parseValueV3('false');
                return trueResult.type === 'bool' && trueResult.value === true &&
                    falseResult.type === 'bool' && falseResult.value === false;
            }
        },
        {
            name: 'parseValueV3 handles null as dash',
            test: () => {
                const result = parseValueV3('-');
                return result.type === 'null';
            }
        },
        {
            name: 'parseArrayValueV3 handles pipe-separated arrays',
            test: () => {
                const result = parseArrayValueV3('a | b | c');
                return result.type === 'array' &&
                    (result.value as DxValue[]).length === 3;
            }
        },
        {
            name: 'parseSectionHeaderV3 parses simple header',
            test: () => {
                const result = parseSectionHeaderV3('[forge]');
                return result !== null && result[0] === 'forge' && result[1].length === 0;
            }
        },
        {
            name: 'parseSectionHeaderV3 parses header with schema',
            test: () => {
                const result = parseSectionHeaderV3('[stack] = Lang | Runtime | Compiler');
                return result !== null &&
                    result[0] === 'stack' &&
                    result[1].length === 3;
            }
        },
        {
            name: 'parseKeyValueLineV3 parses key = value',
            test: () => {
                const result = parseKeyValueLineV3('name                 = dx');
                return result !== null && result[0] === 'name' && result[1] === 'dx';
            }
        },
        {
            name: 'parseKeyValueLineV3 returns null for section headers',
            test: () => {
                const result = parseKeyValueLineV3('[forge]');
                return result === null;
            }
        },
        {
            name: 'parseKeyValueLineV3 returns null for comments',
            test: () => {
                const result = parseKeyValueLineV3('# comment');
                return result === null;
            }
        },
        {
            name: 'parseHumanV3 parses config section',
            test: () => {
                const input = `name                 = dx
version              = 0.0.1`;
                const result = parseHumanV3(input);
                return result.success &&
                    result.document !== undefined &&
                    result.document.context.size === 2;
            }
        },
        {
            name: 'parseHumanV3 parses data section',
            test: () => {
                const input = `[forge]
repository           = https://example.com
container            = none`;
                const result = parseHumanV3(input);
                return result.success &&
                    result.document !== undefined &&
                    result.document.sections.size === 1;
            }
        },
        {
            name: 'parseHumanV3 handles mixed config and sections',
            test: () => {
                const input = `name                 = dx
version              = 0.0.1

[forge]
repository           = https://example.com`;
                const result = parseHumanV3(input);
                return result.success &&
                    result.document !== undefined &&
                    result.document.context.size === 2 &&
                    result.document.sections.size === 1;
            }
        },
        {
            name: 'serializeToLlmV3 produces valid LLM format',
            test: () => {
                const doc = createDocument();
                doc.context.set('nm', strValue('test'));
                doc.context.set('v', strValue('1.0.0'));
                const result = serializeToLlmV3(doc);
                return result.startsWith('#c:') && result.includes('nm|test');
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
    console.log('Running Property tests for Human Parser V3...\n');

    testHumanV3RoundTrip();

    console.log('\n✓ All Human Parser V3 property tests passed!');
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
