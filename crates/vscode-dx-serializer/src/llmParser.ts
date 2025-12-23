/**
 * LLM Format Parser for DX Serializer VS Code Extension
 * 
 * Parses the token-optimized LLM format with sigils (#c, #:, #<letter>)
 * into an internal DxDocument representation.
 * 
 * LLM Format Syntax:
 * - #c:key|value;key|value  - Context section
 * - #:ref|value             - Reference definition
 * - #<letter>(schema)       - Data section header
 * - row1|row2|row3          - Data rows (pipe-separated)
 * 
 * Value Types:
 * - String: plain text or quoted
 * - Number: integer or decimal
 * - Boolean: + (true) or - (false)
 * - Null: ~
 * - Array: *item1,item2,item3
 * - Reference: ^key
 * 
 * Requirements: 1.1-1.9
 */

// ============================================================================
// Types and Interfaces
// ============================================================================

/**
 * Value types in DX LLM format
 */
export type DxValueType = 'string' | 'number' | 'bool' | 'null' | 'array' | 'ref';

/**
 * A value in the DX document
 */
export interface DxValue {
    type: DxValueType;
    value: string | number | boolean | null | DxValue[];
    refKey?: string; // For reference values
}

/**
 * A data section with schema and rows
 */
export interface DxSection {
    id: string;
    schema: string[];
    rows: DxValue[][];
}

/**
 * The complete DX document representation
 */
export interface DxDocument {
    context: Map<string, DxValue>;
    refs: Map<string, string>;
    sections: Map<string, DxSection>;
}

/**
 * Parse error with location information
 */
export interface ParseError {
    message: string;
    line: number;
    column: number;
    hint?: string;
}

/**
 * Parse result
 */
export interface ParseResult {
    success: boolean;
    document?: DxDocument;
    error?: ParseError;
}

// ============================================================================
// Value Constructors
// ============================================================================

export function strValue(s: string): DxValue {
    return { type: 'string', value: s };
}

export function numValue(n: number): DxValue {
    return { type: 'number', value: n };
}

export function boolValue(b: boolean): DxValue {
    return { type: 'bool', value: b };
}

export function nullValue(): DxValue {
    return { type: 'null', value: null };
}

export function arrValue(items: DxValue[]): DxValue {
    return { type: 'array', value: items };
}

export function refValue(key: string): DxValue {
    return { type: 'ref', value: key, refKey: key };
}

// ============================================================================
// Document Constructor
// ============================================================================

export function createDocument(): DxDocument {
    return {
        context: new Map(),
        refs: new Map(),
        sections: new Map(),
    };
}

export function createSection(id: string, schema: string[]): DxSection {
    return {
        id,
        schema,
        rows: [],
    };
}

// ============================================================================
// LLM Parser Implementation
// ============================================================================

/**
 * Parse a single value from LLM format
 */
export function parseValue(raw: string): DxValue {
    const trimmed = raw.trim();

    // Empty string
    if (trimmed === '') {
        return strValue('');
    }

    // Boolean: + for true, - for false
    if (trimmed === '+') {
        return boolValue(true);
    }
    if (trimmed === '-') {
        return boolValue(false);
    }

    // Null: ~
    if (trimmed === '~') {
        return nullValue();
    }

    // Reference: ^key
    if (trimmed.startsWith('^')) {
        return refValue(trimmed.substring(1));
    }

    // Array: *item1,item2,item3
    if (trimmed.startsWith('*')) {
        const items = trimmed.substring(1).split(',').map(item => parseValue(item.trim()));
        return arrValue(items);
    }

    // Number: try to parse as number
    const num = parseFloat(trimmed);
    if (!isNaN(num) && isFinite(num) && /^-?\d+(\.\d+)?$/.test(trimmed)) {
        return numValue(num);
    }

    // Quoted string: remove quotes
    if ((trimmed.startsWith('"') && trimmed.endsWith('"')) ||
        (trimmed.startsWith("'") && trimmed.endsWith("'"))) {
        return strValue(trimmed.slice(1, -1));
    }

    // Plain string
    return strValue(trimmed);
}

/**
 * Parse context section: #c:key|value;key|value
 */
export function parseContextSection(content: string): Map<string, DxValue> {
    const context = new Map<string, DxValue>();

    if (!content.trim()) {
        return context;
    }

    // Split by semicolon for multiple key-value pairs
    const pairs = content.split(';');

    for (const pair of pairs) {
        const trimmed = pair.trim();
        if (!trimmed) continue;

        // Split by pipe for key|value
        const pipeIdx = trimmed.indexOf('|');
        if (pipeIdx === -1) continue;

        const key = trimmed.substring(0, pipeIdx).trim();
        const rawValue = trimmed.substring(pipeIdx + 1);

        if (key) {
            context.set(key, parseValue(rawValue));
        }
    }

    return context;
}

/**
 * Parse reference definition: #:key|value
 */
export function parseRefDefinition(content: string): [string, string] | null {
    const pipeIdx = content.indexOf('|');
    if (pipeIdx === -1) return null;

    const key = content.substring(0, pipeIdx).trim();
    const value = content.substring(pipeIdx + 1).trim();

    if (!key) return null;

    return [key, value];
}

/**
 * Parse data section header: #<letter>(schema)
 * Returns [sectionId, schema] or null if invalid
 */
export function parseDataSectionHeader(line: string): [string, string[]] | null {
    // Match #<letter>(schema)
    const match = line.match(/^#([a-zA-Z])\(([^)]*)\)$/);
    if (!match) return null;

    const sectionId = match[1];
    const schemaStr = match[2];

    // Parse schema columns (pipe-separated)
    const schema = schemaStr.split('|').map(col => col.trim()).filter(col => col);

    return [sectionId, schema];
}

/**
 * Parse a data row (pipe-separated values)
 */
export function parseDataRow(line: string, schemaLength: number): DxValue[] {
    const values = line.split('|');
    const row: DxValue[] = [];

    for (let i = 0; i < schemaLength; i++) {
        const raw = i < values.length ? values[i] : '';
        row.push(parseValue(raw));
    }

    return row;
}

/**
 * Parse complete LLM format content into a DxDocument
 */
export function parseLlm(input: string): ParseResult {
    const doc = createDocument();
    const lines = input.split('\n');

    let currentSection: DxSection | null = null;
    let lineNum = 0;

    for (const line of lines) {
        lineNum++;
        const trimmed = line.trim();

        // Skip empty lines
        if (!trimmed) {
            continue;
        }

        // Skip comments
        if (trimmed.startsWith('//')) {
            continue;
        }

        // Context section: #c:...
        if (trimmed.startsWith('#c:')) {
            const content = trimmed.substring(3);
            const contextMap = parseContextSection(content);
            for (const [key, value] of contextMap) {
                doc.context.set(key, value);
            }
            currentSection = null;
            continue;
        }

        // Reference definition: #:...
        if (trimmed.startsWith('#:')) {
            const content = trimmed.substring(2);
            const ref = parseRefDefinition(content);
            if (ref) {
                doc.refs.set(ref[0], ref[1]);
            }
            currentSection = null;
            continue;
        }

        // Data section header: #<letter>(schema)
        if (trimmed.startsWith('#') && trimmed.includes('(')) {
            const header = parseDataSectionHeader(trimmed);
            if (header) {
                const [sectionId, schema] = header;
                currentSection = createSection(sectionId, schema);
                doc.sections.set(sectionId, currentSection);
                continue;
            } else {
                return {
                    success: false,
                    error: {
                        message: `Invalid section header: ${trimmed}`,
                        line: lineNum,
                        column: 1,
                        hint: 'Section headers should be in format #<letter>(col1|col2|col3)',
                    },
                };
            }
        }

        // Data row (if we're in a section)
        if (currentSection) {
            const row = parseDataRow(trimmed, currentSection.schema.length);
            currentSection.rows.push(row);
            continue;
        }

        // Unknown line format
        if (trimmed.startsWith('#')) {
            return {
                success: false,
                error: {
                    message: `Unknown sigil in line: ${trimmed}`,
                    line: lineNum,
                    column: 1,
                    hint: 'Valid sigils are #c: (context), #: (reference), #<letter>( (data section)',
                },
            };
        }
    }

    return {
        success: true,
        document: doc,
    };
}
