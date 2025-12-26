/**
 * Property-based tests for LLM Parser
 * 
 * Feature: dx-serializer-extension-fix, Property 1: LLM to Human to LLM Round-Trip
 * 
 * For any valid LLM format content, parsing and re-serializing should preserve
 * the semantic content (though formatting may differ).
 * 
 * **Validates: Requirements 1.1-1.9**
 */

import * as fc from 'fast-check';
import {
    parseLlm,
    parseValue,
    parseContextSection,
    parseRefDefinition,
    parseDataSectionHeader,
    parseDataRow,
    DxValue,
    DxDocument,
    strValue,
    numValue,
    boolValue,
    nullValue,
    arrValue,
    refValue,
    createDocument,
    createSection,
} from './llmParser';

// ============================================================================
// Generators for LLM content
// ============================================================================

/**
 * Generate a valid key (alphanumeric with underscores)
 */
const validKey = fc.stringOf(
    fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz0123456789_'),
    { minLength: 1, maxLength: 10 }
).filter((s: string) => /^[a-z]/.test(s));

/**
 * Generate a simple string value (no special characters, must start with letter)
 */
const simpleString = fc.stringOf(
    fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'),
    { minLength: 2, maxLength: 15 }
).filter((s: string) => /^[a-zA-Z]/.test(s)); // Must start with letter to avoid being parsed as number

/**
 * Generate a number value
 */
const numberValue = fc.oneof(
    fc.integer({ min: -10000, max: 10000 }),
    fc.float({ min: -1000, max: 1000, noNaN: true, noDefaultInfinity: true })
        .map((n: number) => Math.round(n * 100) / 100)
);

/**
 * Generate a simple LLM value (string, number, bool, null)
 */
const simpleLlmValue = fc.oneof(
    simpleString,
    numberValue.map((n: number) => n.toString()),
    fc.constant('+'),
    fc.constant('-'),
    fc.constant('~'),
);

// ============================================================================
// Property Tests
// ============================================================================

/**
 * Property 1.1: parseValue correctly parses boolean values
 */
export function testParseValueBooleans(): void {
    // Test true
    const trueResult = parseValue('+');
    if (trueResult.type !== 'bool' || trueResult.value !== true) {
        throw new Error(`Expected bool true, got ${JSON.stringify(trueResult)}`);
    }

    // Test false
    const falseResult = parseValue('-');
    if (falseResult.type !== 'bool' || falseResult.value !== false) {
        throw new Error(`Expected bool false, got ${JSON.stringify(falseResult)}`);
    }

    console.log('✓ Property 1.1: parseValue correctly parses boolean values');
}

/**
 * Property 1.2: parseValue correctly parses null values
 */
export function testParseValueNull(): void {
    const result = parseValue('~');
    if (result.type !== 'null' || result.value !== null) {
        throw new Error(`Expected null, got ${JSON.stringify(result)}`);
    }

    console.log('✓ Property 1.2: parseValue correctly parses null values');
}

/**
 * Property 1.3: parseValue correctly parses numbers
 */
export function testParseValueNumbers(): void {
    fc.assert(
        fc.property(numberValue, (num: number) => {
            const result = parseValue(num.toString());
            if (result.type !== 'number') {
                throw new Error(`Expected number type, got ${result.type}`);
            }
            if (result.value !== num) {
                throw new Error(`Expected ${num}, got ${result.value}`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 1.3: parseValue correctly parses numbers');
}

/**
 * Property 1.4: parseValue correctly parses strings
 */
export function testParseValueStrings(): void {
    fc.assert(
        fc.property(simpleString, (str: string) => {
            const result = parseValue(str);
            if (result.type !== 'string') {
                throw new Error(`Expected string type, got ${result.type}`);
            }
            if (result.value !== str) {
                throw new Error(`Expected '${str}', got '${result.value}'`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 1.4: parseValue correctly parses strings');
}

/**
 * Property 1.5: parseValue correctly parses references
 */
export function testParseValueReferences(): void {
    fc.assert(
        fc.property(validKey, (key: string) => {
            const result = parseValue(`^${key}`);
            if (result.type !== 'ref') {
                throw new Error(`Expected ref type, got ${result.type}`);
            }
            if (result.refKey !== key) {
                throw new Error(`Expected refKey '${key}', got '${result.refKey}'`);
            }
            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 1.5: parseValue correctly parses references');
}

/**
 * Property 1.6: parseValue correctly parses arrays
 */
export function testParseValueArrays(): void {
    fc.assert(
        fc.property(
            fc.array(simpleString, { minLength: 1, maxLength: 5 }),
            (items: string[]) => {
                const arrayStr = '*' + items.join(',');
                const result = parseValue(arrayStr);
                if (result.type !== 'array') {
                    throw new Error(`Expected array type, got ${result.type}`);
                }
                const arr = result.value as DxValue[];
                if (arr.length !== items.length) {
                    throw new Error(`Expected ${items.length} items, got ${arr.length}`);
                }
                return true;
            }
        ),
        { numRuns: 50 }
    );
    console.log('✓ Property 1.6: parseValue correctly parses arrays');
}

/**
 * Property 1.7: parseContextSection parses key-value pairs
 */
export function testParseContextSection(): void {
    fc.assert(
        fc.property(
            fc.array(fc.tuple(validKey, simpleString), { minLength: 1, maxLength: 5 }),
            (pairs: [string, string][]) => {
                // Ensure unique keys
                const uniquePairs = new Map<string, string>();
                for (const [k, v] of pairs) {
                    uniquePairs.set(k, v);
                }

                const content = Array.from(uniquePairs.entries())
                    .map(([k, v]) => `${k}|${v}`)
                    .join(';');

                const result = parseContextSection(content);

                for (const [key, value] of uniquePairs) {
                    if (!result.has(key)) {
                        throw new Error(`Missing key '${key}' in result`);
                    }
                    const parsed = result.get(key)!;
                    if (parsed.value !== value) {
                        throw new Error(`Expected '${value}' for key '${key}', got '${parsed.value}'`);
                    }
                }
                return true;
            }
        ),
        { numRuns: 50 }
    );
    console.log('✓ Property 1.7: parseContextSection parses key-value pairs');
}

/**
 * Property 1.8: parseDataSectionHeader parses section headers
 */
export function testParseDataSectionHeader(): void {
    fc.assert(
        fc.property(
            fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz'),
            fc.array(validKey, { minLength: 1, maxLength: 5 }),
            (letter: string, schema: string[]) => {
                const header = `#${letter}(${schema.join('|')})`;
                const result = parseDataSectionHeader(header);

                if (!result) {
                    throw new Error(`Failed to parse header: ${header}`);
                }

                const [sectionId, parsedSchema] = result;
                if (sectionId !== letter) {
                    throw new Error(`Expected section ID '${letter}', got '${sectionId}'`);
                }
                if (parsedSchema.length !== schema.length) {
                    throw new Error(`Expected ${schema.length} columns, got ${parsedSchema.length}`);
                }
                return true;
            }
        ),
        { numRuns: 50 }
    );
    console.log('✓ Property 1.8: parseDataSectionHeader parses section headers');
}

/**
 * Property 1.9: parseLlm parses complete documents
 */
export function testParseLlmComplete(): void {
    // Test a complete document with new format (root-level key|value pairs)
    const input = `nm|Test
ct|42
#:A|CommonValue
#d(id|nm|ac)
1|Alice|+
2|Bob|-`;

    const result = parseLlm(input);

    if (!result.success) {
        throw new Error(`Parse failed: ${result.error?.message}`);
    }

    const doc = result.document!;

    // Check context
    if (!doc.context.has('nm')) {
        throw new Error('Missing context key "nm"');
    }
    if (!doc.context.has('ct')) {
        throw new Error('Missing context key "ct"');
    }

    // Check refs
    if (!doc.refs.has('A')) {
        throw new Error('Missing ref "A"');
    }

    // Check sections
    if (!doc.sections.has('d')) {
        throw new Error('Missing section "d"');
    }

    const section = doc.sections.get('d')!;
    if (section.schema.length !== 3) {
        throw new Error(`Expected 3 schema columns, got ${section.schema.length}`);
    }
    if (section.rows.length !== 2) {
        throw new Error(`Expected 2 rows, got ${section.rows.length}`);
    }

    console.log('✓ Property 1.9: parseLlm parses complete documents');
}

// ============================================================================
// Unit Tests
// ============================================================================

export function runUnitTests(): void {
    console.log('Running unit tests for LLM Parser...\n');

    let passed = 0;
    let failed = 0;

    const tests: Array<{ name: string; test: () => boolean }> = [
        {
            name: 'parseValue handles empty string',
            test: () => {
                const result = parseValue('');
                return result.type === 'string' && result.value === '';
            }
        },
        {
            name: 'parseValue handles quoted strings',
            test: () => {
                const result = parseValue('"hello world"');
                return result.type === 'string' && result.value === 'hello world';
            }
        },
        {
            name: 'parseValue handles single-quoted strings',
            test: () => {
                const result = parseValue("'hello'");
                return result.type === 'string' && result.value === 'hello';
            }
        },
        {
            name: 'parseRefDefinition parses valid refs',
            test: () => {
                const result = parseRefDefinition('A|CommonValue');
                return result !== null && result[0] === 'A' && result[1] === 'CommonValue';
            }
        },
        {
            name: 'parseRefDefinition returns null for invalid refs',
            test: () => {
                const result = parseRefDefinition('invalid');
                return result === null;
            }
        },
        {
            name: 'parseDataRow handles correct number of values',
            test: () => {
                const row = parseDataRow('a|b|c', 3);
                return row.length === 3;
            }
        },
        {
            name: 'parseDataRow pads missing values',
            test: () => {
                const row = parseDataRow('a|b', 4);
                return row.length === 4 && row[3].type === 'string' && row[3].value === '';
            }
        },
        {
            name: 'parseLlm handles empty input',
            test: () => {
                const result = parseLlm('');
                return result.success && result.document !== undefined;
            }
        },
        {
            name: 'parseLlm skips comments and parses legacy format',
            test: () => {
                // Legacy format with #c: prefix (still supported)
                const result = parseLlm('// comment\n#c:nm|Test');
                return result.success && result.document!.context.has('nm');
            }
        },
        {
            name: 'parseLlm parses new root-level format',
            test: () => {
                // New format: root-level key|value pairs
                const result = parseLlm('nm|Test\nv|1.0');
                return result.success &&
                    result.document!.context.has('nm') &&
                    result.document!.context.has('v');
            }
        },
        {
            name: 'parseLlm handles multiple sections',
            test: () => {
                const input = '#a(x|y)\n1|2\n#b(p|q)\n3|4';
                const result = parseLlm(input);
                return result.success &&
                    result.document!.sections.has('a') &&
                    result.document!.sections.has('b');
            }
        },
        {
            name: 'parseLlm reports error for invalid section header',
            test: () => {
                const result = parseLlm('#invalid(');
                return !result.success && result.error !== undefined;
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
    console.log('Running Property 1: LLM Parser tests...\n');

    testParseValueBooleans();
    testParseValueNull();
    testParseValueNumbers();
    testParseValueStrings();
    testParseValueReferences();
    testParseValueArrays();
    testParseContextSection();
    testParseDataSectionHeader();
    testParseLlmComplete();

    console.log('\n✓ All Property 1 tests passed!');
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
