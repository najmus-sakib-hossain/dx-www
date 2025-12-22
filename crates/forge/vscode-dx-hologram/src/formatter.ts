/**
 * DX Formatter - Bidirectional formatting between Dense (LLM) and Pretty (Human)
 * 
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                    STORAGE vs DISPLAY                                   │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │  LLM FORMAT (Dense - stored on disk):                                   │
 * │  ════════════════════════════════════                                   │
 * │  name:dx                                                                │
 * │  forge.repository:https://dx.vercel.app/essensefromexistence/dx        │
 * │  ^container:none                                                        │
 * │  ^ci/cd:none                                                            │
 * │  forge_items>cli|docs|examples                                          │
 * │                                                                         │
 * │  HUMAN FORMAT (Pretty - shown in editor):                               │
 * │  ════════════════════════════════════════                               │
 * │  name                : dx                                               │
 * │  forge.repository    : https://dx.vercel.app/essensefromexistence/dx   │
 * │     container        : none                                             │
 * │     ci/cd            : none                                             │
 * │                                                                         │
 * │  forge_items >                                                          │
 * │     cli                                                                 │
 * │     docs                                                                │
 * │     examples                                                            │
 * │                                                                         │
 * │  ─────────────────────────────────────────────────────────────────────  │
 * │  KEY TRANSFORMATIONS:                                                   │
 * │  • ^key:value → "   key : value" (caret = child indent)                │
 * │  • Align all colons in a block                                          │
 * │  • key>a|b|c → multi-line with indented items                          │
 * │  • Tables formatted with aligned columns                                │
 * └─────────────────────────────────────────────────────────────────────────┘
 */

interface LineInfo {
    original: string;
    key: string;
    operator: string;  // ':', '>', '='
    value: string;
    indent: number;    // 0 = root, 1 = child (from ^), etc.
    isTableHeader?: boolean;
    isTableRow?: boolean;
    isComment?: boolean;
    isEmpty?: boolean;
}

export class DxFormatter {
    
    private indentSize: number;
    private indent: string;
    private colonAlign: number;  // Column to align colons at
    
    constructor(indentSize: number = 3) {
        this.indentSize = indentSize;
        this.indent = ' '.repeat(indentSize);
        this.colonAlign = 20;  // Default colon alignment column
    }
    
    // ═══════════════════════════════════════════════════════════════
    // DENSE → PRETTY (for display in editor)
    // ═══════════════════════════════════════════════════════════════
    
    toPretty(dense: string): string {
        const lines = dense.split('\n');
        const parsed: LineInfo[] = [];
        
        // First pass: parse all lines
        for (const rawLine of lines) {
            parsed.push(this.parseDenseLine(rawLine));
        }
        
        // Second pass: calculate alignment for blocks
        const blocks = this.identifyBlocks(parsed);
        
        // Third pass: format with proper alignment
        const result: string[] = [];
        
        for (let i = 0; i < parsed.length; i++) {
            const info = parsed[i];
            const block = blocks.get(i);
            
            if (info.isEmpty) {
                result.push('');
                continue;
            }
            
            if (info.isComment) {
                result.push(info.original);
                continue;
            }
            
            const formatted = this.formatPrettyLine(info, block?.colonAlign || this.colonAlign);
            result.push(...formatted);
        }
        
        return result.join('\n');
    }
    
    private parseDenseLine(line: string): LineInfo {
        const trimmed = line.trim();
        
        if (!trimmed) {
            return { original: line, key: '', operator: '', value: '', indent: 0, isEmpty: true };
        }
        
        // Comment lines
        if (trimmed.startsWith('#') && (trimmed.includes('--') || trimmed.startsWith('# '))) {
            return { original: trimmed, key: '', operator: '', value: '', indent: 0, isComment: true };
        }
        if (trimmed.startsWith('//')) {
            return { original: trimmed, key: '', operator: '', value: '', indent: 0, isComment: true };
        }
        
        // Table row: >val|val|val or >val | val | val
        if (trimmed.startsWith('>')) {
            return { 
                original: trimmed, 
                key: '', 
                operator: '>', 
                value: trimmed.substring(1), 
                indent: 0,
                isTableRow: true 
            };
        }
        
        // Check for caret prefix (child/continuation)
        let indent = 0;
        let content = trimmed;
        if (trimmed.startsWith('^')) {
            indent = 1;
            content = trimmed.substring(1);
        }
        
        // Array: key>items or key_name>items
        const arrowIdx = content.indexOf('>');
        if (arrowIdx > 0 && !content.substring(0, arrowIdx).includes(':')) {
            const key = content.substring(0, arrowIdx);
            const value = content.substring(arrowIdx + 1);
            return { original: trimmed, key, operator: '>', value, indent };
        }
        
        // Table definition: key@N=cols
        if (content.includes('@') && content.includes('=')) {
            const atIdx = content.indexOf('@');
            const eqIdx = content.indexOf('=');
            const key = content.substring(0, atIdx);
            const count = content.substring(atIdx + 1, eqIdx);
            const cols = content.substring(eqIdx + 1);
            return { 
                original: trimmed, 
                key: `${key}@${count}`, 
                operator: '=', 
                value: cols, 
                indent,
                isTableHeader: true 
            };
        }
        
        // Key:value pair
        const colonIdx = content.indexOf(':');
        if (colonIdx > 0) {
            const key = content.substring(0, colonIdx).trim();
            const value = content.substring(colonIdx + 1).trim();
            return { original: trimmed, key, operator: ':', value, indent };
        }
        
        // Just a key or identifier
        return { original: trimmed, key: content, operator: '', value: '', indent };
    }
    
    private identifyBlocks(lines: LineInfo[]): Map<number, { colonAlign: number }> {
        const blocks = new Map<number, { colonAlign: number }>();
        let blockStart = -1;
        let maxKeyLen = 0;
        let currentIndent = 0;
        
        for (let i = 0; i < lines.length; i++) {
            const info = lines[i];
            
            if (info.isEmpty || info.isComment || info.isTableRow) {
                // End current block
                if (blockStart >= 0) {
                    const align = Math.max(maxKeyLen + 4, 20);
                    for (let j = blockStart; j < i; j++) {
                        blocks.set(j, { colonAlign: align });
                    }
                }
                blockStart = -1;
                maxKeyLen = 0;
                continue;
            }
            
            // Check if continuing same block or starting new one
            if (blockStart < 0 || info.indent !== currentIndent) {
                // End previous block if exists
                if (blockStart >= 0) {
                    const align = Math.max(maxKeyLen + 4, 20);
                    for (let j = blockStart; j < i; j++) {
                        blocks.set(j, { colonAlign: align });
                    }
                }
                // Start new block
                blockStart = i;
                currentIndent = info.indent;
                maxKeyLen = info.key.length + (info.indent * this.indentSize);
            } else {
                // Continue block
                maxKeyLen = Math.max(maxKeyLen, info.key.length + (info.indent * this.indentSize));
            }
        }
        
        // Close final block
        if (blockStart >= 0) {
            const align = Math.max(maxKeyLen + 4, 20);
            for (let j = blockStart; j < lines.length; j++) {
                blocks.set(j, { colonAlign: align });
            }
        }
        
        return blocks;
    }
    
    private formatPrettyLine(info: LineInfo, colonAlign: number): string[] {
        const result: string[] = [];
        const indentStr = this.indent.repeat(info.indent);
        
        // Table row formatting
        if (info.isTableRow) {
            const values = info.value.split('|').map(v => v.trim());
            // Format as aligned table row
            result.push(values.map(v => v.padEnd(15)).join('  ').trim());
            return result;
        }
        
        // Array formatting: key>a|b|c → multi-line
        if (info.operator === '>') {
            const items = info.value.split('|').map(v => v.trim()).filter(v => v);
            result.push(`${indentStr}${info.key.padEnd(colonAlign - indentStr.length)}> ${items.join(' | ')}`);
            return result;
        }
        
        // Table header: key@N=cols
        if (info.isTableHeader) {
            const cols = info.value.split('^').map(c => c.trim());
            result.push('');
            result.push(`# ${info.key.split('@')[0].toUpperCase()} TABLE (${cols.length} Columns)`);
            result.push(`# ${'-'.repeat(60)}`);
            result.push(cols.map(c => c.padEnd(15)).join('  ').trim());
            return result;
        }
        
        // Key:value formatting with alignment
        if (info.operator === ':') {
            const keyPart = `${indentStr}${info.key}`;
            const padding = Math.max(colonAlign - keyPart.length, 1);
            result.push(`${keyPart}${' '.repeat(padding)}: ${this.prettyValue(info.value)}`);
            return result;
        }
        
        // Just a key
        if (info.key) {
            result.push(`${indentStr}${info.key}`);
        }
        
        return result;
    }
    
    private prettyValue(v: string): string {
        const trimmed = v.trim();
        if (trimmed === '+') return 'true';
        if (trimmed === '-') return 'false';
        if (trimmed === '~') return 'null';
        if (trimmed.startsWith('*')) return `ref(${trimmed.substring(1)})`;
        return trimmed;
    }
    
    // ═══════════════════════════════════════════════════════════════
    // PRETTY → DENSE (for storing on disk)
    // ═══════════════════════════════════════════════════════════════
    
    toDense(pretty: string): string {
        const lines = pretty.split('\n');
        const denseLines: string[] = [];
        
        let i = 0;
        while (i < lines.length) {
            const line = lines[i];
            const trimmed = line.trim();
            
            // Empty line - skip
            if (!trimmed) {
                i++;
                continue;
            }
            
            // Comment line - skip in dense (or convert)
            if (trimmed.startsWith('#') && !trimmed.includes(':')) {
                i++;
                continue;
            }
            if (trimmed.startsWith('//')) {
                i++;
                continue;
            }
            
            // Check indentation level (3 spaces = 1 level = ^)
            const leadingSpaces = line.length - line.trimStart().length;
            const indentLevel = Math.floor(leadingSpaces / this.indentSize);
            
            // Parse the line content
            const result = this.parsePrettyLine(trimmed, indentLevel);
            if (result) {
                denseLines.push(result);
            }
            i++;
        }
        
        return denseLines.join('\n');
    }
    
    private parsePrettyLine(line: string, indentLevel: number): string | null {
        // Table row (no prefix, just values separated by spaces)
        // Skip these as they're handled with table headers
        if (!line.includes(':') && !line.includes('>') && line.includes('  ')) {
            // Could be table row - check if all parts are simple values
            const parts = line.split(/\s{2,}/).map(p => p.trim()).filter(p => p);
            if (parts.length >= 2 && parts.every(p => /^[a-zA-Z0-9_\-./]+$/.test(p))) {
                return `>${parts.join('|')}`;
            }
        }
        
        // Array line: key > items
        const arrowMatch = line.match(/^([\w_./-]+)\s*>\s*(.*)$/);
        if (arrowMatch) {
            const key = arrowMatch[1].trim();
            const items = arrowMatch[2].split('|').map(s => this.denseValue(s.trim())).filter(s => s).join('|');
            const prefix = indentLevel > 0 ? '^' : '';
            return `${prefix}${key}>${items}`;
        }
        
        // Key : value (with spaces around colon)
        const colonMatch = line.match(/^([\w_./-]+)\s*:\s*(.*)$/);
        if (colonMatch) {
            const key = colonMatch[1].trim();
            const value = this.denseValue(colonMatch[2].trim());
            const prefix = indentLevel > 0 ? '^' : '';
            return `${prefix}${key}:${value}`;
        }
        
        // Just return the line compacted
        return line.replace(/\s+/g, '');
    }
    
    private denseValue(v: string): string {
        const lower = v.toLowerCase();
        // Only convert explicit true/false, keep other values as-is
        if (lower === 'true') return '+';
        if (lower === 'false') return '-';
        // Only convert explicit 'null' keyword, not 'none' (which is a valid string value)
        if (lower === 'null') return '~';
        if (v.startsWith('ref(') && v.endsWith(')')) {
            return '*' + v.substring(4, v.length - 1);
        }
        return v;
    }
    
    // ═══════════════════════════════════════════════════════════════
    // UTILITY: Check if content is Dense or Pretty
    // ═══════════════════════════════════════════════════════════════
    
    isDense(content: string): boolean {
        const lines = content.split('\n').filter(l => l.trim());
        if (lines.length === 0) return true;
        
        // Dense characteristics:
        // - No spaces around : or >
        // - Uses ^ for children
        // - No aligned columns
        
        let denseScore = 0;
        let prettyScore = 0;
        
        for (const line of lines.slice(0, 10)) { // Check first 10 lines
            const trimmed = line.trim();
            if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//')) continue;
            
            // Dense: key:value (no spaces)
            if (/^[\w_./-]+:[^\s]/.test(trimmed)) denseScore++;
            // Dense: uses ^ prefix
            if (trimmed.startsWith('^')) denseScore++;
            // Dense: key>items (no spaces)
            if (/^[\w_./-]+>[^\s]/.test(trimmed)) denseScore++;
            
            // Pretty: spaces around :
            if (/:\s+/.test(trimmed) && /\s+:/.test(trimmed)) prettyScore++;
            // Pretty: indented (leading spaces)
            if (line.startsWith('   ') && !line.startsWith('    ')) prettyScore++;
            // Pretty: spaces around >
            if (/>\s+/.test(trimmed)) prettyScore++;
        }
        
        return denseScore >= prettyScore;
    }
}
