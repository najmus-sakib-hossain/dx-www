/**
 * DX Formatter - Bidirectional formatting between Dense (LLM) and Pretty (Human)
 * 
 * Dense (LLM) Format:
 *   name:dx
 *   forge.repository:https://...
 *   ^container:none
 *   ^ci/cd:none
 *   forge_items>cli|docs|examples
 * 
 * Pretty (Human) Format:
 *   name                : dx
 *   forge.repository    : https://...
 *      container        : none
 *      ci/cd            : none
 *   forge_items         > cli | docs | examples
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
    // ═══════════════════════════════════════════════════════════════
    
    toPretty(dense: string): string {
        const lines = dense.split('\n');
        const result: string[] = [];
        
        // First pass: parse and find max key length for alignment
        const parsed: Array<{
            key: string;
            op: string;
            value: string;
            indent: number;
            isComment: boolean;
            isEmpty: boolean;
            raw: string;
        }> = [];
        
        for (const line of lines) {
            const trimmed = line.trim();
            
            if (!trimmed) {
                parsed.push({ key: '', op: '', value: '', indent: 0, isComment: false, isEmpty: true, raw: line });
                continue;
            }
            
            // Comment lines
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
                parsed.push({ key: '', op: '', value: '', indent: 0, isComment: true, isEmpty: false, raw: line });
                continue;
            }
            
            // Check for ^ prefix (child indicator)
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
                parsed.push({ key, op: '>', value, indent: indentLevel, isComment: false, isEmpty: false, raw: line });
                continue;
            }
            
            // Check for : (key-value)
            const colonIdx = content.indexOf(':');
            if (colonIdx > 0) {
                const key = content.substring(0, colonIdx).trim();
                const value = content.substring(colonIdx + 1).trim();
                parsed.push({ key, op: ':', value, indent: indentLevel, isComment: false, isEmpty: false, raw: line });
                continue;
            }
            
            // Table row or other content
            parsed.push({ key: content, op: '', value: '', indent: 0, isComment: false, isEmpty: false, raw: line });
        }
        
        // Calculate max key length for alignment (global across all levels)
        let maxKeyLen = 0;
        for (const p of parsed) {
            if (!p.isComment && !p.isEmpty && p.op) {
                // For child items with ^, account for the ^ prefix
                const effectiveLen = p.indent > 0 ? p.key.length + 1 : p.key.length;
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
            
            if (p.op) {
                // For child items (indent > 0), use ^ prefix with proper spacing
                let keyPart: string;
                if (p.indent > 0) {
                    keyPart = '^' + p.key.padEnd(align - 1);
                } else {
                    keyPart = p.key.padEnd(align);
                }
                
                if (p.op === '>') {
                    // Format array items with spaces around pipes
                    const items = p.value.split('|').map(v => v.trim()).filter(v => v);
                    result.push(`${keyPart}> ${items.join(' | ')}`);
                } else {
                    result.push(`${keyPart}: ${p.value}`);
                }
            } else {
                // Table row or other - keep as is
                result.push(p.raw);
            }
        }
        
        return result.join('\n');
    }
    
    // ═══════════════════════════════════════════════════════════════
    // PRETTY → DENSE (for storing on disk / LLM consumption)
    // ═══════════════════════════════════════════════════════════════
    
    toDense(pretty: string): string {
        const lines = pretty.split('\n');
        const result: string[] = [];
        
        for (const line of lines) {
            const trimmed = line.trim();
            
            // Empty line - skip
            if (!trimmed) {
                continue;
            }
            
            // Comment line - skip in dense output
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
                continue;
            }
            
            // Check for ^ prefix first (caret child items)
            if (trimmed.startsWith('^')) {
                const content = trimmed.substring(1).trim();
                
                // Check for : (key-value)
                const colonMatch = content.match(/^([\w_./-]+)\s*:\s*(.*)$/);
                if (colonMatch) {
                    const key = colonMatch[1];
                    const value = colonMatch[2];
                    result.push(`^${key}:${value}`);
                    continue;
                }
                
                // Check for > (array)
                const arrowMatch = content.match(/^([\w_./-]+)\s*>\s*(.*)$/);
                if (arrowMatch) {
                    const key = arrowMatch[1];
                    const items = arrowMatch[2].split('|').map(v => v.trim()).filter(v => v).join('|');
                    result.push(`^${key}>${items}`);
                    continue;
                }
                
                // Just a key with caret
                result.push(`^${content.replace(/\s+/g, '')}`);
                continue;
            }
            
            // Calculate indent level from leading spaces (for indented lines without ^)
            const leadingSpaces = line.length - line.trimStart().length;
            const indentLevel = Math.floor(leadingSpaces / this.indentSize);
            
            // Prefix for child items (indented without explicit ^)
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
            
            // Table row - compact spaces
            if (trimmed.includes('  ')) {
                const parts = trimmed.split(/\s{2,}/).map(p => p.trim()).filter(p => p);
                if (parts.length >= 2) {
                    result.push(`>${parts.join('|')}`);
                    continue;
                }
            }
            
            // Keep as-is
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
        
        for (const line of lines.slice(0, 10)) {
            // Dense: no spaces around : or >
            if (/^[^:>]+:[^\s]/.test(line.trim())) denseScore++;
            if (/^[^:>]+>[^\s]/.test(line.trim())) denseScore++;
            
            // Pretty: spaces around : or >
            if (/\s+:\s+/.test(line)) prettyScore++;
            if (/\s+>\s+/.test(line)) prettyScore++;
            
            // Pretty: leading whitespace (indentation)
            if (line.startsWith('   ')) prettyScore++;
        }
        
        return denseScore > prettyScore;
    }
}
