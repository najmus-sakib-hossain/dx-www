/**
 * DX Inflater - LLM-Dense → Human-Pretty Transformation
 * 
 * Transforms token-efficient LLM format into beautiful, readable format.
 * 
 * LLM Format (dx file style):
 *   name:dx-ts
 *   forge.repository:https://...
 *   ^container:docker
 *   ^registry:ghcr.io/...
 *   forge_items>cli|docs|examples
 * 
 * Human-Pretty Format:
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
 */

export interface InflaterConfig {
    indentSize: number;
    useUnicode: boolean;
    useBoxDrawing: boolean;
}

interface ParsedEntry {
    type: 'simple' | 'nested' | 'continuation' | 'array' | 'table' | 'tableRow' | 'empty';
    parentKey?: string;
    key?: string;
    value?: string;
    items?: string[];
    columns?: string[];
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
        const lines = dense.split('\n');
        const result: string[] = [];
        
        // Group entries by parent key
        const groups = new Map<string, Array<{ key: string; value: string }>>();
        const arrays = new Map<string, string[]>();
        const tables = new Map<string, { columns: string[]; rows: string[][] }>();
        const rootProps: Array<{ key: string; value: string }> = [];
        
        let currentParent = '';
        let currentTable = '';
        
        for (const rawLine of lines) {
            const entry = this.parseLine(rawLine);
            
            if (entry.type === 'empty') {
                currentParent = '';
                continue;
            }
            
            if (entry.type === 'simple' && entry.key) {
                rootProps.push({ key: entry.key, value: entry.value || '' });
                currentParent = entry.key;
            } else if (entry.type === 'nested' && entry.parentKey && entry.key) {
                if (!groups.has(entry.parentKey)) {
                    groups.set(entry.parentKey, []);
                }
                groups.get(entry.parentKey)!.push({ key: entry.key, value: entry.value || '' });
                currentParent = entry.parentKey;
            } else if (entry.type === 'continuation' && entry.key) {
                if (currentParent) {
                    if (!groups.has(currentParent)) {
                        groups.set(currentParent, []);
                    }
                    groups.get(currentParent)!.push({ key: entry.key, value: entry.value || '' });
                }
            } else if (entry.type === 'array' && entry.key && entry.items) {
                arrays.set(entry.key, entry.items);
            } else if (entry.type === 'table' && entry.key && entry.columns) {
                tables.set(entry.key, { columns: entry.columns, rows: [] });
                currentTable = entry.key;
            } else if (entry.type === 'tableRow' && entry.items && currentTable) {
                const table = tables.get(currentTable);
                if (table) {
                    table.rows.push(entry.items);
                }
            }
        }
        
        // Format root properties
        if (rootProps.length > 0) {
            const maxKeyLen = Math.max(...rootProps.map(p => p.key.length));
            for (const prop of rootProps) {
                // Skip properties that have nested children
                if (!groups.has(prop.key) || prop.value) {
                    const padding = ' '.repeat(maxKeyLen - prop.key.length);
                    const prettyValue = this.inflateValue(prop.value);
                    if (prettyValue) {
                        result.push(`${prop.key}${padding} : ${prettyValue}`);
                    }
                }
            }
            if (result.length > 0) {
                result.push('');
            }
        }
        
        // Format groups (nested objects)
        for (const [parentKey, props] of groups) {
            result.push(...this.formatGroup(parentKey, props));
        }
        
        // Format arrays
        for (const [key, items] of arrays) {
            result.push(...this.formatArray(key, items));
        }
        
        // Format tables
        for (const [key, table] of tables) {
            result.push(...this.formatTable(key, table.columns, table.rows));
        }
        
        return result.join('\n').trim();
    }

    private parseLine(rawLine: string): ParsedEntry {
        const line = rawLine.trim();
        
        if (!line) {
            return { type: 'empty' };
        }
        
        // Table row: >val|val|val
        if (line.startsWith('>')) {
            const items = line.substring(1).split('|').map(s => s.trim());
            return { type: 'tableRow', items };
        }
        
        // Continuation: ^key:value or ^key value
        if (line.startsWith('^')) {
            const rest = line.substring(1);
            const colonIdx = rest.indexOf(':');
            
            if (colonIdx > 0) {
                const key = rest.substring(0, colonIdx).trim();
                const value = rest.substring(colonIdx + 1).trim();
                return { type: 'continuation', key, value };
            }
            
            const spaceIdx = rest.indexOf(' ');
            if (spaceIdx > 0) {
                const key = rest.substring(0, spaceIdx).trim();
                const value = rest.substring(spaceIdx + 1).trim();
                return { type: 'continuation', key, value };
            }
            
            return { type: 'continuation', key: rest.trim(), value: '' };
        }
        
        // Array: key>item|item|item (with optional space)
        const arrayMatch = line.match(/^([\w_]+)\s*>\s*(.+)$/);
        if (arrayMatch) {
            const items = arrayMatch[2].split('|').map(s => s.trim()).filter(s => s);
            return { type: 'array', key: arrayMatch[1], items };
        }
        
        // Table: key=col^col^col
        const tableMatch = line.match(/^([\w_]+)\s*=\s*(.+)$/);
        if (tableMatch && tableMatch[2].includes('^')) {
            const columns = tableMatch[2].split('^').map(s => s.trim());
            return { type: 'table', key: tableMatch[1], columns };
        }
        
        // Nested: parent.child:value or parent.child value
        const nestedColonMatch = line.match(/^([\w_]+)\.([\w_.]+)\s*:\s*(.*)$/);
        if (nestedColonMatch) {
            return {
                type: 'nested',
                parentKey: nestedColonMatch[1],
                key: nestedColonMatch[2],
                value: nestedColonMatch[3].trim()
            };
        }
        
        // Nested with space: parent.child value
        const nestedSpaceMatch = line.match(/^([\w_]+)\.([\w_.]+)\s+(.+)$/);
        if (nestedSpaceMatch) {
            return {
                type: 'nested',
                parentKey: nestedSpaceMatch[1],
                key: nestedSpaceMatch[2],
                value: nestedSpaceMatch[3].trim()
            };
        }
        
        // Simple: key:value or key value
        const simpleColonMatch = line.match(/^([\w_][\w_.]*)\s*:\s*(.*)$/);
        if (simpleColonMatch) {
            return { type: 'simple', key: simpleColonMatch[1], value: simpleColonMatch[2].trim() };
        }
        
        const simpleSpaceMatch = line.match(/^([\w_]+)\s+(.+)$/);
        if (simpleSpaceMatch && !simpleSpaceMatch[2].startsWith('>') && !simpleSpaceMatch[2].includes('=')) {
            return { type: 'simple', key: simpleSpaceMatch[1], value: simpleSpaceMatch[2].trim() };
        }
        
        return { type: 'empty' };
    }

    private formatGroup(parentKey: string, props: Array<{ key: string; value: string }>): string[] {
        const result: string[] = [];
        const indent = ' '.repeat(this.config.indentSize);
        const marker = this.config.useUnicode ? '▼' : '>';
        
        result.push(`${marker} ${parentKey}`);
        
        // Group by sub-parent
        const subGroups = new Map<string, Array<{ key: string; value: string }>>();
        const directProps: Array<{ key: string; value: string }> = [];
        
        for (const prop of props) {
            if (prop.key.includes('.')) {
                const [sub, ...rest] = prop.key.split('.');
                if (!subGroups.has(sub)) {
                    subGroups.set(sub, []);
                }
                subGroups.get(sub)!.push({ key: rest.join('.'), value: prop.value });
            } else {
                directProps.push(prop);
            }
        }
        
        // Format direct properties with alignment
        if (directProps.length > 0) {
            const maxKeyLen = Math.max(...directProps.map(p => p.key.length));
            for (const prop of directProps) {
                const padding = ' '.repeat(maxKeyLen - prop.key.length);
                const prettyValue = this.inflateValue(prop.value);
                result.push(`${indent}${prop.key}${padding} : ${prettyValue}`);
            }
        }
        
        // Format sub-groups
        for (const [subKey, subProps] of subGroups) {
            result.push(`${indent}${marker} ${subKey}`);
            const innerIndent = indent + indent;
            const maxKeyLen = Math.max(...subProps.map(p => p.key.length));
            for (const prop of subProps) {
                const padding = ' '.repeat(maxKeyLen - prop.key.length);
                const prettyValue = this.inflateValue(prop.value);
                result.push(`${innerIndent}${prop.key}${padding} : ${prettyValue}`);
            }
        }
        
        result.push('');
        return result;
    }

    private formatArray(key: string, items: string[]): string[] {
        const result: string[] = [];
        const indent = ' '.repeat(this.config.indentSize);
        const marker = this.config.useUnicode ? '▼' : '>';
        const bullet = this.config.useUnicode ? '•' : '-';
        
        result.push(`${marker} ${key} (${items.length} items)`);
        for (const item of items) {
            result.push(`${indent}${bullet} ${this.inflateValue(item)}`);
        }
        result.push('');
        
        return result;
    }

    private formatTable(key: string, columns: string[], rows: string[][]): string[] {
        const result: string[] = [];
        const indent = ' '.repeat(this.config.indentSize);
        const marker = this.config.useUnicode ? '▼' : '>';
        
        result.push(`${marker} ${key} (${columns.length} columns × ${rows.length} rows)`);
        
        // Calculate column widths
        const colWidths = columns.map(c => c.length);
        for (const row of rows) {
            for (let i = 0; i < row.length && i < colWidths.length; i++) {
                colWidths[i] = Math.max(colWidths[i], row[i].length);
            }
        }
        
        if (this.config.useBoxDrawing) {
            // Top border
            result.push(indent + '┌' + colWidths.map(w => '─'.repeat(w + 2)).join('┬') + '┐');
            
            // Header
            const headerCells = columns.map((c, i) => c.padEnd(colWidths[i]));
            result.push(indent + '│ ' + headerCells.join(' │ ') + ' │');
            
            // Separator
            result.push(indent + '├' + colWidths.map(w => '─'.repeat(w + 2)).join('┼') + '┤');
            
            // Data rows
            for (const row of rows) {
                const cells = row.map((v, i) => v.padEnd(colWidths[i] || v.length));
                result.push(indent + '│ ' + cells.join(' │ ') + ' │');
            }
            
            // Bottom border
            result.push(indent + '└' + colWidths.map(w => '─'.repeat(w + 2)).join('┴') + '┘');
        } else {
            const headerCells = columns.map((c, i) => c.padEnd(colWidths[i]));
            result.push(indent + headerCells.join(' | '));
            result.push(indent + colWidths.map(w => '-'.repeat(w)).join('-+-'));
            
            for (const row of rows) {
                const cells = row.map((v, i) => v.padEnd(colWidths[i] || v.length));
                result.push(indent + cells.join(' | '));
            }
        }
        
        result.push('');
        return result;
    }

    private inflateValue(value: string): string {
        const v = value.trim();
        
        if (!v) return '';
        
        if (this.config.useUnicode) {
            switch (v) {
                case '1': case '+': case 'true': return '✓';
                case '0': case '-': case 'false': return '✗';
                case '~': case 'null': case 'none': return '—';
            }
        }
        
        // Remove surrounding quotes
        if (v.startsWith('"') && v.endsWith('"') && v.length > 2) {
            return v.slice(1, -1);
        }
        
        return v;
    }
}
