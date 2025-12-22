/**
 * DX Extension - Simple & Working
 * 
 * Just provides syntax highlighting and formatting for dx files.
 * No complex file system provider - keeps it simple!
 */

import * as vscode from 'vscode';
import * as path from 'path';
import { DxFormatter } from './formatter';

let formatter: DxFormatter;
let outputChannel: vscode.OutputChannel;
let statusBarItem: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Extension activating...');

    // Create output channel
    outputChannel = vscode.window.createOutputChannel('DX');
    context.subscriptions.push(outputChannel);
    
    // Create formatter with 3-space indent
    formatter = new DxFormatter(3);

    // Create status bar item
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(file-code) DX';
    statusBarItem.tooltip = 'DX Config File';
    context.subscriptions.push(statusBarItem);

    // ═══════════════════════════════════════════════════════════════
    // Show/hide status bar based on active editor
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(editor => {
            if (editor && isDxFile(editor.document.uri)) {
                statusBarItem.show();
            } else {
                statusBarItem.hide();
            }
        })
    );

    // Check current editor
    if (vscode.window.activeTextEditor && isDxFile(vscode.window.activeTextEditor.document.uri)) {
        statusBarItem.show();
    }

    // ═══════════════════════════════════════════════════════════════
    // Register Document Formatting Provider
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider('dx', {
            provideDocumentFormattingEdits(document: vscode.TextDocument): vscode.TextEdit[] {
                const text = document.getText();
                const formatted = formatter.toPretty(text);
                
                const fullRange = new vscode.Range(
                    document.positionAt(0),
                    document.positionAt(text.length)
                );
                
                return [vscode.TextEdit.replace(fullRange, formatted)];
            }
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // Register Commands
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        // Format to Pretty (Human readable)
        vscode.commands.registerCommand('dx.formatPretty', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) {
                vscode.window.showWarningMessage('Open a DX file first');
                return;
            }
            
            const text = editor.document.getText();
            const formatted = formatter.toPretty(text);
            
            await editor.edit(editBuilder => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                editBuilder.replace(fullRange, formatted);
            });
            
            vscode.window.showInformationMessage('Formatted to Human-readable');
        }),

        // Format to Dense (LLM optimized)
        vscode.commands.registerCommand('dx.formatDense', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) {
                vscode.window.showWarningMessage('Open a DX file first');
                return;
            }
            
            const text = editor.document.getText();
            const dense = formatter.toDense(text);
            
            await editor.edit(editBuilder => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                editBuilder.replace(fullRange, dense);
            });
            
            vscode.window.showInformationMessage('Formatted to LLM-optimized Dense');
        }),

        // Show LLM format in side panel
        vscode.commands.registerCommand('dx.showLLMFormat', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) {
                vscode.window.showWarningMessage('Open a DX file first');
                return;
            }
            
            const text = editor.document.getText();
            const dense = formatter.toDense(text);
            
            const doc = await vscode.workspace.openTextDocument({
                content: dense,
                language: 'dx'
            });
            await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
        }),

        // Normalize format command
        vscode.commands.registerCommand('dx.normalizeFormat', async () => {
            await vscode.commands.executeCommand('dx.formatPretty');
        }),

        // Open Hologram (same as format pretty)
        vscode.commands.registerCommand('dx.openHologram', async () => {
            await vscode.commands.executeCommand('dx.formatPretty');
        }),

        // Build Binary (placeholder)
        vscode.commands.registerCommand('dx.buildBinary', async () => {
            vscode.window.showInformationMessage('Binary build requires dx-forge daemon');
        }),

        // Start/Stop Forge (placeholder)
        vscode.commands.registerCommand('dx.startForge', async () => {
            vscode.window.showInformationMessage('Forge daemon not available');
        }),
        vscode.commands.registerCommand('dx.stopForge', async () => {
            vscode.window.showInformationMessage('Forge daemon not running');
        }),

        // Convert from JSON
        vscode.commands.registerCommand('dx.convertFromJson', async () => {
            vscode.window.showInformationMessage('JSON conversion coming soon');
        }),

        // Convert all files
        vscode.commands.registerCommand('dx.convertAllFiles', async () => {
            vscode.window.showInformationMessage('Batch conversion coming soon');
        }),

        // Convert current file
        vscode.commands.registerCommand('dx.convertCurrentFile', async () => {
            await vscode.commands.executeCommand('dx.formatDense');
        })
    );

    outputChannel.appendLine('[DX] Extension activated successfully');
    console.log('[DX] Extension activated');
}

export function deactivate() {
    console.log('[DX] Extension deactivated');
}

/**
 * Check if a URI is a DX file
 */
function isDxFile(uri: vscode.Uri): boolean {
    const basename = path.basename(uri.fsPath);
    // Match: dx, *.dx, dx.* patterns
    return basename === 'dx' || 
           basename.endsWith('.dx') || 
           basename.startsWith('dx.');
}
