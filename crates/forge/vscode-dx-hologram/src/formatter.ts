/**
 * DX Formatter - Bidirectional formatting between Dense (LLM) and Pretty (Human)
 * 
 * LLM FORMAT (stored on disk):
 *   name:dx
 *   forge.repository:https://...
 *   ^container:none
 *   ^ci/cd:none
 *   forge_items>cli|docs|examples
 * 
 * HUMAN FORMAT (shown in editor):
 *   name                : dx
 *   forge.repository    : https://...
 *      container        : none
 *      ci/cd            : none
 *   forge_items         > cli | docs | examples
 * 
 * Key: ^ prefix becomes indentation (3 spaces), NO caret shown to user
 */

export class DxFormatter {
    
    private indentSize: number;
    private indent: string;
    
    constructor(indentSize: number = 3) {
        this.indentSize = indentSize;
        this.indent = ' '.repeat(indentSize);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // DENSE → PRETTY (for display in editor)
    // Removes ^ prefix and adds proper indentation
    // ═══════════════════════════════════════════════════════════════
    
    toPretty(dense: string): string {
        const lines = dense.split('\n');
        const result: string[] = [];
        
        // First pass: parse all lines
        const parsed: Array<{
            key: string;
            op: string;
            value: string;
            indent: number;
            isComment: boolean;
            isEmpty: boolean;
            isTableRow: boolean;
            raw: string;
        }> = [];
        
        for (const line of lines) {
            const trimmed = line.trim();
            
            if (!trimmed) {
                parsed.push({ key: '', op: '', value: '', indent: 0, isComment: false, isEmpty: true, isTableRow: false, raw: line });
                continue;
            }
            
            // Comment lines
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
                parsed.push({ key: '', op: '', value: '', indent: 0, isComment: true, isEmpty: false, isTableRow: false, raw: line });
                continue;
            }
            
            // Table row starting with >
            if (trimmed.startsWith('>')) {
                const values = trimmed.substring(1).split('|').map(v => v.trim());
                parsed.push({ key: '', op: '>', value: values.join('|'), indent: 0, isComment: false, isEmpty: false, isTableRow: true, raw: line });
                continue;
            }
            
            // Check for ^ prefix (child indicator) - this becomes indentation
            let content = trimmed;
            let indentLevel = 0;
            if (trimmed.startsWith('^')) {
                indentLevel = 1;
                content = trimmed.substring(1);
            }
            
            // Check for > (array)
            const arrowIdx = content.indexOf('>');
            if (arrowIdx > 0 && !content.substring(0, arrowIdx).includes(':')) {
                const key = content.substring(0, arrowIdx).trim();
                const value = content.substring(arrowIdx + 1).trim();
                parsed.push({ key, op: '>', value, indent: indentLevel, isComment: false, isEmpty: false, isTableRow: false, raw: line });
                continue;
            }
            
            // Check for : (key-value)
            const colonIdx = content.indexOf(':');
            if (colonIdx > 0) {
                const key = content.substring(0, colonIdx).trim();
                const value = content.substring(colonIdx + 1).trim();
                parsed.push({ key, op: ':', value, indent: indentLevel, isComment: false, isEmpty: false, isTableRow: false, raw: line });
                continue;
            }
            
            // Table header row or other content (multiple words separated by spaces)
            if (content.includes('  ') || /^[A-Z]/.test(content)) {
                parsed.push({ key: content, op: '', value: '', indent: 0, isComment: false, isEmpty: false, isTableRow: false, raw: line });
                continue;
            }
            
            // Simple key without value
            parsed.push({ key: content, op: '', value: '', indent: indentLevel, isComment: false, isEmpty: false, isTableRow: false, raw: line });
        }
        
        // Calculate max key length for alignment (global)
        let maxKeyLen = 0;
        for (const p of parsed) {
            if (!p.isComment && !p.isEmpty && !p.isTableRow && p.op) {
                const effectiveLen = p.key.length + (p.indent * this.indentSize);
                maxKeyLen = Math.max(maxKeyLen, effectiveLen);
            }
        }
        
        // Use 20 as minimum alignment
        const align = Math.max(maxKeyLen + 4, 20);
        
        // Format each line
        for (const p of parsed) {
            if (p.isEmpty) {
                result.push('');
                continue;
            }
            
            if (p.isComment) {
                result.push(p.raw);
                continue;
            }
            
            if (p.isTableRow) {
                // Format table row with aligned columns
                const values = p.value.split('|').map(v => v.trim());
                result.push(values.map(v => v.padEnd(15)).join('  '));
                continue;
            }
            
            if (p.op) {
                // Create indentation string (replaces ^ with spaces)
                const indentStr = this.indent.repeat(p.indent);
                const keyPadded = p.key.padEnd(align - indentStr.length);
                
                if (p.op === '>') {
                    // Format array items with spaces around pipes
                    const items = p.value.split('|').map(v => v.trim()).filter(v => v);
                    result.push(`${indentStr}${keyPadded}> ${items.join(' | ')}`);
                } else {
                    result.push(`${indentStr}${keyPadded}: ${p.value}`);
                }
            } else if (p.key) {
                // Table header or other - keep formatting
                result.push(p.raw);
            }
        }
        
        return result.join('\n');
    }
    
    // ═══════════════════════════════════════════════════════════════
    // PRETTY → DENSE (for storing on disk / LLM consumption)
    // Converts indentation back to ^ prefix
    // ═══════════════════════════════════════════════════════════════
    
    toDense(pretty: string): string {
        const lines = pretty.split('\n');
        const result: string[] = [];
        
        for (const line of lines) {
            const trimmed = line.trim();
            
            // Empty line - skip in dense
            if (!trimmed) {
                continue;
            }
            
            // Comment line - skip in dense output
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
                continue;
            }
            
            // Calculate indent level from leading spaces
            const leadingSpaces = line.length - line.trimStart().length;
            const indentLevel = Math.floor(leadingSpaces / this.indentSize);
            
            // Prefix for child items (indented lines get ^ prefix)
            const prefix = indentLevel > 0 ? '^' : '';
            
            // Check for > (array)
            const arrowMatch = trimmed.match(/^([\w_./-]+)\s*>\s*(.*)$/);
            if (arrowMatch) {
                const key = arrowMatch[1];
                const items = arrowMatch[2].split('|').map(v => v.trim()).filter(v => v).join('|');
                result.push(`${prefix}${key}>${items}`);
                continue;
            }
            
            // Check for : (key-value)
            const colonMatch = trimmed.match(/^([\w_./-]+)\s*:\s*(.*)$/);
            if (colonMatch) {
                const key = colonMatch[1];
                const value = colonMatch[2];
                result.push(`${prefix}${key}:${value}`);
                continue;
            }
            
            // Table row - compact spaces into pipe-separated format
            if (trimmed.includes('  ')) {
                const parts = trimmed.split(/\s{2,}/).map(p => p.trim()).filter(p => p);
                if (parts.length >= 2) {
                    result.push(`>${parts.join('|')}`);
                    continue;
                }
            }
            
            // Keep as-is (single words, etc)
            result.push(trimmed);
        }
        
        return result.join('\n');
    }
    
    // ═══════════════════════════════════════════════════════════════
    // Check if content is Dense or Pretty format
    // ═══════════════════════════════════════════════════════════════
    
    isDense(content: string): boolean {
        const lines = content.split('\n').filter(l => l.trim() && !l.trim().startsWith('#'));
        if (lines.length === 0) return true;
        
        let denseScore = 0;
        let prettyScore = 0;
        
        for (const line of lines.slice(0, 15)) {
            const trimmed = line.trim();
            
            // Dense indicators: no spaces around operators, uses ^ prefix
            if (/^[\w_./-]+:[^\s]/.test(trimmed)) denseScore += 2;
            if (/^[\w_./-]+>[^\s]/.test(trimmed)) denseScore += 2;
            if (trimmed.startsWith('^')) denseScore += 3;
            
            // Pretty indicators: spaces around operators, leading whitespace
            if (/\s+:\s+/.test(line)) prettyScore += 2;
            if (/\s+>\s+/.test(line)) prettyScore += 2;
            if (line.startsWith('   ') && !trimmed.startsWith('^')) prettyScore += 3;
        }
        
        return denseScore > prettyScore;
    }
}
