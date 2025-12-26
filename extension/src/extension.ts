/**
 * DX VS Code Extension - Entry Point
 * 
 * The unified DX ecosystem extension providing:
 * - Serializer: Seamless editing of .dx files with human-readable display
 * - Forge Integration: Connection to Forge daemon for tool orchestration
 * 
 * Features:
 * - Human Format V3: Clean vertical key-value layout
 * - Multi-format input: Auto-convert JSON, YAML, TOML, CSV to DX
 * - Cache generation: .dx/cache/{filename}.human and .machine files
 * - Forge daemon status bar and commands
 * 
 * Requirements: 1.1-1.7, 3.2-3.4, 6.1
 */

import * as vscode from 'vscode';
import { loadDxCore, DxCore } from './dxCore';
import { DxDocumentManager, DocumentManagerConfig } from './dxDocumentManager';
import { DxLensFileSystem } from './dxLensFileSystem';
import { DX_LENS_SCHEME, isExactlyDxFile, getLensUri } from './utils';
import { detectFormat, isSourceFormat, DetectedFormat } from './formatDetector';
import { convertJsonToDocument, convertYamlToDocument, convertTomlToDocument, convertCsvToDocument } from './converters';
import { serializeToLlmV3, parseHumanV3 } from './humanParserV3';
import { formatDocumentV3, DEFAULT_CONFIG } from './humanFormatterV3';
import { getForgeClient, ForgeStatusBar, registerForgeCommands } from './forge';

/**
 * Extension context holding all components
 */
interface ExtensionContext {
    dxCore: DxCore;
    documentManager: DxDocumentManager;
    lensFileSystem: DxLensFileSystem;
    statusBarItem: vscode.StatusBarItem;
    forgeStatusBar?: ForgeStatusBar;
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
        const dxCore = await loadDxCore(context.extensionPath, config.indentSize, config.keyPadding);
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
        setupAutoRedirect(context, dxCore);

        // 10. Set up configuration change listener
        setupConfigurationListener(context, documentManager);

        // 11. Set up active editor change listener for status bar
        setupActiveEditorListener(context, documentManager, statusBarItem);

        // 12. Register DocumentFormattingEditProvider for safe format-on-save
        registerFormattingProvider(context, dxCore);

        // 13. Initialize Forge integration
        await initializeForgeIntegration(context);

        console.log('DX: Extension activated successfully (V3 format + Forge)');

    } catch (error) {
        console.error('DX Serializer: Activation failed:', error);
        vscode.window.showErrorMessage(`DX Serializer: Failed to activate: ${error}`);
    }
}

/**
 * Deactivate the extension
 */
export function deactivate(): void {
    console.log('DX: Deactivating extension...');

    // Disconnect from Forge daemon
    const forgeClient = getForgeClient();
    forgeClient.disconnect();

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
        keyPadding: config.get<number>('keyPadding', 20),
        formatOnSave: config.get<boolean>('formatOnSave', true),
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

    // DX: Set Color Theme
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.setColorTheme', async () => {
            // Open the theme picker with DX themes pre-filtered
            await vscode.commands.executeCommand('workbench.action.selectTheme');
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
 * Also handles format detection and conversion for non-LLM formats
 * 
 * When a user opens a .dx file directly (file://), we:
 * 1. Detect the format (JSON, YAML, TOML, CSV, LLM, Human V3)
 * 2. Convert non-LLM formats to LLM format
 * 3. Redirect to dxlens:// scheme for human display
 * 
 * Requirements: 1.1-1.7, 3.2-3.4
 */
function setupAutoRedirect(context: vscode.ExtensionContext, dxCore: DxCore): void {
    // Track URIs we're currently redirecting to avoid loops
    const redirectingUris = new Set<string>();

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

            const uriString = document.uri.toString();

            // Avoid redirect loops
            if (redirectingUris.has(uriString)) {
                return;
            }

            redirectingUris.add(uriString);

            try {
                // Detect format and convert if needed
                const content = document.getText();
                const detection = detectFormat(content);

                if (isSourceFormat(detection.format)) {
                    // Convert source format to LLM format
                    await convertAndSaveFile(document.uri, content, detection.format, dxCore);
                }

                // Close the file:// document and open with dxlens://
                const lensUri = getLensUri(document.uri);

                // Use a small delay to avoid race conditions
                setTimeout(async () => {
                    try {
                        // Close all editors showing the file:// version
                        for (const tabGroup of vscode.window.tabGroups.all) {
                            for (const tab of tabGroup.tabs) {
                                if (tab.input instanceof vscode.TabInputText) {
                                    if (tab.input.uri.toString() === uriString) {
                                        await vscode.window.tabGroups.close(tab);
                                    }
                                }
                            }
                        }

                        // Open with dxlens:// scheme
                        const doc = await vscode.workspace.openTextDocument(lensUri);
                        await vscode.window.showTextDocument(doc);
                    } catch (error) {
                        console.error('DX Serializer: Auto-redirect failed:', error);
                    } finally {
                        redirectingUris.delete(uriString);
                    }
                }, 50);
            } catch (error) {
                redirectingUris.delete(uriString);
                console.error('DX Serializer: Auto-redirect error:', error);
            }
        })
    );
}

/**
 * Convert a source format file to LLM format and save
 * Requirements: 1.1-1.4
 */
async function convertAndSaveFile(
    uri: vscode.Uri,
    content: string,
    format: DetectedFormat,
    _dxCore: DxCore
): Promise<void> {
    try {
        let conversionResult;

        switch (format) {
            case 'json':
                conversionResult = convertJsonToDocument(content);
                break;
            case 'yaml':
                conversionResult = convertYamlToDocument(content);
                break;
            case 'toml':
                conversionResult = convertTomlToDocument(content);
                break;
            case 'csv':
                conversionResult = convertCsvToDocument(content);
                break;
            default:
                return;
        }

        if (!conversionResult.success || !conversionResult.document) {
            console.warn(`DX Serializer: Failed to convert ${format}: ${conversionResult.error}`);
            return;
        }

        // Serialize to LLM format
        const llmContent = serializeToLlmV3(conversionResult.document);

        // Write back to file
        const fs = await import('fs');
        await fs.promises.writeFile(uri.fsPath, llmContent, 'utf-8');

        vscode.window.showInformationMessage(
            `DX: Converted ${format.toUpperCase()} to LLM format`
        );
    } catch (error) {
        console.error(`DX Serializer: Conversion failed:`, error);
    }
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

/**
 * Format a DX document and return TextEdit array
 * Shared logic for both formatting provider and willSave handler
 * 
 * Handles both LLM format and Human V3 format:
 * - LLM format: Parse with parseLlm, then format to Human V3
 * - Human V3 format: Parse with parseHumanV3, then re-format for alignment
 */
function formatDxDocument(document: vscode.TextDocument): vscode.TextEdit[] | null {
    try {
        const content = document.getText();

        // Skip empty documents
        if (!content.trim()) {
            return null;
        }

        // Detect the format
        const detection = detectFormat(content);
        let dxDocument: import('./llmParser').DxDocument | null = null;

        if (detection.format === 'llm') {
            // Parse LLM format
            const { parseLlm } = require('./llmParser');
            const parseResult = parseLlm(content);
            if (!parseResult.success || !parseResult.document) {
                console.log('DX Format: LLM parse failed, skipping format');
                return null;
            }
            dxDocument = parseResult.document;
        } else if (detection.format === 'human-v3') {
            // Parse Human V3 format
            const parseResult = parseHumanV3(content);
            if (!parseResult.success || !parseResult.document) {
                console.log('DX Format: Human V3 parse failed, skipping format');
                return null;
            }
            dxDocument = parseResult.document;
        } else {
            // Unknown or other format - try Human V3 as fallback
            const parseResult = parseHumanV3(content);
            if (!parseResult.success || !parseResult.document) {
                console.log(`DX Format: Format '${detection.format}' not supported for formatting`);
                return null;
            }
            dxDocument = parseResult.document;
        }

        // Safety check (should never happen due to early returns above)
        if (!dxDocument) {
            return null;
        }

        // Get config from settings
        const config = vscode.workspace.getConfiguration('dx');
        const keyPadding = config.get<number>('keyPadding', 20);

        // Format the document with proper alignment
        const formattedContent = formatDocumentV3(dxDocument, {
            ...DEFAULT_CONFIG,
            keyPadding,
        });

        // Skip if content is unchanged
        if (formattedContent === content) {
            return null;
        }

        // Return a single edit that replaces the entire document
        const fullRange = new vscode.Range(
            document.positionAt(0),
            document.positionAt(content.length)
        );

        return [vscode.TextEdit.replace(fullRange, formattedContent)];
    } catch (error) {
        console.error('DX Format: Error during formatting:', error);
        return null;
    }
}

/**
 * Register DocumentFormattingEditProvider for safe format-on-save
 * 
 * This is the proper VS Code way to handle formatting:
 * 1. Parse the human content
 * 2. Convert to LLM format (normalizes the data)
 * 3. Convert back to human format (applies proper alignment)
 * 4. Return TextEdit array for VS Code to apply safely
 * 
 * Also registers willSaveTextDocument handler for auto-save support
 */
function registerFormattingProvider(
    context: vscode.ExtensionContext,
    dxCore: DxCore
): void {
    // Create the formatting provider
    const formattingProvider: vscode.DocumentFormattingEditProvider = {
        provideDocumentFormattingEdits(
            document: vscode.TextDocument,
            _options: vscode.FormattingOptions,
            _token: vscode.CancellationToken
        ): vscode.TextEdit[] | null {
            return formatDxDocument(document);
        }
    };

    // Register for both dxlens scheme and file scheme
    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider(
            { scheme: DX_LENS_SCHEME, language: 'dx' },
            formattingProvider
        )
    );

    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider(
            { scheme: 'file', language: 'dx' },
            formattingProvider
        )
    );

    // willSaveTextDocument handler for auto-format on save
    // Re-enabled after fixing format detection to handle both LLM and Human V3 formats
    context.subscriptions.push(
        vscode.workspace.onWillSaveTextDocument((event) => {
            // Only format .dx files
            if (!isExactlyDxFile(event.document.uri)) {
                return;
            }

            // Check if formatOnSave is enabled
            const config = vscode.workspace.getConfiguration('dx');
            const formatOnSave = config.get<boolean>('formatOnSave', true);
            if (!formatOnSave) {
                return;
            }

            // Wait for formatting edits and apply them before save
            event.waitUntil(
                Promise.resolve().then(() => {
                    const edits = formatDxDocument(event.document);
                    return edits || [];
                })
            );
        })
    );

    // Register the "DX: Format Document" command
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.formatDocument', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('DX: No active editor');
                return;
            }

            if (!isExactlyDxFile(editor.document.uri)) {
                vscode.window.showWarningMessage('DX: Not a .dx file');
                return;
            }

            // Use VS Code's built-in format command which will use our provider
            await vscode.commands.executeCommand('editor.action.formatDocument');
        })
    );

    console.log('DX Serializer: Formatting provider registered (with auto-save support)');
}


/**
 * Initialize Forge daemon integration
 */
async function initializeForgeIntegration(context: vscode.ExtensionContext): Promise<void> {
    try {
        // Register Forge commands
        registerForgeCommands(context);

        // Get Forge client
        const forgeClient = getForgeClient();

        // Create status bar
        const forgeStatusBar = new ForgeStatusBar(forgeClient);
        context.subscriptions.push({ dispose: () => forgeStatusBar.dispose() });

        // Store in extension context
        if (extensionContext) {
            extensionContext.forgeStatusBar = forgeStatusBar;
        }

        // Auto-connect if enabled - DISABLED to prevent loops
        // Users should manually start the daemon with "DX: Start Forge Daemon"
        const config = vscode.workspace.getConfiguration('dx.forge');
        if (config.get('autoConnect', true)) {
            // Just show disconnected state, don't try to connect automatically
            forgeStatusBar.showDisconnected();
            console.log('DX: Forge daemon not running. Use "DX: Start Forge Daemon" to start.');
        }

        // Set up file change notifications to Forge
        setupForgeFileNotifications(context, forgeClient);

        console.log('DX: Forge integration initialized');
    } catch (error) {
        console.error('DX: Failed to initialize Forge integration:', error);
    }
}

/**
 * Set up file change notifications to Forge daemon
 */
function setupForgeFileNotifications(
    context: vscode.ExtensionContext,
    forgeClient: import('./forge').ForgeClient
): void {
    // Watch for file saves
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(async (document) => {
            if (forgeClient.isConnected()) {
                const relativePath = vscode.workspace.asRelativePath(document.uri);
                await forgeClient.notifyFileChange(relativePath, 'modified');
            }
        })
    );

    // Watch for file creates
    const watcher = vscode.workspace.createFileSystemWatcher('**/*');

    context.subscriptions.push(
        watcher.onDidCreate(async (uri) => {
            if (forgeClient.isConnected()) {
                const relativePath = vscode.workspace.asRelativePath(uri);
                await forgeClient.notifyFileChange(relativePath, 'created');
            }
        })
    );

    context.subscriptions.push(
        watcher.onDidDelete(async (uri) => {
            if (forgeClient.isConnected()) {
                const relativePath = vscode.workspace.asRelativePath(uri);
                await forgeClient.notifyFileChange(relativePath, 'deleted');
            }
        })
    );

    context.subscriptions.push(watcher);
}
