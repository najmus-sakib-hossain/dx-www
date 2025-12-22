/**
 * DX Quantum Extension - Main Entry Point
 * 
 * The Quantum DX System: Three formats in superposition
 * - Human Format: Beautiful, readable (shown in editor)
 * - LLM Format: Token-efficient (stored on disk)
 * - Machine Format: Binary, 0.70ns access (runtime)
 * 
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                         THE DX QUANTUM FILE                             │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │                           ONE FILE: config/dx                           │
 * │                                                                         │
 * │  ┌────────────────┐   ┌────────────────┐   ┌────────────────────┐      │
 * │  │     EDITOR     │   │     DISK       │   │      RUNTIME       │      │
 * │  │                │   │                │   │                    │      │
 * │  │  // Config     │   │ !Config!       │   │  ┌──────────────┐  │      │
 * │  │  server        │   │ server#host:   │   │  │ BINARY .dxb  │  │      │
 * │  │    #host: loc  │──▶│ localhost#     │──▶│  │              │  │      │
 * │  │    #port: 5432 │   │ port:5432      │   │  │ 0.70ns read  │  │      │
 * │  │                │   │                │   │  └──────────────┘  │      │
 * │  └────────────────┘   └────────────────┘   └────────────────────┘      │
 * │                                                                         │
 * │        PRETTY              DENSE               BINARY                   │
 * │     (Human view)        (LLM optimal)      (Machine optimal)            │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 */

import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { DxFileSystem } from './dxFileSystem';
import { DxFormatter } from './formatter';
import { ForgeDaemon } from './forgeDaemon';

let dxFs: DxFileSystem;
let formatter: DxFormatter;
let forgeDaemon: ForgeDaemon;
let outputChannel: vscode.OutputChannel;
let statusBarItem: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Quantum Extension activating...');

    // Create output channel
    outputChannel = vscode.window.createOutputChannel('DX Quantum');
    context.subscriptions.push(outputChannel);
    
    // Create formatter
    const config = vscode.workspace.getConfiguration('dx.hologram');
    formatter = new DxFormatter(config.get('indentSize', 2));

    // Create status bar item
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(zap) DX Quantum';
    statusBarItem.tooltip = 'Viewing: Human Format | Stored: LLM Format | Runtime: Binary';
    statusBarItem.command = 'dx.showDense';
    context.subscriptions.push(statusBarItem);

    // Initialize the FileSystem provider
    dxFs = new DxFileSystem(outputChannel);
    context.subscriptions.push(dxFs);
    
    // Initialize Forge daemon
    forgeDaemon = new ForgeDaemon({
        autoStart: vscode.workspace.getConfiguration('dx.forge').get('autoStart', true),
        autoBuild: vscode.workspace.getConfiguration('dx.forge').get('autoBuild', true)
    });

    // ═══════════════════════════════════════════════════════════════
    // Register the FileSystem Provider for dx-fs:// scheme
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.registerFileSystemProvider('dx-fs', dxFs, {
            isCaseSensitive: true,
            isReadonly: false
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // THE MAGIC: Seamlessly redirect file:// to dx-fs://
    // User clicks "dx" in explorer → Opens via our provider
    // They never notice the scheme change!
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (doc) => {
            // Only intercept real file:// dx files
            if (doc.uri.scheme === 'file' && isDxFile(doc.uri)) {
                const autoInflate = vscode.workspace
                    .getConfiguration('dx.hologram')
                    .get('autoInflate', true);
                    
                if (!autoInflate) {
                    return;
                }
                
                // Get the editor that opened this
                const editor = vscode.window.visibleTextEditors.find(
                    e => e.document.uri.toString() === doc.uri.toString()
                );
                const viewColumn = editor?.viewColumn || vscode.ViewColumn.Active;
                
                // Small delay to ensure the document is fully loaded
                setTimeout(async () => {
                    try {
                        // Close the file:// version (happens so fast user won't notice)
                        await vscode.commands.executeCommand('workbench.action.closeActiveEditor');
                        
                        // Open via our dx-fs:// provider
                        const dxUri = vscode.Uri.parse(`dx-fs://${doc.uri.fsPath.replace(/\\/g, '/')}`);
                        const dxDoc = await vscode.workspace.openTextDocument(dxUri);
                        await vscode.window.showTextDocument(dxDoc, { 
                            viewColumn,
                            preview: false 
                        });
                        
                        logInfo(`Opened quantum file: ${path.basename(doc.uri.fsPath)}`);
                    } catch (error) {
                        logError(`Failed to open quantum file: ${error}`);
                    }
                }, 50);
            }
        })
    );
    
    // ═══════════════════════════════════════════════════════════════
    // Show/hide status bar based on active editor
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(editor => {
            if (editor?.document.uri.scheme === 'dx-fs' || 
                (editor?.document.uri.scheme === 'file' && isDxFile(editor.document.uri))) {
                statusBarItem.show();
            } else {
                statusBarItem.hide();
            }
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // Watch for external file changes
    // ═══════════════════════════════════════════════════════════════
    
    const watcher = vscode.workspace.createFileSystemWatcher('**/dx');
    context.subscriptions.push(
        watcher.onDidChange(uri => {
            dxFs.invalidateCache(uri.fsPath);
            logInfo(`External change: ${path.basename(uri.fsPath)}`);
        }),
        watcher.onDidCreate(uri => logInfo(`DX file created: ${uri.fsPath}`)),
        watcher.onDidDelete(uri => logInfo(`DX file deleted: ${uri.fsPath}`))
    );

    // ═══════════════════════════════════════════════════════════════
    // Register Commands
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        // Open file via quantum provider
        vscode.commands.registerCommand('dx.openHologram', async (uri?: vscode.Uri) => {
            const fileUri = uri || vscode.window.activeTextEditor?.document.uri;
            if (fileUri) {
                await openQuantumFile(fileUri);
            }
        }),

        // Show the dense (LLM) format in a side panel
        vscode.commands.registerCommand('dx.showDense', async () => {
            await showDenseFormat();
        }),
        
        vscode.commands.registerCommand('dx.showLLMFormat', async () => {
            await showDenseFormat();
        }),

        // Show the pretty (human) format
        vscode.commands.registerCommand('dx.showPretty', async () => {
            await showPrettyFormat();
        }),

        // Build binary file
        vscode.commands.registerCommand('dx.buildBinary', async () => {
            await buildBinary();
        }),
        
        vscode.commands.registerCommand('dx.build', async () => {
            await buildBinary();
        }),

        // Forge daemon controls
        vscode.commands.registerCommand('dx.startForge', async () => {
            const started = await forgeDaemon.start();
            updateStatusBar(started);
        }),

        vscode.commands.registerCommand('dx.stopForge', async () => {
            forgeDaemon.stop();
            updateStatusBar(false);
        }),

        // Convert from JSON
        vscode.commands.registerCommand('dx.convertFromJson', async () => {
            await convertFromJson();
        }),

        // Normalize format (roundtrip)
        vscode.commands.registerCommand('dx.normalizeFormat', async () => {
            await normalizeFormat();
        }),

        // Convert all DX files in workspace
        vscode.commands.registerCommand('dx.convertAllFiles', async () => {
            await convertAllDxFiles();
        }),

        // Convert current file
        vscode.commands.registerCommand('dx.convertCurrentFile', async () => {
            await convertCurrentFile();
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // Auto-start Forge daemon
    // ═══════════════════════════════════════════════════════════════
    
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

    logSuccess('DX Quantum Extension activated');
}

// ═══════════════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════════════

function isDxFile(uri: vscode.Uri): boolean {
    const basename = path.basename(uri.fsPath);
    return basename === 'dx' || uri.fsPath.endsWith('.dx');
}

async function openQuantumFile(uri: vscode.Uri): Promise<void> {
    try {
        // Convert to dx-fs:// scheme
        const dxUri = vscode.Uri.parse(`dx-fs://${uri.fsPath.replace(/\\/g, '/')}`);
        const doc = await vscode.workspace.openTextDocument(dxUri);
        await vscode.window.showTextDocument(doc, { preview: false });
        logInfo(`Opened quantum file: ${path.basename(uri.fsPath)}`);
    } catch (error) {
        logError(`Failed to open quantum file: ${error}`);
    }
}

async function showDenseFormat(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No DX file is open');
        return;
    }
    
    let denseContent: string;
    
    if (editor.document.uri.scheme === 'dx-fs') {
        // Read the actual dense content from disk
        const realPath = editor.document.uri.path.startsWith('/') && process.platform === 'win32'
            ? editor.document.uri.path.substring(1)
            : editor.document.uri.path;
            
        try {
            denseContent = fs.readFileSync(realPath, 'utf8');
        } catch (error) {
            // Fall back to deflating current content
            denseContent = formatter.toDense(editor.document.getText());
        }
    } else {
        // Deflate current content
        denseContent = formatter.toDense(editor.document.getText());
    }
    
    // Show in a read-only preview panel
    const doc = await vscode.workspace.openTextDocument({
        content: denseContent,
        language: 'dx'
    });
    await vscode.window.showTextDocument(doc, {
        viewColumn: vscode.ViewColumn.Beside,
        preview: true,
        preserveFocus: true
    });
    
    vscode.window.showInformationMessage('Showing LLM-optimized format (stored on disk)');
}

async function showPrettyFormat(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    
    // Already showing pretty if using dx-fs
    if (editor.document.uri.scheme === 'dx-fs') {
        vscode.window.showInformationMessage('Already viewing Human format');
        return;
    }
    
    // Inflate current content
    const prettyContent = formatter.toPretty(editor.document.getText());
    
    const doc = await vscode.workspace.openTextDocument({
        content: prettyContent,
        language: 'dx'
    });
    await vscode.window.showTextDocument(doc, {
        viewColumn: vscode.ViewColumn.Beside,
        preview: true,
        preserveFocus: true
    });
}

async function buildBinary(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    let filePath = editor.document.uri.fsPath;
    if (editor.document.uri.scheme === 'dx-fs') {
        filePath = editor.document.uri.path.startsWith('/') && process.platform === 'win32'
            ? editor.document.uri.path.substring(1)
            : editor.document.uri.path;
    }

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
        const dxFormat = jsonToDense(json, '');

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

function jsonToDense(obj: any, prefix: string): string {
    const lines: string[] = [];

    for (const [key, value] of Object.entries(obj)) {
        const fullKey = prefix ? `${prefix}.${key}` : key;

        if (value === null) {
            lines.push(`${fullKey}:~`);
        } else if (typeof value === 'boolean') {
            lines.push(`${fullKey}:${value ? '+' : '-'}`);
        } else if (typeof value === 'number' || typeof value === 'string') {
            lines.push(`${fullKey}:${value}`);
        } else if (Array.isArray(value)) {
            const items = value.map(v => String(v)).join('|');
            lines.push(`${fullKey}>${items}`);
        } else if (typeof value === 'object') {
            lines.push(jsonToDense(value, fullKey));
        }
    }

    return lines.join('\n');
}

async function normalizeFormat(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    const content = editor.document.getText();
    
    // Roundtrip: content → dense → pretty
    const denseFormat = formatter.toDense(content);
    const normalized = formatter.toPretty(denseFormat);
    
    const edit = new vscode.WorkspaceEdit();
    const fullRange = new vscode.Range(
        editor.document.positionAt(0),
        editor.document.positionAt(content.length)
    );
    edit.replace(editor.document.uri, fullRange, normalized);
    await vscode.workspace.applyEdit(edit);
    
    logInfo('Normalized format (roundtrip)');
    vscode.window.showInformationMessage('Format normalized');
}

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

    for (const filePath of dxFiles) {
        try {
            const content = fs.readFileSync(filePath, 'utf8');
            const denseContent = formatter.toDense(content);
            
            if (denseContent !== content) {
                fs.writeFileSync(filePath, denseContent, 'utf8');
                converted++;
                logInfo(`Converted: ${path.relative(workspaceRoot, filePath)}`);
            }
        } catch (error) {
            logError(`Failed to convert ${filePath}: ${error}`);
        }
    }

    logSuccess(`Conversion complete: ${converted} files normalized`);
    vscode.window.showInformationMessage(`DX: Converted ${converted} files to LLM format`);
}

async function convertCurrentFile(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    let filePath = editor.document.uri.fsPath;
    if (editor.document.uri.scheme === 'dx-fs') {
        filePath = editor.document.uri.path.startsWith('/') && process.platform === 'win32'
            ? editor.document.uri.path.substring(1)
            : editor.document.uri.path;
    }

    outputChannel.show();
    logInfo(`Converting: ${path.basename(filePath)}`);

    try {
        const content = editor.document.getText();
        const denseContent = formatter.toDense(content);
        
        fs.writeFileSync(filePath, denseContent, 'utf8');
        logSuccess(`Converted: ${path.basename(filePath)}`);
        
        // Invalidate cache so next read picks up the change
        dxFs.invalidateCache(filePath);
        
        vscode.window.showInformationMessage(`DX: File converted to LLM format`);
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

function updateStatusBar(forgeRunning: boolean): void {
    if (forgeRunning) {
        statusBarItem.text = '$(zap) DX Quantum';
        statusBarItem.tooltip = 'DX Quantum: Human View | LLM Disk | Binary Runtime | Forge: Running';
    } else {
        statusBarItem.text = '$(zap) DX Quantum';
        statusBarItem.tooltip = 'DX Quantum: Human View | LLM Disk | Binary Runtime';
    }
}

function logInfo(message: string): void {
    const timestamp = new Date().toISOString().substring(11, 19);
    outputChannel.appendLine(`[${timestamp}] [INFO] ${message}`);
}

function logSuccess(message: string): void {
    const timestamp = new Date().toISOString().substring(11, 19);
    outputChannel.appendLine(`[${timestamp}] [OK] ✓ ${message}`);
}

function logError(message: string): void {
    const timestamp = new Date().toISOString().substring(11, 19);
    outputChannel.appendLine(`[${timestamp}] [ERR] ✗ ${message}`);
}

export function deactivate() {
    if (forgeDaemon) {
        forgeDaemon.stop();
    }
    if (dxFs) {
        dxFs.dispose();
    }
    console.log('DX Quantum Extension deactivated');
}
