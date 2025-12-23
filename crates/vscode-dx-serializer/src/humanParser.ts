/**
 * Human Format Parser for DX Serializer VS Code Extension
 * 
 * Parses human-readable format back to DxDocument:
 * - Recognizes [section] headers
 * - Parses key = value format
 * - Parses Unicode tables
 * - Compresses expanded keys back to abbreviations
 * 
 * Requirements: 3.1-3.5
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
    refValue,
} from './llmParser';
import { compressKey, compressSectionName, REVERSE_SECTION_NAMES } from './humanFormatter';

// ============================================================================
// Types
// ============================================================================

export interface HumanParseResult {
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
// Section Header Parser
// ============================================================================

/**
 * Parse a section header like [config] or [data]
 * Returns the section name or null if not a header
 */
export function parseSectionHeader(line: string): string | null {
    const trimmed = line.trim();
    const match = trimmed.match(/^\[([a-zA-Z0-9_]+)\]$/);
    return match ? match[1] : null;
}

/**
 * Map section names to single-letter IDs
 * Supports both full names (V2) and single letters (V1)
 */
export function sectionNameToId(name: string): string {
    const lower = name.toLowerCase();

    // Special mappings for config
    if (lower === 'config' || lower === 'context' || lower === 'c') {
        return 'c';
    }

    // Check if it's already a single-letter ID
    if (lower.length === 1) {
        return lower;
    }

    // V2: Full name to ID mapping
    return compressSectionName(lower);
}

// ============================================================================
// Config Section Parser
// ============================================================================

/**
 * Parse a key = value line from config section
 * Returns [key, value] or null if not a valid config line
 */
export function parseConfigLine(line: string): [string, DxValue] | null {
    const trimmed = line.trim();

    // Skip comments and empty lines
    if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//')) {
        return null;
    }

    // Match key = value pattern
    const match = trimmed.match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.+)$/);
    if (!match) {
        return null;
    }

    const key = match[1];
    const rawValue = match[2].trim();

    // Compress the key back to abbreviation
    const compressedKey = compressKey(key);

    // Parse the value
    const value = parseHumanValue(rawValue);

    return [compressedKey, value];
}

/**
 * Parse a value from human format
 * Supports both V1 (bracketed arrays) and V2 (comma-separated arrays)
 */
export function parseHumanValue(raw: string): DxValue {
    const trimmed = raw.trim();

    // Empty string
    if (trimmed === '') {
        return strValue('');
    }

    // Boolean: true/false
    if (trimmed === 'true') {
        return boolValue(true);
    }
    if (trimmed === 'false') {
        return boolValue(false);
    }

    // Null
    if (trimmed === 'null') {
        return nullValue();
    }

    // Number
    const num = parseFloat(trimmed);
    if (!isNaN(num) && isFinite(num) && /^-?\d+(\.\d+)?$/.test(trimmed)) {
        return numValue(num);
    }

    // Quoted string
    if ((trimmed.startsWith('"') && trimmed.endsWith('"')) ||
        (trimmed.startsWith("'") && trimmed.endsWith("'"))) {
        return strValue(trimmed.slice(1, -1));
    }

    // V1: Array with brackets: [a, b, c]
    if (trimmed.startsWith('[') && trimmed.endsWith(']')) {
        const content = trimmed.slice(1, -1);
        if (content.trim() === '') {
            return arrValue([]);
        }
        const items = content.split(',').map(item => parseHumanValue(item.trim()));
        return arrValue(items);
    }

    // V2: Comma-separated array (no brackets): a, b, c
    if (trimmed.includes(', ') || trimmed.includes(',')) {
        const items = trimmed.split(/,\s*/).map(item => parseHumanValue(item.trim()));
        if (items.length > 1) {
            return arrValue(items);
        }
    }

    // Plain string
    return strValue(trimmed);
}

// ============================================================================
// Table Parser
// ============================================================================

/**
 * Check if a line is a table border
 */
export function isTableBorder(line: string): boolean {
    const trimmed = line.trim();
    return /^[┌┐└┘├┤┬┴┼─│]+$/.test(trimmed);
}

/**
 * Check if a line is a table row (contains │)
 */
export function isTableRow(line: string): boolean {
    return line.includes('│');
}

/**
 * Parse a table row into cells
 */
export function parseTableRow(line: string): string[] {
    // Split by │ and filter out empty parts from borders
    const parts = line.split('│');

    // Remove first and last (empty from borders)
    const cells = parts.slice(1, -1).map(cell => cell.trim());

    return cells;
}

/**
 * Parse a table cell value
 */
export function parseTableCellValue(cell: string): DxValue {
    const trimmed = cell.trim();

    // Boolean checkmarks
    if (trimmed === '✓') {
        return boolValue(true);
    }
    if (trimmed === '✗') {
        return boolValue(false);
    }

    // Null dash
    if (trimmed === '—' || trimmed === '-') {
        return nullValue();
    }

    // Number
    const num = parseFloat(trimmed);
    if (!isNaN(num) && isFinite(num) && /^-?\d+(\.\d+)?$/.test(trimmed)) {
        return numValue(num);
    }

    // String
    return strValue(trimmed);
}

/**
 * Parse schema from a comment line like "# Schema: id | name | value"
 */
export function parseSchemaComment(line: string): string[] | null {
    const trimmed = line.trim();
    const match = trimmed.match(/^#\s*Schema:\s*(.+)$/i);
    if (!match) {
        return null;
    }

    const schemaStr = match[1];
    const columns = schemaStr.split('|').map(col => compressKey(col.trim()));

    return columns;
}

// ============================================================================
// Reference Detection
// ============================================================================

/**
 * Detect repeated strings that could be references
 * Returns a map of value -> suggested reference key
 */
export function detectReferences(doc: DxDocument): Map<string, string> {
    const valueCounts = new Map<string, number>();
    const refs = new Map<string, string>();

    // Count string occurrences
    for (const section of doc.sections.values()) {
        for (const row of section.rows) {
            for (const value of row) {
                if (value.type === 'string' && typeof value.value === 'string') {
                    const str = value.value;
                    if (str.length > 5) { // Only consider longer strings
                        valueCounts.set(str, (valueCounts.get(str) || 0) + 1);
                    }
                }
            }
        }
    }

    // Create references for repeated values
    let refKey = 'A';
    for (const [value, count] of valueCounts) {
        if (count >= 2) {
            refs.set(value, refKey);
            refKey = String.fromCharCode(refKey.charCodeAt(0) + 1);
        }
    }

    return refs;
}

// ============================================================================
// Main Parser
// ============================================================================

/**
 * Parse human-readable format into DxDocument
 */
export function parseHuman(input: string): HumanParseResult {
    const doc = createDocument();
    const lines = input.split('\n');

    let currentSection: string | null = null;
    let currentDataSection: DxSection | null = null;
    let inTable = false;
    let tableSchema: string[] = [];
    let isHeaderRow = true;
    let lineNum = 0;

    for (const line of lines) {
        lineNum++;
        const trimmed = line.trim();

        // Skip empty lines
        if (!trimmed) {
            continue;
        }

        // Skip section header decorations (lines with only box chars or #)
        if (/^#\s*[─═]+\s*$/.test(trimmed) || /^#\s*[A-Z\s:]+\s*$/.test(trimmed)) {
            continue;
        }

        // Check for section header [name]
        const sectionName = parseSectionHeader(trimmed);
        if (sectionName) {
            currentSection = sectionName.toLowerCase();
            inTable = false;
            isHeaderRow = true;

            // Create data section if not config
            if (currentSection !== 'config' && currentSection !== 'context') {
                const sectionId = sectionNameToId(currentSection);
                currentDataSection = createSection(sectionId, []);
                doc.sections.set(sectionId, currentDataSection);
            } else {
                currentDataSection = null;
            }
            continue;
        }

        // Parse schema comment
        const schema = parseSchemaComment(trimmed);
        if (schema && currentDataSection) {
            currentDataSection.schema = schema;
            tableSchema = schema;
            continue;
        }

        // Skip other comments
        if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
            continue;
        }

        // Handle config section
        if (currentSection === 'config' || currentSection === 'context') {
            const configLine = parseConfigLine(trimmed);
            if (configLine) {
                doc.context.set(configLine[0], configLine[1]);
            }
            continue;
        }

        // Handle table borders
        if (isTableBorder(trimmed)) {
            inTable = true;
            continue;
        }

        // Handle table rows
        if (isTableRow(trimmed) && currentDataSection) {
            const cells = parseTableRow(trimmed);

            if (isHeaderRow) {
                // First row is header - extract schema if not already set
                if (currentDataSection.schema.length === 0) {
                    currentDataSection.schema = cells.map(cell => compressKey(cell.toLowerCase()));
                    tableSchema = currentDataSection.schema;
                }
                isHeaderRow = false;
            } else {
                // Data row
                const row = cells.map(cell => parseTableCellValue(cell));
                currentDataSection.rows.push(row);
            }
            continue;
        }

        // Skip summary lines
        if (trimmed.startsWith('Total:') || trimmed.includes('total:')) {
            continue;
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
export function serializeValue(value: DxValue): string {
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
            return '*' + items.map(v => serializeValue(v)).join(',');
        case 'ref':
            return '^' + (value.refKey || value.value);
        default:
            return String(value.value);
    }
}

/**
 * Serialize DxDocument to LLM format
 */
export function serializeToLlm(doc: DxDocument): string {
    const lines: string[] = [];

    // Context section
    if (doc.context.size > 0) {
        const pairs: string[] = [];
        for (const [key, value] of doc.context) {
            pairs.push(`${key}|${serializeValue(value)}`);
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
                const cells = row.map(v => serializeValue(v));
                lines.push(cells.join('|'));
            }
        }
    }

    return lines.join('\n');
}
