#!/usr/bin/env node
/**
 * Test Human Format Conversion
 */

const fs = require('fs');
const path = require('path');

// Parse DX LLM format to structured data
function parseToStructured(content) {
    const lines = content.split('\n');
    const result = [];
    let currentParent = '';
    
    for (const rawLine of lines) {
        const line = rawLine.trim();
        if (!line) continue;
        
        // Continuation: ^key:value
        if (line.startsWith('^')) {
            const rest = line.substring(1);
            const colonIdx = rest.indexOf(':');
            if (colonIdx > 0) {
                result.push({
                    type: 'continuation',
                    parent: currentParent,
                    key: rest.substring(0, colonIdx).trim(),
                    value: rest.substring(colonIdx + 1).trim()
                });
            } else {
                const spaceIdx = rest.indexOf(' ');
                if (spaceIdx > 0) {
                    result.push({
                        type: 'continuation',
                        parent: currentParent,
                        key: rest.substring(0, spaceIdx).trim(),
                        value: rest.substring(spaceIdx + 1).trim()
                    });
                }
            }
            continue;
        }
        
        // Array: key>item|item
        if (line.includes('>') && !line.startsWith('>')) {
            const arrowIdx = line.indexOf('>');
            const key = line.substring(0, arrowIdx).trim();
            const items = line.substring(arrowIdx + 1).split('|').map(s => s.trim());
            result.push({ type: 'array', key, items });
            continue;
        }
        
        // Table header: key=col^col
        if (line.includes('=') && line.includes('^')) {
            const eqIdx = line.indexOf('=');
            const key = line.substring(0, eqIdx).trim();
            const cols = line.substring(eqIdx + 1).split('^').map(s => s.trim());
            result.push({ type: 'table', key, columns: cols });
            continue;
        }
        
        // Table row: >val|val
        if (line.startsWith('>')) {
            const values = line.substring(1).split('|').map(s => s.trim());
            result.push({ type: 'tableRow', values });
            continue;
        }
        
        // Nested: parent.child:value
        const nestedMatch = line.match(/^([\w_]+)\.([\w_.]+)\s*:\s*(.*)$/);
        if (nestedMatch) {
            currentParent = nestedMatch[1];
            result.push({
                type: 'nested',
                parent: nestedMatch[1],
                key: nestedMatch[2],
                value: nestedMatch[3].trim()
            });
            continue;
        }
        
        // Nested with space
        const nestedSpaceMatch = line.match(/^([\w_]+)\.([\w_.]+)\s+(.+)$/);
        if (nestedSpaceMatch) {
            currentParent = nestedSpaceMatch[1];
            result.push({
                type: 'nested',
                parent: nestedSpaceMatch[1],
                key: nestedSpaceMatch[2],
                value: nestedSpaceMatch[3].trim()
            });
            continue;
        }
        
        // Simple: key:value
        const simpleMatch = line.match(/^([\w_][\w_.]*)\s*:\s*(.*)$/);
        if (simpleMatch) {
            result.push({
                type: 'simple',
                key: simpleMatch[1],
                value: simpleMatch[2].trim()
            });
            continue;
        }
        
        // Simple with space
        const spaceMatch = line.match(/^([\w_]+)\s+(.+)$/);
        if (spaceMatch && !spaceMatch[2].includes('>') && !spaceMatch[2].includes('=')) {
            result.push({
                type: 'simple',
                key: spaceMatch[1],
                value: spaceMatch[2].trim()
            });
        }
    }
    
    return result;
}

// Format structured data to human-readable
function toHumanFormat(structured) {
    const result = [];
    const indent = '    ';
    
    // Group by parent
    const groups = new Map();
    const arrays = [];
    const tables = [];
    const simples = [];
    
    let currentTable = null;
    
    for (const item of structured) {
        if (item.type === 'simple') {
            simples.push(item);
        } else if (item.type === 'nested' || item.type === 'continuation') {
            const parent = item.parent;
            if (!groups.has(parent)) {
                groups.set(parent, []);
            }
            groups.get(parent).push({ key: item.key, value: item.value });
        } else if (item.type === 'array') {
            arrays.push(item);
        } else if (item.type === 'table') {
            currentTable = { key: item.key, columns: item.columns, rows: [] };
            tables.push(currentTable);
        } else if (item.type === 'tableRow' && currentTable) {
            currentTable.rows.push(item.values);
        }
    }
    
    // Format simples
    if (simples.length > 0) {
        const maxKeyLen = Math.max(...simples.map(s => s.key.length));
        for (const s of simples) {
            const padding = ' '.repeat(maxKeyLen - s.key.length);
            result.push(`${s.key}${padding} : ${s.value}`);
        }
        result.push('');
    }
    
    // Format groups
    for (const [parent, items] of groups) {
        result.push(`▼ ${parent}`);
        const maxKeyLen = Math.max(...items.map(i => i.key.length));
        for (const item of items) {
            const padding = ' '.repeat(maxKeyLen - item.key.length);
            result.push(`${indent}${item.key}${padding} : ${item.value}`);
        }
        result.push('');
    }
    
    // Format arrays
    for (const arr of arrays) {
        result.push(`▼ ${arr.key} (${arr.items.length} items)`);
        for (const item of arr.items) {
            result.push(`${indent}• ${item}`);
        }
        result.push('');
    }
    
    // Format tables
    for (const table of tables) {
        result.push(`▼ ${table.key} (${table.columns.length} columns × ${table.rows.length} rows)`);
        
        const colWidths = table.columns.map(c => c.length);
        for (const row of table.rows) {
            for (let i = 0; i < row.length && i < colWidths.length; i++) {
                colWidths[i] = Math.max(colWidths[i], row[i].length);
            }
        }
        
        // Top border
        result.push(indent + '┌' + colWidths.map(w => '─'.repeat(w + 2)).join('┬') + '┐');
        
        // Header
        const headerCells = table.columns.map((c, i) => c.padEnd(colWidths[i]));
        result.push(indent + '│ ' + headerCells.join(' │ ') + ' │');
        
        // Separator
        result.push(indent + '├' + colWidths.map(w => '─'.repeat(w + 2)).join('┼') + '┤');
        
        // Rows
        for (const row of table.rows) {
            const cells = row.map((v, i) => v.padEnd(colWidths[i] || v.length));
            result.push(indent + '│ ' + cells.join(' │ ') + ' │');
        }
        
        // Bottom border
        result.push(indent + '└' + colWidths.map(w => '─'.repeat(w + 2)).join('┴') + '┘');
        result.push('');
    }
    
    return result.join('\n').trim();
}

// Main
const dxPath = path.resolve(__dirname, '..', 'dx');
const content = fs.readFileSync(dxPath, 'utf8');

console.log('=== Original DX File (LLM Format) ===');
console.log(content.substring(0, 500) + '...\n');

const structured = parseToStructured(content);
console.log('=== Parsed Structure (first 10 entries) ===');
console.log(JSON.stringify(structured.slice(0, 10), null, 2) + '\n');

const human = toHumanFormat(structured);
console.log('=== Human Format ===');
console.log(human);

// Write human format for reference
fs.writeFileSync(path.resolve(__dirname, 'dx-human-output.dx'), human);
console.log('\nWritten to scripts/dx-human-output.dx');
