/**
 * DxDocumentManager - Document state management for DX files
 * 
 * Manages document state, validation, and save coordination for .dx files.
 * Handles:
 * - Document state tracking (disk content, editor content, validation status)
 * - Save coordination with validation gating and grace period
 * - External file change detection
 * - Diagnostic updates for validation errors
 * - Cache file generation (.dx/cache/{filename}.human and .machine)
 * 
 * Requirements: 3.1-3.5, 4.1-4.5, 6.1-6.5
 */

import * as vscode from 'vscode';
import * as fs from 'fs';
import { DxCore, ValidationResult } from './dxCore';
import { getDiskUri, getDocumentKey, DX_LENS_SCHEME } from './utils';
import { writeCache, deleteCache } from './cacheManager';
import { parseLlm } from './llmParser';
import { parseHumanV3 } from './humanParserV3';
import { formatDocumentV3, DEFAULT_CONFIG } from './humanFormatterV3';

/**
 * State for a single document
 */
export interface DocumentState {
    /** Dense content currently on disk */
    diskDense: string;

    /** Last successfully saved dense content */
    lastValidDense: string;

    /** Current human content in editor */
    currentHuman: string;

    /** Whether content is syntactically valid */
    isValid: boolean;

    /** Last validation error */
    lastError: string | null;

    /** Timestamp of last keystroke */
    lastKeystroke: number;

    /** Pending save timeout */
    saveTimeout: NodeJS.Timeout | null;

    /** Whether a save is in progress */
    isSaving: boolean;
}

/**
 * Configuration for the document manager
 */
export interface DocumentManagerConfig {
    /** Validate syntax before saving (default: true) */
    validateBeforeSave: boolean;

    /** Grace period in ms after last keystroke (default: 2000) */
    autoSaveGracePeriod: number;

    /** Indent size: 2 or 4 spaces (default: 2) */
    indentSize: number;

    /** Minimum key padding width (default: 20) */
    keyPadding: number;

    /** Format on save (default: true) */
    formatOnSave: boolean;
}


/**
 * DxDocumentManager - Manages document state and save coordination
 */
export class DxDocumentManager implements vscode.Disposable {
    private documents: Map<string, DocumentState> = new Map();
    private writingFiles: Set<string> = new Set();
    private diagnosticCollection: vscode.DiagnosticCollection;
    private config: DocumentManagerConfig;
    private dxCore: DxCore;
    private disposables: vscode.Disposable[] = [];

    constructor(dxCore: DxCore, config?: Partial<DocumentManagerConfig>) {
        this.dxCore = dxCore;
        this.config = {
            validateBeforeSave: config?.validateBeforeSave ?? true,
            autoSaveGracePeriod: config?.autoSaveGracePeriod ?? 2000,
            indentSize: config?.indentSize ?? 2,
            keyPadding: config?.keyPadding ?? 20,
            formatOnSave: config?.formatOnSave ?? true,
        };

        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('dx');
        this.disposables.push(this.diagnosticCollection);
    }

    /**
     * Update configuration
     */
    updateConfig(config: Partial<DocumentManagerConfig>): void {
        if (config.validateBeforeSave !== undefined) {
            this.config.validateBeforeSave = config.validateBeforeSave;
        }
        if (config.autoSaveGracePeriod !== undefined) {
            this.config.autoSaveGracePeriod = config.autoSaveGracePeriod;
        }
        if (config.indentSize !== undefined) {
            this.config.indentSize = config.indentSize;
        }
        if (config.keyPadding !== undefined) {
            this.config.keyPadding = config.keyPadding;
        }
        if (config.formatOnSave !== undefined) {
            this.config.formatOnSave = config.formatOnSave;
        }
    }

    /**
     * Initialize document when first opened
     * 
     * @param uri - The URI of the document
     * @returns The human-readable content for display
     */
    async initializeDocument(uri: vscode.Uri): Promise<string> {
        const diskUri = getDiskUri(uri);
        const key = getDocumentKey(uri);

        // Read dense content from disk
        let diskDense = '';
        try {
            const content = await fs.promises.readFile(diskUri.fsPath, 'utf-8');
            diskDense = content;
        } catch (error) {
            // File might not exist yet
            diskDense = '';
        }

        // Transform to human format
        const humanResult = this.dxCore.toHuman(diskDense);
        const currentHuman = humanResult.success ? humanResult.content : diskDense;

        // Validate content
        const validation = this.dxCore.validate(currentHuman);

        // Create document state
        const state: DocumentState = {
            diskDense,
            lastValidDense: diskDense,
            currentHuman,
            isValid: validation.success,
            lastError: validation.error ?? null,
            lastKeystroke: Date.now(),
            saveTimeout: null,
            isSaving: false,
        };

        this.documents.set(key, state);

        // Update diagnostics
        this.updateDiagnostics(uri, validation);

        return currentHuman;
    }


    /**
     * Handle content change in editor
     * 
     * @param uri - The URI of the document
     * @param content - The new human-readable content
     */
    handleContentChange(uri: vscode.Uri, content: string): void {
        const key = getDocumentKey(uri);
        const state = this.documents.get(key);

        if (!state) {
            return;
        }

        // Update state
        state.currentHuman = content;
        state.lastKeystroke = Date.now();

        // Validate content
        const validation = this.dxCore.validate(content);
        state.isValid = validation.success;
        state.lastError = validation.error ?? null;

        // Update diagnostics
        this.updateDiagnostics(uri, validation);
    }

    /**
     * Save document with validation gating and format-on-save
     * 
     * @param uri - The URI of the document
     * @param content - The human-readable content to save
     * @returns Whether the save was successful
     */
    async saveDocument(uri: vscode.Uri, content: Uint8Array): Promise<boolean> {
        const key = getDocumentKey(uri);
        const state = this.documents.get(key);

        if (!state) {
            // Initialize if not tracked
            await this.initializeDocument(uri);
            return this.saveDocument(uri, content);
        }

        let humanContent = new TextDecoder().decode(content);
        state.currentHuman = humanContent;

        // Note: Grace period check removed to ensure saves happen immediately
        // The grace period was causing saves to be skipped

        // Validate if configured
        if (this.config.validateBeforeSave) {
            const validation = this.dxCore.validate(humanContent);
            state.isValid = validation.success;
            state.lastError = validation.error ?? null;

            // Update diagnostics
            this.updateDiagnostics(uri, validation);

            if (!validation.success) {
                // Skip save for invalid content
                this.showValidationWarning(validation);
                return false;
            }
        }

        // Format-on-save: parse and reformat content for consistency
        if (this.config.formatOnSave) {
            try {
                const parseResult = parseHumanV3(humanContent);
                if (parseResult.success && parseResult.document) {
                    const formattedContent = formatDocumentV3(parseResult.document, {
                        ...DEFAULT_CONFIG,
                        keyPadding: this.config.keyPadding,
                    });
                    humanContent = formattedContent;
                    state.currentHuman = humanContent;
                }
                // If parsing fails, continue with original content (graceful degradation)
            } catch (formatError) {
                // Format-on-save failed - log warning but continue with original content
                console.warn(`DX: Format-on-save failed, using original content: ${formatError}`);
            }
        }

        // Transform to dense format
        const denseResult = this.dxCore.toDense(humanContent);
        if (!denseResult.success) {
            vscode.window.showErrorMessage(`DX: Transform failed: ${denseResult.error}`);
            return false;
        }

        // Write to disk
        const diskUri = getDiskUri(uri);
        try {
            state.isSaving = true;
            this.writingFiles.add(key);

            await fs.promises.writeFile(diskUri.fsPath, denseResult.content, 'utf-8');

            // Update state
            state.diskDense = denseResult.content;
            state.lastValidDense = denseResult.content;

            // Generate cache files (Requirements: 4.1-4.3)
            await this.generateCacheFiles(diskUri.fsPath, denseResult.content);

            // Update editor with formatted content if format-on-save is enabled
            if (this.config.formatOnSave) {
                await this.updateEditorContent(uri, humanContent);
            }

            return true;
        } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            vscode.window.showErrorMessage(`DX: Failed to save file: ${message}`);
            return false;
        } finally {
            state.isSaving = false;
            // Remove from writing set after a short delay to handle file watcher events
            setTimeout(() => this.writingFiles.delete(key), 100);
        }
    }

    /**
     * Update editor content with reformatted text
     * 
     * @param uri - The URI of the document
     * @param newContent - The new content to display
     */
    private async updateEditorContent(uri: vscode.Uri, newContent: string): Promise<void> {
        // Find the editor showing this document
        for (const editor of vscode.window.visibleTextEditors) {
            // Check both the direct URI and the lens URI
            const editorUriStr = editor.document.uri.toString();
            const targetUriStr = uri.toString();
            const lensUriStr = uri.with({ scheme: DX_LENS_SCHEME }).toString();

            if (editorUriStr === targetUriStr || editorUriStr === lensUriStr) {
                const fullRange = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(editor.document.getText().length)
                );

                const edit = new vscode.WorkspaceEdit();
                edit.replace(editor.document.uri, fullRange, newContent);
                await vscode.workspace.applyEdit(edit);

                // Save the document again to persist the reformatted content
                // (This won't cause infinite loop because content will match)
                break;
            }
        }
    }

    /**
     * Generate cache files for a saved document
     * Requirements: 4.1, 4.2, 4.3
     */
    private async generateCacheFiles(filePath: string, llmContent: string): Promise<void> {
        try {
            const parseResult = parseLlm(llmContent);
            if (parseResult.success && parseResult.document) {
                const cacheResult = await writeCache(filePath, parseResult.document);
                if (!cacheResult.success) {
                    console.warn(`DX: Cache generation failed: ${cacheResult.error}`);
                }
            }
        } catch (error) {
            console.warn(`DX: Cache generation error: ${error}`);
        }
    }


    /**
     * Force save without validation
     * 
     * @param uri - The URI of the document
     */
    async forceSave(uri: vscode.Uri): Promise<boolean> {
        const key = getDocumentKey(uri);
        const state = this.documents.get(key);

        if (!state) {
            return false;
        }

        // Transform to dense format
        const denseResult = this.dxCore.toDense(state.currentHuman);
        if (!denseResult.success) {
            vscode.window.showErrorMessage(`DX: Transform failed: ${denseResult.error}`);
            return false;
        }

        // Write to disk
        const diskUri = getDiskUri(uri);
        try {
            state.isSaving = true;
            this.writingFiles.add(key);

            await fs.promises.writeFile(diskUri.fsPath, denseResult.content, 'utf-8');

            // Update state
            state.diskDense = denseResult.content;
            state.lastValidDense = denseResult.content;

            vscode.window.showInformationMessage('DX: File saved (validation bypassed)');
            return true;
        } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            vscode.window.showErrorMessage(`DX: Failed to save file: ${message}`);
            return false;
        } finally {
            state.isSaving = false;
            setTimeout(() => this.writingFiles.delete(key), 100);
        }
    }

    /**
     * Handle external file changes
     * 
     * @param uri - The URI of the changed file
     */
    async handleExternalChange(uri: vscode.Uri): Promise<void> {
        const key = getDocumentKey(uri);

        // Ignore if we're currently writing this file
        if (this.writingFiles.has(key)) {
            return;
        }

        const state = this.documents.get(key);
        if (!state) {
            return;
        }

        // Read new content from disk
        const diskUri = getDiskUri(uri);
        try {
            const newDense = await fs.promises.readFile(diskUri.fsPath, 'utf-8');

            // Skip if content hasn't changed
            if (newDense === state.diskDense) {
                return;
            }

            // Transform to human format
            const humanResult = this.dxCore.toHuman(newDense);
            const newHuman = humanResult.success ? humanResult.content : newDense;

            // Update state
            state.diskDense = newDense;
            state.lastValidDense = newDense;
            state.currentHuman = newHuman;

            // Validate
            const validation = this.dxCore.validate(newHuman);
            state.isValid = validation.success;
            state.lastError = validation.error ?? null;

            // Update diagnostics
            this.updateDiagnostics(uri, validation);

        } catch (error) {
            // File might have been deleted
            console.warn(`DX: Failed to read external change: ${error}`);
        }
    }

    /**
     * Force refresh from disk
     * 
     * @param uri - The URI of the document
     * @returns The refreshed human-readable content
     */
    async forceRefresh(uri: vscode.Uri): Promise<string> {
        const key = getDocumentKey(uri);

        // Remove existing state
        this.documents.delete(key);

        // Re-initialize
        return this.initializeDocument(uri);
    }


    /**
     * Handle file deletion
     * 
     * @param uri - The URI of the deleted file
     */
    handleFileDeleted(uri: vscode.Uri): void {
        const key = getDocumentKey(uri);

        // Clear state
        const state = this.documents.get(key);
        if (state?.saveTimeout) {
            clearTimeout(state.saveTimeout);
        }
        this.documents.delete(key);

        // Clear diagnostics
        this.diagnosticCollection.delete(uri);

        // Delete cache files (Requirement: 4.5)
        const diskUri = getDiskUri(uri);
        deleteCache(diskUri.fsPath).catch(error => {
            console.warn(`DX: Failed to delete cache: ${error}`);
        });
    }

    /**
     * Get current document state
     * 
     * @param uri - The URI of the document
     * @returns The document state or undefined
     */
    getState(uri: vscode.Uri): DocumentState | undefined {
        const key = getDocumentKey(uri);
        return this.documents.get(key);
    }

    /**
     * Check if extension is currently writing to a file
     * 
     * @param uri - The URI to check
     * @returns Whether the extension is writing to this file
     */
    isWriting(uri: vscode.Uri): boolean {
        const key = getDocumentKey(uri);
        return this.writingFiles.has(key);
    }

    /**
     * Get the dense content for a document
     * 
     * @param uri - The URI of the document
     * @returns The dense content or undefined
     */
    getDenseContent(uri: vscode.Uri): string | undefined {
        const state = this.getState(uri);
        if (!state) {
            return undefined;
        }

        // Transform current human content to dense
        const result = this.dxCore.toDense(state.currentHuman);
        return result.success ? result.content : state.diskDense;
    }

    /**
     * Update diagnostics for a document
     */
    private updateDiagnostics(uri: vscode.Uri, validation: ValidationResult): void {
        if (validation.success) {
            this.diagnosticCollection.delete(uri);
            return;
        }

        const line = (validation.line ?? 1) - 1; // Convert to 0-indexed
        const column = (validation.column ?? 1) - 1;

        const range = new vscode.Range(
            new vscode.Position(line, column),
            new vscode.Position(line, column + 1)
        );

        const diagnostic = new vscode.Diagnostic(
            range,
            validation.error ?? 'Syntax error',
            vscode.DiagnosticSeverity.Error
        );

        diagnostic.source = 'DX Serializer';

        if (validation.hint) {
            diagnostic.message += `\n\nHint: ${validation.hint}`;
        }

        this.diagnosticCollection.set(uri, [diagnostic]);
    }

    /**
     * Show validation warning in status bar
     */
    private showValidationWarning(validation: ValidationResult): void {
        const message = validation.error ?? 'Syntax error';
        vscode.window.setStatusBarMessage(`$(warning) DX: ${message}`, 5000);
    }

    /**
     * Dispose of resources
     */
    dispose(): void {
        // Clear all timeouts
        for (const state of this.documents.values()) {
            if (state.saveTimeout) {
                clearTimeout(state.saveTimeout);
            }
        }

        // Clear collections
        this.documents.clear();
        this.writingFiles.clear();

        // Dispose of VS Code resources
        for (const disposable of this.disposables) {
            disposable.dispose();
        }
    }
}
