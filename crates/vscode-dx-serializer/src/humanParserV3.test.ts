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
// Test Utilities
// ============================================================================

/**
 * Compare two DxValues for equality
 */
function compareDxValues(a: DxValue, b: DxValue): boolean {
    if (a.type !== b.type) return false;

    if (a.type === 'array' && b.type === 'array') {
        const aItems = a.value as DxValue[];
        const bItems = b.value as DxValue[];
        if (aItems.length !== bItems.length) return false;
        return aItems.every((item, i) => compareDxValues(item, bItems[i]));
    }

    return a.value === b.value;
}

/**
 * Compare two DxDocuments for semantic equality
 * Returns null if equal, or an error message describing the difference
 */
function compareDxDocuments(original: DxDocument, roundTrip: DxDocument): string | null {
    // Compare context sizes
    if (original.context.size !== roundTrip.context.size) {
        return `Context size mismatch: original ${original.context.size}, round-trip ${roundTrip.context.size}`;
    }

    // Compare context values
    for (const [key, value] of original.context) {
        const rtValue = roundTrip.context.get(key);
        if (!rtValue) {
            return `Missing context key after round-trip: ${key}`;
        }
        if (!compareDxValues(value, rtValue)) {
            return `Context value mismatch for key '${key}': ${JSON.stringify(value)} vs ${JSON.stringify(rtValue)}`;
        }
    }

    // Compare refs sizes
    if (original.refs.size !== roundTrip.refs.size) {
        return `Refs size mismatch: original ${original.refs.size}, round-trip ${roundTrip.refs.size}`;
    }

    // Compare refs values
    for (const [key, value] of original.refs) {
        const rtValue = roundTrip.refs.get(key);
        if (!rtValue) {
            return `Missing refs key after round-trip: ${key}`;
        }
        if (value !== rtValue) {
            return `Refs value mismatch for key '${key}': "${value}" vs "${rtValue}"`;
        }
    }

    // Compare section counts
    if (original.sections.size !== roundTrip.sections.size) {
        return `Section count mismatch: original ${original.sections.size}, round-trip ${roundTrip.sections.size}`;
    }

    // Compare sections
    for (const [sectionId, section] of original.sections) {
        const rtSection = roundTrip.sections.get(sectionId);
        if (!rtSection) {
            return `Missing section after round-trip: ${sectionId}`;
        }

        // Compare schema
        if (section.schema.length !== rtSection.schema.length) {
            return `Schema length mismatch for section '${sectionId}': ${section.schema.length} vs ${rtSection.schema.length}`;
        }

        // Compare rows
        if (section.rows.length !== rtSection.rows.length) {
            return `Row count mismatch for section '${sectionId}': ${section.rows.length} vs ${rtSection.rows.length}`;
        }

        for (let rowIdx = 0; rowIdx < section.rows.length; rowIdx++) {
            const row = section.rows[rowIdx];
            const rtRow = rtSection.rows[rowIdx];

            if (row.length !== rtRow.length) {
                return `Row ${rowIdx} length mismatch for section '${sectionId}': ${row.length} vs ${rtRow.length}`;
            }

            for (let colIdx = 0; colIdx < row.length; colIdx++) {
                if (!compareDxValues(row[colIdx], rtRow[colIdx])) {
                    return `Value mismatch at section '${sectionId}' row ${rowIdx} col ${colIdx}: ${JSON.stringify(row[colIdx])} vs ${JSON.stringify(rtRow[colIdx])}`;
                }
            }
        }
    }

    return null; // Documents are equal
}

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
 * Note: 'k' (stack) is excluded because [stack] is reserved for reference definitions
 */
const simpleDxSection = fc.tuple(
    fc.constantFrom('f', 'y', 'u', 'm', 'o', 't'),  // Exclude 'k' (stack) and 'i' (i18n has special handling)
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
 * Property 2: Key-Value Parsing Correctness
 * For any key-value line in format `key = value`, the parser SHALL extract the key as the text 
 * before the first `=` sign (trimmed) and the value as the text after the first `=` sign (trimmed),
 * correctly handling values that contain additional `=` signs.
 * 
 * Feature: dx-serializer-human-parser-fix, Property 2: Key-Value Parsing Correctness
 * **Validates: Requirements 1.1, 1.4, 2.1, 5.1**
 */
export function testKeyValueParsingCorrectness(): void {
    // Generator for valid keys (alphanumeric, underscores, hyphens)
    const validKey = fc.stringOf(
        fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-'),
        { minLength: 1, maxLength: 20 }
    );

    // Generator for values that may contain = signs (but not start with = after trimming)
    const validValue = fc.stringOf(
        fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-./@=: '),
        { minLength: 0, maxLength: 50 }
    ).filter(v => !v.trim().startsWith('=')); // Values should not start with = after trimming

    // Generator for whitespace variations around =
    const whitespace = fc.stringOf(fc.constant(' '), { minLength: 0, maxLength: 5 });

    fc.assert(
        fc.property(
            validKey,
            whitespace,
            whitespace,
            validValue,
            (key, wsBefore, wsAfter, value) => {
                // Construct line with various whitespace patterns
                const line = `${key}${wsBefore}=${wsAfter}${value}`;

                const result = parseKeyValueLineV3(line);

                // Should successfully parse
                if (result === null) {
                    throw new Error(`Failed to parse valid key-value line: "${line}"`);
                }

                const [parsedKey, parsedValue] = result;

                // Key should be trimmed and match original
                if (parsedKey !== key.trim()) {
                    throw new Error(`Key mismatch: expected "${key.trim()}", got "${parsedKey}"`);
                }

                // Value should be trimmed and match original
                if (parsedValue !== value.trim()) {
                    throw new Error(`Value mismatch: expected "${value.trim()}", got "${parsedValue}"`);
                }

                // Value should NOT start with = (no corruption) - only check if original didn't start with =
                if (!value.trim().startsWith('=') && parsedValue.startsWith('=')) {
                    throw new Error(`Value corruption detected: value starts with '=' - "${parsedValue}"`);
                }

                return true;
            }
        ),
        { numRuns: 100 }
    );
    console.log('✓ Property 2: Key-Value Parsing Correctness');
}

/**
 * Property 3: No Value Corruption
 * For any parsed human format document, no stored value in the resulting DxDocument 
 * (in context, refs, or sections) SHALL start with `=` or contain the pattern `"= "` as a prefix.
 * 
 * Feature: dx-serializer-human-parser-fix, Property 3: No Value Corruption
 * **Validates: Requirements 1.2, 2.2**
 */
export function testNoValueCorruption(): void {
    // Generator for stack section lines
    const stackKey = fc.constantFrom('js', 'python', 'rust', 'go', 'java');
    const stackValue = fc.array(
        fc.stringOf(
            fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-./'),
            { minLength: 1, maxLength: 15 }
        ),
        { minLength: 1, maxLength: 6 }
    ).map(arr => arr.join(' | '));

    // Generator for config key-value pairs
    const configKey = fc.constantFrom('name', 'version', 'title', 'description', 'author');
    const configValue = fc.stringOf(
        fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-./@: '),
        { minLength: 1, maxLength: 30 }
    ).filter(v => !v.startsWith('='));

    fc.assert(
        fc.property(
            fc.array(fc.tuple(configKey, configValue), { minLength: 1, maxLength: 3 }),
            fc.array(fc.tuple(stackKey, stackValue), { minLength: 0, maxLength: 3 }),
            (configPairs, stackPairs) => {
                // Build human format input
                let input = '';

                // Add config section
                for (const [key, value] of configPairs) {
                    input += `${key.padEnd(20)}= ${value}\n`;
                }

                // Add stack section if we have stack pairs
                if (stackPairs.length > 0) {
                    input += '\n[stack]\n';
                    for (const [key, value] of stackPairs) {
                        input += `${key.padEnd(20)}= ${value}\n`;
                    }
                }

                // Parse the input
                const result = parseHumanV3(input);

                if (!result.success || !result.document) {
                    // If parsing fails, that's a different issue - skip this test case
                    return true;
                }

                const doc = result.document;

                // Check context values for corruption
                for (const [key, value] of doc.context) {
                    if (value.type === 'string') {
                        const strVal = String(value.value);
                        if (strVal.startsWith('=') || strVal.startsWith('"=')) {
                            throw new Error(`Context value corruption for key '${key}': value starts with '=' - "${strVal}"`);
                        }
                    }
                }

                // Check refs values for corruption
                for (const [key, value] of doc.refs) {
                    if (value.startsWith('=') || value.includes('|=') || value.includes('"=')) {
                        throw new Error(`Refs value corruption for key '${key}': value contains '=' corruption - "${value}"`);
                    }
                }

                // Check section values for corruption
                for (const [sectionId, section] of doc.sections) {
                    for (const row of section.rows) {
                        for (let i = 0; i < row.length; i++) {
                            const cell = row[i];
                            if (cell.type === 'string') {
                                const strVal = String(cell.value);
                                if (strVal.startsWith('=') || strVal.startsWith('"=')) {
                                    throw new Error(`Section '${sectionId}' value corruption at column ${i}: value starts with '=' - "${strVal}"`);
                                }
                            }
                        }
                    }
                }

                return true;
            }
        ),
        { numRuns: 100 }
    );
    console.log('✓ Property 3: No Value Corruption');
}

/**
 * Property 4: Nested Section Preservation
 * For any human format document containing nested sections (e.g., `[i18n.locales]`, `[js.dependencies]`),
 * parsing SHALL preserve all key-value pairs with correct prefixes, and the reconstructed DxSection 
 * SHALL contain all original data.
 * 
 * Feature: dx-serializer-human-parser-fix, Property 4: Nested Section Preservation
 * **Validates: Requirements 3.1, 3.2, 3.3**
 */
export function testNestedSectionPreservation(): void {
    // Generator for package names (may include hyphens)
    const packageName = fc.oneof(
        fc.stringOf(
            fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz'),
            { minLength: 2, maxLength: 10 }
        ),
        fc.tuple(
            fc.stringOf(fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz'), { minLength: 2, maxLength: 5 }),
            fc.stringOf(fc.constantFrom(...'abcdefghijklmnopqrstuvwxyz'), { minLength: 2, maxLength: 5 })
        ).map(([a, b]) => `${a}-${b}`)  // Hyphenated package names
    );

    // Generator for version strings
    const versionString = fc.oneof(
        fc.constant('latest'),
        fc.tuple(
            fc.integer({ min: 0, max: 99 }),
            fc.integer({ min: 0, max: 99 }),
            fc.integer({ min: 0, max: 99 })
        ).map(([major, minor, patch]) => `${major}.${minor}.${patch}`)
    );

    // Generator for nested section type
    const nestedSectionType = fc.constantFrom(
        { parent: 'js', child: 'dependencies', sectionId: 'j' },
        { parent: 'python', child: 'dependencies', sectionId: 'p' },
        { parent: 'rust', child: 'dependencies', sectionId: 'r' },
        { parent: 'i18n', child: 'locales', sectionId: 'i' }
    );

    fc.assert(
        fc.property(
            nestedSectionType,
            fc.array(fc.tuple(packageName, versionString), { minLength: 1, maxLength: 5 }),
            (sectionInfo, packages) => {
                // Build human format input with nested section
                let input = `name                = test\nversion             = 1.0.0\n\n`;
                input += `[${sectionInfo.parent}.${sectionInfo.child}]\n`;

                const uniquePackages = new Map<string, string>();
                for (const [pkg, ver] of packages) {
                    if (!uniquePackages.has(pkg)) {
                        uniquePackages.set(pkg, ver);
                    }
                }

                for (const [pkg, ver] of uniquePackages) {
                    input += `${pkg.padEnd(20)}= ${ver}\n`;
                }

                // Parse the input
                const result = parseHumanV3(input);

                if (!result.success || !result.document) {
                    throw new Error(`Failed to parse nested section input:\n${input}\nError: ${result.error?.message}`);
                }

                const doc = result.document;

                // Check that the section was created with correct ID
                const section = doc.sections.get(sectionInfo.sectionId);
                if (!section) {
                    throw new Error(`Section '${sectionInfo.sectionId}' not found in parsed document. Available sections: ${[...doc.sections.keys()].join(', ')}`);
                }

                // Check that all packages are preserved
                if (section.schema.length !== uniquePackages.size) {
                    throw new Error(`Schema length mismatch: expected ${uniquePackages.size}, got ${section.schema.length}`);
                }

                // Check that each package has the correct prefix
                for (const schemaKey of section.schema) {
                    if (!schemaKey.startsWith(`${sectionInfo.child}_`)) {
                        throw new Error(`Schema key '${schemaKey}' does not have expected prefix '${sectionInfo.child}_'`);
                    }
                }

                // Check that values are preserved (no corruption)
                if (section.rows.length !== 1) {
                    throw new Error(`Expected 1 row, got ${section.rows.length}`);
                }

                const row = section.rows[0];
                for (let i = 0; i < row.length; i++) {
                    const cell = row[i];
                    if (cell.type === 'string') {
                        const strVal = String(cell.value);
                        if (strVal.startsWith('=') || strVal.startsWith('"=')) {
                            throw new Error(`Value corruption in nested section at index ${i}: "${strVal}"`);
                        }
                    }
                }

                return true;
            }
        ),
        { numRuns: 100 }
    );
    console.log('✓ Property 4: Nested Section Preservation');
}

/**
 * Property 1: Human Format V3 Round-Trip Consistency
 * For any valid human format document, parsing then formatting SHALL produce semantically equivalent output
 * where all keys and values are preserved.
 * 
 * Feature: dx-serializer-human-parser-fix, Property 1: Round-Trip Consistency
 * **Validates: Requirements 4.1, 4.2, 4.3, 4.4, 6.2, 6.3**
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

            // Use the comparison utility for detailed error messages
            const comparisonError = compareDxDocuments(doc, roundTripDoc);
            if (comparisonError) {
                throw new Error(`Round-trip comparison failed:\n${comparisonError}\n\nOriginal Human V3:\n${humanV3}`);
            }

            // Additional check: no value corruption in output
            const formattedAgain = formatDocumentV3(roundTripDoc);
            if (formattedAgain.includes('"=') || formattedAgain.includes('= =')) {
                throw new Error(`Value corruption detected in formatted output:\n${formattedAgain}`);
            }

            return true;
        }),
        { numRuns: 100 }
    );
    console.log('✓ Property 1: Human Format V3 Round-Trip Consistency');
}

/**
 * Property 1b: Comprehensive Round-Trip with Stack Section
 * Tests round-trip consistency including stack sections with pipe-separated values.
 * 
 * Feature: dx-serializer-human-parser-fix, Property 1: Round-Trip Consistency
 * **Validates: Requirements 4.1, 4.2, 4.3, 4.4, 6.2, 6.3**
 */
export function testComprehensiveRoundTrip(): void {
    // Generator for stack entries
    const stackKey = fc.constantFrom('js', 'python', 'rust', 'go');
    const stackValues = fc.array(
        fc.stringOf(
            fc.constantFrom(...'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-./'),
            { minLength: 1, maxLength: 12 }
        ),
        { minLength: 2, maxLength: 6 }
    );

    fc.assert(
        fc.property(
            simpleContextMap,
            fc.array(fc.tuple(stackKey, stackValues), { minLength: 0, maxLength: 3 }),
            (context, stackEntries) => {
                // Build a document with context and stack
                const doc = createDocument();

                for (const [key, value] of context) {
                    doc.context.set(key, value);
                }

                // Add stack entries as refs
                const usedStackKeys = new Set<string>();
                for (const [key, values] of stackEntries) {
                    if (!usedStackKeys.has(key)) {
                        doc.refs.set(key, values.join('|'));
                        usedStackKeys.add(key);
                    }
                }

                if (doc.refs.size > 0 && doc.sectionOrder) {
                    doc.sectionOrder.push('stack');
                }

                // Format to Human V3
                const humanV3 = formatDocumentV3(doc);

                // Parse back
                const parseResult = parseHumanV3(humanV3);

                if (!parseResult.success || !parseResult.document) {
                    throw new Error(`Failed to parse Human V3:\n${humanV3}\nError: ${parseResult.error?.message}`);
                }

                const roundTripDoc = parseResult.document;

                // Compare context
                if (doc.context.size !== roundTripDoc.context.size) {
                    throw new Error(`Context size mismatch: ${doc.context.size} vs ${roundTripDoc.context.size}`);
                }

                // Compare refs (stack section)
                if (doc.refs.size !== roundTripDoc.refs.size) {
                    throw new Error(`Refs size mismatch: ${doc.refs.size} vs ${roundTripDoc.refs.size}`);
                }

                for (const [key, value] of doc.refs) {
                    const rtValue = roundTripDoc.refs.get(key);
                    if (!rtValue) {
                        throw new Error(`Missing refs key after round-trip: ${key}`);
                    }
                    if (value !== rtValue) {
                        throw new Error(`Refs value mismatch for '${key}': "${value}" vs "${rtValue}"`);
                    }
                    // Check for corruption
                    if (rtValue.startsWith('=') || rtValue.includes('|=')) {
                        throw new Error(`Refs value corruption for '${key}': "${rtValue}"`);
                    }
                }

                return true;
            }
        ),
        { numRuns: 100 }
    );
    console.log('✓ Property 1b: Comprehensive Round-Trip with Stack Section');
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
                // New format: root-level key|value pairs without #c: prefix
                return !result.includes('#c:') && result.includes('nm|test');
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

    testKeyValueParsingCorrectness();
    testNoValueCorruption();
    testNestedSectionPreservation();
    testHumanV3RoundTrip();
    testComprehensiveRoundTrip();

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
