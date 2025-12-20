/**
 * DX Inflater - LLM-Dense → Human-Pretty Transformation
 * 
 * Transforms token-efficient LLM format into beautiful, readable format.
 * 
 * LLM-Dense: server#host:localhost#port:5432#ssl:1
 * Human-Pretty:
 *   ▼ server
 *       host: localhost
 *       port: 5432
 *       ssl:  ✓
 */

export interface InflaterConfig {
    indentSize: number;
    useUnicode: boolean;
    useBoxDrawing: boolean;
}

interface TableContext {
    key: string;
    columns: string[];
    expectedRows: number;
    rows: string[][];
    comment: string | null;
}

export class DxInflater {
    private config: InflaterConfig;

    constructor(config?: Partial<InflaterConfig>) {
        this.config = {
            indentSize: config?.indentSize ?? 4,
            useUnicode: config?.useUnicode ?? true,
            useBoxDrawing: config?.useBoxDrawing ?? true
        };
    }

    /**
     * Inflate LLM-dense format to human-pretty format
     */
    inflate(dense: string): string {
        const lines: string[] = [];
        let tableContext: TableContext | null = null;

        for (const rawLine of dense.split('\n')) {
            const line = rawLine.trim();

            if (!line) {
                if (tableContext) {
                    lines.push(...this.closeTable(tableContext));
                    tableContext = null;
                }
                lines.push('');
                continue;
            }

            const element = this.parseDenseLine(line);

            if (element.type === 'tableRow') {
                if (tableContext) {
                    tableContext.rows.push(element.values);
                }
                continue;
            }

            // Close pending table
            if (tableContext) {
                lines.push(...this.closeTable(tableContext));
                tableContext = null;
            }

            switch (element.type) {
                case 'object':
                    lines.push(...this.formatObject(element));
                    break;
                case 'array':
                    lines.push(...this.formatArray(element));
                    break;
                case 'tableHeader':
                    tableContext = {
                        key: element.key,
                        columns: element.columns,
                        expectedRows: element.rowCount,
                        rows: [],
                        comment: element.comment
                    };
                    break;
                case 'keyValue':
                    lines.push(...this.formatKeyValue(element));
                    break;
                case 'comment':
                    lines.push(`// ${element.text}`);
                    break;
            }
        }

        // Close remaining table
        if (tableContext) {
            lines.push(...this.closeTable(tableContext));
        }

        return lines.join('\n');
    }

    private parseDenseLine(line: string): any {
        let trimmed = line.trim();
        let comment: string | null = null;

        // Check for anchored comment: !text!content
        if (trimmed.startsWith('!')) {
            const endIdx = trimmed.indexOf('!', 1);
            if (endIdx > 0) {
                comment = trimmed.substring(1, endIdx);
                trimmed = trimmed.substring(endIdx + 1).trim();
                if (!trimmed) {
                    return { type: 'comment', text: comment };
                }
            }
        }

        // Table row: >val|val|val
        if (trimmed.startsWith('>')) {
            const values = trimmed.substring(1).split('|').map(s => s.trim());
            return { type: 'tableRow', values };
        }

        // Table header: key@N=col^col^col
        const atIdx = trimmed.indexOf('@');
        const eqIdx = trimmed.indexOf('=');
        if (atIdx > 0 && eqIdx > atIdx) {
            const key = trimmed.substring(0, atIdx);
            const rowCount = parseInt(trimmed.substring(atIdx + 1, eqIdx)) || 0;
            const columns = trimmed.substring(eqIdx + 1).split('^').map(s => s.trim());
            return { type: 'tableHeader', key, rowCount, columns, comment };
        }

        // Array: key@N>item|item|item
        const arrowIdx = trimmed.indexOf('>');
        if (atIdx > 0 && arrowIdx > atIdx) {
            const key = trimmed.substring(0, atIdx);
            const items = trimmed.substring(arrowIdx + 1).split('|').map(s => s.trim());
            return { type: 'array', key, items, comment };
        }

        // Object: key#field:val#field:val
        if (trimmed.includes('#')) {
            const parts = trimmed.split('#');
            const key = parts[0];
            const fields: [string, string][] = [];

            for (let i = 1; i < parts.length; i++) {
                const colonIdx = parts[i].indexOf(':');
                if (colonIdx > 0) {
                    const fieldName = parts[i].substring(0, colonIdx);
                    const fieldValue = parts[i].substring(colonIdx + 1);
                    fields.push([fieldName, fieldValue]);
                }
            }

            if (fields.length > 0) {
                return { type: 'object', key, fields, comment };
            }
        }

        // Simple key:value
        const colonIdx = trimmed.indexOf(':');
        if (colonIdx > 0) {
            const key = trimmed.substring(0, colonIdx);
            const value = trimmed.substring(colonIdx + 1);
            return { type: 'keyValue', key, value, comment };
        }

        return { type: 'comment', text: trimmed };
    }

    private formatObject(element: any): string[] {
        const result: string[] = [];
        const indent = ' '.repeat(this.config.indentSize);
        const marker = this.config.useUnicode ? '▼' : '>';

        if (element.comment) {
            result.push(`// ${element.comment}`);
        }

        result.push(`${marker} ${element.key}`);

        // Calculate max field name length for alignment
        const maxLen = Math.max(...element.fields.map(([k]: [string, string]) => k.length));

        for (const [name, value] of element.fields) {
            const padding = ' '.repeat(maxLen - name.length);
            const prettyValue = this.inflateValue(value);
            result.push(`${indent}${name}:${padding} ${prettyValue}`);
        }

        return result;
    }

    private formatArray(element: any): string[] {
        const result: string[] = [];
        const indent = ' '.repeat(this.config.indentSize);
        const marker = this.config.useUnicode ? '▼' : '>';
        const bullet = this.config.useUnicode ? '•' : '-';

        if (element.comment) {
            result.push(`// ${element.comment}`);
        }

        result.push(`${marker} ${element.key} (${element.items.length} items)`);

        for (const item of element.items) {
            const prettyValue = this.inflateValue(item);
            result.push(`${indent}${bullet} ${prettyValue}`);
        }

        return result;
    }

    private formatKeyValue(element: any): string[] {
        const result: string[] = [];

        if (element.comment) {
            result.push(`// ${element.comment}`);
        }

        const prettyValue = this.inflateValue(element.value);
        result.push(`${element.key}: ${prettyValue}`);

        return result;
    }

    private closeTable(ctx: TableContext): string[] {
        const result: string[] = [];
        const indent = ' '.repeat(this.config.indentSize);
        const marker = this.config.useUnicode ? '▼' : '>';

        if (ctx.comment) {
            result.push(`// ${ctx.comment}`);
        }

        result.push(`${marker} ${ctx.key} (${ctx.columns.length} columns × ${ctx.rows.length} rows)`);

        // Calculate column widths
        const colWidths = ctx.columns.map(c => c.length);
        for (const row of ctx.rows) {
            for (let i = 0; i < row.length; i++) {
                const pretty = this.inflateValue(row[i]);
                if (i < colWidths.length) {
                    colWidths[i] = Math.max(colWidths[i], pretty.length);
                }
            }
        }

        if (this.config.useBoxDrawing) {
            result.push(...this.formatTableBoxed(ctx, colWidths, indent));
        } else {
            result.push(...this.formatTableSimple(ctx, colWidths, indent));
        }

        return result;
    }

    private formatTableBoxed(ctx: TableContext, colWidths: number[], indent: string): string[] {
        const result: string[] = [];

        // Top border
        result.push(indent + '┌' + colWidths.map(w => '─'.repeat(w + 2)).join('┬') + '┐');

        // Header row
        const headerCells = ctx.columns.map((col, i) => col.padEnd(colWidths[i]));
        result.push(indent + '│ ' + headerCells.join(' │ ') + ' │');

        // Separator
        result.push(indent + '├' + colWidths.map(w => '─'.repeat(w + 2)).join('┼') + '┤');

        // Data rows
        for (const row of ctx.rows) {
            const cells = row.map((val, i) => {
                const pretty = this.inflateValue(val);
                return pretty.padEnd(colWidths[i] || pretty.length);
            });
            result.push(indent + '│ ' + cells.join(' │ ') + ' │');
        }

        // Bottom border
        result.push(indent + '└' + colWidths.map(w => '─'.repeat(w + 2)).join('┴') + '┘');

        return result;
    }

    private formatTableSimple(ctx: TableContext, colWidths: number[], indent: string): string[] {
        const result: string[] = [];

        // Header row
        const headerCells = ctx.columns.map((col, i) => col.padEnd(colWidths[i]));
        result.push(indent + headerCells.join(' | '));

        // Separator
        result.push(indent + colWidths.map(w => '-'.repeat(w)).join('-+-'));

        // Data rows
        for (const row of ctx.rows) {
            const cells = row.map((val, i) => {
                const pretty = this.inflateValue(val);
                return pretty.padEnd(colWidths[i] || pretty.length);
            });
            result.push(indent + cells.join(' | '));
        }

        return result;
    }

    private inflateValue(value: string): string {
        const v = value.trim();

        if (this.config.useUnicode) {
            switch (v) {
                case '1': case '+': case 'true': return '✓';
                case '0': case '-': case 'false': return '✗';
                case '~': case 'null': case 'none': return '—';
            }
        } else {
            switch (v) {
                case '1': case '+': return 'true';
                case '0': case '-': return 'false';
                case '~': return 'null';
            }
        }

        // Reference: *ref → →ref
        if (v.startsWith('*')) {
            return this.config.useUnicode ? `→${v.substring(1)}` : `>${v.substring(1)}`;
        }

        // Quoted string - show without quotes for simple strings
        if (v.startsWith('"') && v.endsWith('"') && v.length > 2) {
            const inner = v.slice(1, -1);
            if (!inner.includes('#') && !inner.includes('|') && !inner.includes('^')) {
                return inner;
            }
        }

        return v;
    }
}
