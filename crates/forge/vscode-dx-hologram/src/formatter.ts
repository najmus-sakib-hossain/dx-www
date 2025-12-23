/**
 * DX Formatter - Bidirectional Dense ↔ Pretty
 * 
 * NEW DX FORMAT:
 * 
 * Dense (LLM):
 *   name=dx
 *   description="Orchestrate don't just own your code"
 *   repository=https://example.com/path
 *   stack=Lang|Runtime|Compiler
 * 
 * Pretty (Human):
 *   name                 = dx
 *   description          = "Orchestrate don't just own your code"
 *   repository           = https://example.com/path
 *   stack                = Lang | Runtime | Compiler
 * 
 * Special handling:
 * - URLs with :// are preserved (not treated as key:value)
 * - Values with special chars (', ", spaces) are quoted
 */

export class DxFormatter {
    private indentSize: number;
    private indent: string;

    constructor(indentSize: number = 3) {
        this.indentSize = indentSize;
        this.indent = ' '.repeat(indentSize);
    }

    /**
     * Check if a value needs to be quoted
     */
    private needsQuotes(value: string): boolean {
        // Already quoted
        if (value.startsWith('"') && value.endsWith('"')) return false;
        // Contains special characters that need quoting
        return /['\s]/.test(value) && !value.includes('|') && !value.includes(',');
    }

    /**
     * Ensure value is properly quoted if needed
     */
    private quoteIfNeeded(value: string): string {
        if (this.needsQuotes(value)) {
            // Remove existing quotes if any, then re-quote
            const unquoted = value.replace(/^["']|["']$/g, '');
            return `"${unquoted}"`;
        }
        return value;
    }

    /**
     * Find the first = that is the key-value separator (not part of URL or value)
     */
    private findKeyValueSeparator(line: string): number {
        // Find first = that's not preceded by : (like in https:// or http://)
        let i = 0;
        while (i < line.length) {
            if (line[i] === '=') {
                // Check if this is after :// (URL scheme)
                const before = line.substring(0, i);
                if (!before.endsWith(':') && !before.endsWith(':/')) {
                    return i;
                }
            }
            i++;
        }
        return line.indexOf('=');
    }

    /**
     * Convert Dense (LLM) format to Pretty (Human) format
     */
    toPretty(content: string, isRootConfig: boolean = false): string {
        const lines = content.split('\n');
        const result: string[] = [];
        
        // First pass: find max key length for alignment
        let maxKeyLen = 0;
        for (const line of lines) {
            const trimmed = line.trim();
            if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//') || trimmed.startsWith('[')) {
                continue;
            }
            
            const eqIdx = this.findKeyValueSeparator(trimmed);
            if (eqIdx > 0) {
                const key = trimmed.substring(0, eqIdx);
                maxKeyLen = Math.max(maxKeyLen, key.length);
            } else {
                maxKeyLen = Math.max(maxKeyLen, trimmed.length);
            }
        }
        
        const alignCol = Math.max(maxKeyLen + 1, 20);
        let inTableData = false;
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const trimmed = line.trim();
            
            // Empty lines - preserve
            if (!trimmed) {
                result.push('');
                continue;
            }
            
            // Comments - preserve
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
                result.push(trimmed);
                continue;
            }
            
            // Section headers like [forge], [style], [media]
            if (trimmed.startsWith('[') && trimmed.endsWith(']')) {
                if (result.length > 0 && result[result.length - 1] !== '') {
                    result.push('');
                }
                result.push(trimmed);
                inTableData = false;
                continue;
            }
            
            // Check if this is table data (starts with number or is continuation)
            if (inTableData && /^\d/.test(trimmed)) {
                const formatted = trimmed.split(',').map(v => v.trim()).join(', ');
                result.push(this.indent + formatted);
                continue;
            }
            
            // Parse key=value using smart separator detection
            const eqIdx = this.findKeyValueSeparator(trimmed);
            
            if (eqIdx < 0) {
                // No = sign, just a key (like "context")
                result.push(trimmed);
                inTableData = false;
                continue;
            }
            
            const key = trimmed.substring(0, eqIdx);
            let value = trimmed.substring(eqIdx + 1);
            
            // Check if this is a table header (ends with }= )
            if (key.includes('{') && key.includes('}')) {
                const bracketIdx = key.indexOf('{');
                const closeBracket = key.indexOf('}');
                const basePart = key.substring(0, bracketIdx);
                const schema = key.substring(bracketIdx + 1, closeBracket);
                const formattedSchema = schema.split(',').map(s => s.trim()).join(', ');
                const formattedKey = `${basePart}{${formattedSchema}}`;
                
                const paddedKey = formattedKey.padEnd(alignCol);
                result.push(`${paddedKey} =`);
                inTableData = true;
                
                if (value.trim()) {
                    const formatted = value.split(',').map(v => v.trim()).join(', ');
                    result.push(this.indent + formatted);
                }
                continue;
            }
            
            // Check if this is an array (has [n] but no {})
            if (key.includes('[') && !key.includes('{')) {
                const bracketIdx = key.indexOf('[');
                const basePart = key.substring(0, bracketIdx);
                const countPart = key.substring(bracketIdx);
                
                let formattedValue = value;
                if (value.includes('|')) {
                    formattedValue = value.split('|').map(v => v.trim()).join(' | ');
                } else if (value.includes(',')) {
                    formattedValue = value.split(',').map(v => v.trim()).join(', ');
                }
                
                const paddedKey = (basePart + countPart).padEnd(alignCol);
                result.push(`${paddedKey} = ${formattedValue}`);
                inTableData = false;
                continue;
            }
            
            // Regular key=value
            let formattedValue = value;
            
            // Format pipe-separated values (but not URLs)
            if (value.includes('|') && !value.includes('://')) {
                formattedValue = value.split('|').map(v => v.trim()).join(' | ');
            }
            
            // Quote if needed
            formattedValue = this.quoteIfNeeded(formattedValue);
            
            const paddedKey = key.padEnd(alignCol);
            result.push(`${paddedKey} = ${formattedValue}`);
            inTableData = false;
        }
        
        return result.join('\n');
    }

    /**
     * Convert Pretty (Human) format to Dense (LLM) format
     */
    toDense(content: string): string {
        const lines = content.split('\n');
        const result: string[] = [];
        
        for (const line of lines) {
            const trimmed = line.trim();
            
            // Skip empty lines
            if (!trimmed) continue;
            
            // Skip comments
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) continue;
            
            // Section headers - keep as-is
            if (trimmed.startsWith('[') && trimmed.endsWith(']')) {
                result.push(trimmed);
                continue;
            }
            
            // Table data rows (indented, starts with number)
            if (line.startsWith(' ') || line.startsWith('\t')) {
                if (/^\s*\d/.test(line)) {
                    const compacted = trimmed.split(',').map(v => v.trim()).join(',');
                    result.push(compacted);
                    continue;
                }
            }
            
            // Parse key = value (with spaces)
            const eqMatch = trimmed.match(/^([^=]+?)\s*=\s*(.*)$/);
            
            if (!eqMatch) {
                // No = sign, just a key
                result.push(trimmed.replace(/\s+/g, ''));
                continue;
            }
            
            let key = eqMatch[1].trim();
            let value = eqMatch[2].trim();
            
            // Remove spaces from key (but preserve brackets and braces)
            key = key.replace(/\s+/g, '');
            
            // Compact schema in braces: {id, name} -> {id,name}
            key = key.replace(/\{\s*([^}]+)\s*\}/g, (match, inner) => {
                return '{' + inner.split(',').map((s: string) => s.trim()).join(',') + '}';
            });
            
            // Compact value (but preserve quoted strings and URLs)
            if (!value.startsWith('"') && !value.includes('://')) {
                if (value.includes('|')) {
                    value = value.split('|').map(v => v.trim()).join('|');
                } else if (value.includes(',') && !value.startsWith('"')) {
                    value = value.split(',').map(v => v.trim()).join(',');
                }
            }
            
            result.push(`${key}=${value}`);
        }
        
        return result.join('\n');
    }
}
