/**
 * DX Formatter - Bidirectional formatting between Dense and Pretty
 * 
 * Key principle: SAME syntax, DIFFERENT whitespace
 * 
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                    SAME SYNTAX, DIFFERENT FORMAT                        │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │  LLM FORMAT (Dense):                    HUMAN FORMAT (Pretty):          │
 * │  ══════════════════                     ════════════════════            │
 * │                                                                         │
 * │  server#host:localhost#port:5432        server                          │
 * │  users@3>alice|bob|charlie                #host: localhost              │
 * │                                           #port: 5432                   │
 * │                                                                         │
 * │                                         users @3                        │
 * │                                           >alice                        │
 * │                                           >bob                          │
 * │                                           >charlie                      │
 * │                                                                         │
 * │  ─────────────────────────────────────────────────────────────────────  │
 * │                                                                         │
 * │  BOTH ARE VALID DX SYNTAX!                                              │
 * │  • Same tokens: # @ > | : ^                                             │
 * │  • Same semantics                                                       │
 * │  • Just different whitespace/newlines                                   │
 * │  • LLMs can read BOTH (but dense = fewer tokens)                        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 */

export class DxFormatter {
    
    private indentSize: number;
    private indent: string;
    
    constructor(indentSize: number = 2) {
        this.indentSize = indentSize;
        this.indent = ' '.repeat(indentSize);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // DENSE → PRETTY (for display in editor)
    // ═══════════════════════════════════════════════════════════════
    
    toPretty(dense: string): string {
        const lines: string[] = [];
        
        for (const rawLine of dense.split('\n')) {
            const line = rawLine.trim();
            if (!line) {
                lines.push('');
                continue;
            }
            
            // Handle comments: !comment text!content
            if (line.startsWith('!')) {
                const endIdx = line.indexOf('!', 1);
                if (endIdx > 0) {
                    const comment = line.substring(1, endIdx);
                    const rest = line.substring(endIdx + 1);
                    lines.push(`// ${comment}`);
                    if (rest.trim()) {
                        lines.push(...this.expandLine(rest.trim()));
                    }
                    continue;
                }
            }
            
            lines.push(...this.expandLine(line));
        }
        
        return lines.join('\n');
    }
    
    private expandLine(line: string): string[] {
        const result: string[] = [];
        
        // Table row: >val|val|val
        if (line.startsWith('>')) {
            const values = line.substring(1).split('|');
            result.push(this.indent + '>' + values.map(v => v.trim()).join(' | '));
            return result;
        }
        
        // Table definition: key@N=col^col^col
        if (line.includes('@') && line.includes('=')) {
            const atIdx = line.indexOf('@');
            const eqIdx = line.indexOf('=');
            const key = line.substring(0, atIdx);
            const count = line.substring(atIdx + 1, eqIdx);
            const cols = line.substring(eqIdx + 1).split('^');
            
            result.push(`${key} @${count}`);
            result.push(this.indent + '=' + cols.map(c => c.trim()).join(' ^ '));
            return result;
        }
        
        // Array: key@N>item|item|item or key>item|item|item
        const arrowMatch = line.match(/^([\w_.]+)(@\d+)?>/);
        if (arrowMatch) {
            const key = arrowMatch[1];
            const count = arrowMatch[2] || '';
            const arrowIdx = line.indexOf('>');
            const items = line.substring(arrowIdx + 1).split('|');
            
            if (count) {
                result.push(`${key} ${count}`);
            } else {
                result.push(key);
            }
            
            for (const item of items) {
                const trimmed = item.trim();
                if (trimmed) {
                    result.push(this.indent + '>' + trimmed);
                }
            }
            return result;
        }
        
        // Object: key#field:val#field:val
        if (line.includes('#') && !line.startsWith('#')) {
            const parts = line.split('#');
            const key = parts[0];
            result.push(key);
            
            for (let i = 1; i < parts.length; i++) {
                const part = parts[i];
                if (part.includes(':')) {
                    const colonIdx = part.indexOf(':');
                    const fieldName = part.substring(0, colonIdx);
                    const fieldValue = part.substring(colonIdx + 1);
                    result.push(`${this.indent}#${fieldName}: ${this.prettyValue(fieldValue)}`);
                } else {
                    result.push(`${this.indent}#${part}`);
                }
            }
            return result;
        }
        
        // Continuation: ^key:value → ^key : value (with alignment)
        if (line.startsWith('^')) {
            const rest = line.substring(1);
            const colonIdx = rest.indexOf(':');
            if (colonIdx > 0) {
                const key = rest.substring(0, colonIdx).trim();
                const value = rest.substring(colonIdx + 1).trim();
                result.push(`^${key} : ${this.prettyValue(value)}`);
                return result;
            }
        }
        
        // Simple key:value → key : value (with alignment for multi-line blocks)
        const colonIdx = line.indexOf(':');
        if (colonIdx > 0 && !line.substring(0, colonIdx).includes(' ')) {
            const key = line.substring(0, colonIdx).trim();
            const value = line.substring(colonIdx + 1).trim();
            result.push(`${key} : ${this.prettyValue(value)}`);
            return result;
        }
        
        // Simple line (e.g., table rows, etc.)
        result.push(line);
        return result;
    }
    
    private prettyValue(v: string): string {
        const trimmed = v.trim();
        if (trimmed === '1' || trimmed === '+') return 'true';
        if (trimmed === '0' || trimmed === '-') return 'false';
        if (trimmed === '~') return 'null';
        if (trimmed.startsWith('*')) return `ref(${trimmed.substring(1)})`;
        return trimmed;
    }
    
    // ═══════════════════════════════════════════════════════════════
    // PRETTY → DENSE (for storing on disk)
    // ═══════════════════════════════════════════════════════════════
    
    toDense(pretty: string): string {
        const denseLines: string[] = [];
        const lines = pretty.split('\n');
        
        let i = 0;
        while (i < lines.length) {
            const line = lines[i];
            const trimmed = line.trim();
            
            // Empty line - skip
            if (!trimmed) {
                i++;
                continue;
            }
            
            // Comment line: // comment → !comment!
            if (trimmed.startsWith('//')) {
                const comment = trimmed.substring(2).trim();
                // Look ahead and attach to next content
                let nextContent = '';
                let j = i + 1;
                while (j < lines.length) {
                    const nextLine = lines[j].trim();
                    if (nextLine && !nextLine.startsWith('//')) {
                        // Collect this structure
                        const result = this.collectStructure(lines, j);
                        nextContent = result.dense;
                        i = result.nextIndex;
                        break;
                    }
                    j++;
                }
                if (nextContent) {
                    denseLines.push(`!${comment}!${nextContent}`);
                } else {
                    i++;
                }
                continue;
            }
            
            // Collect structure starting at this line
            const result = this.collectStructure(lines, i);
            if (result.dense) {
                denseLines.push(result.dense);
            }
            i = result.nextIndex;
        }
        
        return denseLines.join('\n');
    }
    
    private collectStructure(lines: string[], startIdx: number): { dense: string; nextIndex: number } {
        const firstLine = lines[startIdx].trim();
        
        // Already compact table row: >val|val|val
        if (firstLine.startsWith('>') && !firstLine.includes(' ')) {
            return { dense: firstLine, nextIndex: startIdx + 1 };
        }
        
        // Table row with spaces: > val | val | val
        if (firstLine.startsWith('>')) {
            const values = firstLine.substring(1).split('|').map(v => v.trim());
            return { dense: `>${values.join('|')}`, nextIndex: startIdx + 1 };
        }
        
        // Check for array or table header: key @N or just key followed by indented items
        if (firstLine.includes('@') && !firstLine.includes('#') && !firstLine.includes(':')) {
            return this.collectArrayOrTable(lines, startIdx);
        }
        
        // Check if next line is indented with # (object fields)
        if (startIdx + 1 < lines.length) {
            const nextLine = lines[startIdx + 1];
            if ((nextLine.startsWith(' ') || nextLine.startsWith('\t')) && nextLine.trim().startsWith('#')) {
                return this.collectObject(lines, startIdx);
            }
            // Check for indented > (array items)
            if ((nextLine.startsWith(' ') || nextLine.startsWith('\t')) && nextLine.trim().startsWith('>')) {
                return this.collectArrayItems(lines, startIdx);
            }
        }
        
        // Single line - compact it
        return { dense: this.compactLine(firstLine), nextIndex: startIdx + 1 };
    }
    
    private collectObject(lines: string[], startIdx: number): { dense: string; nextIndex: number } {
        const key = lines[startIdx].trim();
        const fields: string[] = [];
        let i = startIdx + 1;
        
        while (i < lines.length) {
            const line = lines[i];
            // Check if indented (part of this object)
            if (line.startsWith(' ') || line.startsWith('\t')) {
                const trimmed = line.trim();
                if (trimmed.startsWith('#')) {
                    // Field line: #name: value or #name : value
                    const content = trimmed.substring(1); // Remove leading #
                    const colonIdx = content.indexOf(':');
                    if (colonIdx > 0) {
                        const fieldName = content.substring(0, colonIdx).trim();
                        const fieldValue = this.denseValue(content.substring(colonIdx + 1).trim());
                        fields.push(`${fieldName}:${fieldValue}`);
                    } else {
                        fields.push(content.trim());
                    }
                    i++;
                } else {
                    // Not a field - end of object
                    break;
                }
            } else {
                // Not indented - end of object
                break;
            }
        }
        
        const dense = fields.length > 0 ? `${key}#${fields.join('#')}` : key;
        return { dense, nextIndex: i };
    }
    
    private collectArrayItems(lines: string[], startIdx: number): { dense: string; nextIndex: number } {
        const key = lines[startIdx].trim();
        const items: string[] = [];
        let i = startIdx + 1;
        
        while (i < lines.length) {
            const line = lines[i];
            if (line.startsWith(' ') || line.startsWith('\t')) {
                const trimmed = line.trim();
                if (trimmed.startsWith('>')) {
                    const item = trimmed.substring(1).trim();
                    if (item) {
                        items.push(this.denseValue(item));
                    }
                    i++;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        const dense = items.length > 0 ? `${key}>${items.join('|')}` : key;
        return { dense, nextIndex: i };
    }
    
    private collectArrayOrTable(lines: string[], startIdx: number): { dense: string; nextIndex: number } {
        const header = lines[startIdx].trim();
        // Parse: key @N or key@N
        const atMatch = header.match(/^([\w_.]+)\s*@(\d+)$/);
        if (!atMatch) {
            return { dense: this.compactLine(header), nextIndex: startIdx + 1 };
        }
        
        const key = atMatch[1];
        const count = atMatch[2];
        
        let i = startIdx + 1;
        const items: string[] = [];
        const columns: string[] = [];
        let isTable = false;
        
        while (i < lines.length) {
            const line = lines[i];
            if (line.startsWith(' ') || line.startsWith('\t')) {
                const trimmed = line.trim();
                
                // Column definition: =col ^ col ^ col
                if (trimmed.startsWith('=')) {
                    isTable = true;
                    const colDef = trimmed.substring(1);
                    columns.push(...colDef.split('^').map(c => c.trim()));
                    i++;
                }
                // Row: >val | val | val or >val
                else if (trimmed.startsWith('>')) {
                    const rowContent = trimmed.substring(1);
                    if (rowContent.includes('|')) {
                        const values = rowContent.split('|').map(v => this.denseValue(v.trim()));
                        items.push(`>${values.join('|')}`);
                    } else {
                        items.push(this.denseValue(rowContent.trim()));
                    }
                    i++;
                }
                else {
                    break;
                }
            } else {
                break;
            }
        }
        
        let dense: string;
        if (isTable && columns.length > 0) {
            dense = `${key}@${count}=${columns.join('^')}\n${items.join('\n')}`;
        } else if (items.length > 0) {
            // Array: join items with |
            const arrayItems = items.map(item => item.startsWith('>') ? item.substring(1) : item);
            dense = `${key}@${count}>${arrayItems.join('|')}`;
        } else {
            dense = `${key}@${count}`;
        }
        
        return { dense, nextIndex: i };
    }
    
    private compactLine(line: string): string {
        // Continuation: ^key : value → ^key:value
        if (line.startsWith('^')) {
            const rest = line.substring(1);
            const colonMatch = rest.match(/^([\w_.]+)\s*:\s*(.*)$/);
            if (colonMatch) {
                const key = colonMatch[1].trim();
                const value = this.denseValue(colonMatch[2].trim());
                return `^${key}:${value}`;
            }
            return line;
        }
        
        // Array: key > item | item → key>item|item
        const arrowMatch = line.match(/^([\w_.]+)\s*>\s*(.+)$/);
        if (arrowMatch) {
            const key = arrowMatch[1].trim();
            const items = arrowMatch[2].split('|').map(s => this.denseValue(s.trim())).join('|');
            return `${key}>${items}`;
        }
        
        // Simple key : value → key:value
        const colonMatch = line.match(/^([\w_.]+)\s*:\s*(.*)$/);
        if (colonMatch) {
            const key = colonMatch[1].trim();
            const value = this.denseValue(colonMatch[2].trim());
            return `${key}:${value}`;
        }
        
        // Remove extra spaces around operators
        return line
            .replace(/\s*#\s*/g, '#')
            .replace(/\s*:\s*/g, ':')
            .replace(/\s*\|\s*/g, '|')
            .replace(/\s*\^\s*/g, '^')
            .replace(/\s*@\s*/g, '@')
            .replace(/\s*>\s*/g, '>');
    }
    
    private denseValue(v: string): string {
        const lower = v.toLowerCase();
        if (lower === 'true' || lower === 'yes') return '+';
        if (lower === 'false' || lower === 'no') return '-';
        if (lower === 'null' || lower === 'none') return '~';
        if (v.startsWith('ref(') && v.endsWith(')')) {
            return '*' + v.substring(4, v.length - 1);
        }
        // Remove quotes if simple string
        if (v.startsWith('"') && v.endsWith('"')) {
            const inner = v.slice(1, -1);
            if (!inner.includes('#') && !inner.includes('|') && !inner.includes(':')) {
                return inner;
            }
        }
        return v;
    }
}
