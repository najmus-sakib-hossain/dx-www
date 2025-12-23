/**
 * Property-based tests for Human Formatter
 * 
 * Feature: dx-serializer-extension-fix, Property 6: Key Abbreviation Consistency
 * 
 * For any abbreviated key, expanding and then compressing should return
 * the original abbreviation. For any full key name, compressing and then
 * expanding should return the original name.
 * 
 * **Validates: Requirements 2.1-2.7, 6.1-6.9**
 */

import * as fc from 'fast-check';
import {
    ABBREVIATIONS,
    REVERSE_ABBREVIATIONS,
    expandKey,
    compressKey,
    formatValue,
    formatTableValue,
    formatSectionHeader,
    formatConfigSection,
    formatDataSection,
    generateSummary,
    formatDocument,
} from './humanFormatter';
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
    refValue,
} from './llmParser';

// ============================================================================
// Generators
// ============================================================================

/**
 * Generate a known abbreviation key
 */
const abbreviationKey = fc.constantFrom(...Object.keys(ABBREVIATIONS));

/**
 * Generate a known full key name
 */
const fullKeyName = fc.constantFrom(...Object.values(ABBREVIATIONS));

/**
 * Generate a simple string value
 */
const simpleString = fc.stringOf(
    fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'),
    { minLength: 1, maxLength: 15 }
);

// ============================================================================
// Property Tests
// ============================================================================

/**
 * Property 6.1: Expand then compress returns original abbreviation
 */
export function testExpandCompressRoundTrip(): void {
    fc.assert(
        fc.property(abbreviationKey, (abbrev: string) => {
            const expanded = expandKey(abbrev);
            const compressed = compressKey(expanded);
            if (compressed !== abbrev) {
                throw new Error(`Expected '${abbrev}', got '${compressed}' after expand('${expanded}')`);
            }
            return true;
        }),
        { numRuns: Object.keys(ABBREVIATIONS).length }
    );
    console.log('✓ Property 6.1: Expand then compress returns original abbreviation');
}

/**
 * Property 6.2: Compress then expand returns original full name
 */
export function testCompressExpandRoundTrip(): void {
    fc.assert(
        fc.property(fullKeyName, (full: string) => {
            const compressed = compressKey(full);
            const expanded = expandKey(compressed);
            if (expanded !== full) {
                throw new Error(`Expected '${full}', got '${expanded}' after compress('${compressed}')`);
            }
            return true;
        }),
        { numRuns: Object.values(ABBREVIATIONS).length }
    );
    console.log('✓ Property 6.2: Compress then expand returns original full name');
}

/**
 * Property 6.3: Unknown keys pass through unchanged
 */
export function testUnknownKeysPassThrough(): void {
    const unknownKey = fc.stringOf(
        fc.constantFrom(...'xyz'),
        { minLength: 3, maxLength: 8 }
    ).filter((s: string) => !ABBREVIATIONS[s] && !REVERSE_ABBREVIATIONS[s]);

    fc.assert(
        fc.property(unknownKey, (key: string) => {
            const expanded = expandKey(key);
            const compressed = compressKey(key);
            if (expanded !== key) {
                throw new Error(`expandKey should pass through unknown key '${key}', got '${expanded}'`);
            }
            if (compressed !== key) {
                throw new Error(`compressKey should pass through unknown key '${key}', got '${compressed}'`);
            }
            return true;
        }),
        { numRuns: 50 }
    );
    console.log('✓ Property 6.3: Unknown keys pass through unchanged');
}

/**
 * Property 2.1: formatValue handles all value types
 */
export function testFormatValueTypes(): void {
    // String
    const strResult = formatValue(strValue('hello'));
    if (strResult !== 'hello') {
        throw new Error(`Expected 'hello', got '${strResult}'`);
    }

    // Number
    const numResult = formatValue(numValue(42));
    if (numResult !== '42') {
        throw new Error(`Expected '42', got '${numResult}'`);
    }

    // Boolean
    const boolTrueResult = formatValue(boolValue(true));
    if (boolTrueResult !== 'true') {
        throw new Error(`Expected 'true', got '${boolTrueResult}'`);
    }

    const boolFalseResult = formatValue(boolValue(false));
    if (boolFalseResult !== 'false') {
        throw new Error(`Expected 'false', got '${boolFalseResult}'`);
    }

    // Null
    const nullResult = formatValue(nullValue());
    if (nullResult !== 'null') {
        throw new Error(`Expected 'null', got '${nullResult}'`);
    }

    // Array
    const arrResult = formatValue(arrValue([strValue('a'), strValue('b')]));
    if (arrResult !== '[a, b]') {
        throw new Error(`Expected '[a, b]', got '${arrResult}'`);
    }

    // Reference (without refs map)
    const refResult = formatValue(refValue('A'));
    if (refResult !== '^A') {
        throw new Error(`Expected '^A', got '${refResult}'`);
    }

    // Reference (with refs map)
    const refs = new Map([['A', 'ResolvedValue']]);
    const refResolvedResult = formatValue(refValue('A'), refs);
    if (refResolvedResult !== 'ResolvedValue') {
        throw new Error(`Expected 'ResolvedValue', got '${refResolvedResult}'`);
    }

    console.log('✓ Property 2.1: formatValue handles all value types');
}

/**
 * Property 2.3: formatTableValue uses special symbols
 */
export function testFormatTableValueSymbols(): void {
    // Boolean true -> ✓
    const trueResult = formatTableValue(boolValue(true));
    if (trueResult !== '✓') {
        throw new Error(`Expected '✓', got '${trueResult}'`);
    }

    // Boolean false -> ✗
    const falseResult = formatTableValue(boolValue(false));
    if (falseResult !== '✗') {
        throw new Error(`Expected '✗', got '${falseResult}'`);
    }

    // Null -> —
    const nullResult = formatTableValue(nullValue());
    if (nullResult !== '—') {
        throw new Error(`Expected '—', got '${nullResult}'`);
    }

    console.log('✓ Property 2.3: formatTableValue uses special symbols');
}

/**
 * Property 2.2: formatSectionHeader creates box-drawing headers
 */
export function testFormatSectionHeader(): void {
    const header = formatSectionHeader('TEST', 40);
    const lines = header.split('\n');

    if (lines.length !== 3) {
        throw new Error(`Expected 3 lines, got ${lines.length}`);
    }

    // Check that lines start with #
    for (const line of lines) {
        if (!line.startsWith('#')) {
            throw new Error(`Line should start with '#': ${line}`);
        }
    }

    // Check that header contains the title
    if (!header.includes('TEST')) {
        throw new Error(`Header should contain 'TEST': ${header}`);
    }

    // Check for box-drawing characters
    if (!header.includes('─')) {
        throw new Error(`Header should contain box-drawing characters: ${header}`);
    }

    console.log('✓ Property 2.2: formatSectionHeader creates box-drawing headers');
}

/**
 * Property 2.4: formatConfigSection expands keys
 */
export function testFormatConfigSectionExpandsKeys(): void {
    const context = new Map<string, DxValue>();
    context.set('nm', strValue('Test'));
    context.set('ct', numValue(42));

    const result = formatConfigSection(context);

    // Should contain expanded key names
    if (!result.includes('name')) {
        throw new Error(`Should contain expanded key 'name': ${result}`);
    }
    if (!result.includes('count')) {
        throw new Error(`Should contain expanded key 'count': ${result}`);
    }

    // Should contain values
    if (!result.includes('Test')) {
        throw new Error(`Should contain value 'Test': ${result}`);
    }
    if (!result.includes('42')) {
        throw new Error(`Should contain value '42': ${result}`);
    }

    console.log('✓ Property 2.4: formatConfigSection expands keys');
}

/**
 * Property 2.5: formatDataSection creates Unicode tables
 */
export function testFormatDataSectionCreatesTable(): void {
    const section = createSection('d', ['id', 'nm', 'ac']);
    section.rows.push([numValue(1), strValue('Alice'), boolValue(true)]);
    section.rows.push([numValue(2), strValue('Bob'), boolValue(false)]);

    const result = formatDataSection(section);

    // Should contain section header
    if (!result.includes('[d]')) {
        throw new Error(`Should contain section header '[d]': ${result}`);
    }

    // Should contain box-drawing characters
    if (!result.includes('┌') || !result.includes('┐') || !result.includes('└') || !result.includes('┘')) {
        throw new Error(`Should contain box corners: ${result}`);
    }
    if (!result.includes('│')) {
        throw new Error(`Should contain vertical lines: ${result}`);
    }

    // Should contain data
    if (!result.includes('Alice') || !result.includes('Bob')) {
        throw new Error(`Should contain data values: ${result}`);
    }

    // Should use checkmarks for booleans
    if (!result.includes('✓') || !result.includes('✗')) {
        throw new Error(`Should use checkmarks for booleans: ${result}`);
    }

    console.log('✓ Property 2.5: formatDataSection creates Unicode tables');
}

/**
 * Property 2.7: generateSummary calculates totals
 */
export function testGenerateSummaryCalculatesTotals(): void {
    const section = createSection('d', ['id', 'pr', 'ac']);
    section.rows.push([numValue(1), numValue(100), boolValue(true)]);
    section.rows.push([numValue(2), numValue(200), boolValue(true)]);
    section.rows.push([numValue(3), numValue(150), boolValue(false)]);

    const summary = generateSummary(section);

    // Should contain total count
    if (!summary.includes('Total: 3 items')) {
        throw new Error(`Should contain 'Total: 3 items': ${summary}`);
    }

    // Should contain numeric total (100 + 200 + 150 = 450)
    if (!summary.includes('450')) {
        throw new Error(`Should contain numeric total '450': ${summary}`);
    }

    // Should contain boolean count (2/3 true)
    if (!summary.includes('2/3')) {
        throw new Error(`Should contain boolean count '2/3': ${summary}`);
    }

    console.log('✓ Property 2.7: generateSummary calculates totals');
}

// ============================================================================
// Unit Tests
// ============================================================================

export function runUnitTests(): void {
    console.log('Running unit tests for Human Formatter...\n');

    let passed = 0;
    let failed = 0;

    const tests: Array<{ name: string; test: () => boolean }> = [
        {
            name: 'expandKey expands known abbreviations',
            test: () => {
                return expandKey('nm') === 'name' &&
                    expandKey('ct') === 'count' &&
                    expandKey('em') === 'email';
            }
        },
        {
            name: 'compressKey compresses known full names',
            test: () => {
                return compressKey('name') === 'nm' &&
                    compressKey('count') === 'ct' &&
                    compressKey('email') === 'em';
            }
        },
        {
            name: 'formatValue handles nested arrays',
            test: () => {
                const nested = arrValue([arrValue([strValue('a'), strValue('b')]), strValue('c')]);
                const result = formatValue(nested);
                return result === '[[a, b], c]';
            }
        },
        {
            name: 'formatConfigSection handles empty context',
            test: () => {
                const result = formatConfigSection(new Map());
                return result === '';
            }
        },
        {
            name: 'formatConfigSection quotes strings with special chars',
            test: () => {
                const context = new Map<string, DxValue>();
                context.set('nm', strValue('hello world'));
                const result = formatConfigSection(context);
                return result.includes('"hello world"');
            }
        },
        {
            name: 'formatDataSection handles empty schema',
            test: () => {
                const section = createSection('d', []);
                const result = formatDataSection(section);
                return result.includes('[d]') && !result.includes('┌');
            }
        },
        {
            name: 'formatDocument combines all sections',
            test: () => {
                const doc = createDocument();
                doc.context.set('nm', strValue('Test'));
                const section = createSection('d', ['id']);
                section.rows.push([numValue(1)]);
                doc.sections.set('d', section);

                const result = formatDocument(doc);
                return result.includes('CONFIG') && result.includes('DATA');
            }
        },
        {
            name: 'generateSummary handles empty section',
            test: () => {
                const section = createSection('d', ['id']);
                const summary = generateSummary(section);
                return summary.includes('Total: 0 items');
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
    console.log('Running Property 6: Key Abbreviation Consistency tests...\n');

    testExpandCompressRoundTrip();
    testCompressExpandRoundTrip();
    testUnknownKeysPassThrough();
    testFormatValueTypes();
    testFormatTableValueSymbols();
    testFormatSectionHeader();
    testFormatConfigSectionExpandsKeys();
    testFormatDataSectionCreatesTable();
    testGenerateSummaryCalculatesTotals();

    console.log('\n✓ All Property 6 tests passed!');
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
