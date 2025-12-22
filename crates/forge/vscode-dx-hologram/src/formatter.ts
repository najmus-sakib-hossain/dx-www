/**
 * DX Formatter - Bidirectional Dense ↔ Pretty
 * 
 * Dense (LLM):   name:dx, ^container:none
 * Pretty (Human): name       : dx
 *                 container  : none
 * 
 * The ^ symbol should NEVER appear in Human format.
 */

export class DxFormatter {
    private indentSize: number;
    private indent: string;

    constructor(indentSize: number = 3) {
        this.indentSize = indentSize;
        this.indent = ' '.repeat(indentSize);
    }

    /**
     * Convert to Human-readable Pretty format
     * - Remove ALL ^ prefixes, convert to indentation
     * - Add spaces around : and >
     * - Add section breaks
     * - Add comments for root 'dx' config file
     * - Align keys
     */
    toPretty(content: string, isRootConfig: boolean = false): string {
        const lines = content.split('\n');
        
        interface Parsed {
            indentLevel: number;
            key: string;
            op: string;
            value: string;
            isContent: boolean;
            raw: string;
            section: string;
        }

        const parsed: Parsed[] = [];
        let maxKeyLen = 0;
        let lastSection = '';

        // First pass: parse all lines
        for (const line of lines) {
            const trimmed = line.trim();
            
            // Empty lines
            if (!trimmed) {
                parsed.push({ indentLevel: 0, key: '', op: '', value: '', isContent: false, raw: '', section: '' });
                continue;
            }

            // Comments - preserve as-is
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) {
                parsed.push({ indentLevel: 0, key: '', op: '', value: '', isContent: false, raw: trimmed, section: '' });
                continue;
            }

            // Determine indent level - strip ALL ^ and convert to indent
            let work = trimmed;
            let indentLevel = 0;

            // Strip ^ prefix (can be multiple: ^^key would be level 2)
            while (work.startsWith('^')) {
                indentLevel++;
                work = work.substring(1);
            }

            // If no ^, check for existing whitespace indent
            if (indentLevel === 0) {
                const leadSpaces = line.length - line.trimStart().length;
                indentLevel = Math.floor(leadSpaces / this.indentSize);
            }

            work = work.trim();

            // Detect section changes (keys with dots indicate new sections)
            const dotIdx = work.indexOf('.');
            const section = dotIdx > 0 ? work.substring(0, dotIdx) : '';

            // Parse operator and key/value
            const arrowIdx = work.indexOf('>');
            const colonIdx = work.indexOf(':');

            let opIdx = -1;
            let op = '';

            if (arrowIdx > 0 && (colonIdx < 0 || arrowIdx < colonIdx)) {
                opIdx = arrowIdx;
                op = '>';
            } else if (colonIdx > 0) {
                opIdx = colonIdx;
                op = ':';
            }

            if (opIdx < 0) {
                // No operator - just content
                parsed.push({ indentLevel, key: work, op: '', value: '', isContent: true, raw: line, section });
                maxKeyLen = Math.max(maxKeyLen, work.length);
                lastSection = section;
                continue;
            }

            const key = work.substring(0, opIdx).trim();
            const value = work.substring(opIdx + 1).trim();

            parsed.push({ indentLevel, key, op, value, isContent: true, raw: line, section });
            maxKeyLen = Math.max(maxKeyLen, key.length + (indentLevel * this.indentSize));
            lastSection = section;
        }

        // Second pass: format with alignment and section breaks
        const alignCol = Math.max(maxKeyLen + 2, 18);
        const result: string[] = [];
        let prevSection = '';
        
        // Add comments for root dx config
        const sectionComments: Record<string, string> = isRootConfig ? {
            'stack': '# Technology Stack',
            'forge': '# Forge Configuration',
            'style': '# Style Configuration',
            'ui': '# UI Components',
            'media': '# Media Assets',
            'i18n': '# Internationalization',
            'icon': '# Icon Configuration',
            'font': '# Font Configuration',
            'workspace': '# Workspace & Editors'
        } : {};

        for (let i = 0; i < parsed.length; i++) {
            const p = parsed[i];

            // Add blank line before new section (unless at start)
            if (i > 0 && p.section && p.section !== prevSection && p.indentLevel === 0) {
                result.push('');
                
                // Add section comment for root config
                if (isRootConfig && sectionComments[p.section]) {
                    result.push(sectionComments[p.section]);
                }
            }

            if (!p.isContent) {
                result.push(p.raw);
                continue;
            }

            const indentStr = this.indent.repeat(p.indentLevel);

            if (!p.op) {
                result.push(indentStr + p.key);
                prevSection = p.section;
                continue;
            }

            const keyWidth = alignCol - indentStr.length;
            const paddedKey = p.key.padEnd(Math.max(keyWidth, p.key.length + 1));

            if (p.op === '>') {
                const items = p.value.split('|').map(v => v.trim()).filter(Boolean);
                result.push(`${indentStr}${paddedKey} > ${items.join(' | ')}`);
            } else {
                result.push(`${indentStr}${paddedKey} : ${p.value}`);
            }

            prevSection = p.section;
        }

        return result.join('\n');
    }

    /**
     * Convert to LLM-optimized Dense format
     * - Add ^ prefix for indented lines
     * - Remove spaces around operators
     * - Strip comments and empty lines
     */
    toDense(content: string): string {
        const lines = content.split('\n');
        const result: string[] = [];

        for (const line of lines) {
            const trimmed = line.trim();

            // Skip empty and comments
            if (!trimmed) continue;
            if (trimmed.startsWith('#') || trimmed.startsWith('//')) continue;

            // Calculate indent level from spaces
            const leadSpaces = line.length - line.trimStart().length;
            const indentLevel = Math.floor(leadSpaces / this.indentSize);
            const prefix = indentLevel > 0 ? '^' : '';

            // Strip any existing ^ prefix
            let work = trimmed;
            while (work.startsWith('^')) {
                work = work.substring(1);
            }
            work = work.trim();

            // Parse and compact
            const arrowIdx = work.indexOf('>');
            const colonIdx = work.indexOf(':');

            if (arrowIdx > 0 && (colonIdx < 0 || arrowIdx < colonIdx)) {
                const key = work.substring(0, arrowIdx).trim().replace(/\s+/g, '');
                const items = work.substring(arrowIdx + 1).split('|').map(v => v.trim()).filter(Boolean).join('|');
                result.push(`${prefix}${key}>${items}`);
            } else if (colonIdx > 0) {
                const key = work.substring(0, colonIdx).trim().replace(/\s+/g, '');
                const value = work.substring(colonIdx + 1).trim();
                result.push(`${prefix}${key}:${value}`);
            } else {
                result.push(`${prefix}${work.replace(/\s+/g, '')}`);
            }
        }

        return result.join('\n');
    }
}
