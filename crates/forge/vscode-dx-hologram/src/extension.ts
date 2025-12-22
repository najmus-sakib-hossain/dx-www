/**
 * DX Extension - Hologram View
 * File stored in LLM dense format, displayed in Human pretty format
 */

import * as vscode from 'vscode';
import * as path from 'path';
import { DxFormatter } from './formatter';

let formatter: DxFormatter;
let statusBarItem: vscode.StatusBarItem;
const originalContent = new Map<string, string>();

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Extension activating...');
    
    formatter = new DxFormatter(3);

    // Status bar
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.command = 'dx.formatPretty';
    context.subscriptions.push(statusBarItem);

    // Auto-format on open
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (doc) => {
            if (isDxFile(doc.uri)) {
                await autoFormatDocument(doc);
            }
        })
    );

    // Auto-convert to dense on save
    context.subscriptions.push(
        vscode.workspace.onWillSaveTextDocument((e) => {
            if (isDxFile(e.document.uri)) {
                e.waitUntil(convertToDenseOnSave(e.document));
            }
        })
    );

    // Update status bar on editor change
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(updateStatusBar)
    );
    updateStatusBar(vscode.window.activeTextEditor);

    // Format already open documents
    for (const doc of vscode.workspace.textDocuments) {
        if (isDxFile(doc.uri)) {
            await autoFormatDocument(doc);
        }
    }

    // Format already open documents
    for (const doc of vscode.workspace.textDocuments) {
        if (isDxFile(doc.uri)) {
            await autoFormatDocument(doc);
        }
    }

    // Document formatting provider
    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider('dx', {
            provideDocumentFormattingEdits(doc: vscode.TextDocument): vscode.TextEdit[] {
                const text = doc.getText();
                const formatted = formatter.toPretty(text);
                return [vscode.TextEdit.replace(
                    new vscode.Range(doc.positionAt(0), doc.positionAt(text.length)),
                    formatted
                )];
            }
        })
    );

    // Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.formatPretty', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) return;
            
            const text = editor.document.getText();
            const formatted = formatter.toPretty(text);
            
            await editor.edit(eb => {
                eb.replace(new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                ), formatted);
            });
        }),

        vscode.commands.registerCommand('dx.formatDense', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) return;
            
            const text = editor.document.getText();
            const dense = formatter.toDense(text);
            
            await editor.edit(eb => {
                eb.replace(new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                ), dense);
            });
        }),

        vscode.commands.registerCommand('dx.showLLMFormat', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) return;
            
            const dense = formatter.toDense(editor.document.getText());
            const doc = await vscode.workspace.openTextDocument({ content: dense, language: 'dx' });
            await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
        }),

        // Placeholder commands
        vscode.commands.registerCommand('dx.openHologram', () => vscode.commands.executeCommand('dx.formatPretty')),
        vscode.commands.registerCommand('dx.normalizeFormat', () => vscode.commands.executeCommand('dx.formatPretty')),
        vscode.commands.registerCommand('dx.buildBinary', () => vscode.window.showInformationMessage('Binary build coming soon')),
        vscode.commands.registerCommand('dx.startForge', () => {}),
        vscode.commands.registerCommand('dx.stopForge', () => {}),
        vscode.commands.registerCommand('dx.convertFromJson', () => {}),
        vscode.commands.registerCommand('dx.convertAllFiles', () => {}),
        vscode.commands.registerCommand('dx.convertCurrentFile', () => vscode.commands.executeCommand('dx.formatDense'))
    );

    console.log('[DX] Extension activated');
}

async function autoFormatDocument(doc: vscode.TextDocument) {
    const text = doc.getText();
    
    // Store original dense content
    originalContent.set(doc.uri.toString(), text);
    
    // Check if already in pretty format (has proper spacing)
    if (text.includes(' : ') || text.includes(' > ')) {
        return; // Already formatted
    }
    
    // Convert to pretty format
    const pretty = formatter.toPretty(text);
    
    // Apply the formatting
    const edit = new vscode.WorkspaceEdit();
    edit.replace(
        doc.uri,
        new vscode.Range(doc.positionAt(0), doc.positionAt(text.length)),
        pretty
    );
    await vscode.workspace.applyEdit(edit);
}

async function convertToDenseOnSave(doc: vscode.TextDocument): Promise<vscode.TextEdit[]> {
    const text = doc.getText();
    
    // Only convert if in pretty format
    if (text.includes(' : ') || text.includes(' > ')) {
        const dense = formatter.toDense(text);
        return [vscode.TextEdit.replace(
            new vscode.Range(doc.positionAt(0), doc.positionAt(text.length)),
            dense
        )];
    }
    
    return [];
}

function updateStatusBar(editor: vscode.TextEditor | undefined) {
    if (editor && isDxFile(editor.document.uri)) {
        statusBarItem.text = '$(file-code) DX Format';
        statusBarItem.tooltip = 'Click to format as Human-readable';
        statusBarItem.show();
    } else {
        statusBarItem.hide();
    }
}

function isDxFile(uri: vscode.Uri): boolean {
    if (uri.scheme !== 'file') return false;
    const basename = path.basename(uri.fsPath);
    return basename === 'dx' || basename.endsWith('.dx') || basename.startsWith('dx.');
}

export function deactivate() {}
