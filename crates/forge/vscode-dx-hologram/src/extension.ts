/**
 * DX Extension - Cache Layer Architecture
 * 
 * The CLEAN solution:
 * - dx file on disk = LLM-Dense format (source of truth for git)
 * - .dx/cache/dx.human = Human-Pretty format (what you edit)
 * - On open dx: intercept and open the human file instead
 * - On save human: sync back to dx (deflate)
 * 
 * NO BUGS. NO FLICKERING. JUST WORKS.
 */

import * as vscode from 'vscode';
import * as path from 'path';
import { SyncManager } from './syncManager';
import { DxPaths, isDxFile, isHumanFile, findAllDxFiles } from './pathUtils';
import { DxFormatter } from './formatter';

let syncManager: SyncManager;
let statusBar: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext) {
    console.log('[DX] Cache Layer Extension activating...');
    
    syncManager = new SyncManager();
    
    // ═══════════════════════════════════════════════════════════════
    // INTERCEPT: When user clicks "dx" file, open human version
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (doc) => {
            // Only intercept actual dx files (not human files)
            if (doc.uri.scheme === 'file' && isDxFile(doc.uri.fsPath)) {
                // Small delay to let VS Code finish opening
                setTimeout(async () => {
                    // Close the raw dx file
                    await vscode.commands.executeCommand('workbench.action.closeActiveEditor');
                    
                    // Open the human version instead
                    await syncManager.openHumanVersion(doc.uri.fsPath);
                }, 50);
            }
        })
    );
    
    // ═══════════════════════════════════════════════════════════════
    // SYNC: When human file is saved, sync to dense
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(async (doc) => {
            if (isHumanFile(doc.uri.fsPath)) {
                await syncManager.onHumanFileSaved(doc.uri.fsPath);
            }
        })
    );
    
    // ═══════════════════════════════════════════════════════════════
    // EXTERNAL CHANGES: Detect when dx file changes (git pull, etc)
    // ═══════════════════════════════════════════════════════════════
    
    const dxWatcher = vscode.workspace.createFileSystemWatcher('**/dx');
    
    context.subscriptions.push(
        dxWatcher.onDidChange(async (uri) => {
            const dxPaths = new DxPaths(uri.fsPath);
            
            // Check if we have the human file open
            const openDoc = vscode.workspace.textDocuments.find(
                doc => doc.uri.fsPath === dxPaths.humanFile
            );
            
            if (openDoc) {
                await syncManager.checkForExternalChanges(dxPaths.humanFile);
            }
        })
    );
    
    context.subscriptions.push(dxWatcher);
    
    // ═══════════════════════════════════════════════════════════════
    // COMMANDS
    // ═══════════════════════════════════════════════════════════════
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.openHuman', async (uri?: vscode.Uri) => {
            const targetUri = uri || vscode.window.activeTextEditor?.document.uri;
            if (targetUri) {
                if (isDxFile(targetUri.fsPath)) {
                    await syncManager.openHumanVersion(targetUri.fsPath);
                } else if (isHumanFile(targetUri.fsPath)) {
                    // Already a human file, just ensure it's visible
                    const doc = await vscode.workspace.openTextDocument(targetUri);
                    await vscode.window.showTextDocument(doc);
                }
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.showDense', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            
            const filePath = editor.document.uri.fsPath;
            if (!isHumanFile(filePath)) {
                vscode.window.showWarningMessage('Not a DX human file');
                return;
            }
            
            const dxPaths = new DxPaths(filePath);
            if (!dxPaths.isValid()) {
                vscode.window.showWarningMessage('No corresponding dx file found');
                return;
            }
            
            try {
                const content = await vscode.workspace.fs.readFile(vscode.Uri.file(dxPaths.dxFile));
                const doc = await vscode.workspace.openTextDocument({
                    content: Buffer.from(content).toString('utf-8'),
                    language: 'dx'
                });
                await vscode.window.showTextDocument(doc, {
                    viewColumn: vscode.ViewColumn.Beside,
                    preview: true,
                    preserveFocus: true
                });
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to read dx file: ${error}`);
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.sync', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            
            if (isHumanFile(editor.document.uri.fsPath)) {
                await editor.document.save();
                vscode.window.showInformationMessage('DX synced!');
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.init', async (uri?: vscode.Uri) => {
            let targetDir: string;
            
            if (uri) {
                const stat = await vscode.workspace.fs.stat(uri);
                targetDir = stat.type === vscode.FileType.Directory 
                    ? uri.fsPath 
                    : path.dirname(uri.fsPath);
            } else {
                const folders = vscode.workspace.workspaceFolders;
                if (!folders) {
                    vscode.window.showErrorMessage('No workspace open');
                    return;
                }
                targetDir = folders[0].uri.fsPath;
            }
            
            await syncManager.initDxFile(targetDir);
        })
    );
    
    // Format commands (for manual use)
    const formatter = new DxFormatter(3);
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.formatPretty', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            
            const text = editor.document.getText();
            const pretty = formatter.toPretty(text, true);
            
            await editor.edit(eb => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                eb.replace(fullRange, pretty);
            });
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.formatDense', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            
            const text = editor.document.getText();
            const dense = formatter.toDense(text);
            
            await editor.edit(eb => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                eb.replace(fullRange, dense);
            });
        })
    );
    
    // Placeholder commands
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.openHologram', () => vscode.commands.executeCommand('dx.openHuman')),
        vscode.commands.registerCommand('dx.normalizeFormat', () => vscode.commands.executeCommand('dx.formatDense')),
        vscode.commands.registerCommand('dx.buildBinary', () => vscode.window.showInformationMessage('Binary build coming soon')),
        vscode.commands.registerCommand('dx.startForge', () => {}),
        vscode.commands.registerCommand('dx.stopForge', () => {}),
        vscode.commands.registerCommand('dx.convertFromJson', () => {}),
        vscode.commands.registerCommand('dx.convertAllFiles', () => {}),
        vscode.commands.registerCommand('dx.convertCurrentFile', () => vscode.commands.executeCommand('dx.formatDense')),
        vscode.commands.registerCommand('dx.toggleFormat', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            
            const text = editor.document.getText();
            const isHuman = text.includes(' : ') || text.includes(' > ');
            const converted = isHuman ? formatter.toDense(text) : formatter.toPretty(text, true);
            
            await editor.edit(eb => {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(text.length)
                );
                eb.replace(fullRange, converted);
            });
        })
    );
    
    // ═══════════════════════════════════════════════════════════════
    // STATUS BAR
    // ═══════════════════════════════════════════════════════════════
    
    statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBar.command = 'dx.showDense';
    context.subscriptions.push(statusBar);
    
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(editor => {
            if (editor && isHumanFile(editor.document.uri.fsPath)) {
                statusBar.text = '$(zap) DX Quantum';
                statusBar.tooltip = 'Click to view LLM format';
                statusBar.show();
            } else {
                statusBar.hide();
            }
        })
    );
    
    // ═══════════════════════════════════════════════════════════════
    // STARTUP: Initialize caches for existing dx files
    // ═══════════════════════════════════════════════════════════════
    
    const folders = vscode.workspace.workspaceFolders;
    if (folders) {
        for (const folder of folders) {
            try {
                const dxFiles = await findAllDxFiles(folder.uri.fsPath);
                console.log(`[DX] Found ${dxFiles.length} dx files in ${folder.name}`);
                
                // Pre-create caches
                for (const dxPaths of dxFiles) {
                    await syncManager.getCacheManager().ensureHumanCache(dxPaths);
                }
            } catch (error) {
                console.error(`[DX] Error scanning ${folder.name}:`, error);
            }
        }
    }
    
    console.log('[DX] Cache Layer Extension activated!');
}

export function deactivate() {
    console.log('[DX] Extension deactivated');
}
