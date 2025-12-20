/**
 * DX Deflater - Human-Pretty → LLM-Dense Transformation
 * 
 * Transforms beautiful, readable format back to token-efficient LLM format.
 * 
 * Human-Pretty:
 *   ▼ server
 *       host: localhost
 *       port: 5432
 *       ssl:  ✓
 * 
 * LLM-Dense: server#host:localhost#port:5432#ssl:1
 */

interface DeflateState {
    currentKey: string;
    fields: [string, string][];
    inTable: boolean;
    tableColumns: string[];
    tableRows: string[][];
    inArray: boolean;
    arrayItems: string[];
    comment: string | null;
}

export class DxDeflater {
    /**
     * Deflate human-pretty format to LLM-dense format
     */
    deflate(pretty: string): string {
        const lines = pretty.split('\n');
        const result: string[] = [];
        const state: DeflateState = {
            currentKey: '',
            fields: [],
            inTable: false,
            tableColumns: [],
            tableRows: [],
            inArray: false,
            arrayItems: [],
            comment: null
        };

        for (const rawLine of lines) {
            const line = rawLine.trim();

            // Empty line - flush state
            if (!line) {
                result.push(...this.flushState(state));
                result.push('');
                continue;
            }

            // Comment line
            if (line.startsWith('//')) {
                const text = line.substring(2).trim();
                state.comment = text;
                continue;
            }

            // Section header: ▼ key or > key
            const sectionMatch = line.match(/^[▼>]\s+(.+?)(?:\s+\(.*\))?$/);
            if (sectionMatch) {
                result.push(...this.flushState(state));
                state.currentKey = sectionMatch[1];
                continue;
            }

            // Table header row: │ col │ col │ or col | col
            if (line.includes('│') || (state.inTable && line.includes('|'))) {
                if (line.startsWith('┌') || line.startsWith('├') || line.startsWith('└') ||
                    line.startsWith('+-') || line.includes('---')) {
                    // Border line - skip but mark we're in table
                    if (!state.inTable) {
                        state.inTable = true;
                    }
                    continue;
                }

                const cells = line.split(/[│|]/)
                    .map(c => c.trim())
                    .filter(c => c.length > 0);

                if (cells.length > 0) {
                    state.inTable = true;
                    if (state.tableColumns.length === 0) {
                        state.tableColumns = cells;
                    } else {
                        state.tableRows.push(cells.map(c => this.deflateValue(c)));
                    }
                }
                continue;
            }

            // Bullet item: • item or - item
            const bulletMatch = line.match(/^[•\-]\s+(.+)$/);
            if (bulletMatch) {
                state.inArray = true;
                state.arrayItems.push(this.deflateValue(bulletMatch[1]));
                continue;
            }

            // Simple key: value inside object
            const kvMatch = line.match(/^(\w+):\s*(.*)$/);
            if (kvMatch && state.currentKey) {
                const value = this.deflateValue(kvMatch[2]);
                state.fields.push([kvMatch[1], value]);
                continue;
            }

            // Top-level key: value
            if (kvMatch && !state.currentKey) {
                result.push(...this.flushState(state));
                const value = this.deflateValue(kvMatch[2]);
                const commentPrefix = state.comment ? `!${state.comment}!` : '';
                result.push(`${commentPrefix}${kvMatch[1]}:${value}`);
                state.comment = null;
                continue;
            }
        }

        // Flush remaining state
        result.push(...this.flushState(state));

        // Clean up trailing empty lines
        while (result.length > 0 && result[result.length - 1] === '') {
            result.pop();
        }

        return result.join('\n');
    }

    private flushState(state: DeflateState): string[] {
        const result: string[] = [];
        const commentPrefix = state.comment ? `!${state.comment}!` : '';

        if (state.inTable && state.tableColumns.length > 0) {
            // Table: key@N=col^col^col
            result.push(`${commentPrefix}${state.currentKey}@${state.tableRows.length}=${state.tableColumns.join('^')}`);
            for (const row of state.tableRows) {
                result.push(`>${row.join('|')}`);
            }
        } else if (state.inArray && state.arrayItems.length > 0) {
            // Array: key@N>item|item|item
            result.push(`${commentPrefix}${state.currentKey}@${state.arrayItems.length}>${state.arrayItems.join('|')}`);
        } else if (state.currentKey && state.fields.length > 0) {
            // Object: key#field:val#field:val
            const fieldParts = state.fields.map(([k, v]) => `${k}:${v}`).join('#');
            result.push(`${commentPrefix}${state.currentKey}#${fieldParts}`);
        }

        // Reset state
        state.currentKey = '';
        state.fields = [];
        state.inTable = false;
        state.tableColumns = [];
        state.tableRows = [];
        state.inArray = false;
        state.arrayItems = [];
        state.comment = null;

        return result;
    }

    private deflateValue(value: string): string {
        const v = value.trim();

        // Boolean values
        switch (v) {
            case '✓': case 'true': case 'yes': return '1';
            case '✗': case 'false': case 'no': return '0';
            case '—': case 'null': case 'none': return '~';
        }

        // Reference: →ref → *ref
        if (v.startsWith('→') || v.startsWith('>')) {
            return `*${v.substring(1)}`;
        }

        // Quote strings with special characters
        if (v.includes('#') || v.includes('|') || v.includes('^') || v.includes(' ')) {
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
                lines.push(`${fullKey}:${value ? '1' : '0'}`);
            } else if (typeof value === 'number' || typeof value === 'string') {
                const strVal = String(value);
                const needsQuote = strVal.includes('#') || strVal.includes('|') || strVal.includes('^');
                lines.push(`${fullKey}:${needsQuote ? `"${strVal}"` : strVal}`);
            } else if (Array.isArray(value)) {
                if (this.isTableData(value)) {
                    // Format as table
                    const columns = Object.keys(value[0]);
                    lines.push(`${fullKey}@${value.length}=${columns.join('^')}`);
                    for (const row of value) {
                        const vals = columns.map(c => this.primitiveToString((row as any)[c]));
                        lines.push(`>${vals.join('|')}`);
                    }
                } else {
                    // Format as array
                    const items = value.map(v => this.primitiveToString(v));
                    lines.push(`${fullKey}@${items.length}>${items.join('|')}`);
                }
            } else if (typeof value === 'object') {
                // Nested object - check if flat or recursive
                const fields = Object.entries(value);
                const allPrimitive = fields.every(([_, v]) =>
                    v === null || typeof v === 'boolean' || typeof v === 'number' || typeof v === 'string'
                );

                if (allPrimitive && fields.length > 0) {
                    // Flat object
                    const fieldParts = fields.map(([k, v]) => `${k}:${this.primitiveToString(v)}`).join('#');
                    lines.push(`${fullKey}#${fieldParts}`);
                } else {
                    // Recursive
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
        if (typeof value === 'boolean') return value ? '1' : '0';
        const str = String(value);
        if (str.includes('#') || str.includes('|') || str.includes('^')) {
            return `"${str}"`;
        }
        return str;
    }
}
