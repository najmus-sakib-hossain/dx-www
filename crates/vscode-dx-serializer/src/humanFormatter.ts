/**
 * Human Format V2 Formatter for DX Serializer VS Code Extension
 * 
 * Converts DxDocument to clean, simple human-readable format with:
 * - Flat TOML-like structure (no indentation)
 * - Full section names in brackets ([forge] not [f])
 * - Flat data rows (no indentation, no table borders)
 * - Expanded key names
 * - Comma-separated arrays without brackets
 */

import { DxDocument, DxValue, DxSection } from './llmParser';

// ============================================================================
// Abbreviation Dictionary
// ============================================================================

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
            const hasNestedArray = items.some(v => v.type === 'array');
            if (hasNestedArray) {
                const formatted = items.map(v => {
                    if (v.type === 'array') {
                        const innerItems = v.value as DxValue[];
                        return '[' + innerItems.map(iv => formatValue(iv, refs)).join(', ') + ']';
                    }
                    return formatValue(v, refs);
                });
                return '[' + formatted.join(', ') + ']';
            }
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

export function formatTableValue(value: DxValue, refs?: Map<string, string>): string {
    switch (value.type) {
        case 'bool':
            return value.value ? 'true' : 'false';
        case 'null':
            return '-';
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
// Config Section Formatting
// ============================================================================

export function formatConfigSection(context: Map<string, DxValue>, refs?: Map<string, string>): string {
    if (context.size === 0) {
        return '';
    }

    const lines: string[] = [];
    lines.push('[config]');

    const expandedKeys = Array.from(context.keys()).map(k => expandKey(k));
    const maxKeyLen = Math.max(...expandedKeys.map(k => k.length));

    for (const [abbrevKey, value] of context) {
        const fullKey = expandKey(abbrevKey);
        const padding = ' '.repeat(maxKeyLen - fullKey.length);
        const formattedValue = formatValue(value, refs);

        let displayValue = formattedValue;
        if (value.type === 'string' && /[ =\[\]{}]/.test(formattedValue)) {
            displayValue = `"${formattedValue}"`;
        }

        lines.push(`${fullKey}${padding} = ${displayValue}`);
    }

    return lines.join('\n');
}

// ============================================================================
// Data Section Formatting (Flat Rows - No Indentation)
// ============================================================================

export function formatDataSection(section: DxSection, refs?: Map<string, string>): string {
    const lines: string[] = [];

    const fullSectionName = expandSectionName(section.id);
    lines.push(`[${fullSectionName}]`);

    if (section.schema.length === 0 || section.rows.length === 0) {
        return lines.join('\n');
    }

    // Expand schema column names
    const expandedSchema = section.schema.map(col => expandKey(col));

    // Calculate column widths for alignment
    const widths: number[] = expandedSchema.map(col => col.length);
    for (const row of section.rows) {
        for (let i = 0; i < row.length && i < widths.length; i++) {
            const cellValue = formatTableValue(row[i], refs);
            widths[i] = Math.max(widths[i], cellValue.length);
        }
    }

    // Header row (no indentation)
    const headerCells = expandedSchema.map((col, i) => col.padEnd(widths[i]));
    lines.push(headerCells.join('  '));

    // Data rows (no indentation)
    for (const row of section.rows) {
        const cells = row.map((value, i) => {
            const cellValue = formatTableValue(value, refs);
            return cellValue.padEnd(widths[i] || cellValue.length);
        });
        lines.push(cells.join('  '));
    }

    return lines.join('\n');
}

// ============================================================================
// Summary Generation (kept for API compatibility but simplified)
// ============================================================================

export function generateSummary(section: DxSection, refs?: Map<string, string>): string {
    return `Total: ${section.rows.length} items`;
}

// ============================================================================
// Document Formatting
// ============================================================================

export function formatDocument(doc: DxDocument): string {
    const sections: string[] = [];

    // Config section
    if (doc.context.size > 0) {
        sections.push(formatConfigSection(doc.context, doc.refs));
    }

    // Data sections
    for (const [id, section] of doc.sections) {
        sections.push('');
        sections.push(formatDataSection(section, doc.refs));
    }

    return sections.join('\n');
}
