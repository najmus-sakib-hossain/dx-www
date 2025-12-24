/**
 * Human Format V3 Parser for DX Serializer VS Code Extension
 * 
 * Parses the vertical key-value human format back to DxDocument:
 * - Config values without section header (key = value at start)
 * - [section] headers for data sections
 * - [section] = Schema | Headers for multi-row sections
 * - Pipe (|) separated arrays
 * - Quoted strings
 * 
 * Requirements: 3.1
 */

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
import { compressKey, compressSectionName, REVERSE_SECTION_NAMES } from './humanFormatterV3';

// ============================================================================
// Types
// ============================================================================

export interface HumanParseResultV3 {
    success: boolean;
    document?: DxDocument;
    sectionOrder?: string[];
    error?: {
        message: string;
        line: number;
        column: number;
        hint?: string;
    };
}

// Extended document with section order tracking
export interface DxDocumentWithOrder extends DxDocument {
    sectionOrder: string[];
}

// Nested section info for tracking parent.child relationships
export interface NestedSectionInfo {
    parent: string;      // e.g., 'js', 'python', 'rust'
    child: string;       // e.g., 'dependencies'
    fullName: string;    // e.g., 'js.dependencies'
}

// Map of parent section IDs to their nested section info
const NESTED_SECTION_PARENTS: Record<string, string> = {
    'js': 'j',
    'python': 'p',
    'rust': 'r',
    'i18n': 'i',
};

// ============================================================================
// Value Parsing
// ============================================================================

export function parseValueV3(raw: string): DxValue {
    const trimmed = raw.trim();
    if (trimmed === '') return strValue('');
    if (trimmed === '-' || trimmed === '~') return nullValue();
    if (trimmed.toLowerCase() === 'true') return boolValue(true);
    if (trimmed.toLowerCase() === 'false') return boolValue(false);
    if ((trimmed.startsWith('"') && trimmed.endsWith('"')) ||
        (trimmed.startsWith("'") && trimmed.endsWith("'"))) {
        return strValue(trimmed.slice(1, -1));
    }
    const num = parseFloat(trimmed);
    if (!isNaN(num) && isFinite(num) && /^-?\d+(\.\d+)?$/.test(trimmed)) {
        return numValue(num);
    }
    return strValue(trimmed);
}

export function parseArrayValueV3(raw: string): DxValue {
    const trimmed = raw.trim();
    if (trimmed.includes(' | ')) {
        const items = trimmed.split(' | ').map(item => parseValueV3(item.trim()));
        return arrValue(items);
    }
    if (trimmed.includes('|') && !trimmed.includes(' | ')) {
        const items = trimmed.split('|').map(item => parseValueV3(item.trim()));
        if (items.length > 1) return arrValue(items);
    }
    return parseValueV3(trimmed);
}

// ============================================================================
// Line Parsing
// ============================================================================

export function parseSectionHeaderV3(line: string): [string, string[]] | null {
    const trimmed = line.trim();
    const match = trimmed.match(/^\[([a-zA-Z0-9_.]+)\](?:\s*=\s*(.+))?$/);
    if (!match) return null;
    const sectionName = match[1].toLowerCase();
    const schemaStr = match[2];
    let schema: string[] = [];
    if (schemaStr) {
        schema = schemaStr.split(' | ').map(col => compressKey(col.trim().toLowerCase()));
    }
    return [sectionName, schema];
}

export function parseKeyValueLineV3(line: string): [string, string] | null {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//')) return null;
    if (trimmed.startsWith('[')) return null;
    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) return null;
    const key = trimmed.substring(0, eqIndex).trim();
    const value = trimmed.substring(eqIndex + 1).trim();
    if (!key) return null;
    return [key, value];
}

// ============================================================================
// Error Helpers
// ============================================================================

/**
 * Create a parse error result with line number, column, and hint
 */
function createParseError(
    message: string,
    line: number,
    column: number = 1,
    hint?: string
): HumanParseResultV3 {
    return {
        success: false,
        error: {
            message,
            line,
            column,
            hint,
        },
    };
}

/**
 * Validate a section header format
 * Returns error message if invalid, null if valid
 */
function validateSectionHeader(line: string, lineNum: number): HumanParseResultV3 | null {
    const trimmed = line.trim();

    // Check for unclosed bracket
    if (trimmed.startsWith('[') && !trimmed.includes(']')) {
        return createParseError(
            `Unclosed section header at line ${lineNum}`,
            lineNum,
            trimmed.length,
            'Section headers should be [name] or [parent.child]'
        );
    }

    // Check for invalid characters in section name
    const match = trimmed.match(/^\[([^\]]+)\]/);
    if (match) {
        const sectionName = match[1];
        if (!/^[a-zA-Z0-9_.]+$/.test(sectionName)) {
            return createParseError(
                `Invalid section name "${sectionName}" at line ${lineNum}`,
                lineNum,
                2,
                'Section names can only contain letters, numbers, underscores, and dots'
            );
        }
    }

    return null;
}

/**
 * Validate a key-value line format
 * Returns error message if invalid, null if valid
 */
function validateKeyValueLine(line: string, lineNum: number): HumanParseResultV3 | null {
    const trimmed = line.trim();

    // Skip empty lines, comments, and section headers
    if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//') || trimmed.startsWith('[')) {
        return null;
    }

    const eqIndex = trimmed.indexOf('=');

    // Check for missing equals sign (only if line looks like it should be a key-value)
    if (eqIndex === -1) {
        // If it's not a valid line format, report error
        return createParseError(
            `Invalid line format at line ${lineNum}`,
            lineNum,
            1,
            'Expected key = value format or section header [name]'
        );
    }

    // Check for missing key
    const key = trimmed.substring(0, eqIndex).trim();
    if (!key) {
        return createParseError(
            `Missing key before '=' at line ${lineNum}`,
            lineNum,
            1,
            'Add a key name before the equals sign'
        );
    }

    // Check for unclosed quotes in value
    const value = trimmed.substring(eqIndex + 1).trim();
    const doubleQuotes = (value.match(/"/g) || []).length;
    const singleQuotes = (value.match(/'/g) || []).length;

    if (doubleQuotes % 2 !== 0) {
        return createParseError(
            `Unclosed double quote at line ${lineNum}`,
            lineNum,
            eqIndex + 2 + value.indexOf('"'),
            'Add closing double quote'
        );
    }

    if (singleQuotes % 2 !== 0) {
        return createParseError(
            `Unclosed single quote at line ${lineNum}`,
            lineNum,
            eqIndex + 2 + value.indexOf("'"),
            'Add closing single quote'
        );
    }

    return null;
}

// ============================================================================
// Main Parser
// ============================================================================

export function parseHumanV3(input: string): HumanParseResultV3 {
    const doc = createDocument() as DxDocumentWithOrder;
    doc.sectionOrder = [];
    const lines = input.split('\n');

    let currentSection: string | null = null;
    let currentDataSection: DxSection | null = null;
    let currentSubsection: string | null = null;
    let currentNestedInfo: NestedSectionInfo | null = null;
    let isStackSection = false;
    let isMultiRowSection = false;
    let lineNum = 0;

    const nestedSections: Map<string, Map<string, Array<{ key: string; value: DxValue }>>> = new Map();

    for (const line of lines) {
        lineNum++;
        const trimmed = line.trim();
        if (!trimmed) continue;
        if (trimmed.startsWith('#') || trimmed.startsWith('//')) continue;

        // Validate section headers
        if (trimmed.startsWith('[')) {
            const headerError = validateSectionHeader(trimmed, lineNum);
            if (headerError) {
                return headerError;
            }
        } else {
            // Validate key-value lines
            const kvError = validateKeyValueLine(trimmed, lineNum);
            if (kvError) {
                return kvError;
            }
        }

        const sectionHeader = parseSectionHeaderV3(trimmed);
        if (sectionHeader) {
            const [sectionName, schema] = sectionHeader;

            if (sectionName === 'stack') {
                isStackSection = true;
                isMultiRowSection = false;
                currentSection = sectionName;
                currentSubsection = null;
                currentNestedInfo = null;
                currentDataSection = null;
                if (!doc.sectionOrder.includes('stack')) {
                    doc.sectionOrder.push('stack');
                }
                continue;
            }

            isStackSection = false;
            isMultiRowSection = schema.length > 0;

            if (sectionName.includes('.')) {
                const [parentName, subsectionName] = sectionName.split('.');
                currentSection = parentName;
                currentSubsection = subsectionName;
                currentNestedInfo = {
                    parent: parentName,
                    child: subsectionName,
                    fullName: sectionName,
                };

                // Check if this is a language-specific dependency section
                const parentId = NESTED_SECTION_PARENTS[parentName];
                if (parentId && subsectionName === 'dependencies') {
                    // Store as a section with prefixed keys for round-trip
                    if (!nestedSections.has(parentName)) {
                        nestedSections.set(parentName, new Map());
                    }
                    if (!nestedSections.get(parentName)!.has(subsectionName)) {
                        nestedSections.get(parentName)!.set(subsectionName, []);
                    }
                    currentDataSection = null;
                } else {
                    // Regular nested section (like i18n.locales)
                    if (!nestedSections.has(parentName)) {
                        nestedSections.set(parentName, new Map());
                    }
                    if (!nestedSections.get(parentName)!.has(subsectionName)) {
                        nestedSections.get(parentName)!.set(subsectionName, []);
                    }
                    currentDataSection = null;
                }
            } else {
                currentSection = sectionName;
                currentSubsection = null;
                currentNestedInfo = null;
                const sectionId = compressSectionName(sectionName);
                currentDataSection = createSection(sectionId, schema);
                doc.sections.set(sectionId, currentDataSection);
                if (!doc.sectionOrder.includes(sectionId)) {
                    doc.sectionOrder.push(sectionId);
                }
            }
            continue;
        }

        const keyValue = parseKeyValueLineV3(trimmed);
        if (keyValue) {
            const [key, value] = keyValue;
            // Don't lowercase keys that contain hyphens (like package names)
            const compressedKey = key.includes('-') ? key : compressKey(key.toLowerCase());

            if (currentSection === null) {
                const parsedValue = parseArrayValueV3(value);
                doc.context.set(compressedKey, parsedValue);
            } else if (isStackSection) {
                const refValue = value.split(' | ').map(v => v.trim()).join('|');
                doc.refs.set(key, refValue);
            } else if (currentSubsection !== null) {
                const parsedValue = parseArrayValueV3(value);
                const parentSections = nestedSections.get(currentSection);
                if (parentSections) {
                    const subsectionData = parentSections.get(currentSubsection);
                    if (subsectionData) {
                        // Store key as-is (preserve hyphens in package names)
                        subsectionData.push({ key: key.includes('-') ? key : compressedKey, value: parsedValue });
                    }
                }
            } else if (currentDataSection) {
                if (isMultiRowSection) {
                    const rowValues = value.split(' | ').map(v => parseValueV3(v.trim()));
                    const row: DxValue[] = [strValue(key)];
                    row.push(...rowValues);
                    while (row.length < currentDataSection.schema.length) {
                        row.push(nullValue());
                    }
                    currentDataSection.rows.push(row);
                } else {
                    // Store key as-is (preserve hyphens in package names)
                    currentDataSection.schema.push(key.includes('-') ? key : compressedKey);
                    if (currentDataSection.rows.length === 0) {
                        currentDataSection.rows.push([]);
                    }
                    const parsedValue = parseArrayValueV3(value);
                    currentDataSection.rows[0].push(parsedValue);
                }
            }
            continue;
        }
    }

    for (const [parentName, subsections] of nestedSections) {
        const sectionId = NESTED_SECTION_PARENTS[parentName] || compressSectionName(parentName);
        const schema: string[] = [];
        const row: DxValue[] = [];
        for (const [subsectionName, fields] of subsections) {
            for (const field of fields) {
                const prefixedKey = `${subsectionName}_${field.key}`;
                schema.push(prefixedKey);
                row.push(field.value);
            }
        }
        if (schema.length > 0) {
            const section = createSection(sectionId, schema);
            section.rows.push(row);
            doc.sections.set(sectionId, section);
            if (!doc.sectionOrder.includes(sectionId)) {
                doc.sectionOrder.push(sectionId);
            }
        }
    }

    return { success: true, document: doc, sectionOrder: doc.sectionOrder };
}


// ============================================================================
// LLM Serializer
// ============================================================================

export function serializeValueV3(value: DxValue): string {
    switch (value.type) {
        case 'string': return String(value.value);
        case 'number': return String(value.value);
        case 'bool': return value.value ? '+' : '-';
        case 'null': return '~';
        case 'array':
            const items = value.value as DxValue[];
            return '*' + items.map(v => serializeValueV3(v)).join(',');
        case 'ref': return '^' + (value.refKey || value.value);
        default: return String(value.value);
    }
}

/**
 * Serialize DxDocument to LLM format
 * Preserves the order of sections as they appear in the document
 */
export function serializeToLlmV3(doc: DxDocument): string {
    const lines: string[] = [];

    // Context section (always first)
    if (doc.context.size > 0) {
        const pairs: string[] = [];
        for (const [key, value] of doc.context) {
            pairs.push(`${key}|${serializeValueV3(value)}`);
        }
        lines.push(`#c:${pairs.join(';')}`);
    }

    // Check if document has section order tracking
    const docWithOrder = doc as DxDocumentWithOrder;
    const sectionOrder = docWithOrder.sectionOrder || [];

    // If we have section order, use it to output sections in the correct order
    if (sectionOrder.length > 0) {
        for (const sectionId of sectionOrder) {
            if (sectionId === 'stack') {
                // Output reference definitions
                if (doc.refs.size > 0) {
                    for (const [key, value] of doc.refs) {
                        lines.push(`#:${key}|${value}`);
                    }
                }
            } else {
                // Output data section
                const section = doc.sections.get(sectionId);
                if (section && section.schema.length > 0) {
                    lines.push(`#${sectionId}(${section.schema.join('|')})`);
                    for (const row of section.rows) {
                        const cells = row.map(v => serializeValueV3(v));
                        lines.push(cells.join('|'));
                    }
                }
            }
        }

        // Output any sections not in the order list
        for (const [id, section] of doc.sections) {
            if (!sectionOrder.includes(id) && section.schema.length > 0) {
                lines.push(`#${id}(${section.schema.join('|')})`);
                for (const row of section.rows) {
                    const cells = row.map(v => serializeValueV3(v));
                    lines.push(cells.join('|'));
                }
            }
        }

        // Output refs if not in order list
        if (!sectionOrder.includes('stack') && doc.refs.size > 0) {
            for (const [key, value] of doc.refs) {
                lines.push(`#:${key}|${value}`);
            }
        }
    } else {
        // Fallback: refs first, then sections (old behavior)
        if (doc.refs.size > 0) {
            for (const [key, value] of doc.refs) {
                lines.push(`#:${key}|${value}`);
            }
        }

        for (const [id, section] of doc.sections) {
            if (section.schema.length > 0) {
                lines.push(`#${id}(${section.schema.join('|')})`);
                for (const row of section.rows) {
                    const cells = row.map(v => serializeValueV3(v));
                    lines.push(cells.join('|'));
                }
            }
        }
    }

    return lines.join('\n');
}
