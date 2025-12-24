/**
 * DX Hologram Provider
 * 
 * Manages the virtual document system that shows human-readable format
 * while storing LLM-optimized format on disk.
 */

import * as vscode from 'vscode';
import * as fs from 'fs';
import * as crypto from 'crypto';
import { DxInflater } from './inflater';
import { DxDeflater } from './deflater';

interface FileState {
    realPath: string;
    denseContent: string;
    prettyContent: string;
    denseHash: string;
}

export class DxHologramProvider implements vscode.TextDocumentContentProvider {
    private _onDidChange = new vscode.EventEmitter<vscode.Uri>();
    readonly onDidChange = this._onDidChange.event;

    private uriToPath: Map<string, string> = new Map();
    private pathToUri: Map<string, vscode.Uri> = new Map();
    private stateCache: Map<string, FileState> = new Map();

    private inflater: DxInflater;
    private deflater: DxDeflater;
    private outputChannel: vscode.OutputChannel;

    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;
        
        const config = vscode.workspace.getConfiguration('dx.hologram');
        this.inflater = new DxInflater({
            keyWidth: config.get('keyWidth', 20)
        });
        this.deflater = new DxDeflater();
    }

    /**
     * Register a file for hologram handling
     */
    registerFile(realPath: string, hologramUri: vscode.Uri): void {
        this.uriToPath.set(hologramUri.toString(), realPath);
        this.pathToUri.set(realPath, hologramUri);
    }

    /**
     * Get the real disk path for a hologram URI
     */
    getRealPath(hologramUri: vscode.Uri): string | undefined {
        return this.uriToPath.get(hologramUri.toString());
    }

    /**
     * Get the hologram URI for a real path
     */
    getHologramUri(realPath: string): vscode.Uri | undefined {
        return this.pathToUri.get(realPath);
    }

    /**
     * VS Code calls this to get content of a hologram document
     * INFLATE: Dense → Pretty
     */
    provideTextDocumentContent(uri: vscode.Uri): string {
        const realPath = this.getRealPath(uri);
        if (!realPath) {
            return '// No file path associated';
        }

        try {
            const denseContent = fs.readFileSync(realPath, 'utf8');
            const denseHash = this.hash(denseContent);

            // Check cache
            const cached = this.stateCache.get(uri.toString());
            if (cached && cached.denseHash === denseHash) {
                return cached.prettyContent;
            }

            // Inflate to pretty format
            const prettyContent = this.inflater.inflate(denseContent);

            // Update cache
            this.stateCache.set(uri.toString(), {
                realPath,
                denseContent,
                prettyContent,
                denseHash
            });

            return prettyContent;
        } catch (error) {
            return `// Error loading file: ${error}`;
        }
    }

    /**
     * Deflate pretty content to dense format
     */
    deflate(prettyContent: string): string {
        return this.deflater.deflate(prettyContent);
    }

    /**
     * Called after successful save to update cache
     */
    markAsSaved(uri: vscode.Uri, denseContent: string): void {
        const cached = this.stateCache.get(uri.toString());
        if (cached) {
            cached.denseContent = denseContent;
            cached.denseHash = this.hash(denseContent);
        }
    }

    /**
     * Handle external file changes (git pull, etc.)
     */
    onExternalChange(fileUri: vscode.Uri): void {
        const hologramUri = this.pathToUri.get(fileUri.fsPath);
        if (hologramUri) {
            this.stateCache.delete(hologramUri.toString());
            this._onDidChange.fire(hologramUri);
            this.log(`External change detected: ${fileUri.fsPath}`);
        }
    }

    private hash(content: string): string {
        return crypto.createHash('sha256')
            .update(content)
            .digest('hex')
            .substring(0, 16);
    }

    private log(message: string): void {
        const timestamp = new Date().toISOString().substring(11, 19);
        this.outputChannel.appendLine(`[${timestamp}] 📁 ${message}`);
    }
}
