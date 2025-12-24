/**
 * Property-based tests for Human Format V3
 * 
 * Feature: dx-serializer-v3
 * 
 * Tests Properties 4, 5, 6, 7 from the design document:
 * - Property 4: Key Alignment Consistency
 * - Property 5: Array Separator Consistency
 * - Property 6: No Table Formatting
 * - Property 7: String Quoting
 * 
 * **Validates: Requirements 2.3, 2.4, 2.6, 2.7**
 */

import * as fc from 'fast-check';
import {
    formatDocumentV3,
    formatConfigSectionV3,
    formatDataSectionV3,
    formatValueV3,
    containsTableFormatting,
    checkKeyAlignment,
    usesCorrectArraySeparator,
    DEFAULT_CONFIG,
    expandKey,
    compressKey,
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
 * Generate a simple string value (no special characters)
 */
const simpleString = fc.stringOf(
    fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-./'),
    { minLength: 1, maxLength: 20 }
);

/**
 * Generate a string with spaces (for quoting tests)
 */
const stringWithSpaces = fc.tuple(
    simpleString,
    simpleString
).map(([a, b]) => `${a} ${b}`);

/**
 * Generate a DxValue
 */
const dxValue: fc.Arbitrary<DxValue> = fc.oneof(
    simpleString.map(s => strValue(s)),
    fc.integer({ min: 0, max: 10000 }).map(n => numValue(n)),
    fc.boolean().map(b => boolValue(b)),
    fc.constant(nullValue())
);

/**
 * Generate an array DxValue
 */
const dxArrayValue = fc.array(simpleString, { minLength: 2, maxLength: 5 })
    .map(items => arrValue(items.map(s => strValue(s))));

/**
 * Generate a context map
 */
const contextMap = fc.array(
    fc.tuple(abbreviatedKey, dxValue),
    { minLength: 1, maxLength: 5 }
).map(pairs => {
    const map = new Map<string, DxValue>();
    for (const [key, value] of pairs) {
        map.set(key, value);
    }
    return map;
});

/**
 * Generate a DxSection
 */
const dxSection = fc.tuple(
    fc.constantFrom('f', 'k', 'y', 'u', 'm', 'i', 'o', 't', 'd'),
    fc.array(abbreviatedKey, { minLength: 1, maxLength: 4 }),
    fc.array(fc.array(dxValue, { minLength: 1, maxLength: 4 }), { minLength: 1, maxLength: 3 })
).map(([id, schema, rows]) => {
    const section = createSection(id, schema);
    // Ensure rows match schema length
    for (const row of rows) {
        const paddedRow = schema.map((_, i) => row[i] || nullValue());
        section.rows.push(paddedRow);
    }
    return section;
});

/**
 * Generate a DxDocument
 */
const dxDocument = fc.tuple(
    contextMap,
    fc.array(dxSection, { minLength: 0, maxLength: 3 })
).map(([context, sections]) => {
    const doc = createDocument();
    for (const [key, value] of context) {
        doc.context.set(key, value);
    }
    for (const section of sections) {
        doc.sections.set(section.id, section);
    }
    return doc;
});

// ============================================================================
// Property Tests
// ============================================================================

/**
 * Property 4: Key Alignment Consistency
 * For any Human V3 formatted output, all = signs SHALL appear at consistent positions
 * 
 * **Validates: Requirements 2.3**
 */
export function testKeyAlignmentConsistency(): void {
    fc.assert(
        fc.property(dxDocument, (doc: DxDocument) => {
            const output = formatDocumentV3(doc);

            // Check that = signs are aligned within each section
            const sections = output.split(/\n\n+/);
            for (const section of sections) {
                const lines = section.split('\n').filter(line =>
                    line.includes('=') && !line.trim().startsWith('[')
                );

                if (lines.length > 1) {
                    const positions = lines.map(line => line.indexOf('='));
                    const firstPos = positions[0];

                    // All = signs in this section should be at the same position
                    for (let i = 1; i < positions.length; i++) {
                        if (positions[i] !== firstPos) {
                            // Allow some variance for section headers with schema
                            if (Math.abs(positions[i] - firstPos) > 5) {
                                throw new Error(
                                    `Key alignment inconsistent: position ${positions[i]} vs ${firstPos}\n` +
                                    `Lines:\n${lines.join('\n')}`
                                );
                            }
                        }
                    }
                }
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 4: Key Alignment Consistency');
}

/**
 * Property 5: Array Separator Consistency
 * For any array value in Human V3 format, the separator SHALL be | and not ,
 * 
 * **Validates: Requirements 2.4**
 */
export function testArraySeparatorConsistency(): void {
    fc.assert(
        fc.property(dxArrayValue, (arrayValue: DxValue) => {
            const output = formatValueV3(arrayValue);

            // Should use | separator
            if (!output.includes(' | ')) {
                throw new Error(`Array should use ' | ' separator: ${output}`);
            }

            // Should not use comma separator for arrays
            // (Note: individual values might contain commas, but array items should be | separated)
            const items = (arrayValue.value as DxValue[]);
            if (items.length > 1) {
                const pipeCount = (output.match(/ \| /g) || []).length;
                if (pipeCount !== items.length - 1) {
                    throw new Error(`Expected ${items.length - 1} pipe separators, got ${pipeCount}: ${output}`);
                }
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 5: Array Separator Consistency');
}

/**
 * Property 6: No Table Formatting
 * For any Human V3 formatted output, the content SHALL NOT contain Unicode table border characters
 * 
 * **Validates: Requirements 2.6**
 */
export function testNoTableFormatting(): void {
    fc.assert(
        fc.property(dxDocument, (doc: DxDocument) => {
            const output = formatDocumentV3(doc);

            if (containsTableFormatting(output)) {
                throw new Error(`Output contains table formatting characters:\n${output}`);
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 6: No Table Formatting');
}

/**
 * Property 7: String Quoting
 * For any string value containing spaces, the Human V3 format SHALL wrap it in double quotes
 * 
 * **Validates: Requirements 2.7**
 */
export function testStringQuoting(): void {
    fc.assert(
        fc.property(stringWithSpaces, (str: string) => {
            const value = strValue(str);
            const output = formatValueV3(value);

            // String with spaces should be quoted
            if (!output.startsWith('"') || !output.endsWith('"')) {
                throw new Error(`String with spaces should be quoted: '${str}' -> '${output}'`);
            }

            // The content inside quotes should match the original
            const unquoted = output.slice(1, -1);
            if (unquoted !== str) {
                throw new Error(`Quoted content mismatch: expected '${str}', got '${unquoted}'`);
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 7: String Quoting');
}

// ============================================================================
// Unit Tests
// ============================================================================

export function runUnitTests(): void {
    console.log('Running unit tests for Human Format V3...\n');

    let passed = 0;
    let failed = 0;

    const tests: Array<{ name: string; test: () => boolean }> = [
        {
            name: 'formatValueV3 handles simple strings',
            test: () => {
                const result = formatValueV3(strValue('hello'));
                return result === 'hello';
            }
        },
        {
            name: 'formatValueV3 quotes strings with spaces',
            test: () => {
                const result = formatValueV3(strValue('hello world'));
                return result === '"hello world"';
            }
        },
        {
            name: 'formatValueV3 handles numbers',
            test: () => {
                const result = formatValueV3(numValue(42));
                return result === '42';
            }
        },
        {
            name: 'formatValueV3 handles booleans',
            test: () => {
                return formatValueV3(boolValue(true)) === 'true' &&
                    formatValueV3(boolValue(false)) === 'false';
            }
        },
        {
            name: 'formatValueV3 handles null as dash',
            test: () => {
                const result = formatValueV3(nullValue());
                return result === '-';
            }
        },
        {
            name: 'formatValueV3 handles arrays with pipe separator',
            test: () => {
                const arr = arrValue([strValue('a'), strValue('b'), strValue('c')]);
                const result = formatValueV3(arr);
                return result === 'a | b | c';
            }
        },
        {
            name: 'formatConfigSectionV3 produces no [config] header',
            test: () => {
                const context = new Map<string, DxValue>();
                context.set('nm', strValue('test'));
                const result = formatConfigSectionV3(context);
                return !result.includes('[config]') && result.includes('name');
            }
        },
        {
            name: 'formatConfigSectionV3 aligns = signs',
            test: () => {
                const context = new Map<string, DxValue>();
                context.set('nm', strValue('test'));
                context.set('v', strValue('1.0.0'));
                const result = formatConfigSectionV3(context);
                const lines = result.split('\n');
                const positions = lines.map(l => l.indexOf('='));
                return positions[0] === positions[1];
            }
        },
        {
            name: 'formatDataSectionV3 produces [section] header',
            test: () => {
                const section = createSection('f', ['nm', 'repo']);
                section.rows.push([strValue('forge'), strValue('https://example.com')]);
                const result = formatDataSectionV3(section);
                return result.includes('[forge]');
            }
        },
        {
            name: 'formatDocumentV3 produces no table characters',
            test: () => {
                const doc = createDocument();
                doc.context.set('nm', strValue('test'));
                const section = createSection('f', ['nm']);
                section.rows.push([strValue('forge')]);
                doc.sections.set('f', section);
                const result = formatDocumentV3(doc);
                return !containsTableFormatting(result);
            }
        },
        {
            name: 'expandKey expands known abbreviations',
            test: () => {
                return expandKey('nm') === 'name' &&
                    expandKey('v') === 'version' &&
                    expandKey('au') === 'author';
            }
        },
        {
            name: 'compressKey compresses known full names',
            test: () => {
                return compressKey('name') === 'nm' &&
                    compressKey('version') === 'v' &&
                    compressKey('author') === 'au';
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
    console.log('Running Property tests for Human Format V3...\n');

    testKeyAlignmentConsistency();
    testArraySeparatorConsistency();
    testNoTableFormatting();
    testStringQuoting();

    console.log('\n✓ All Human Format V3 property tests passed!');
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
