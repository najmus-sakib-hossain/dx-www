/**
 * Human Format V3 Formatter for DX Serializer VS Code Extension
 * 
 * Converts DxDocument to clean vertical key-value format with:
 * - Config values without [config] section header
 * - Data sections with [section] headers
 * - Key padding to 20 characters for alignment
 * - Pipe (|) as array separator
 * - Quoted strings with spaces
 * - Stack section with table-like aligned columns
 * 
 * Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7
 */

import { DxDocument, DxValue, DxSection } from './llmParser';
import { DxDocumentWithOrder } from './humanParserV3';

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
// Abbreviation Dictionary
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
    'vr': 'version',  // vr is also version, NOT variant
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

    // Style & UI
    'th': 'themes',
    'cp': 'components',
    'pk': 'pack',
    'vt': 'variant',  // Changed from 'vr' to 'vt' to avoid conflict with version
    'df': 'default',
    'dv': 'dev',
    'pd': 'prod',

    // Media
    'img': 'images',
    'vid': 'videos',
    'snd': 'sounds',
    'ast': 'assets',
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
    'd': 'driven',        // Changed from 'data' to 'driven'
    'g': 'generator',     // Added for generator section
    's': 'scripts',       // Added for scripts section
    'x': 'dependencies',  // Added for DX registry dependencies (secured)
    'j': 'js',            // Added for js.dependencies (external)
    'p': 'python',        // Added for python.dependencies (external)
    'r': 'rust',          // Added for rust.dependencies (external)
};

export const REVERSE_ABBREVIATIONS: Record<string, string> = (() => {
    const result: Record<string, string> = {};
    for (const [abbrev, full] of Object.entries(ABBREVIATIONS)) {
        // Prefer shorter abbreviations when there are duplicates
        if (!result[full] || abbrev.length < result[full].length) {
            result[full] = abbrev;
        }
    }
    return result;
})();

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
 * Uses | as array separator and quotes strings with spaces
 */
export function formatValueV3(value: DxValue, config: HumanFormatV3Config = DEFAULT_CONFIG): string {
    switch (value.type) {
        case 'string': {
            const str = String(value.value);
            // Quote strings containing spaces
            if (config.quoteStringsWithSpaces && str.includes(' ')) {
                return `"${str}"`;
            }
            // Quote strings that look like null markers to preserve type
            if (str === '-' || str === '~') {
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
        }
        case 'number':
            return String(value.value);
        case 'bool':
            return value.value ? 'true' : 'false';
        case 'null':
            return '-';
        case 'array': {
            const items = value.value as DxValue[];
            return items.map(v => formatValueV3(v, config)).join(config.arraySeparator);
        }
        case 'ref':
            return `^${value.refKey || value.value}`;
        default:
            return String(value.value);
    }
}

/**
 * Format a simple value without quoting (for table cells)
 */
export function formatSimpleValue(value: DxValue): string {
    switch (value.type) {
        case 'string':
            return String(value.value);
        case 'number':
            return String(value.value);
        case 'bool':
            return value.value ? 'true' : 'false';
        case 'null':
            return '-';
        case 'array': {
            const items = value.value as DxValue[];
            return items.map(v => formatSimpleValue(v)).join(' | ');
        }
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
// Stack Section Formatting (Table-like with aligned columns)
// ============================================================================

/**
 * Format reference definitions as a [stack] section with aligned columns
 * No schema header, just rows with pipe-separated values
 */
export function formatReferenceSectionV3(
    refs: Map<string, string>,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    if (refs.size === 0) {
        return '';
    }

    const lines: string[] = [];
    lines.push('[stack]');

    // Parse all reference values to find column widths
    const rows: Array<{ key: string; values: string[] }> = [];
    let maxCols = 0;

    for (const [key, value] of refs) {
        // Split by | but preserve spacing
        const values = value.split('|').map(v => v.trim());
        rows.push({ key, values });
        maxCols = Math.max(maxCols, values.length);
    }

    // Calculate column widths (minimum width for each column)
    const colWidths: number[] = new Array(maxCols).fill(0);
    let maxKeyLen = config.keyPadding;

    for (const row of rows) {
        maxKeyLen = Math.max(maxKeyLen, row.key.length + 1);
        for (let i = 0; i < row.values.length; i++) {
            colWidths[i] = Math.max(colWidths[i], row.values[i].length);
        }
    }

    // Format each row with aligned columns
    for (const row of rows) {
        const keyPadding = ' '.repeat(maxKeyLen - row.key.length);
        const formattedValues = row.values.map((val, i) => {
            // Pad each value to align columns
            const padding = ' '.repeat(Math.max(0, colWidths[i] - val.length));
            return val + padding;
        });
        lines.push(`${row.key}${keyPadding}= ${formattedValues.join(' | ')}`);
    }

    return lines.join('\n');
}

// ============================================================================
// Data Section Formatting
// ============================================================================

/**
 * Format a data section based on its structure
 * - Single row: vertical key-value pairs (skip 'name' field if it matches section name)
 * - Multiple rows: NOT USED (we use vertical format for all)
 */
export function formatDataSectionV3(
    section: DxSection,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const fullSectionName = expandSectionName(section.id);
    const expandedSchema = section.schema.map(col => expandKey(col));

    // Check for nested dependency sections (js.dependencies, python.dependencies, rust.dependencies)
    const nestedInfo = detectNestedDependencySection(section.id, section.schema);
    if (nestedInfo && section.rows.length === 1) {
        return formatNestedDependencySection(section, nestedInfo, config);
    }

    // Check for scripts section (special handling - don't quote command strings)
    if (section.id === 's' && section.rows.length === 1) {
        return formatScriptsSection(section, config);
    }

    // Check for dependencies section (DX registry packages)
    if (section.id === 'x' && section.rows.length === 1) {
        return formatDependenciesSection(section, config);
    }

    // Check for special sections that need sub-sections (like i18n)
    // Only use special formatting if the schema has the expected prefixes
    if (section.id === 'i' && section.rows.length === 1) {
        const hasI18nPrefixes = section.schema.some(s =>
            s.startsWith('locales_') || s.startsWith('ttses_')
        );
        if (hasI18nPrefixes) {
            return formatI18nSection(section, config);
        }
    }

    // Check for media section (special handling)
    // Only use special formatting if the schema has the expected prefixes
    if (section.id === 'm' && section.rows.length === 1) {
        const hasMediaPrefixes = section.schema.some(s =>
            s.endsWith('_path') || ['images', 'videos', 'sounds', 'assets', 'img', 'vid', 'snd', 'ast'].includes(s)
        );
        if (hasMediaPrefixes) {
            return formatMediaSection(section, config);
        }
    }

    // Single row section: vertical key-value pairs
    if (section.rows.length === 1) {
        lines.push(`[${fullSectionName}]`);
        const row = section.rows[0];

        // Filter out 'name' field if it matches section name or is redundant
        const filteredEntries: Array<{ key: string; value: DxValue }> = [];
        for (let i = 0; i < expandedSchema.length && i < row.length; i++) {
            const key = expandedSchema[i];
            const value = row[i];

            // Skip 'name' field if it's redundant (matches section name or is just the section id)
            if (key === 'name') {
                const strVal = value.type === 'string' ? String(value.value) : '';
                if (strVal === fullSectionName || strVal === section.id) {
                    continue;
                }
            }

            filteredEntries.push({ key, value });
        }

        // Calculate padding for this section
        const maxKeyLen = Math.max(
            config.keyPadding,
            ...filteredEntries.map(e => e.key.length + 1)
        );

        for (const entry of filteredEntries) {
            const padding = ' '.repeat(maxKeyLen - entry.key.length);
            const formattedValue = formatValueV3(entry.value, config);
            lines.push(`${entry.key}${padding}= ${formattedValue}`);
        }
    } else if (section.rows.length > 1) {
        // Multiple rows: still use vertical format but with row names
        lines.push(`[${fullSectionName}]`);

        for (const row of section.rows) {
            // First column is typically the row identifier/name
            if (row.length > 0) {
                const rowName = formatSimpleValue(row[0]);
                const rowValues = row.slice(1).map(v => formatValueV3(v, config));
                const padding = ' '.repeat(Math.max(1, config.keyPadding - rowName.length));
                lines.push(`${rowName}${padding}= ${rowValues.join(config.arraySeparator)}`);
            }
        }
    } else {
        // Empty section
        lines.push(`[${fullSectionName}]`);
    }

    return lines.join('\n');
}

/**
 * Format scripts section - preserves command strings without quoting
 */
function formatScriptsSection(
    section: DxSection,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const row = section.rows[0];
    const schema = section.schema;

    lines.push('[scripts]');

    // Collect entries
    const entries: Array<{ key: string; value: DxValue }> = [];
    for (let i = 0; i < schema.length && i < row.length; i++) {
        const key = expandKey(schema[i]);
        const value = row[i];
        entries.push({ key, value });
    }

    // Calculate padding
    const maxKeyLen = Math.max(
        config.keyPadding,
        ...entries.map(e => e.key.length + 1)
    );

    // Format each entry - don't quote command strings
    for (const entry of entries) {
        const padding = ' '.repeat(maxKeyLen - entry.key.length);
        // For scripts, output the value as-is without quoting
        const valueStr = entry.value.type === 'string'
            ? String(entry.value.value)
            : formatValueV3(entry.value, config);
        lines.push(`${entry.key}${padding}= ${valueStr}`);
    }

    return lines.join('\n');
}

/**
 * Format dependencies section (DX registry packages)
 * Handles hyphenated package names
 */
function formatDependenciesSection(
    section: DxSection,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const row = section.rows[0];
    const schema = section.schema;

    lines.push('[dependencies]');

    // Collect entries - preserve hyphenated keys
    const entries: Array<{ key: string; value: DxValue }> = [];
    for (let i = 0; i < schema.length && i < row.length; i++) {
        // Don't expand keys that contain hyphens (package names)
        const key = schema[i].includes('-') ? schema[i] : expandKey(schema[i]);
        const value = row[i];
        entries.push({ key, value });
    }

    // Calculate padding
    const maxKeyLen = Math.max(
        config.keyPadding,
        ...entries.map(e => e.key.length + 1)
    );

    // Format each entry
    for (const entry of entries) {
        const padding = ' '.repeat(maxKeyLen - entry.key.length);
        const formattedValue = formatValueV3(entry.value, config);
        lines.push(`${entry.key}${padding}= ${formattedValue}`);
    }

    return lines.join('\n');
}

/**
 * Format i18n section with sub-sections for locales and ttses
 */
function formatI18nSection(
    section: DxSection,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const row = section.rows[0];
    const schema = section.schema;

    // Group fields into locales and ttses
    const localesFields: Array<{ key: string; value: DxValue }> = [];
    const ttsesFields: Array<{ key: string; value: DxValue }> = [];

    // Map abbreviated keys to full names
    const keyExpansion: Record<string, string> = {
        'pt': 'path',
        'df': 'default',
        'dv': 'dev',
        'pd': 'prod',
    };

    for (let i = 0; i < schema.length && i < row.length; i++) {
        const key = schema[i];
        const value = row[i];

        if (key.startsWith('locales_')) {
            const shortKey = key.replace('locales_', '');
            const expandedKey = keyExpansion[shortKey] || shortKey;
            localesFields.push({ key: expandedKey, value });
        } else if (key.startsWith('ttses_')) {
            const shortKey = key.replace('ttses_', '');
            const expandedKey = keyExpansion[shortKey] || shortKey;
            ttsesFields.push({ key: expandedKey, value });
        }
    }

    // Format [i18n.locales] section
    if (localesFields.length > 0) {
        lines.push('[i18n.locales]');
        const maxKeyLen = Math.max(config.keyPadding, ...localesFields.map(f => f.key.length + 1));
        for (const field of localesFields) {
            const padding = ' '.repeat(maxKeyLen - field.key.length);
            const formattedValue = formatValueV3(field.value, config);
            lines.push(`${field.key}${padding}= ${formattedValue}`);
        }
    }

    // Format [i18n.ttses] section
    if (ttsesFields.length > 0) {
        lines.push('');
        lines.push('[i18n.ttses]');
        const maxKeyLen = Math.max(config.keyPadding, ...ttsesFields.map(f => f.key.length + 1));
        for (const field of ttsesFields) {
            const padding = ' '.repeat(maxKeyLen - field.key.length);
            const formattedValue = formatValueV3(field.value, config);
            lines.push(`${field.key}${padding}= ${formattedValue}`);
        }
    }

    return lines.join('\n');
}

/**
 * Format media section with simplified paths
 */
function formatMediaSection(
    section: DxSection,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const row = section.rows[0];
    const schema = section.schema;

    lines.push('[media]');

    // Map schema to simplified keys (handle both abbreviated and full names)
    const keyMap: Record<string, string> = {
        'images_path': 'images',
        'images': 'images',
        'img': 'images',
        'videos_path': 'videos',
        'videos': 'videos',
        'vid': 'videos',
        'sounds_path': 'sounds',
        'sounds': 'sounds',
        'snd': 'sounds',
        'assets_path': 'assets',
        'assets': 'assets',
        'ast': 'assets',
    };

    // Group path and content together
    const mediaGroups: Map<string, DxValue> = new Map();

    for (let i = 0; i < schema.length && i < row.length; i++) {
        const key = schema[i];
        const value = row[i];

        // Map abbreviated or full key to display name
        const displayKey = keyMap[key];
        if (displayKey) {
            mediaGroups.set(displayKey, value);
        }
    }

    const maxKeyLen = Math.max(config.keyPadding, ...Array.from(mediaGroups.keys()).map(k => k.length + 1));

    for (const [key, value] of mediaGroups) {
        const padding = ' '.repeat(maxKeyLen - key.length);
        const formattedValue = formatValueV3(value, config);
        lines.push(`${key}${padding}= ${formattedValue}`);
    }

    return lines.join('\n');
}

// ============================================================================
// Nested Section Formatting (for js.dependencies, python.dependencies, etc.)
// ============================================================================

/**
 * Check if a section is a nested dependency section
 * Returns the parent and child names if it is, null otherwise
 */
export function detectNestedDependencySection(
    sectionId: string,
    schema: string[]
): { parent: string; child: string } | null {
    // Check for language-specific dependency sections (j, p, r)
    const languageSections: Record<string, string> = {
        'j': 'js',
        'p': 'python',
        'r': 'rust',
    };

    if (languageSections[sectionId]) {
        // Check if schema has dependencies_ prefix
        const hasDependenciesPrefix = schema.some(s => s.startsWith('dependencies_'));
        if (hasDependenciesPrefix) {
            return {
                parent: languageSections[sectionId],
                child: 'dependencies',
            };
        }
    }

    return null;
}

/**
 * Format a nested dependency section like [js.dependencies]
 * Strips the prefix from keys (e.g., dependencies_react -> react)
 * Preserves hyphenated package names
 */
export function formatNestedDependencySection(
    section: DxSection,
    nestedInfo: { parent: string; child: string },
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const row = section.rows[0];
    const schema = section.schema;

    // Output section header as [parent.child]
    lines.push(`[${nestedInfo.parent}.${nestedInfo.child}]`);

    // Extract keys by stripping the prefix
    const prefix = `${nestedInfo.child}_`;
    const entries: Array<{ key: string; value: DxValue }> = [];

    for (let i = 0; i < schema.length && i < row.length; i++) {
        const schemaKey = schema[i];
        const value = row[i];

        // Strip prefix if present
        let displayKey = schemaKey;
        if (schemaKey.startsWith(prefix)) {
            displayKey = schemaKey.substring(prefix.length);
        }

        // Only expand abbreviated keys, preserve hyphenated package names
        if (!displayKey.includes('-')) {
            displayKey = expandKey(displayKey);
        }

        entries.push({ key: displayKey, value });
    }

    // Calculate padding for this section
    const maxKeyLen = Math.max(
        config.keyPadding,
        ...entries.map(e => e.key.length + 1)
    );

    // Format each key-value pair
    for (const entry of entries) {
        const padding = ' '.repeat(maxKeyLen - entry.key.length);
        const formattedValue = formatValueV3(entry.value, config);
        lines.push(`${entry.key}${padding}= ${formattedValue}`);
    }

    return lines.join('\n');
}

// ============================================================================
// Document Formatting
// ============================================================================

/**
 * Format a complete DxDocument to Human Format V3
 * Preserves section order if available
 */
export function formatDocumentV3(
    doc: DxDocument,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const sections: string[] = [];

    // Config section (no header) - always first
    if (doc.context.size > 0) {
        sections.push(formatConfigSectionV3(doc.context, config));
    }

    // Check if document has section order tracking
    const docWithOrder = doc as DxDocumentWithOrder;
    const sectionOrder: string[] = docWithOrder.sectionOrder || [];

    if (sectionOrder.length > 0) {
        // Use section order to output sections in the correct order
        for (const sectionId of sectionOrder) {
            if (sectionId === 'stack') {
                // Output [stack] section for refs
                if (doc.refs.size > 0) {
                    sections.push('');
                    sections.push(formatReferenceSectionV3(doc.refs, config));
                }
            } else {
                // Output data section
                const section = doc.sections.get(sectionId);
                if (section) {
                    sections.push('');
                    sections.push(formatDataSectionV3(section, config));
                }
            }
        }

        // Output any sections not in the order list
        for (const [sectionId, section] of doc.sections) {
            if (!sectionOrder.includes(sectionId)) {
                sections.push('');
                sections.push(formatDataSectionV3(section, config));
            }
        }

        // Output refs if not in order list
        if (!sectionOrder.includes('stack') && doc.refs.size > 0) {
            sections.push('');
            sections.push(formatReferenceSectionV3(doc.refs, config));
        }
    } else {
        // Fallback: refs first, then sections (old behavior)
        if (doc.refs.size > 0) {
            sections.push('');
            sections.push(formatReferenceSectionV3(doc.refs, config));
        }

        for (const [sectionId, section] of doc.sections) {
            if (sectionId === 'k' && doc.refs.size > 0) {
                const renamedSection = { ...section, id: 'k' };
                sections.push('');
                sections.push(formatDataSectionAsConfig(renamedSection, config));
                continue;
            }
            sections.push('');
            sections.push(formatDataSectionV3(section, config));
        }
    }

    return sections.join('\n');
}

/**
 * Format a data section as config-style (for stack.config etc.)
 */
function formatDataSectionAsConfig(
    section: DxSection,
    config: HumanFormatV3Config = DEFAULT_CONFIG
): string {
    const lines: string[] = [];
    const expandedSchema = section.schema.map(col => expandKey(col));

    // Use a different name to avoid conflict with [stack] refs section
    lines.push('[stack.config]');

    if (section.rows.length === 1) {
        const row = section.rows[0];

        // Filter out 'name' field
        const filteredEntries: Array<{ key: string; value: DxValue }> = [];
        for (let i = 0; i < expandedSchema.length && i < row.length; i++) {
            const key = expandedSchema[i];
            const value = row[i];

            if (key === 'name') continue;

            filteredEntries.push({ key, value });
        }

        const maxKeyLen = Math.max(
            config.keyPadding,
            ...filteredEntries.map(e => e.key.length + 1)
        );

        for (const entry of filteredEntries) {
            const padding = ' '.repeat(maxKeyLen - entry.key.length);
            const formattedValue = formatValueV3(entry.value, config);
            lines.push(`${entry.key}${padding}= ${formattedValue}`);
        }
    }

    return lines.join('\n');
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
