/**
 * DX Deflater - Human-Pretty → LLM-Dense Transformation
 * 
 * Transforms readable human format into ultra-compact LLM format.
 * 
 * Human Format (shown in editor - with proper spacing):
 *   name                : dx-ts
 *   version             : 0.0.1
 *   forge.repository    : https://...
 *   ^container          : docker
 *   forge_items         > cli | docs | examples
 * 
 * LLM Format (stored on disk - ultra compact):
 *   name:dx-ts
 *   version:0.0.1
 *   forge.repository:https://...
 *   ^container:docker
 *   forge_items>cli|docs|examples
 */

export class DxDeflater {
    /**
     * Deflate human-pretty format to LLM-dense format
     * 
     * Rules:
     * - Remove spaces around : (key:value)
     * - Remove key padding
     * - Remove spaces around | in arrays (item|item|item)
     * - Remove spaces around > in arrays (key>item|item)
     * - Preserve tables as-is
     * - Preserve empty lines (but collapse multiple)
     */
    deflate(pretty: string): string {
        const lines = pretty.split('\n');
        const result: string[] = [];
        let lastWasEmpty = false;
        
        for (const line of lines) {
            const trimmed = line.trim();
            
            // Empty line - keep one
            if (!trimmed) {
                if (!lastWasEmpty) {
                    result.push('');
                    lastWasEmpty = true;
                }
                continue;
            }
            
            lastWasEmpty = false;
            
            // Table header or row (preserve as-is)
            if (this.isTableHeader(trimmed) || this.isTableRow(line)) {
                result.push(line);
                continue;
            }
            
            // Compact the line
            result.push(this.compactLine(trimmed));
        }
        
        // Remove trailing empty lines
        while (result.length > 0 && result[result.length - 1] === '') {
            result.pop();
        }
        
        return result.join('\n');
    }

    private isTableHeader(line: string): boolean {
        // Table header: key=col^col^col
        return /^\w+=\w+(\^\w+)+$/.test(line.replace(/\s/g, ''));
    }

    private isTableRow(line: string): boolean {
        // Table row has multiple space-separated columns
        return /^\w+\s{2,}\w/.test(line) || /^[\w\/]+\s{2,}/.test(line);
    }

    private compactLine(line: string): string {
        // Continuation: ^key : value → ^key:value
        if (line.startsWith('^')) {
            const rest = line.substring(1);
            const colonIdx = rest.indexOf(':');
            if (colonIdx > 0) {
                const key = rest.substring(0, colonIdx).trim();
                const value = rest.substring(colonIdx + 1).trim();
                return `^${key}:${value}`;
            }
            return line;
        }
        
        // Array: key > item | item → key>item|item
        const arrowMatch = line.match(/^([\w_.]+)\s*>\s*(.+)$/);
        if (arrowMatch) {
            const key = arrowMatch[1].trim();
            const items = arrowMatch[2].split('|').map(s => s.trim()).join('|');
            return `${key}>${items}`;
        }
        
        // Table header: keep as-is
        if (line.includes('=') && line.includes('^')) {
            return line;
        }
        
        // Simple key : value → key:value
        const colonMatch = line.match(/^([\w_.]+)\s*:\s*(.*)$/);
        if (colonMatch) {
            const key = colonMatch[1].trim();
            const value = colonMatch[2].trim();
            return `${key}:${value}`;
        }
        
        return line;
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
                lines.push(`${fullKey}:${value}`);
            } else if (Array.isArray(value)) {
                const items = value.map(v => String(v)).join('|');
                lines.push(`${fullKey}>${items}`);
            } else if (typeof value === 'object') {
                lines.push(this.objectToDense(value, fullKey));
            }
        }

        return lines.join('\n');
    }
}
