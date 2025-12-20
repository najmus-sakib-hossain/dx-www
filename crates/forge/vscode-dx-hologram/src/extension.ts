/**
 * DX Hologram Extension - Main Entry Point
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
    outputChannel = vscode.window.createOutputChannel('DX Hologram');
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

    // ═══════════════════════════════════════════════════════════════
    // Register the Hologram Virtual Document Provider
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.registerTextDocumentContentProvider(
            'dx-hologram',
            hologramProvider
        )
    );

    // ═══════════════════════════════════════════════════════════════
    // Intercept dx file opens to show human format
    // ═══════════════════════════════════════════════════════════════

    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (doc) => {
            if (isDxFile(doc.uri) && doc.uri.scheme === 'file') {
                const config = vscode.workspace.getConfiguration('dx.hologram');
                if (config.get('autoInflate', true)) {
                    // Show the hologram (human-readable) version
                    await openAsHologram(doc.uri);
                }
            }
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // Save handler: Deflate human → LLM, then build binary
    // ═══════════════════════════════════════════════════════════════

    context.subscriptions.push(
        vscode.workspace.onWillSaveTextDocument(async (event) => {
            if (isDxFile(event.document.uri)) {
                event.waitUntil(handleSave(event.document));
            }
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // Watch for external file changes
    // ═══════════════════════════════════════════════════════════════

    const watcher = vscode.workspace.createFileSystemWatcher('**/dx');
    context.subscriptions.push(
        watcher.onDidChange(uri => hologramProvider.onExternalChange(uri)),
        watcher.onDidCreate(uri => logInfo(`DX file created: ${uri.fsPath}`)),
        watcher.onDidDelete(uri => logInfo(`DX file deleted: ${uri.fsPath}`))
    );

    // ═══════════════════════════════════════════════════════════════
    // Register Commands
    // ═══════════════════════════════════════════════════════════════

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
            await forgeDaemon.start();
            updateStatusBar(true);
        }),

        vscode.commands.registerCommand('dx.stopForge', async () => {
            await forgeDaemon.stop();
            updateStatusBar(false);
        }),

        vscode.commands.registerCommand('dx.convertFromJson', async () => {
            await convertFromJson();
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // Auto-start Forge daemon
    // ═══════════════════════════════════════════════════════════════

    const forgeConfig = vscode.workspace.getConfiguration('dx.forge');
    if (forgeConfig.get('autoStart', true)) {
        try {
            await forgeDaemon.start();
            updateStatusBar(true);
            logSuccess('Forge daemon started automatically');
        } catch (error) {
            logError(`Failed to start Forge daemon: ${error}`);
            updateStatusBar(false);
        }
    }

    statusBarItem.show();
    logSuccess('DX Hologram Extension activated!');
}

// ═══════════════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════════════

function isDxFile(uri: vscode.Uri): boolean {
    const basename = path.basename(uri.fsPath);
    return basename === 'dx';
}

async function openAsHologram(fileUri: vscode.Uri): Promise<void> {
    try {
        // Read the LLM-dense format from disk
        const content = fs.readFileSync(fileUri.fsPath, 'utf8');
        
        // Inflate to human-pretty format
        const inflater = new DxInflater();
        const humanFormat = inflater.inflate(content);
        
        // Create a new document with the human format
        const doc = await vscode.workspace.openTextDocument({
            content: humanFormat,
            language: 'dx'
        });
        
        // Store the mapping
        hologramProvider.registerFile(fileUri.fsPath, doc.uri);
        
        await vscode.window.showTextDocument(doc, { preview: false });
        logInfo(`Opened dx file as hologram: ${fileUri.fsPath}`);
    } catch (error) {
        logError(`Failed to open hologram: ${error}`);
    }
}

async function handleSave(doc: vscode.TextDocument): Promise<vscode.TextEdit[]> {
    try {
        const realPath = hologramProvider.getRealPath(doc.uri);
        if (!realPath) {
            // This is a direct dx file edit
            const deflater = new DxDeflater();
            const llmFormat = deflater.deflate(doc.getText());
            
            // Write LLM format to disk
            fs.writeFileSync(doc.uri.fsPath, llmFormat, 'utf8');
            logInfo(`Saved LLM format: ${doc.uri.fsPath}`);
            
            // Build binary if configured
            const config = vscode.workspace.getConfiguration('dx.forge');
            if (config.get('autoBuild', true)) {
                await buildBinaryForFile(doc.uri.fsPath);
            }
        }
    } catch (error) {
        logError(`Save failed: ${error}`);
    }
    return [];
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
        // Use Forge daemon to build binary
        if (forgeDaemon.getStatus() === 'running') {
            await forgeDaemon.buildBinary(filePath);
            logSuccess(`Built binary: ${filePath}.dxb`);
            vscode.window.showInformationMessage('DX binary built successfully!');
        } else {
            logInfo('Forge daemon not running. Starting...');
            await forgeDaemon.start();
            await forgeDaemon.buildBinary(filePath);
            logSuccess(`Built binary: ${filePath}.dxb`);
        }
    } catch (error) {
        logError(`Build failed: ${error}`);
        vscode.window.showErrorMessage(`Failed to build binary: ${error}`);
    }
}

async function convertFromJson(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
    }

    try {
        const json = JSON.parse(editor.document.getText());
        const deflater = new DxDeflater();
        const dxFormat = deflater.jsonToDense(json);

        const doc = await vscode.workspace.openTextDocument({
            content: dxFormat,
            language: 'dx'
        });

        await vscode.window.showTextDocument(doc);
        vscode.window.showInformationMessage('Converted JSON to DX format!');
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
        statusBarItem.text = '$(circle-outline) DX';
        statusBarItem.tooltip = 'DX Serializer: Active | Forge: Stopped';
        statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
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

export function deactivate() {
    if (forgeDaemon) {
        forgeDaemon.stop();
    }
    console.log('DX Hologram Extension deactivated');
}
