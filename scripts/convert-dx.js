#!/usr/bin/env node
/**
 * DX Converter CLI
 * 
 * Converts dx files to LLM format and builds .dxs binary files.
 * 
 * Usage:
 *   node convert-dx.js              # Convert all dx files
 *   node convert-dx.js dx           # Convert specific file
 */

const fs = require('fs');
const path = require('path');

// Configuration
const PROJECT_ROOT = path.resolve(__dirname, '..');
const OUTPUT_DIR = path.join(PROJECT_ROOT, '.dx', 'serializer');

// FNV-1a hash (matches Rust implementation)
function hashPath(relativePath) {
    const FNV_OFFSET = BigInt('0xcbf29ce484222325');
    const FNV_PRIME = BigInt('0x100000001b3');
    
    let hash = FNV_OFFSET;
    for (let i = 0; i < relativePath.length; i++) {
        hash ^= BigInt(relativePath.charCodeAt(i));
        hash = (hash * FNV_PRIME) & BigInt('0xffffffffffffffff');
    }
    
    return (hash & BigInt('0xffffffff')).toString(16).padStart(8, '0');
}

// Build binary format
function buildBinaryFormat(content) {
    const entries = [];
    const strings = new Map();
    let stringId = 0;
    
    function internString(s) {
        if (strings.has(s)) return strings.get(s);
        const id = stringId++;
        strings.set(s, id);
        return id;
    }
    
    // Parse content into entries
    const lines = content.split('\n');
    let currentParent = '';
    
    const TYPE_SIMPLE = 0;
    const TYPE_NESTED = 1;
    const TYPE_ARRAY = 2;
    const TYPE_TABLE = 3;
    
    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed) continue;
        
        if (trimmed.startsWith('^')) {
            const rest = trimmed.substring(1);
            const colonIdx = rest.indexOf(':');
            if (colonIdx > 0) {
                const key = currentParent + '.' + rest.substring(0, colonIdx).trim();
                const value = rest.substring(colonIdx + 1).trim();
                entries.push({ key, value, type: TYPE_NESTED, flags: 0 });
            }
        } else if (trimmed.includes('>') && !trimmed.startsWith('>')) {
            const arrowIdx = trimmed.indexOf('>');
            const key = trimmed.substring(0, arrowIdx).trim();
            const value = trimmed.substring(arrowIdx + 1).trim();
            entries.push({ key, value, type: TYPE_ARRAY, flags: 0 });
        } else if (trimmed.startsWith('>')) {
            // Table row - append to previous
            const value = trimmed.substring(1).trim();
            if (entries.length > 0) {
                entries[entries.length - 1].value += '\n' + value;
            }
        } else if (trimmed.includes('=') && trimmed.includes('^')) {
            const eqIdx = trimmed.indexOf('=');
            const key = trimmed.substring(0, eqIdx).trim();
            const value = trimmed.substring(eqIdx + 1).trim();
            entries.push({ key, value, type: TYPE_TABLE, flags: 0 });
        } else {
            const colonIdx = trimmed.indexOf(':');
            const spaceIdx = trimmed.indexOf(' ');
            
            if (colonIdx > 0) {
                const key = trimmed.substring(0, colonIdx).trim();
                const value = trimmed.substring(colonIdx + 1).trim();
                
                if (key.includes('.')) {
                    const dotIdx = key.indexOf('.');
                    currentParent = key.substring(0, dotIdx);
                    entries.push({ key, value, type: TYPE_NESTED, flags: 0 });
                } else {
                    entries.push({ key, value, type: TYPE_SIMPLE, flags: 0 });
                }
            } else if (spaceIdx > 0 && !trimmed.includes('>') && !trimmed.includes('=')) {
                const key = trimmed.substring(0, spaceIdx).trim();
                const value = trimmed.substring(spaceIdx + 1).trim();
                entries.push({ key, value, type: TYPE_SIMPLE, flags: 0 });
            }
        }
    }
    
    // Intern strings
    for (const entry of entries) {
        internString(entry.key);
        internString(entry.value);
    }
    
    // Calculate sizes
    const headerSize = 16;
    const entrySize = 10;
    const entriesSize = entries.length * entrySize;
    const stringTableOffset = headerSize + entriesSize;
    
    // Build string table
    const stringParts = [];
    const sortedStrings = Array.from(strings.entries()).sort((a, b) => a[1] - b[1]);
    for (const [str] of sortedStrings) {
        const bytes = Buffer.from(str, 'utf8');
        const lenBuf = Buffer.alloc(2);
        lenBuf.writeUInt16LE(bytes.length, 0);
        stringParts.push(lenBuf, bytes);
    }
    const stringTable = Buffer.concat(stringParts);
    
    // Build final buffer
    const totalSize = headerSize + entriesSize + stringTable.length;
    const buffer = Buffer.alloc(totalSize);
    let offset = 0;
    
    // Header
    buffer.write('DXS1', offset); offset += 4;
    buffer.writeUInt32LE(1, offset); offset += 4;
    buffer.writeUInt32LE(entries.length, offset); offset += 4;
    buffer.writeUInt32LE(stringTableOffset, offset); offset += 4;
    
    // Entries
    for (const entry of entries) {
        buffer.writeUInt32LE(strings.get(entry.key), offset); offset += 4;
        buffer.writeUInt32LE(strings.get(entry.value), offset); offset += 4;
        buffer.writeUInt8(entry.type, offset); offset += 1;
        buffer.writeUInt8(entry.flags, offset); offset += 1;
    }
    
    // String table
    stringTable.copy(buffer, offset);
    
    return buffer;
}

// Find all dx files
function findDxFiles(dir) {
    const results = [];
    
    function walk(currentDir) {
        try {
            const entries = fs.readdirSync(currentDir, { withFileTypes: true });
            
            for (const entry of entries) {
                const fullPath = path.join(currentDir, entry.name);
                
                if (entry.isDirectory()) {
                    if (!entry.name.startsWith('.') && 
                        entry.name !== 'node_modules' && 
                        entry.name !== 'target') {
                        walk(fullPath);
                    }
                } else if (entry.name === 'dx' || entry.name.endsWith('.dx')) {
                    results.push(fullPath);
                }
            }
        } catch (e) {
            // Skip unreadable directories
        }
    }
    
    walk(dir);
    return results;
}

// Convert and build
function processFile(filePath) {
    const relativePath = path.relative(PROJECT_ROOT, filePath).replace(/\\/g, '/');
    const hash = hashPath(relativePath);
    const filename = path.basename(filePath, path.extname(filePath)) || 'dx';
    
    console.log(`Processing: ${relativePath}`);
    console.log(`  Hash: ${hash}`);
    
    // Read content
    const content = fs.readFileSync(filePath, 'utf8');
    
    // Build binary
    const binary = buildBinaryFormat(content);
    
    // Create output directory
    const outputDir = path.join(OUTPUT_DIR, hash);
    fs.mkdirSync(outputDir, { recursive: true });
    
    // Write binary
    const outputPath = path.join(outputDir, `${filename}.dxs`);
    fs.writeFileSync(outputPath, binary);
    
    console.log(`  Binary: .dx/serializer/${hash}/${filename}.dxs (${binary.length} bytes)`);
    console.log(`  Entries: ${binary.readUInt32LE(8)}`);
    
    return outputPath;
}

// Main
function main() {
    const args = process.argv.slice(2);
    
    console.log('DX Converter');
    console.log('============');
    console.log(`Project root: ${PROJECT_ROOT}`);
    console.log(`Output dir: ${OUTPUT_DIR}`);
    console.log('');
    
    let files;
    if (args.length > 0) {
        // Convert specific file
        files = args.map(f => path.resolve(f));
    } else {
        // Find all dx files
        files = findDxFiles(PROJECT_ROOT);
    }
    
    console.log(`Found ${files.length} DX file(s)`);
    console.log('');
    
    let success = 0;
    for (const file of files) {
        try {
            processFile(file);
            success++;
            console.log('');
        } catch (error) {
            console.error(`  Error: ${error.message}`);
            console.log('');
        }
    }
    
    console.log(`Completed: ${success}/${files.length} files`);
}

main();
