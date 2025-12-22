/**
 * DX Extension - Auto Hologram View
 * 
 * On Open: Auto-convert LLM → Human (for editing)
 * On Close/Save: Auto-convert Human → LLM (for storage/git)
 */

import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { DxFormatter } from './formatter';

let formatter: DxFormatter;
let statusBarItem: vscode.StatusBarItem;
let isProcessing = false;

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Hologram Extension activating...');
    
    formatter = new DxFormatter(3);

    // Status bar
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(mirror) Hologram';
    statusBarItem.tooltip = 'Auto-converting: LLM on disk ↔ Human in editor';
    context.subscriptions.push(statusBarItem);

    // Auto-convert to Human when opening
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (doc) => {
            if (isDxFile(doc.uri) && !isProcessing) {
                await convertToHumanIfNeeded(doc);
            }
        })
    );

    // Convert already open documents
    for (const doc of vscode.workspace.textDocuments) {
        if (isDxFile(doc.uri)) {
            await convertToHumanIfNeeded(doc);
        }
    }

    // Convert to LLM when closing (via workspace event)
    context.subscriptions.push(
        vscode.workspace.onDidCloseTextDocument(async (doc) => {
            if (isDxFile(doc.uri) && !isProcessing) {
                await convertToLLMOnClose(doc);
            }
        })
    );

    // Also convert to LLM on save (for git commits)
    context.subscriptions.push(
        vscode.workspace.onWillSaveTextDocument((e) => {
            if (isDxFile(e.document.uri)) {
                e.waitUntil(convertToLLMIfNeeded(e.document));
            }
        })
    );

    // Update status bar
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(updateStatusBar)
    );
    
    updateStatusBar(vscode.window.activeTextEditor);

    // Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.toggleFormat', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) return;
            
            const text = editor.document.getText();
            const isHuman = text.includes(' : ') || text.includes(' > ');
            const converted = isHuman ? formatter.toDense(text) : formatter.toPretty(text);
            
            await editor.edit(eb => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                eb.replace(fullRange, converted);
            });
            
            // Auto-save if converting to LLM format
            if (isHuman) {
                await editor.document.save();
            }
        }),
        
        vscode.commands.registerCommand('dx.formatPretty', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) return;
            
            const text = editor.document.getText();
            const formatted = formatter.toPretty(text);
            
            await editor.edit(eb => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                eb.replace(fullRange, formatted);
            });
        }),

        vscode.commands.registerCommand('dx.formatDense', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !isDxFile(editor.document.uri)) return;
            
            const text = editor.document.getText();
            const dense = formatter.toDense(text);
            
            await editor.edit(eb => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                eb.replace(fullRange, dense);
            });
            
            // Auto-save after converting to dense
            await editor.document.save();
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
        vscode.commands.registerCommand('dx.normalizeFormat', () => vscode.commands.executeCommand('dx.formatDense')),
        vscode.commands.registerCommand('dx.buildBinary', () => vscode.window.showInformationMessage('Binary build coming soon')),
        vscode.commands.registerCommand('dx.startForge', () => {}),
        vscode.commands.registerCommand('dx.stopForge', () => {}),
        vscode.commands.registerCommand('dx.convertFromJson', () => {}),
        vscode.commands.registerCommand('dx.convertAllFiles', () => {}),
        vscode.commands.registerCommand('dx.convertCurrentFile', () => vscode.commands.executeCommand('dx.formatDense'))
    );

    console.log('[DX] Hologram Extension activated');
}

async function convertToHumanIfNeeded(doc: vscode.TextDocument) {
    if (isProcessing) return;
    
    const text = doc.getText();
    
    // Already in human format?
    if (text.includes(' : ') || text.includes(' > ')) {
        return;
    }
    
    isProcessing = true;
    
    try {
        // Check if this is the root dx config file
        const basename = path.basename(doc.uri.fsPath);
        const isRootConfig = basename === 'dx' && !basename.includes('.');
        
        const pretty = formatter.toPretty(text, isRootConfig);
        const edit = new vscode.WorkspaceEdit();
        const fullRange = new vscode.Range(
            doc.positionAt(0),
            doc.positionAt(text.length)
        );
        edit.replace(doc.uri, fullRange, pretty);
        await vscode.workspace.applyEdit(edit);
    } finally {
        isProcessing = false;
    }
}

async function convertToLLMIfNeeded(doc: vscode.TextDocument): Promise<vscode.TextEdit[]> {
    if (isProcessing) return [];
    
    const text = doc.getText();
    
    // Already in LLM format?
    if (!text.includes(' : ') && !text.includes(' > ')) {
        return [];
    }
    
    const dense = formatter.toDense(text);
    const fullRange = new vscode.Range(
        doc.positionAt(0),
        doc.positionAt(text.length)
    );
    
    return [vscode.TextEdit.replace(fullRange, dense)];
}

async function convertToLLMOnClose(doc: vscode.TextDocument) {
    if (isProcessing) return;
    
    const text = doc.getText();
    
    // Already in LLM format?
    if (!text.includes(' : ') && !text.includes(' > ')) {
        return;
    }
    
    isProcessing = true;
    
    try {
        const dense = formatter.toDense(text);
        // Write directly to file system
        const fs = require('fs');
        fs.writeFileSync(doc.uri.fsPath, dense, 'utf8');
    } finally {
        isProcessing = false;
    }
}

function updateStatusBar(editor: vscode.TextEditor | undefined) {
    if (editor && isDxFile(editor.document.uri)) {
        statusBarItem.text = '$(mirror) Hologram';
        statusBarItem.tooltip = 'LLM on disk ↔ Human in editor';
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
