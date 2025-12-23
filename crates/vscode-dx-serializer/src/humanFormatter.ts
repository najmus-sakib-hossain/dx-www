/**
 * Human Format V2 Formatter for DX Serializer VS Code Extension
 * 
 * Converts DxDocument to beautiful human-readable format with:
 * - Flat TOML-like structure (no indentation)
 * - Full section names in brackets ([forge] not [f])
 * - Unicode tables without indentation
 * - Expanded key names
 * - Comma-separated arrays without brackets
 * - Resolved references
 * - Summary footers
 * 
 * Requirements: 2.1-2.7, 6.1-6.9
 */

import { DxDocument, DxValue, DxSection } from './llmParser';

// ============================================================================
// Abbreviation Dictionary
// ============================================================================

/**
 * Standard abbreviation mappings for key compression/expansion
 * Requirements: 6.1-6.9
 */
export const ABBREVIATIONS: Record<string, string> = {
    // Identity
    'nm': 'name',
    'tt': 'title',
    'ds': 'description',
    'id': 'id',

    // Version & Author
    'v': 'version',
    'au': 'author',

    // Workspace & Editors
    'ws': 'workspace',
    'ed': 'editors',

    // Repository & Container
    'repo': 'repository',
    'cont': 'container',
    'ci': 'ci_cd',

    // State
    'st': 'status',
    'ac': 'active',
    'en': 'enabled',

    // Metrics
    'ct': 'count',
    'tl': 'total',
    'pr': 'price',
    'am': 'amount',
    'qt': 'quantity',

    // Contact
    'em': 'email',
    'ph': 'phone',

    // Web
    'ur': 'url',
    'pt': 'path',

    // Common
    'vl': 'value',
    'tp': 'type',
    'dt': 'date',
    'tm': 'time',
    'sz': 'size',
    'ln': 'length',
    'wd': 'width',
    'ht': 'height',
    'cl': 'color',
    'bg': 'background',
    'fg': 'foreground',
    'tx': 'text',
    'lb': 'label',
    'ms': 'message',
    'er': 'error',
    'wr': 'warning',
    'in': 'info',
    'ok': 'success',
};

/**
 * Section ID to full name mapping
 */
export const SECTION_NAMES: Record<string, string> = {
    'c': 'config',
    'f': 'forge',
    'k': 'stack',
    'y': 'style',
    'u': 'ui',
    'm': 'media',
    'i': 'i18n',
    'o': 'icon',
    't': 'font',
    'd': 'data',
};

/**
 * Reverse mapping for compression (full name -> abbreviation)
 */
export const REVERSE_ABBREVIATIONS: Record<string, string> = Object.fromEntries(
    Object.entries(ABBREVIATIONS).map(([abbrev, full]) => [full, abbrev])
);

/**
 * Reverse section name mapping (full name -> ID)
 */
export const REVERSE_SECTION_NAMES: Record<string, string> = Object.fromEntries(
    Object.entries(SECTION_NAMES).map(([id, name]) => [name, id])
);

/**
 * Expand an abbreviated key to its full name
 */
export function expandKey(abbrev: string): string {
    return ABBREVIATIONS[abbrev] || abbrev;
}

/**
 * Compress a full key name to its abbreviation
 */
export function compressKey(full: string): string {
    return REVERSE_ABBREVIATIONS[full] || full;
}

/**
 * Expand a section ID to its full name
 */
export function expandSectionName(id: string): string {
    return SECTION_NAMES[id] || id;
}

/**
 * Compress a section name to its ID
 */
export function compressSectionName(name: string): string {
    return REVERSE_SECTION_NAMES[name.toLowerCase()] || name.charAt(0).toLowerCase();
}

// ============================================================================
// Box Drawing Characters
// ============================================================================

const BOX = {
    // Corners
    topLeft: '┌',
    topRight: '┐',
    bottomLeft: '└',
    bottomRight: '┘',

    // Lines
    horizontal: '─',
    vertical: '│',

    // T-junctions
    topT: '┬',
    bottomT: '┴',
    leftT: '├',
    rightT: '┤',

    // Cross
    cross: '┼',
};

// ============================================================================
// Value Formatting
// ============================================================================

/**
 * Format a DxValue for human display
 */
export function formatValue(value: DxValue, refs?: Map<string, string>): string {
    switch (value.type) {
        case 'string':
            return String(value.value);
        case 'number':
            return String(value.value);
        case 'bool':
            return value.value ? 'true' : 'false';
        case 'null':
            return 'null';
        case 'array':
            const items = value.value as DxValue[];
            // Check if any item is an array (nested)
            const hasNestedArray = items.some(v => v.type === 'array');
            if (hasNestedArray) {
                // For nested arrays, use brackets and recursive formatting
                const formatted = items.map(v => {
                    if (v.type === 'array') {
                        const innerItems = v.value as DxValue[];
                        return '[' + innerItems.map(iv => formatValue(iv, refs)).join(', ') + ']';
                    }
                    return formatValue(v, refs);
                });
                return '[' + formatted.join(', ') + ']';
            }
            // V2: comma-separated without brackets for simple arrays
            return items.map(v => formatValue(v, refs)).join(', ');
        case 'ref':
            const refKey = value.refKey || String(value.value);
            if (refs && refs.has(refKey)) {
                return refs.get(refKey)!;
            }
            return `^${refKey}`;
        default:
            return String(value.value);
    }
}

/**
 * Format a value for table display
 */
export function formatTableValue(value: DxValue, refs?: Map<string, string>): string {
    switch (value.type) {
        case 'bool':
            return value.value ? '✓' : '✗';
        case 'null':
            return '—';
        case 'ref':
            const refKey = value.refKey || String(value.value);
            if (refs && refs.has(refKey)) {
                return refs.get(refKey)!;
            }
            return `^${refKey}`;
        case 'array':
            const items = value.value as DxValue[];
            return items.map(v => formatTableValue(v, refs)).join(', ');
        default:
            return formatValue(value, refs);
    }
}

// ============================================================================
// Section Header Formatting (V2 - Flat Structure)
// ============================================================================

/**
 * Generate a section header with box-drawing characters (V2 style)
 */
export function formatSectionHeader(title: string, width: number = 80): string {
    const line = '═'.repeat(width);
    const padding = Math.max(0, Math.floor((width - title.length) / 2));
    const paddedTitle = ' '.repeat(padding) + title.toUpperCase() + ' '.repeat(width - title.length - padding);

    return [
        `# ${line}`,
        `#${paddedTitle}`,
        `# ${line}`,
    ].join('\n');
}

// ============================================================================
// Config Section Formatting (V2 - Flat Structure)
// ============================================================================

/**
 * Format the config/context section (V2 - no indentation)
 */
export function formatConfigSection(context: Map<string, DxValue>, refs?: Map<string, string>): string {
    if (context.size === 0) {
        return '';
    }

    const lines: string[] = [];
    lines.push('[config]');

    // Calculate max key length for alignment
    const expandedKeys = Array.from(context.keys()).map(k => expandKey(k));
    const maxKeyLen = Math.max(...expandedKeys.map(k => k.length));

    for (const [abbrevKey, value] of context) {
        const fullKey = expandKey(abbrevKey);
        const padding = ' '.repeat(maxKeyLen - fullKey.length);
        const formattedValue = formatValue(value, refs);

        // Quote strings that contain spaces or special characters
        let displayValue = formattedValue;
        if (value.type === 'string' && /[ =\[\]{}]/.test(formattedValue)) {
            displayValue = `"${formattedValue}"`;
        }

        // V2: No indentation - flat structure
        lines.push(`${fullKey}${padding} = ${displayValue}`);
    }

    return lines.join('\n');
}

// ============================================================================
// Table Formatting (V2 - No Indentation)
// ============================================================================

/**
 * Calculate column widths for a table
 */
function calculateColumnWidths(schema: string[], rows: DxValue[][], refs?: Map<string, string>): number[] {
    const widths: number[] = schema.map(col => expandKey(col).length);

    for (const row of rows) {
        for (let i = 0; i < row.length && i < widths.length; i++) {
            const cellValue = formatTableValue(row[i], refs);
            widths[i] = Math.max(widths[i], cellValue.length);
        }
    }

    // Add padding (1 space on each side)
    return widths.map(w => w + 2);
}

/**
 * Format a table row
 */
function formatTableRow(cells: string[], widths: number[], alignments: ('left' | 'right' | 'center')[]): string {
    const formatted = cells.map((cell, i) => {
        const width = widths[i] - 2; // Account for padding
        const align = alignments[i] || 'left';

        if (align === 'right') {
            return ' ' + cell.padStart(width) + ' ';
        } else if (align === 'center') {
            const leftPad = Math.floor((width - cell.length) / 2);
            const rightPad = width - cell.length - leftPad;
            return ' ' + ' '.repeat(leftPad) + cell + ' '.repeat(rightPad) + ' ';
        } else {
            return ' ' + cell.padEnd(width) + ' ';
        }
    });

    return BOX.vertical + formatted.join(BOX.vertical) + BOX.vertical;
}

/**
 * Format a horizontal line for the table
 */
function formatTableLine(widths: number[], left: string, mid: string, right: string): string {
    const segments = widths.map(w => BOX.horizontal.repeat(w));
    return left + segments.join(mid) + right;
}

/**
 * Determine alignment for a column based on its values
 */
function determineAlignment(values: DxValue[]): 'left' | 'right' | 'center' {
    const types = values.map(v => v.type);

    // Numbers are right-aligned
    if (types.every(t => t === 'number')) {
        return 'right';
    }

    // Booleans are centered
    if (types.every(t => t === 'bool')) {
        return 'center';
    }

    // Default to left
    return 'left';
}

/**
 * Format a data section as a Unicode table (V2 - no indentation)
 */
export function formatDataSection(section: DxSection, refs?: Map<string, string>): string {
    const lines: string[] = [];

    // V2: Full section name in brackets (but keep original ID for unknown sections)
    const fullSectionName = expandSectionName(section.id);
    lines.push(`[${fullSectionName}]`);

    if (section.schema.length === 0) {
        return lines.join('\n');
    }

    // Expand schema column names
    const expandedSchema = section.schema.map(col => expandKey(col));

    // Calculate column widths
    const widths = calculateColumnWidths(section.schema, section.rows, refs);

    // Determine alignments
    const alignments: ('left' | 'right' | 'center')[] = section.schema.map((_, i) => {
        const columnValues = section.rows.map(row => row[i]).filter(v => v !== undefined);
        return determineAlignment(columnValues);
    });

    // V2: No indentation - table directly after header
    // Top border
    lines.push(formatTableLine(widths, BOX.topLeft, BOX.topT, BOX.topRight));

    // Header row
    const headerCells = expandedSchema.map(col => col.charAt(0).toUpperCase() + col.slice(1));
    lines.push(formatTableRow(headerCells, widths, alignments.map(() => 'center' as const)));

    // Header separator
    lines.push(formatTableLine(widths, BOX.leftT, BOX.cross, BOX.rightT));

    // Data rows
    for (const row of section.rows) {
        const cells = row.map((value, i) => formatTableValue(value, refs));
        lines.push(formatTableRow(cells, widths, alignments));
    }

    // Bottom border
    lines.push(formatTableLine(widths, BOX.bottomLeft, BOX.bottomT, BOX.bottomRight));

    // Summary footer
    lines.push('');
    lines.push(`Total: ${section.rows.length} rows`);

    return lines.join('\n');
}

// ============================================================================
// Summary Generation
// ============================================================================

/**
 * Generate a summary footer for a data section
 */
export function generateSummary(section: DxSection, refs?: Map<string, string>): string {
    const parts: string[] = [];

    // Total row count
    parts.push(`Total: ${section.rows.length} items`);

    // Calculate numeric totals
    for (let i = 0; i < section.schema.length; i++) {
        const columnValues = section.rows.map(row => row[i]).filter(v => v !== undefined);
        const allNumbers = columnValues.every(v => v.type === 'number');

        if (allNumbers && columnValues.length > 0) {
            const sum = columnValues.reduce((acc, v) => acc + (v.value as number), 0);
            const colName = expandKey(section.schema[i]);
            parts.push(`${colName} total: ${sum}`);
        }

        // Count boolean true values
        const allBools = columnValues.every(v => v.type === 'bool');
        if (allBools && columnValues.length > 0) {
            const trueCount = columnValues.filter(v => v.value === true).length;
            const colName = expandKey(section.schema[i]);
            parts.push(`${colName}: ${trueCount}/${columnValues.length}`);
        }
    }

    return parts.join(' | ');
}

// ============================================================================
// Document Formatting (V2 - Flat Structure)
// ============================================================================

/**
 * Format a complete DxDocument to human-readable format (V2)
 */
export function formatDocument(doc: DxDocument): string {
    const sections: string[] = [];

    // Config section
    if (doc.context.size > 0) {
        sections.push(formatSectionHeader('CONFIGURATION'));
        sections.push('');
        sections.push(formatConfigSection(doc.context, doc.refs));
    }

    // Data sections
    for (const [id, section] of doc.sections) {
        sections.push('');
        sections.push(formatSectionHeader(expandSectionName(id).toUpperCase()));
        sections.push('');
        sections.push(formatDataSection(section, doc.refs));
    }

    return sections.join('\n');
}
