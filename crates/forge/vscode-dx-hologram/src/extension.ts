/**
 * DX Extension - Transparent Format Conversion
 * 
 * The file is stored in LLM-dense format (with ^, no spaces)
 * But displayed to user in Human-pretty format (with indentation, spaces)
 * 
 * Flow:
 * 1. User opens file → Dense on disk → converted to Pretty for display
 * 2. User saves file → Pretty in editor → converted to Dense for storage
 */

import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { DxFormatter } from './formatter';

let formatter: DxFormatter;
let outputChannel: vscode.OutputChannel;
let statusBarItem: vscode.StatusBarItem;

// Track which documents we're managing to avoid infinite loops
const managedDocs = new Set<string>();
let isTransforming = false;

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
    statusBarItem.tooltip = 'DX Config - Click to toggle format view';
    statusBarItem.command = 'dx.toggleFormat';
    context.subscriptions.push(statusBarItem);

    // ═══════════════════════════════════════════════════════════════
    // AUTO-CONVERT: When opening a DX file, convert Dense → Pretty
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (doc) => {
            if (!isDxFile(doc.uri) || isTransforming) return;
            
            // Wait a bit for the editor to be ready
            setTimeout(async () => {
                await convertToHumanFormat(doc);
            }, 100);
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // AUTO-CONVERT: Before saving, convert Pretty → Dense
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.onWillSaveTextDocument((event) => {
            if (!isDxFile(event.document.uri) || isTransforming) return;
            
            const doc = event.document;
            const text = doc.getText();
            
            // Only convert if it's in pretty format
            if (!formatter.isDense(text)) {
                const dense = formatter.toDense(text);
                
                // Create edit to replace content with dense format
                const edit = new vscode.WorkspaceEdit();
                const fullRange = new vscode.Range(
                    doc.positionAt(0),
                    doc.positionAt(text.length)
                );
                edit.replace(doc.uri, fullRange, dense);
                
                // Apply the edit before save
                event.waitUntil(
                    vscode.workspace.applyEdit(edit).then(() => {
                        outputChannel.appendLine(`[DX] Saved as LLM-dense format: ${path.basename(doc.uri.fsPath)}`);
                    })
                );
            }
        })
    );

    // ═══════════════════════════════════════════════════════════════
    // After saving, convert back to Pretty for continued editing
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(async (doc) => {
            if (!isDxFile(doc.uri) || isTransforming) return;
            
            // Small delay to ensure save is complete
            setTimeout(async () => {
                await convertToHumanFormat(doc);
            }, 50);
        })
    );

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
        // Convert to human format if needed
        await convertToHumanFormat(vscode.window.activeTextEditor.document);
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
        // Toggle between formats
        vscode.commands.registerCommand('dx.toggleFormat', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) return;
            
            const text = editor.document.getText();
            const isDense = formatter.isDense(text);
            
            isTransforming = true;
            try {
                const newText = isDense ? formatter.toPretty(text) : formatter.toDense(text);
                await editor.edit(editBuilder => {
                    const fullRange = new vscode.Range(
                        editor.document.positionAt(0),
                        editor.document.positionAt(text.length)
                    );
                    editBuilder.replace(fullRange, newText);
                });
                
                statusBarItem.text = isDense ? '$(eye) DX Human' : '$(file-code) DX LLM';
                vscode.window.showInformationMessage(
                    isDense ? 'Showing Human-readable format' : 'Showing LLM-dense format'
                );
            } finally {
                isTransforming = false;
            }
        }),

        // Format to Pretty (Human readable)
        vscode.commands.registerCommand('dx.formatPretty', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) {
                vscode.window.showWarningMessage('Open a DX file first');
                return;
            }
            
            await convertToHumanFormat(editor.document);
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
            
            isTransforming = true;
            try {
                await editor.edit(editBuilder => {
                    const fullRange = new vscode.Range(
                        editor.document.positionAt(0),
                        editor.document.positionAt(text.length)
                    );
                    editBuilder.replace(fullRange, dense);
                });
            } finally {
                isTransforming = false;
            }
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

        // Other commands (placeholders)
        vscode.commands.registerCommand('dx.openHologram', async () => {
            await vscode.commands.executeCommand('dx.formatPretty');
        }),
        vscode.commands.registerCommand('dx.normalizeFormat', async () => {
            await vscode.commands.executeCommand('dx.formatPretty');
        }),
        vscode.commands.registerCommand('dx.buildBinary', async () => {
            vscode.window.showInformationMessage('Binary build requires dx-forge daemon');
        }),
        vscode.commands.registerCommand('dx.startForge', async () => {
            vscode.window.showInformationMessage('Forge daemon not available');
        }),
        vscode.commands.registerCommand('dx.stopForge', async () => {
            vscode.window.showInformationMessage('Forge daemon not running');
        }),
        vscode.commands.registerCommand('dx.convertFromJson', async () => {
            vscode.window.showInformationMessage('JSON conversion coming soon');
        }),
        vscode.commands.registerCommand('dx.convertAllFiles', async () => {
            vscode.window.showInformationMessage('Batch conversion coming soon');
        }),
        vscode.commands.registerCommand('dx.convertCurrentFile', async () => {
            await vscode.commands.executeCommand('dx.formatDense');
        })
    );

    outputChannel.appendLine('[DX] Extension activated successfully');
    console.log('[DX] Extension activated');
}

/**
 * Convert document to human-readable format
 */
async function convertToHumanFormat(doc: vscode.TextDocument): Promise<void> {
    const text = doc.getText();
    
    // Only convert if it's in dense format
    if (!formatter.isDense(text)) {
        return;
    }
    
    const pretty = formatter.toPretty(text);
    
    // Find the editor for this document
    const editor = vscode.window.visibleTextEditors.find(
        e => e.document.uri.toString() === doc.uri.toString()
    );
    
    if (!editor) return;
    
    isTransforming = true;
    try {
        await editor.edit(editBuilder => {
            const fullRange = new vscode.Range(
                doc.positionAt(0),
                doc.positionAt(text.length)
            );
            editBuilder.replace(fullRange, pretty);
        });
        
        statusBarItem.text = '$(eye) DX Human';
        outputChannel.appendLine(`[DX] Converted to human format: ${path.basename(doc.uri.fsPath)}`);
    } finally {
        isTransforming = false;
    }
}

export function deactivate() {
    console.log('[DX] Extension deactivated');
}

/**
 * Check if a URI is a DX file
 */
function isDxFile(uri: vscode.Uri): boolean {
    if (uri.scheme !== 'file') return false;
    
    const basename = path.basename(uri.fsPath);
    // Match: dx, *.dx, dx.* patterns
    return basename === 'dx' || 
           basename.endsWith('.dx') || 
           basename.startsWith('dx.');
}
