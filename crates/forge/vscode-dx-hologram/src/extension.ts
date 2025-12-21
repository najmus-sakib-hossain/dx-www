/**
 * DX Serializer Extension - Main Entry Point
 * 
 * The Holographic DX System: Three formats in quantum superposition
 * - Human Format: Beautiful, readable (shown in editor)
 * - LLM Format: Token-efficient (stored on disk)
 * - Machine Format: Binary, 0.70ns access (runtime)
 */

import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { DxHologramProvider } from './hologramProvider';
import { DxInflater } from './inflater';
import { DxDeflater } from './deflater';
import { ForgeDaemon } from './forgeDaemon';

let hologramProvider: DxHologramProvider;
let forgeDaemon: ForgeDaemon;
let outputChannel: vscode.OutputChannel;
let statusBarItem: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Extension activating...');

    // Create output channel
    outputChannel = vscode.window.createOutputChannel('DX Serializer');
    context.subscriptions.push(outputChannel);

    // Create status bar item
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(symbol-structure) DX';
    statusBarItem.tooltip = 'DX Serializer: Human | LLM | Machine';
    context.subscriptions.push(statusBarItem);

    // Initialize the hologram system
    hologramProvider = new DxHologramProvider(outputChannel);
    forgeDaemon = new ForgeDaemon({
        autoStart: vscode.workspace.getConfiguration('dx-hologram.forge').get('autoStart', true),
        autoBuild: vscode.workspace.getConfiguration('dx-hologram.forge').get('autoBuild', true)
    });

    // Register the Hologram Virtual Document Provider
    context.subscriptions.push(
        vscode.workspace.registerTextDocumentContentProvider(
            'dx-hologram',
            hologramProvider
        )
    );

    // NOTE: We don't intercept file opens anymore - just show files as-is
    // The deflate/inflate is handled on explicit save commands
    // This avoids the "content newer on disk" conflict

    // NOTE: onWillSaveTextDocument removed - was causing "file content newer" conflicts
    // Deflation happens via explicit command only (dx.normalizeFormat or dx.showLLMFormat)

    // Watch for external file changes
    const watcher = vscode.workspace.createFileSystemWatcher('**/dx');
    context.subscriptions.push(
        watcher.onDidChange(uri => hologramProvider.onExternalChange(uri)),
        watcher.onDidCreate(uri => logInfo(`DX file created: ${uri.fsPath}`)),
        watcher.onDidDelete(uri => logInfo(`DX file deleted: ${uri.fsPath}`))
    );

    // Register Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.openHologram', async (uri?: vscode.Uri) => {
            const fileUri = uri || vscode.window.activeTextEditor?.document.uri;
            if (fileUri) {
                await openAsHologram(fileUri);
            }
        }),

        vscode.commands.registerCommand('dx.showLLMFormat', async () => {
            await showLLMFormat();
        }),

        vscode.commands.registerCommand('dx.buildBinary', async () => {
            await buildBinary();
        }),

        vscode.commands.registerCommand('dx.startForge', async () => {
            const started = await forgeDaemon.start();
            updateStatusBar(started);
        }),

        vscode.commands.registerCommand('dx.stopForge', async () => {
            forgeDaemon.stop();
            updateStatusBar(false);
        }),

        vscode.commands.registerCommand('dx.convertFromJson', async () => {
            await convertFromJson();
        }),

        vscode.commands.registerCommand('dx.normalizeFormat', async () => {
            await normalizeFormat();
        }),

        vscode.commands.registerCommand('dx.convertAllFiles', async () => {
            await convertAllDxFiles();
        }),

        vscode.commands.registerCommand('dx.convertCurrentFile', async () => {
            await convertCurrentFile();
        })
    );

    // Auto-start Forge daemon (with graceful failure)
    const forgeConfig = vscode.workspace.getConfiguration('dx.forge');
    if (forgeConfig.get('autoStart', true)) {
        try {
            const started = await forgeDaemon.start();
            updateStatusBar(started);
            if (started) {
                logSuccess('Forge daemon started');
            } else {
                logInfo('Forge daemon not available - binary builds disabled');
            }
        } catch (error) {
            logInfo('Forge daemon not available - binary builds disabled');
            updateStatusBar(false);
        }
    }

    statusBarItem.show();
    logSuccess('DX Serializer Extension activated');
}

// ═══════════════════════════════════════════════════════════════════════
// Format Detection
// ═══════════════════════════════════════════════════════════════════════

/**
 * Detect if content is in Human format or LLM format
 * 
 * Human format indicators:
 * - Lines with multiple spaces for indentation
 * - Unicode symbols (▼, ✓, ✗, •, —)
 * - Box-drawing characters (┌, │, └, ─)
 * - Lines starting with spaces
 * 
 * LLM format indicators:
 * - # separators in key:value format
 * - @ for arrays (key@N>)
 * - ^ for inheritance
 * - > for arrays
 * - No leading whitespace (except empty lines)
 */
function detectFormat(content: string): 'human' | 'llm' | 'unknown' {
    const lines = content.split('\n').filter(l => l.trim().length > 0);
    
    if (lines.length === 0) {
        return 'unknown';
    }

    let humanScore = 0;
    let llmScore = 0;

    for (const line of lines) {
        // Human format indicators
        if (line.startsWith('    ') || line.startsWith('\t')) {
            humanScore += 2;
        }
        if (/[▼▲✓✗•—→←]/.test(line)) {
            humanScore += 3;
        }
        if (/[┌┐└┘├┤┬┴┼│─]/.test(line)) {
            humanScore += 3;
        }
        if (/^\s+\w+\s*:/.test(line)) {
            humanScore += 1;
        }

        // LLM format indicators
        if (/^\w+#\w+:/.test(line)) {
            llmScore += 3;
        }
        if (/^\w+@\d+[>=]/.test(line)) {
            llmScore += 3;
        }
        if (/^\^/.test(line)) {
            llmScore += 2;
        }
        if (/^>\w+\|/.test(line) || /\|/.test(line) && !/[│]/.test(line)) {
            llmScore += 1;
        }
        if (/^\w+\s+>/.test(line)) {
            llmScore += 2;
        }
    }

    if (humanScore > llmScore + 2) {
        return 'human';
    } else if (llmScore > humanScore + 2) {
        return 'llm';
    }
    
    // Check for simple key:value without # (could be either, but lean towards human-like)
    const simpleKV = lines.filter(l => /^\w[\w.]*\s*:/.test(l.trim()) && !l.includes('#'));
    if (simpleKV.length > lines.length * 0.5) {
        // More than half are simple key:value - check indentation
        const indented = lines.filter(l => l.startsWith(' ') || l.startsWith('\t'));
        if (indented.length > lines.length * 0.3) {
            return 'human';
        }
    }

    return 'unknown';
}

// ═══════════════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════════════

function isDxFile(uri: vscode.Uri): boolean {
    const basename = path.basename(uri.fsPath);
    return basename === 'dx' || uri.fsPath.endsWith('.dx');
}

async function openAsHologram(fileUri: vscode.Uri): Promise<void> {
    try {
        const content = fs.readFileSync(fileUri.fsPath, 'utf8');
        const format = detectFormat(content);
        
        logInfo(`Detected format: ${format} for ${path.basename(fileUri.fsPath)}`);
        
        let humanFormat: string;
        
        if (format === 'human' || format === 'unknown') {
            // Already human-readable or unknown - show as-is but normalize on save
            humanFormat = content;
            logInfo('File appears to be human-readable, showing as-is');
        } else {
            // LLM format - inflate to human
            const inflater = new DxInflater();
            humanFormat = inflater.inflate(content);
            logInfo('Inflated LLM format to human-readable');
        }
        
        // Create a new document with the human format
        const doc = await vscode.workspace.openTextDocument({
            content: humanFormat,
            language: 'dx'
        });
        
        // Store the mapping
        hologramProvider.registerFile(fileUri.fsPath, doc.uri);
        
        await vscode.window.showTextDocument(doc, { preview: false });
        logInfo(`Opened dx file: ${fileUri.fsPath}`);
    } catch (error) {
        logError(`Failed to open hologram: ${error}`);
    }
}

async function handleSave(doc: vscode.TextDocument): Promise<vscode.TextEdit[]> {
    try {
        const realPath = hologramProvider.getRealPath(doc.uri);
        const targetPath = realPath || doc.uri.fsPath;
        
        if (!targetPath || !isDxFile(vscode.Uri.file(targetPath))) {
            return [];
        }

        const content = doc.getText();
        const format = detectFormat(content);
        
        // Always deflate to LLM format for storage
        const deflater = new DxDeflater();
        let llmFormat: string;
        
        if (format === 'llm') {
            // Already LLM - use as-is
            llmFormat = content;
        } else {
            // Human or unknown - deflate to LLM
            llmFormat = deflater.deflate(content);
            logInfo('Deflated human format to LLM format for storage');
        }
        
        // Write LLM format to disk
        fs.writeFileSync(targetPath, llmFormat, 'utf8');
        logInfo(`Saved LLM format: ${targetPath}`);
        
        // Build binary if forge is available
        const config = vscode.workspace.getConfiguration('dx.forge');
        if (config.get('autoBuild', true) && forgeDaemon.getStatus() === 'running') {
            await buildBinaryForFile(targetPath);
        }
    } catch (error) {
        logError(`Save failed: ${error}`);
    }
    return [];
}

async function normalizeFormat(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const content = editor.document.getText();
    const format = detectFormat(content);
    
    if (format === 'human' || format === 'unknown') {
        // Convert human to LLM, then back to normalized human
        const deflater = new DxDeflater();
        const inflater = new DxInflater();
        const llmFormat = deflater.deflate(content);
        const normalized = inflater.inflate(llmFormat);
        
        const edit = new vscode.WorkspaceEdit();
        const fullRange = new vscode.Range(
            editor.document.positionAt(0),
            editor.document.positionAt(content.length)
        );
        edit.replace(editor.document.uri, fullRange, normalized);
        await vscode.workspace.applyEdit(edit);
        
        logInfo('Normalized format (Human -> LLM -> Human)');
        vscode.window.showInformationMessage('Format normalized');
    } else {
        const inflater = new DxInflater();
        const humanFormat = inflater.inflate(content);
        
        const edit = new vscode.WorkspaceEdit();
        const fullRange = new vscode.Range(
            editor.document.positionAt(0),
            editor.document.positionAt(content.length)
        );
        edit.replace(editor.document.uri, fullRange, humanFormat);
        await vscode.workspace.applyEdit(edit);
        
        logInfo('Converted LLM to Human format');
        vscode.window.showInformationMessage('Converted to human-readable format');
    }
}

async function showLLMFormat(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const content = editor.document.getText();
    const deflater = new DxDeflater();
    const llmFormat = deflater.deflate(content);

    const doc = await vscode.workspace.openTextDocument({
        content: llmFormat,
        language: 'dx'
    });

    await vscode.window.showTextDocument(doc, {
        viewColumn: vscode.ViewColumn.Beside,
        preview: true,
        preserveFocus: true
    });

    logInfo('Showing LLM-dense format');
}

async function buildBinary(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    let filePath = editor.document.uri.fsPath;
    const realPath = hologramProvider.getRealPath(editor.document.uri);
    if (realPath) {
        filePath = realPath;
    }

    await buildBinaryForFile(filePath);
}

async function buildBinaryForFile(filePath: string): Promise<void> {
    try {
        const status = forgeDaemon.getStatus();
        
        if (status === 'running') {
            await forgeDaemon.buildBinary(filePath);
            logSuccess(`Built binary for: ${path.basename(filePath)}`);
        } else if (status === 'error' || status === 'stopped') {
            // Try to start forge
            const started = await forgeDaemon.start();
            if (started) {
                await forgeDaemon.buildBinary(filePath);
                logSuccess(`Built binary for: ${path.basename(filePath)}`);
            } else {
                logInfo('Binary build skipped - dx-forge not available');
            }
        }
    } catch (error) {
        logInfo(`Binary build skipped: ${error}`);
    }
}

async function convertFromJson(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    try {
        const text = editor.document.getText();
        const json = JSON.parse(text);
        const deflater = new DxDeflater();
        const dxFormat = deflater.jsonToDense(JSON.stringify(json));

        const doc = await vscode.workspace.openTextDocument({
            content: dxFormat,
            language: 'dx'
        });

        await vscode.window.showTextDocument(doc);
        vscode.window.showInformationMessage('Converted JSON to DX format');
    } catch (error) {
        vscode.window.showErrorMessage(`Conversion failed: ${error}`);
    }
}

function updateStatusBar(forgeRunning: boolean): void {
    if (forgeRunning) {
        statusBarItem.text = '$(symbol-structure) DX';
        statusBarItem.tooltip = 'DX Serializer: Active | Forge: Running';
        statusBarItem.backgroundColor = undefined;
    } else {
        statusBarItem.text = '$(symbol-structure) DX';
        statusBarItem.tooltip = 'DX Serializer: Active | Forge: Not Available';
        statusBarItem.backgroundColor = undefined;
    }
}

function logInfo(message: string): void {
    const timestamp = new Date().toISOString().substring(11, 19);
    outputChannel.appendLine(`[${timestamp}] [INFO] ${message}`);
}

function logSuccess(message: string): void {
    const timestamp = new Date().toISOString().substring(11, 19);
    outputChannel.appendLine(`[${timestamp}] [OK] ${message}`);
}

function logError(message: string): void {
    const timestamp = new Date().toISOString().substring(11, 19);
    outputChannel.appendLine(`[${timestamp}] [ERR] ${message}`);
}

// ═══════════════════════════════════════════════════════════════════════
// Convert All DX Files
// ═══════════════════════════════════════════════════════════════════════

async function convertAllDxFiles(): Promise<void> {
    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!workspaceRoot) {
        vscode.window.showWarningMessage('No workspace folder open');
        return;
    }

    outputChannel.show();
    logInfo('Starting conversion of all DX files...');

    // Find all dx files
    const dxFiles = await findDxFiles(workspaceRoot);
    logInfo(`Found ${dxFiles.length} DX files`);

    let converted = 0;
    let binaryBuilt = 0;

    for (const filePath of dxFiles) {
        try {
            const result = await convertAndBuildFile(filePath, workspaceRoot);
            if (result.converted) converted++;
            if (result.binary) binaryBuilt++;
        } catch (error) {
            logError(`Failed to convert ${filePath}: ${error}`);
        }
    }

    logSuccess(`Conversion complete: ${converted} files normalized, ${binaryBuilt} binaries built`);
    vscode.window.showInformationMessage(`DX: Converted ${converted} files, built ${binaryBuilt} binaries`);
}

async function convertCurrentFile(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const filePath = editor.document.uri.fsPath;
    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || path.dirname(filePath);

    outputChannel.show();
    logInfo(`Converting: ${path.basename(filePath)}`);

    try {
        const result = await convertAndBuildFile(filePath, workspaceRoot);
        if (result.converted) {
            logSuccess(`Converted: ${path.basename(filePath)}`);
        }
        if (result.binary) {
            logSuccess(`Binary built: ${result.binaryPath}`);
        }
        vscode.window.showInformationMessage(`DX: File converted and binary built`);
    } catch (error) {
        logError(`Failed: ${error}`);
        vscode.window.showErrorMessage(`Conversion failed: ${error}`);
    }
}

async function findDxFiles(rootDir: string): Promise<string[]> {
    const results: string[] = [];
    
    async function walk(dir: string): Promise<void> {
        try {
            const entries = fs.readdirSync(dir, { withFileTypes: true });
            
            for (const entry of entries) {
                const fullPath = path.join(dir, entry.name);
                
                // Skip hidden directories and node_modules
                if (entry.isDirectory()) {
                    if (!entry.name.startsWith('.') && 
                        entry.name !== 'node_modules' && 
                        entry.name !== 'target') {
                        await walk(fullPath);
                    }
                } else if (entry.name === 'dx' || entry.name.endsWith('.dx')) {
                    results.push(fullPath);
                }
            }
        } catch (error) {
            // Skip directories we can't read
        }
    }
    
    await walk(rootDir);
    return results;
}

interface ConvertResult {
    converted: boolean;
    binary: boolean;
    binaryPath?: string;
}

async function convertAndBuildFile(filePath: string, workspaceRoot: string): Promise<ConvertResult> {
    const result: ConvertResult = { converted: false, binary: false };
    
    // Read current content
    const content = fs.readFileSync(filePath, 'utf8');
    const format = detectFormat(content);
    
    logInfo(`Processing: ${path.relative(workspaceRoot, filePath)} (detected: ${format})`);
    
    // Normalize to LLM format if needed
    let llmContent: string;
    if (format === 'human' || format === 'unknown') {
        const deflater = new DxDeflater();
        llmContent = deflater.deflate(content);
        
        // Only write if different
        if (llmContent !== content) {
            fs.writeFileSync(filePath, llmContent, 'utf8');
            result.converted = true;
            logInfo(`Normalized to LLM format: ${path.basename(filePath)}`);
        }
    } else {
        llmContent = content;
        result.converted = true;
    }
    
    // Build binary (.dxs)
    const binaryPath = buildBinaryFile(filePath, llmContent, workspaceRoot);
    if (binaryPath) {
        result.binary = true;
        result.binaryPath = binaryPath;
    }
    
    return result;
}

/**
 * Build binary .dxs file using FNV-1a hashed path
 */
function buildBinaryFile(sourcePath: string, content: string, workspaceRoot: string): string | null {
    try {
        // Get relative path for hashing
        const relativePath = path.relative(workspaceRoot, sourcePath).replace(/\\/g, '/');
        
        // FNV-1a hash (matching Rust implementation)
        const hash = hashPath(relativePath);
        
        // Get filename
        const filename = path.basename(sourcePath, path.extname(sourcePath)) || 'dx';
        
        // Create output directory
        const outputDir = path.join(workspaceRoot, '.dx', 'serializer', hash);
        fs.mkdirSync(outputDir, { recursive: true });
        
        // Build binary representation
        const binary = buildBinaryFormat(content);
        
        // Write binary file
        const outputPath = path.join(outputDir, `${filename}.dxs`);
        fs.writeFileSync(outputPath, binary);
        
        logInfo(`Binary written: .dx/serializer/${hash}/${filename}.dxs (${binary.length} bytes)`);
        return outputPath;
    } catch (error) {
        logError(`Binary build failed: ${error}`);
        return null;
    }
}

/**
 * FNV-1a hash (matches Rust implementation)
 */
function hashPath(relativePath: string): string {
    const FNV_OFFSET = BigInt('0xcbf29ce484222325');
    const FNV_PRIME = BigInt('0x100000001b3');
    
    let hash = FNV_OFFSET;
    for (let i = 0; i < relativePath.length; i++) {
        hash ^= BigInt(relativePath.charCodeAt(i));
        hash = (hash * FNV_PRIME) & BigInt('0xffffffffffffffff');
    }
    
    return (hash & BigInt('0xffffffff')).toString(16).padStart(8, '0');
}

/**
 * Build binary format from LLM content
 * 
 * Binary format:
 * - 4 bytes: Magic "DXS1"
 * - 4 bytes: Version (1)
 * - 4 bytes: Entry count
 * - 4 bytes: String table offset
 * - Entries: [keyId(4) + valueId(4) + type(1) + flags(1)]
 * - String table: [length(2) + utf8 bytes...]
 */
function buildBinaryFormat(content: string): Buffer {
    const entries: Array<{ key: string; value: string; type: number; flags: number }> = [];
    const strings = new Map<string, number>();
    let stringId = 0;
    
    function internString(s: string): number {
        if (strings.has(s)) {
            return strings.get(s)!;
        }
        const id = stringId++;
        strings.set(s, id);
        return id;
    }
    
    // Parse content into entries
    const lines = content.split('\n');
    let currentParent = '';
    
    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed) continue;
        
        // Entry types
        const TYPE_SIMPLE = 0;
        const TYPE_NESTED = 1;
        const TYPE_ARRAY = 2;
        const TYPE_TABLE = 3;
        
        if (trimmed.startsWith('^')) {
            // Continuation
            const rest = trimmed.substring(1);
            const colonIdx = rest.indexOf(':');
            if (colonIdx > 0) {
                const key = currentParent + '.' + rest.substring(0, colonIdx).trim();
                const value = rest.substring(colonIdx + 1).trim();
                entries.push({
                    key,
                    value,
                    type: TYPE_NESTED,
                    flags: 0
                });
            }
        } else if (trimmed.includes('>')) {
            // Array
            const arrowIdx = trimmed.indexOf('>');
            const key = trimmed.substring(0, arrowIdx).trim();
            const value = trimmed.substring(arrowIdx + 1).trim();
            entries.push({ key, value, type: TYPE_ARRAY, flags: 0 });
        } else if (trimmed.includes('=') && trimmed.includes('^')) {
            // Table header
            const eqIdx = trimmed.indexOf('=');
            const key = trimmed.substring(0, eqIdx).trim();
            const value = trimmed.substring(eqIdx + 1).trim();
            entries.push({ key, value, type: TYPE_TABLE, flags: 0 });
        } else {
            // Simple key:value or nested
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
            } else if (spaceIdx > 0) {
                const key = trimmed.substring(0, spaceIdx).trim();
                const value = trimmed.substring(spaceIdx + 1).trim();
                entries.push({ key, value, type: TYPE_SIMPLE, flags: 0 });
            }
        }
    }
    
    // Intern all strings
    for (const entry of entries) {
        internString(entry.key);
        internString(entry.value);
    }
    
    // Calculate sizes
    const headerSize = 16;
    const entrySize = 10; // keyId(4) + valueId(4) + type(1) + flags(1)
    const entriesSize = entries.length * entrySize;
    const stringTableOffset = headerSize + entriesSize;
    
    // Build string table
    const stringParts: Buffer[] = [];
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
    buffer.writeUInt32LE(1, offset); offset += 4; // version
    buffer.writeUInt32LE(entries.length, offset); offset += 4;
    buffer.writeUInt32LE(stringTableOffset, offset); offset += 4;
    
    // Entries
    for (const entry of entries) {
        buffer.writeUInt32LE(strings.get(entry.key)!, offset); offset += 4;
        buffer.writeUInt32LE(strings.get(entry.value)!, offset); offset += 4;
        buffer.writeUInt8(entry.type, offset); offset += 1;
        buffer.writeUInt8(entry.flags, offset); offset += 1;
    }
    
    // String table
    stringTable.copy(buffer, offset);
    
    return buffer;
}

export function deactivate() {
    if (forgeDaemon) {
        forgeDaemon.stop();
    }
    console.log('DX Serializer Extension deactivated');
}
