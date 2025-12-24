/**
 * DX Inflater - LLM-Dense → Human-Pretty Transformation
 * 
 * Transforms ultra-compact LLM format into readable human format.
 * 
 * LLM Format (stored on disk - ultra compact):
 *   name:dx-ts
 *   version:0.0.1
 *   forge.repository:https://...
 *   ^container:docker
 *   forge_items>cli|docs|examples
 * 
 * Human Format (shown in editor - with proper spacing):
 *   name                : dx-ts
 *   version             : 0.0.1
 *   forge.repository    : https://...
 *   ^container          : docker
 *   forge_items         > cli | docs | examples
 */

export interface InflaterConfig {
    keyWidth: number;
}

export class DxInflater {
    private config: InflaterConfig;

    constructor(config?: Partial<InflaterConfig>) {
        this.config = {
            keyWidth: config?.keyWidth ?? 20
        };
    }

    /**
     * Inflate LLM-dense format to human-pretty format
     * 
     * Rules:
     * - Add spaces around : (key : value)
     * - Pad keys to align colons
     * - Add spaces around | in arrays (item | item | item)
     * - Add spaces around > in arrays (key > item | item)
     * - Preserve tables as-is
     * - Preserve empty lines
     */
    inflate(dense: string): string {
        const lines = dense.split('\n');
        const result: string[] = [];
        
        // First pass: find max key width per section
        let maxKeyWidth = this.config.keyWidth;
        
        for (const line of lines) {
            const trimmed = line.trim();
            if (!trimmed) continue;
            
            // Skip table rows
            if (this.isTableRow(trimmed)) continue;
            
            const keyWidth = this.getKeyWidth(trimmed);
            if (keyWidth > 0 && keyWidth < 30) {
                maxKeyWidth = Math.max(maxKeyWidth, keyWidth);
            }
        }
        
        // Second pass: format each line
        for (const line of lines) {
            const trimmed = line.trim();
            
            // Empty line
            if (!trimmed) {
                result.push('');
                continue;
            }
            
            // Table header or row (preserve as-is for tables)
            if (this.isTableHeader(trimmed) || this.isTableRow(trimmed)) {
                result.push(line);
                continue;
            }
            
            // Format the line
            result.push(this.formatLine(trimmed, maxKeyWidth));
        }
        
        return result.join('\n');
    }

    private isTableHeader(line: string): boolean {
        // Table header: key=col^col^col
        return /^\w+=\w+(\^\w+)+$/.test(line.replace(/\s/g, ''));
    }

    private isTableRow(line: string): boolean {
        // Table row starts with data or is aligned with spaces
        return /^\w+\s{2,}\w/.test(line) || /^[\w\/]+\s{2,}/.test(line);
    }

    private getKeyWidth(line: string): number {
        // Continuation: ^key:value
        if (line.startsWith('^')) {
            const rest = line.substring(1);
            const colonIdx = rest.indexOf(':');
            if (colonIdx > 0) return colonIdx + 1; // +1 for ^
        }
        
        // Array: key>items
        const arrowIdx = line.indexOf('>');
        if (arrowIdx > 0 && !line.substring(0, arrowIdx).includes(':')) {
            return arrowIdx;
        }
        
        // Simple: key:value
        const colonIdx = line.indexOf(':');
        if (colonIdx > 0) {
            return colonIdx;
        }
        
        return 0;
    }

    private formatLine(line: string, maxKeyWidth: number): string {
        // Continuation: ^key:value → ^key          : value
        if (line.startsWith('^')) {
            const rest = line.substring(1);
            const colonIdx = rest.indexOf(':');
            if (colonIdx > 0) {
                const key = rest.substring(0, colonIdx).trim();
                const value = rest.substring(colonIdx + 1).trim();
                const paddedKey = ('^' + key).padEnd(maxKeyWidth);
                return `${paddedKey} : ${value}`;
            }
            return line;
        }
        
        // Array: key>items → key         > item | item | item
        const arrowIdx = line.indexOf('>');
        if (arrowIdx > 0 && !line.substring(0, arrowIdx).includes(':')) {
            const key = line.substring(0, arrowIdx).trim();
            const items = line.substring(arrowIdx + 1).trim();
            const paddedKey = key.padEnd(maxKeyWidth);
            // Add spaces around pipes
            const formattedItems = items.split('|').map(s => s.trim()).join(' | ');
            return `${paddedKey} > ${formattedItems}`;
        }
        
        // Table header: keep as-is
        if (line.includes('=') && line.includes('^')) {
            return line;
        }
        
        // Simple key:value → key         : value
        const colonIdx = line.indexOf(':');
        if (colonIdx > 0) {
            const key = line.substring(0, colonIdx).trim();
            const value = line.substring(colonIdx + 1).trim();
            const paddedKey = key.padEnd(maxKeyWidth);
            return `${paddedKey} : ${value}`;
        }
        
        return line;
    }
}
