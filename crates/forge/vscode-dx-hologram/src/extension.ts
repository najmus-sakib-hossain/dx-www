/**
 * DX Extension - True Hologram View
 * 
 * File on disk: ALWAYS LLM dense format
 * Editor display: Human pretty format (virtual overlay)
 * User edits in pretty format, saved as dense format
 */

import * as vscode from 'vscode';
import * as path from 'path';
import { DxFormatter } from './formatter';

let formatter: DxFormatter;
let statusBarItem: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Hologram Extension activating...');
    
    formatter = new DxFormatter(3);

    // Status bar
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(mirror) LLM→Human';
    statusBarItem.tooltip = 'Hologram View: File stored as LLM, displayed as Human';
    context.subscriptions.push(statusBarItem);

    // Update status bar
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(updateStatusBar)
    );
    updateStatusBar(vscode.window.activeTextEditor);

    // Commands - Manual conversion only
    context.subscriptions.push(
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

    console.log('[DX] Extension activated - Use Ctrl+Shift+P → "DX: Format as Human-Readable" to view');
}

function updateStatusBar(editor: vscode.TextEditor | undefined) {
    if (editor && isDxFile(editor.document.uri)) {
        const text = editor.document.getText();
        const isLLM = !text.includes(' : ') && !text.includes(' > ');
        statusBarItem.text = isLLM ? '$(file-code) LLM Format' : '$(eye) Human Format';
        statusBarItem.tooltip = isLLM 
            ? 'Click to view as Human-Readable' 
            : 'Viewing Human Format (file saved as LLM)';
        statusBarItem.command = isLLM ? 'dx.formatPretty' : 'dx.formatDense';
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
