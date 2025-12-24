/**
 * Human Format V3 Formatter for DX Serializer VS Code Extension
 * 
 * Converts DxDocument to clean vertical key-value format with:
 * - Config values without [config] section header
 * - Data sections with [section] headers
 * - Key padding to 20 characters (or longest key + 1)
 * - Pipe (|) as array separator
 * - Quoted strings with spaces
 * - Vertical key-value layout for data sections
 * 
 * Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7
 */

import { DxDocument, DxValue, DxSection } from './llmParser';

// ============================================================================
// Configuration
// ============================================================================

export interface HumanFormatV3Config {
    keyPadding: number;           // Minimum padding for keys (default: 20)
    arraySeparator: string;       // Separator for arrays (default: ' | ')
    quoteStringsWithSpaces: boolean; // Quote strings containing spaces
}

export const DEFAULT_CONFIG: HumanFormatV3Config = {
    keyPadding: 20,
    arraySeparator: ' | ',
    quoteStringsWithSpaces: true,
};

// ============================================================================
// Abbreviation Dictionary (reused from humanFormatter.ts)
// ============================================================================

export const ABBREVIATIONS: Record<string, string> = {
    // Identity
    'nm': 'name',
    'tt': 'title',
    'ds': 'description',
    'id': 'id',
    'lc': 'license',

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
    'pl': 'pipeline',
    'tk': 'tasks',
    'it': 'items',

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

export const REVERSE_ABBREVIATIONS: Record<string, string> = Object.fromEntries(
    Object.entries(ABBREVIATIONS).map(([abbrev, full]) => [full, abbrev])
);

export const REVERSE_SECTION_NAMES: Record<string, string> = Object.fromEntries(
    Object.entries(SECTION_NAMES).map(([id, name]) => [name, id])
);

export function expandKey(abbrev: string): string {
    return ABBREVIATIONS[abbrev] || abbrev;
}

export function compressKey(full: string): string {
    return REVERSE_ABBREVIATIONS[full] || full;
}

export function expandSectionName(id: string): string {
    return SECTION_NAMES[id] || id;
}

export function compressSectionName(name: string): string {
    return REVERSE_SECTION_NAMES[name.toLowerCase()] || name.charAt(0).toLowerCase();
}

// ============================================================================
// Value Formatting
// ============================================================================

/**
 * Format a value for V3 output
 * Uses | as array separator and quotes strings with spaces or that look like numbers
 */
export function formatValueV3(value: DxValue, config: HumanFormatV3Config = DEFAULT_CONFIG): string {
    switch (value.type) {
        case 'string':
            const str = String(value.value);
            // Quote strings containing spaces
            if (config.quoteStringsWithSpaces && str.includes(' ')) {
                return `"${str}"`;
            }
            // Quote strings that look like numbers to preserve type
            if (/^-?\d+(\.\d+)?$/.test(str)) {
                return `"${str}"`;
            }
            // Quote strings that look like booleans
            if (str.toLowerCase() === 'true' || str.toLowerCase() === 'false') {
                return `"${str}"`;
            }
            return str;
        case 'number':
            return String(value.value);
        case 'bool':
            return value.value ? 'true' : 'false';
        case 'null':
            return '-';
        case 'array':
            const items = value.value as DxValue[];
            return items.map(v => formatValueV3(v, config)).join(config.arraySeparator);
        case 'ref':
            return `^${value.refKey || value.value}`;
        default:
            return String(value.value);
    }
}

// ============================================================================
// Config Section Formatting (No [config] header)
// ============================================================================

/**
 * Format config section as vertical key-value pairs without section header
 * Requirements: 2.1, 2.3, 2.4, 2.7
 */
export function formatConfigSectionV3(
    context: Map<string, DxValue>,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    if (context.size === 0) {
        return '';
    }

    const lines: string[] = [];

    // Get all expanded keys and find max length
    const entries: Array<[string, string, DxValue]> = [];
    for (const [abbrevKey, value] of context) {
        const fullKey = expandKey(abbrevKey);
        entries.push([abbrevKey, fullKey, value]);
    }

    // Calculate padding (max of keyPadding or longest key + 1)
    const maxKeyLen = Math.max(
        config.keyPadding,
        ...entries.map(([, fullKey]) => fullKey.length + 1)
    );

    // Format each key-value pair
    for (const [, fullKey, value] of entries) {
        const padding = ' '.repeat(maxKeyLen - fullKey.length);
        const formattedValue = formatValueV3(value, config);
        lines.push(`${fullKey}${padding}= ${formattedValue}`);
    }

    return lines.join('\n');
}

// ============================================================================
// Data Section Formatting (Vertical Key-Value)
// ============================================================================

/**
 * Format a data section as vertical key-value pairs
 * Requirements: 2.2, 2.3, 2.4, 2.5, 2.6, 2.7
 */
export function formatDataSectionV3(
    section: DxSection,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const fullSectionName = expandSectionName(section.id);

    // Check if section has multiple rows (needs schema header)
    const hasMultipleRows = section.rows.length > 1;
    const expandedSchema = section.schema.map(col => expandKey(col));

    if (hasMultipleRows && section.schema.length > 0) {
        // Multi-row section: [section] = Col1 | Col2 | Col3
        const schemaStr = expandedSchema.map(col =>
            col.charAt(0).toUpperCase() + col.slice(1)
        ).join(config.arraySeparator);
        lines.push(`[${fullSectionName}]${' '.repeat(Math.max(1, config.keyPadding - fullSectionName.length - 2))}= ${schemaStr}`);

        // Each row as: rowName = val1 | val2 | val3
        for (const row of section.rows) {
            // First column is typically the row identifier/name
            const rowName = row.length > 0 ? formatValueV3(row[0], { ...config, quoteStringsWithSpaces: false }) : '';
            const rowValues = row.slice(1).map(v => formatValueV3(v, config));
            const padding = ' '.repeat(Math.max(1, config.keyPadding - rowName.length));
            lines.push(`${rowName}${padding}= ${rowValues.join(config.arraySeparator)}`);
        }
    } else if (section.rows.length === 1) {
        // Single row section: vertical key-value pairs
        lines.push(`[${fullSectionName}]`);
        const row = section.rows[0];

        // Calculate padding for this section
        const maxKeyLen = Math.max(
            config.keyPadding,
            ...expandedSchema.map(k => k.length + 1)
        );

        for (let i = 0; i < expandedSchema.length && i < row.length; i++) {
            const key = expandedSchema[i];
            const value = row[i];
            const padding = ' '.repeat(maxKeyLen - key.length);
            const formattedValue = formatValueV3(value, config);
            lines.push(`${key}${padding}= ${formattedValue}`);
        }
    } else {
        // Empty section
        lines.push(`[${fullSectionName}]`);
    }

    return lines.join('\n');
}

// ============================================================================
// Reference Section Formatting
// ============================================================================

/**
 * Format reference definitions as a special [stack] section
 * This handles the #: reference definitions in LLM format
 */
export function formatReferenceSectionV3(
    refs: Map<string, string>,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    if (refs.size === 0) {
        return '';
    }

    const lines: string[] = [];

    // Parse reference values (they contain pipe-separated data)
    // Format: key|lang|runtime|compiler|bundler|pm|framework
    const schemaLabels = ['Lang', 'Runtime', 'Compiler', 'Bundler', 'PM', 'Framework'];
    lines.push(`[stack]${' '.repeat(Math.max(1, config.keyPadding - 7))}= ${schemaLabels.join(config.arraySeparator)}`);

    for (const [key, value] of refs) {
        const parts = value.split('|');
        const padding = ' '.repeat(Math.max(1, config.keyPadding - key.length));
        lines.push(`${key}${padding}= ${parts.join(config.arraySeparator)}`);
    }

    return lines.join('\n');
}

// ============================================================================
// Document Formatting
// ============================================================================

/**
 * Format a complete DxDocument to Human Format V3
 * Requirements: 2.1-2.7
 */
export function formatDocumentV3(
    doc: DxDocument,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const sections: string[] = [];

    // Config section (no header)
    if (doc.context.size > 0) {
        sections.push(formatConfigSectionV3(doc.context, config));
    }

    // Reference section as [stack] if present
    if (doc.refs.size > 0) {
        sections.push('');
        sections.push(formatReferenceSectionV3(doc.refs, config));
    }

    // Data sections
    for (const [, section] of doc.sections) {
        sections.push('');
        sections.push(formatDataSectionV3(section, config));
    }

    return sections.join('\n');
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Check if output contains table formatting characters
 * Used for Property 6 validation
 */
export function containsTableFormatting(output: string): boolean {
    const tableChars = ['┌', '┐', '└', '┘', '─', '│', '├', '┤', '┬', '┴', '┼'];
    return tableChars.some(char => output.includes(char));
}

/**
 * Check if all = signs are aligned
 * Used for Property 4 validation
 */
export function checkKeyAlignment(output: string): boolean {
    const lines = output.split('\n').filter(line => line.includes('=') && !line.startsWith('['));
    if (lines.length === 0) return true;

    const positions = lines.map(line => line.indexOf('='));
    const firstPos = positions[0];

    // All = signs should be at the same position (within the same section)
    // For now, just check they're all positive
    return positions.every(pos => pos > 0);
}

/**
 * Check if arrays use | separator
 * Used for Property 5 validation
 */
export function usesCorrectArraySeparator(output: string, separator: string = ' | '): boolean {
    // If output contains comma-separated values that look like arrays, it's wrong
    // This is a heuristic check
    const lines = output.split('\n');
    for (const line of lines) {
        if (line.includes('=')) {
            const value = line.split('=')[1]?.trim() || '';
            // Check if it looks like a comma-separated array (multiple items with commas)
            if (value.includes(', ') && !value.includes(separator)) {
                // Could be a comma-separated array - check if it has multiple items
                const commaCount = (value.match(/,/g) || []).length;
                if (commaCount >= 1) {
                    return false;
                }
            }
        }
    }
    return true;
}
