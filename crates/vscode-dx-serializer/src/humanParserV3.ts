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
    error?: {
        message: string;
        line: number;
        column: number;
        hint?: string;
    };
}

// ============================================================================
// Value Parsing
// ============================================================================

/**
 * Parse a value from Human V3 format
 * Handles: strings, quoted strings, numbers, booleans, null (-), arrays (|)
 */
export function parseValueV3(raw: string): DxValue {
    const trimmed = raw.trim();

    // Empty string
    if (trimmed === '') {
        return strValue('');
    }

    // Null: - or ~
    if (trimmed === '-' || trimmed === '~') {
        return nullValue();
    }

    // Boolean: true/false
    if (trimmed.toLowerCase() === 'true') {
        return boolValue(true);
    }
    if (trimmed.toLowerCase() === 'false') {
        return boolValue(false);
    }

    // Quoted string - always return as string
    if ((trimmed.startsWith('"') && trimmed.endsWith('"')) ||
        (trimmed.startsWith("'") && trimmed.endsWith("'"))) {
        return strValue(trimmed.slice(1, -1));
    }

    // Number
    const num = parseFloat(trimmed);
    if (!isNaN(num) && isFinite(num) && /^-?\d+(\.\d+)?$/.test(trimmed)) {
        return numValue(num);
    }

    // Plain string
    return strValue(trimmed);
}

/**
 * Parse a pipe-separated array value
 */
export function parseArrayValueV3(raw: string): DxValue {
    const trimmed = raw.trim();

    // Check if it contains pipe separator
    if (trimmed.includes(' | ')) {
        const items = trimmed.split(' | ').map(item => parseValueV3(item.trim()));
        return arrValue(items);
    }

    // Also handle single pipe without spaces (for compatibility)
    if (trimmed.includes('|') && !trimmed.includes(' | ')) {
        const items = trimmed.split('|').map(item => parseValueV3(item.trim()));
        if (items.length > 1) {
            return arrValue(items);
        }
    }

    // Single value
    return parseValueV3(trimmed);
}

// ============================================================================
// Line Parsing
// ============================================================================

/**
 * Parse a section header like [forge] or [forge] = Col1 | Col2 or [i18n.locales]
 * Returns [sectionName, schema] or null if not a header
 */
export function parseSectionHeaderV3(line: string): [string, string[]] | null {
    const trimmed = line.trim();

    // Match [section] or [section.subsection] or [section] = schema
    const match = trimmed.match(/^\[([a-zA-Z0-9_.]+)\](?:\s*=\s*(.+))?$/);
    if (!match) {
        return null;
    }

    const sectionName = match[1].toLowerCase();
    const schemaStr = match[2];

    // Parse schema if present
    let schema: string[] = [];
    if (schemaStr) {
        schema = schemaStr.split(' | ').map(col => compressKey(col.trim().toLowerCase()));
    }

    return [sectionName, schema];
}

/**
 * Parse a key = value line
 * Returns [key, value] or null if not a valid line
 */
export function parseKeyValueLineV3(line: string): [string, string] | null {
    const trimmed = line.trim();

    // Skip empty lines and comments
    if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//')) {
        return null;
    }

    // Skip section headers
    if (trimmed.startsWith('[')) {
        return null;
    }

    // Match key = value pattern (with flexible spacing around =)
    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) {
        return null;
    }

    const key = trimmed.substring(0, eqIndex).trim();
    const value = trimmed.substring(eqIndex + 1).trim();

    if (!key) {
        return null;
    }

    return [key, value];
}

// ============================================================================
// Main Parser
// ============================================================================

/**
 * Parse Human Format V3 content into a DxDocument
 * 
 * Format:
 * - Lines before first [section] are config values
 * - [stack] section contains reference definitions (key = val1 | val2 | ...)
 * - [section] starts a new data section
 * - [section.subsection] creates nested sections (merged into parent)
 * - [section] = Col1 | Col2 defines schema for multi-row section
 * - key = value lines are either config or section data
 */
export function parseHumanV3(input: string): HumanParseResultV3 {
    const doc = createDocument();
    const lines = input.split('\n');

    let currentSection: string | null = null;
    let currentDataSection: DxSection | null = null;
    let currentSubsection: string | null = null;
    let isStackSection = false; // Track if we're in [stack] section (reference definitions)
    let isMultiRowSection = false; // Track if section has schema from header (multi-row mode)
    let lineNum = 0;

    // Track nested sections for merging
    const nestedSections: Map<string, Map<string, Array<{ key: string; value: DxValue }>>> = new Map();

    for (const line of lines) {
        lineNum++;
        const trimmed = line.trim();

        // Skip empty lines
        if (!trimmed) {
            continue;
        }

        // Skip comments
        if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
            continue;
        }

        // Check for section header
        const sectionHeader = parseSectionHeaderV3(trimmed);
        if (sectionHeader) {
            const [sectionName, schema] = sectionHeader;

            // Check if this is the [stack] section (reference definitions)
            if (sectionName === 'stack') {
                isStackSection = true;
                isMultiRowSection = false;
                currentSection = sectionName;
                currentSubsection = null;
                currentDataSection = null;
                continue;
            }

            isStackSection = false;
            // Multi-row mode if schema was provided in header
            isMultiRowSection = schema.length > 0;

            // Check for nested section (e.g., i18n.locales)
            if (sectionName.includes('.')) {
                const [parentName, subsectionName] = sectionName.split('.');
                currentSection = parentName;
                currentSubsection = subsectionName;

                // Initialize nested section tracking
                if (!nestedSections.has(parentName)) {
                    nestedSections.set(parentName, new Map());
                }
                if (!nestedSections.get(parentName)!.has(subsectionName)) {
                    nestedSections.get(parentName)!.set(subsectionName, []);
                }

                currentDataSection = null; // Don't create a data section yet
            } else {
                currentSection = sectionName;
                currentSubsection = null;

                // Map section name to ID
                const sectionId = compressSectionName(sectionName);

                // Create data section
                currentDataSection = createSection(sectionId, schema);
                doc.sections.set(sectionId, currentDataSection);
            }
            continue;
        }

        // Parse key = value line
        const keyValue = parseKeyValueLineV3(trimmed);
        if (keyValue) {
            const [key, value] = keyValue;
            const compressedKey = compressKey(key.toLowerCase());

            if (currentSection === null) {
                // Config section (before any [section] header)
                const parsedValue = parseArrayValueV3(value);
                doc.context.set(compressedKey, parsedValue);
            } else if (isStackSection) {
                // [stack] section: reference definitions
                // key = val1 | val2 | val3 becomes #:key|val1|val2|val3
                // Store as raw pipe-separated string (not parsed as array)
                const refValue = value.split(' | ').map(v => v.trim()).join('|');
                doc.refs.set(key, refValue);
            } else if (currentSubsection !== null) {
                // Nested section (e.g., i18n.locales)
                const parsedValue = parseArrayValueV3(value);
                const parentSections = nestedSections.get(currentSection);
                if (parentSections) {
                    const subsectionData = parentSections.get(currentSubsection);
                    if (subsectionData) {
                        subsectionData.push({ key: compressedKey, value: parsedValue });
                    }
                }
            } else if (currentDataSection) {
                if (isMultiRowSection) {
                    // Multi-row section: key = val1 | val2 | val3
                    // First value is the row name, rest are column values
                    const rowValues = value.split(' | ').map(v => parseValueV3(v.trim()));

                    // Create row with key as first column
                    const row: DxValue[] = [strValue(key)];
                    row.push(...rowValues);

                    // Pad to schema length
                    while (row.length < currentDataSection.schema.length) {
                        row.push(nullValue());
                    }

                    currentDataSection.rows.push(row);
                } else {
                    // Single-row section: vertical key-value pairs
                    // Add key to schema and value to first row
                    currentDataSection.schema.push(compressedKey);

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

    // Merge nested sections into parent sections
    for (const [parentName, subsections] of nestedSections) {
        const sectionId = compressSectionName(parentName);

        // Build schema and row from nested sections
        const schema: string[] = [];
        const row: DxValue[] = [];

        for (const [subsectionName, fields] of subsections) {
            for (const field of fields) {
                // Create prefixed key like locales_path, ttses_default
                const prefixedKey = `${subsectionName}_${field.key}`;
                schema.push(prefixedKey);
                row.push(field.value);
            }
        }

        if (schema.length > 0) {
            const section = createSection(sectionId, schema);
            section.rows.push(row);
            doc.sections.set(sectionId, section);
        }
    }

    return {
        success: true,
        document: doc,
    };
}

// ============================================================================
// LLM Serializer
// ============================================================================

/**
 * Serialize a DxValue to LLM format
 */
export function serializeValueV3(value: DxValue): string {
    switch (value.type) {
        case 'string':
            return String(value.value);
        case 'number':
            return String(value.value);
        case 'bool':
            return value.value ? '+' : '-';
        case 'null':
            return '~';
        case 'array':
            const items = value.value as DxValue[];
            return '*' + items.map(v => serializeValueV3(v)).join(',');
        case 'ref':
            return '^' + (value.refKey || value.value);
        default:
            return String(value.value);
    }
}

/**
 * Serialize DxDocument to LLM format
 */
export function serializeToLlmV3(doc: DxDocument): string {
    const lines: string[] = [];

    // Context section
    if (doc.context.size > 0) {
        const pairs: string[] = [];
        for (const [key, value] of doc.context) {
            pairs.push(`${key}|${serializeValueV3(value)}`);
        }
        lines.push(`#c:${pairs.join(';')}`);
    }

    // Reference definitions
    for (const [key, value] of doc.refs) {
        lines.push(`#:${key}|${value}`);
    }

    // Data sections
    for (const [id, section] of doc.sections) {
        if (section.schema.length > 0) {
            lines.push(`#${id}(${section.schema.join('|')})`);

            for (const row of section.rows) {
                const cells = row.map(v => serializeValueV3(v));
                lines.push(cells.join('|'));
            }
        }
    }

    return lines.join('\n');
}
