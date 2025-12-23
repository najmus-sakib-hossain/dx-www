/**
 * DX Serializer VS Code Extension - Entry Point
 * 
 * Provides seamless editing of .dx files by displaying human-readable format
 * in the editor while storing dense, token-efficient format on disk.
 * 
 * Requirements: 1.1, 6.1
 */

import * as vscode from 'vscode';
import { loadDxCore, DxCore } from './dxCore';
import { DxDocumentManager, DocumentManagerConfig } from './dxDocumentManager';
import { DxLensFileSystem } from './dxLensFileSystem';
import { DX_LENS_SCHEME, isExactlyDxFile, getLensUri } from './utils';

/**
 * Extension context holding all components
 */
interface ExtensionContext {
    dxCore: DxCore;
    documentManager: DxDocumentManager;
    lensFileSystem: DxLensFileSystem;
    statusBarItem: vscode.StatusBarItem;
}

let extensionContext: ExtensionContext | undefined;

/**
 * Activate the extension
 */
export async function activate(context: vscode.ExtensionContext): Promise<void> {
    console.log('DX Serializer: Activating extension...');

    try {
        // 1. Load configuration
        const config = loadConfiguration();

        // 2. Load WASM core (with fallback)
        const dxCore = await loadDxCore(context.extensionPath, config.indentSize);
        console.log(`DX Serializer: Using ${dxCore.isWasm ? 'WASM' : 'TypeScript fallback'} core`);

        // 3. Initialize DocumentManager
        const documentManager = new DxDocumentManager(dxCore, config);
        context.subscriptions.push(documentManager);

        // 4. Initialize LensFileSystem
        const lensFileSystem = new DxLensFileSystem(dxCore, documentManager);
        context.subscriptions.push(lensFileSystem);

        // 5. Register FileSystemProvider for 'dxlens' scheme
        context.subscriptions.push(
            vscode.workspace.registerFileSystemProvider(DX_LENS_SCHEME, lensFileSystem, {
                isCaseSensitive: false,
                isReadonly: false,
            })
        );

        // 6. Create status bar item
        const statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Right,
            100
        );
        statusBarItem.command = 'dx.showDenseView';
        context.subscriptions.push(statusBarItem);

        // Store extension context
        extensionContext = {
            dxCore,
            documentManager,
            lensFileSystem,
            statusBarItem,
        };

        // 7. Set up file watcher for external changes
        setupFileWatcher(context, documentManager);

        // 8. Register commands
        registerCommands(context, documentManager, statusBarItem);

        // 9. Set up auto-redirect from file:// to dxlens:// for .dx files
        setupAutoRedirect(context);

        // 10. Set up configuration change listener
        setupConfigurationListener(context, documentManager);

        // 11. Set up active editor change listener for status bar
        setupActiveEditorListener(context, documentManager, statusBarItem);

        console.log('DX Serializer: Extension activated successfully');

    } catch (error) {
        console.error('DX Serializer: Activation failed:', error);
        vscode.window.showErrorMessage(`DX Serializer: Failed to activate: ${error}`);
    }
}

/**
 * Deactivate the extension
 */
export function deactivate(): void {
    console.log('DX Serializer: Deactivating extension...');
    extensionContext = undefined;
}

/**
 * Load configuration from VS Code settings
 */
function loadConfiguration(): DocumentManagerConfig {
    const config = vscode.workspace.getConfiguration('dx');
    return {
        validateBeforeSave: config.get<boolean>('validateBeforeSave', true),
        autoSaveGracePeriod: config.get<number>('autoSaveGracePeriod', 2000),
        indentSize: config.get<number>('indentSize', 2) as 2 | 4,
    };
}

/**
 * Set up file watcher for external changes
 */
function setupFileWatcher(
    context: vscode.ExtensionContext,
    documentManager: DxDocumentManager
): void {
    // Watch for .dx file changes in the workspace
    const watcher = vscode.workspace.createFileSystemWatcher('**/*.dx');

    // Handle external file changes
    context.subscriptions.push(
        watcher.onDidChange(async (uri) => {
            if (isExactlyDxFile(uri) && !documentManager.isWriting(uri)) {
                await documentManager.handleExternalChange(uri);
            }
        })
    );

    // Handle file deletion
    context.subscriptions.push(
        watcher.onDidDelete((uri) => {
            if (isExactlyDxFile(uri)) {
                documentManager.handleFileDeleted(uri);
            }
        })
    );

    context.subscriptions.push(watcher);
}


/**
 * Register extension commands
 */
function registerCommands(
    context: vscode.ExtensionContext,
    documentManager: DxDocumentManager,
    _statusBarItem: vscode.StatusBarItem
): void {
    // DX: Refresh from Disk
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.refreshFromDisk', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('DX: No active editor');
                return;
            }

            const uri = editor.document.uri;
            if (!isExactlyDxFile(uri)) {
                vscode.window.showWarningMessage('DX: Not a .dx file');
                return;
            }

            try {
                const newContent = await documentManager.forceRefresh(uri);

                // Update the editor with new content
                const edit = new vscode.WorkspaceEdit();
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(editor.document.getText().length)
                );
                edit.replace(uri, fullRange, newContent);
                await vscode.workspace.applyEdit(edit);

                vscode.window.showInformationMessage('DX: Refreshed from disk');
            } catch (error) {
                vscode.window.showErrorMessage(`DX: Refresh failed: ${error}`);
            }
        })
    );

    // DX: Force Save
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.forceSave', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('DX: No active editor');
                return;
            }

            const uri = editor.document.uri;
            if (!isExactlyDxFile(uri)) {
                vscode.window.showWarningMessage('DX: Not a .dx file');
                return;
            }

            await documentManager.forceSave(uri);
        })
    );

    // DX: Show Dense View
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.showDenseView', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('DX: No active editor');
                return;
            }

            const uri = editor.document.uri;
            if (!isExactlyDxFile(uri)) {
                vscode.window.showWarningMessage('DX: Not a .dx file');
                return;
            }

            const denseContent = documentManager.getDenseContent(uri);
            if (!denseContent) {
                vscode.window.showWarningMessage('DX: No content available');
                return;
            }

            // Open a new untitled document with the dense content
            const doc = await vscode.workspace.openTextDocument({
                content: denseContent,
                language: 'dx',
            });
            await vscode.window.showTextDocument(doc, {
                viewColumn: vscode.ViewColumn.Beside,
                preview: true,
                preserveFocus: true,
            });
        })
    );
}


/**
 * Set up auto-redirect from file:// to dxlens:// for .dx files
 * 
 * When a user opens a .dx file directly (file://), we redirect to the
 * dxlens:// scheme so our FileSystemProvider handles the transformation.
 */
function setupAutoRedirect(context: vscode.ExtensionContext): void {
    // Listen for document opens
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(async (document) => {
            // Only redirect file:// scheme .dx files
            if (document.uri.scheme !== 'file') {
                return;
            }

            if (!isExactlyDxFile(document.uri)) {
                return;
            }

            // Close the file:// document and open with dxlens://
            const lensUri = getLensUri(document.uri);

            // Use a small delay to avoid race conditions
            setTimeout(async () => {
                try {
                    // Close the current editor if it's showing the file:// version
                    const editors = vscode.window.visibleTextEditors;
                    for (const editor of editors) {
                        if (editor.document.uri.toString() === document.uri.toString()) {
                            await vscode.commands.executeCommand('workbench.action.closeActiveEditor');
                            break;
                        }
                    }

                    // Open with dxlens:// scheme
                    const doc = await vscode.workspace.openTextDocument(lensUri);
                    await vscode.window.showTextDocument(doc);
                } catch (error) {
                    console.error('DX Serializer: Auto-redirect failed:', error);
                }
            }, 50);
        })
    );
}

/**
 * Set up configuration change listener
 */
function setupConfigurationListener(
    context: vscode.ExtensionContext,
    documentManager: DxDocumentManager
): void {
    context.subscriptions.push(
        vscode.workspace.onDidChangeConfiguration((event) => {
            if (event.affectsConfiguration('dx')) {
                const config = loadConfiguration();
                documentManager.updateConfig(config);
                console.log('DX Serializer: Configuration updated');
            }
        })
    );
}

/**
 * Set up active editor change listener for status bar updates
 */
function setupActiveEditorListener(
    context: vscode.ExtensionContext,
    documentManager: DxDocumentManager,
    statusBarItem: vscode.StatusBarItem
): void {
    // Update status bar when active editor changes
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor((editor) => {
            updateStatusBar(editor, documentManager, statusBarItem);
        })
    );

    // Update status bar when document changes
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument((event) => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document === event.document) {
                // Notify document manager of content change
                if (isExactlyDxFile(event.document.uri)) {
                    documentManager.handleContentChange(
                        event.document.uri,
                        event.document.getText()
                    );
                }
                updateStatusBar(editor, documentManager, statusBarItem);
            }
        })
    );

    // Initial status bar update
    updateStatusBar(vscode.window.activeTextEditor, documentManager, statusBarItem);
}

/**
 * Update the status bar based on current editor state
 */
function updateStatusBar(
    editor: vscode.TextEditor | undefined,
    documentManager: DxDocumentManager,
    statusBarItem: vscode.StatusBarItem
): void {
    if (!editor || !isExactlyDxFile(editor.document.uri)) {
        statusBarItem.hide();
        return;
    }

    const state = documentManager.getState(editor.document.uri);

    if (!state) {
        statusBarItem.text = '$(file-code) DX';
        statusBarItem.tooltip = 'DX Serializer active';
        statusBarItem.backgroundColor = undefined;
        statusBarItem.show();
        return;
    }

    if (state.isValid) {
        statusBarItem.text = '$(check) DX';
        statusBarItem.tooltip = 'DX: Valid - Click to show dense format';
        statusBarItem.backgroundColor = undefined;
    } else {
        statusBarItem.text = '$(warning) DX';
        statusBarItem.tooltip = `DX: ${state.lastError || 'Invalid'} - Click to show dense format`;
        statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
    }

    statusBarItem.show();
}
