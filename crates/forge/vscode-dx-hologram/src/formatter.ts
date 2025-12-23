/**
 * DX Formatter - Bidirectional Dense ↔ Pretty
 * 
 * NEW DX FORMAT:
 * 
 * Dense (LLM):
 *   name=dx
 *   stack=Lang|Runtime|Compiler
 *   [forge]
 *   repository=https://example.com
 *   friends[3]=ana,luis,sam
 *   hikes[3]{id,name,distance}=
 *   1,BlueLake,7.5
 * 
 * Pretty (Human):
 *   name                 = dx
 *   stack                = Lang | Runtime | Compiler
 *   
 *   [forge]
 *   repository           = https://example.com
 *   
 *   friends[3]           = ana, luis, sam
 *   
 *   hikes[3]{id, name, distance} =
 *     1, BlueLake, 7.5
 */

export class DxFormatter {
    private indentSize: number;
    private indent: string;

    constructor(indentSize: number = 3) {
        this.indentSize = indentSize;
        this.indent = ' '.repeat(indentSize);
    }

    /**
     * Convert Dense (LLM) format to Pretty (Human) format
     * - Add spaces around = 
     * - Add spaces around | in pipe-separated values
     * - Add spaces after commas
     * - Align keys
     * - Add blank lines before sections
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
            
            const eqIdx = trimmed.indexOf('=');
            if (eqIdx > 0) {
                const key = trimmed.substring(0, eqIdx);
                maxKeyLen = Math.max(maxKeyLen, key.length);
            } else {
                maxKeyLen = Math.max(maxKeyLen, trimmed.length);
            }
        }
        
        const alignCol = Math.max(maxKeyLen + 1, 20);
        let prevWasSection = false;
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
                // Add blank line before section (unless at start)
                if (result.length > 0 && result[result.length - 1] !== '') {
                    result.push('');
                }
                result.push(trimmed);
                prevWasSection = true;
                inTableData = false;
                continue;
            }
            
            // Check if this is table data (starts with number or is continuation)
            if (inTableData && /^\d/.test(trimmed)) {
                // Table row - add indent and space after commas
                const formatted = trimmed.split(',').map(v => v.trim()).join(', ');
                result.push(this.indent + formatted);
                continue;
            }
            
            // Parse key=value
            const eqIdx = trimmed.indexOf('=');
            
            if (eqIdx < 0) {
                // No = sign, just a key (like "context")
                result.push(trimmed);
                inTableData = false;
                continue;
            }
            
            const key = trimmed.substring(0, eqIdx);
            const value = trimmed.substring(eqIdx + 1);
            
            // Check if this is a table header (ends with }= )
            if (key.includes('{') && key.includes('}')) {
                // Table definition - format schema
                const bracketIdx = key.indexOf('{');
                const closeBracket = key.indexOf('}');
                const basePart = key.substring(0, bracketIdx);
                const schema = key.substring(bracketIdx + 1, closeBracket);
                const formattedSchema = schema.split(',').map(s => s.trim()).join(', ');
                const formattedKey = `${basePart}{${formattedSchema}}`;
                
                const paddedKey = formattedKey.padEnd(alignCol);
                result.push(`${paddedKey} =`);
                inTableData = true;
                
                // If there's inline data after =, format it
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
                
                // Format the value (comma or pipe separated)
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
            
            // Format pipe-separated values
            if (value.includes('|')) {
                formattedValue = value.split('|').map(v => v.trim()).join(' | ');
            }
            
            const paddedKey = key.padEnd(alignCol);
            result.push(`${paddedKey} = ${formattedValue}`);
            inTableData = false;
            prevWasSection = false;
        }
        
        return result.join('\n');
    }

    /**
     * Convert Pretty (Human) format to Dense (LLM) format
     * - Remove spaces around =
     * - Remove spaces around | 
     * - Remove spaces after commas
     * - Remove blank lines
     * - Remove comments
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
                    // Table row - compact commas
                    const compacted = trimmed.split(',').map(v => v.trim()).join(',');
                    result.push(compacted);
                    continue;
                }
            }
            
            // Parse key = value
            const eqIdx = trimmed.indexOf('=');
            
            if (eqIdx < 0) {
                // No = sign, just a key
                result.push(trimmed.replace(/\s+/g, ''));
                continue;
            }
            
            let key = trimmed.substring(0, eqIdx).trim();
            let value = trimmed.substring(eqIdx + 1).trim();
            
            // Remove spaces from key (but preserve brackets and braces)
            key = key.replace(/\s+/g, '');
            
            // Compact schema in braces: {id, name} -> {id,name}
            key = key.replace(/\{\s*([^}]+)\s*\}/g, (match, inner) => {
                return '{' + inner.split(',').map((s: string) => s.trim()).join(',') + '}';
            });
            
            // Compact value
            if (value.includes('|')) {
                // Pipe-separated: remove spaces around pipes
                value = value.split('|').map(v => v.trim()).join('|');
            } else if (value.includes(',')) {
                // Comma-separated: remove spaces after commas
                value = value.split(',').map(v => v.trim()).join(',');
            }
            
            result.push(`${key}=${value}`);
        }
        
        return result.join('\n');
    }
}
