/**
 * DX Deflater - Human-Pretty → LLM-Dense Transformation
 * 
 * Transforms beautiful, readable format back to token-efficient LLM format.
 * 
 * Human-Pretty:
 *   name : dx-ts
 *   
 *   ▼ forge
 *       repository : https://...
 *       container  : docker
 *       registry   : ghcr.io/...
 * 
 *   ▼ forge_items (7 items)
 *       • cli
 *       • docs
 *       • examples
 * 
 * LLM Format:
 *   name:dx-ts
 *   forge.repository:https://...
 *   ^container:docker
 *   ^registry:ghcr.io/...
 *   forge_items>cli|docs|examples
 */

interface DeflateState {
    parentKey: string;
    inArray: boolean;
    arrayKey: string;
    arrayItems: string[];
    inTable: boolean;
    tableKey: string;
    tableColumns: string[];
    tableRows: string[][];
    lastEmittedParent: string;
}

export class DxDeflater {
    /**
     * Deflate human-pretty format to LLM-dense format
     */
    deflate(pretty: string): string {
        const lines = pretty.split('\n');
        const result: string[] = [];
        const state: DeflateState = {
            parentKey: '',
            inArray: false,
            arrayKey: '',
            arrayItems: [],
            inTable: false,
            tableKey: '',
            tableColumns: [],
            tableRows: [],
            lastEmittedParent: ''
        };

        for (const rawLine of lines) {
            const line = rawLine;
            const trimmed = line.trim();

            // Empty line - flush state
            if (!trimmed) {
                result.push(...this.flushState(state));
                result.push('');
                state.parentKey = '';
                state.lastEmittedParent = '';
                continue;
            }

            // Skip box-drawing borders
            if (/^[┌├└┐┤┘┬┴┼─]+$/.test(trimmed) || /^[+\-|]+$/.test(trimmed.replace(/\s/g, ''))) {
                continue;
            }

            // Section header: ▼ key or > key (with optional count)
            const sectionMatch = trimmed.match(/^[▼>]\s+(\w+)(?:\s+\((\d+)\s*(?:items|columns).*\))?$/);
            if (sectionMatch) {
                result.push(...this.flushState(state));
                state.parentKey = sectionMatch[1];
                
                // Check if next lines are array items or table
                // Will be determined by content
                continue;
            }

            // Nested section: indent + ▼ key
            const nestedSectionMatch = trimmed.match(/^[▼>]\s+(\w+)$/);
            if (nestedSectionMatch && line.startsWith('    ')) {
                // This is a sub-section
                state.parentKey = state.parentKey + '.' + nestedSectionMatch[1];
                continue;
            }

            // Table header row: │ col │ col │
            if (trimmed.includes('│') && !trimmed.startsWith('│ ')) {
                continue; // Border line
            }
            
            if (trimmed.startsWith('│') && trimmed.endsWith('│')) {
                const cells = trimmed.split('│')
                    .map(c => c.trim())
                    .filter(c => c.length > 0);
                
                if (cells.length > 0) {
                    if (!state.inTable) {
                        state.inTable = true;
                        state.tableKey = state.parentKey;
                        state.tableColumns = cells;
                        state.tableRows = [];
                    } else {
                        state.tableRows.push(cells.map(c => this.deflateValue(c)));
                    }
                }
                continue;
            }

            // Bullet item: • item or - item
            const bulletMatch = trimmed.match(/^[•\-]\s+(.+)$/);
            if (bulletMatch) {
                if (!state.inArray) {
                    state.inArray = true;
                    state.arrayKey = state.parentKey;
                    state.arrayItems = [];
                }
                state.arrayItems.push(this.deflateValue(bulletMatch[1]));
                continue;
            }

            // Key-value with alignment: key : value (spaces around colon)
            const kvMatch = trimmed.match(/^([\w_.]+)\s*:\s*(.*)$/);
            if (kvMatch) {
                const key = kvMatch[1].trim();
                const value = this.deflateValue(kvMatch[2].trim());
                
                // Check indentation to determine if nested
                const leadingSpaces = line.match(/^(\s*)/)?.[1].length || 0;
                
                if (leadingSpaces > 0 && state.parentKey) {
                    // Nested under parent
                    if (state.lastEmittedParent !== state.parentKey) {
                        // First property under this parent - use parent.key format
                        result.push(`${state.parentKey}.${key}:${value}`);
                        state.lastEmittedParent = state.parentKey;
                    } else {
                        // Continuation - use ^ format
                        result.push(`^${key}:${value}`);
                    }
                } else {
                    // Root level
                    result.push(`${key}:${value}`);
                }
                continue;
            }
        }

        // Flush remaining state
        result.push(...this.flushState(state));

        // Clean up
        while (result.length > 0 && result[result.length - 1] === '') {
            result.pop();
        }

        // Remove duplicate empty lines
        const cleaned: string[] = [];
        let lastWasEmpty = false;
        for (const line of result) {
            if (line === '') {
                if (!lastWasEmpty) {
                    cleaned.push(line);
                    lastWasEmpty = true;
                }
            } else {
                cleaned.push(line);
                lastWasEmpty = false;
            }
        }

        return cleaned.join('\n');
    }

    private flushState(state: DeflateState): string[] {
        const result: string[] = [];

        if (state.inArray && state.arrayItems.length > 0) {
            result.push(`${state.arrayKey}>${state.arrayItems.join('|')}`);
            state.inArray = false;
            state.arrayKey = '';
            state.arrayItems = [];
        }

        if (state.inTable && state.tableColumns.length > 0) {
            result.push(`${state.tableKey}=${state.tableColumns.join('^')}`);
            for (const row of state.tableRows) {
                result.push(`>${row.join('|')}`);
            }
            state.inTable = false;
            state.tableKey = '';
            state.tableColumns = [];
            state.tableRows = [];
        }

        return result;
    }

    private deflateValue(value: string): string {
        const v = value.trim();

        // Boolean values
        switch (v) {
            case '✓': case 'true': case 'yes': return 'true';
            case '✗': case 'false': case 'no': return 'false';
            case '—': case 'null': case 'none': return '~';
        }

        // Quote strings with special characters
        if (v.includes('|') || v.includes('^') || v.includes('>')) {
            if (!v.startsWith('"')) {
                return `"${v}"`;
            }
        }

        return v;
    }

    /**
     * Convert JSON to LLM-dense format
     */
    jsonToDense(jsonStr: string): string {
        const obj = JSON.parse(jsonStr);
        return this.objectToDense(obj);
    }

    private objectToDense(obj: any, prefix: string = ''): string {
        const lines: string[] = [];

        for (const [key, value] of Object.entries(obj)) {
            const fullKey = prefix ? `${prefix}.${key}` : key;

            if (value === null) {
                lines.push(`${fullKey}:~`);
            } else if (typeof value === 'boolean') {
                lines.push(`${fullKey}:${value ? 'true' : 'false'}`);
            } else if (typeof value === 'number' || typeof value === 'string') {
                const strVal = String(value);
                const needsQuote = strVal.includes('|') || strVal.includes('^') || strVal.includes('>');
                lines.push(`${fullKey}:${needsQuote ? `"${strVal}"` : strVal}`);
            } else if (Array.isArray(value)) {
                if (this.isTableData(value)) {
                    const columns = Object.keys(value[0]);
                    lines.push(`${fullKey}=${columns.join('^')}`);
                    for (const row of value) {
                        const vals = columns.map(c => this.primitiveToString((row as any)[c]));
                        lines.push(`>${vals.join('|')}`);
                    }
                } else {
                    const items = value.map(v => this.primitiveToString(v));
                    lines.push(`${fullKey}>${items.join('|')}`);
                }
            } else if (typeof value === 'object') {
                const fields = Object.entries(value);
                const allPrimitive = fields.every(([_, v]) =>
                    v === null || typeof v === 'boolean' || typeof v === 'number' || typeof v === 'string'
                );

                if (allPrimitive && fields.length > 0) {
                    // Use first property with parent.child format, rest with ^
                    const [firstKey, firstVal] = fields[0];
                    lines.push(`${fullKey}.${firstKey}:${this.primitiveToString(firstVal)}`);
                    for (let i = 1; i < fields.length; i++) {
                        const [k, v] = fields[i];
                        lines.push(`^${k}:${this.primitiveToString(v)}`);
                    }
                } else {
                    lines.push(this.objectToDense(value, fullKey));
                }
            }
        }

        return lines.join('\n');
    }

    private isTableData(arr: any[]): boolean {
        if (arr.length === 0) return false;
        return arr.every(item =>
            item !== null &&
            typeof item === 'object' &&
            !Array.isArray(item) &&
            Object.values(item).every(v =>
                v === null || typeof v === 'boolean' || typeof v === 'number' || typeof v === 'string'
            )
        );
    }

    private primitiveToString(value: any): string {
        if (value === null) return '~';
        if (typeof value === 'boolean') return value ? 'true' : 'false';
        const str = String(value);
        if (str.includes('|') || str.includes('^') || str.includes('>')) {
            return `"${str}"`;
        }
        return str;
    }
}
