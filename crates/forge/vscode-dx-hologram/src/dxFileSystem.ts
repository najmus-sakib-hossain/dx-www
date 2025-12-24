/**
 * DX FileSystem Provider - The Transparent Layer
 * 
 * This intercepts ALL file operations for dx files:
 * - READ: Dense (disk) → Pretty (editor)
 * - WRITE: Pretty (editor) → Dense (disk)
 * 
 * The user sees ONE file, ONE tab, NO dirty indicators.
 * 
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                    THE TRANSPARENT LAYER                                │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │     USER CLICKS                                                         │
 * │     config/dx                                                           │
 * │         │                                                               │
 * │         ▼                                                               │
 * │   ┌───────────────────────────────────────────────────────────────┐    │
 * │   │              DX FILESYSTEM PROVIDER                            │    │
 * │   │                                                                 │    │
 * │   │   readFile():                    writeFile():                   │    │
 * │   │   ─────────────                  ──────────────                 │    │
 * │   │   1. Read dense from disk        1. Receive pretty from VS Code│    │
 * │   │   2. Format to pretty            2. Format to dense            │    │
 * │   │   3. Return pretty               3. Write dense to disk        │    │
 * │   │                                                                 │    │
 * │   └───────────────────────────────────────────────────────────────┘    │
 * │         │                                     │                         │
 * │         ▼                                     ▼                         │
 * │   ┌─────────────┐                       ┌─────────────┐                │
 * │   │  VS CODE    │                       │    DISK     │                │
 * │   │  EDITOR     │                       │             │                │
 * │   │             │                       │  (Dense)    │                │
 * │   │  (Pretty)   │                       │             │                │
 * │   └─────────────┘                       └─────────────┘                │
 * │                                                                         │
 * │   • ONE tab (not two)                                                   │
 * │   • Same file name in tab                                               │
 * │   • Same path in explorer                                               │
 * │   • NO dirty indicator (buffer matches what provider "returned")        │
 * │   • NO save dialogs on close                                            │
 * │   • 100% transparent to user                                            │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 */

import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { DxFormatter } from './formatter';

interface CacheEntry {
    diskContent: string;      // What's actually on disk (dense)
    editorContent: string;    // What we showed to editor (pretty)
    diskMtime: number;        // Last modification time
}

/**
 * DxFileSystem: Transparent layer that shows Pretty, stores Dense
 */
export class DxFileSystem implements vscode.FileSystemProvider {
    
    private _emitter = new vscode.EventEmitter<vscode.FileChangeEvent[]>();
    readonly onDidChangeFile = this._emitter.event;
    
    private formatter: DxFormatter;
    private outputChannel: vscode.OutputChannel;
    
    // Cache to prevent unnecessary re-formatting
    private cache: Map<string, CacheEntry> = new Map();
    
    // File watchers
    private watchers: Map<string, fs.FSWatcher> = new Map();
    
    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;
        const config = vscode.workspace.getConfiguration('dx.hologram');
        this.formatter = new DxFormatter(config.get('indentSize', 2));
    }
    
    // ═══════════════════════════════════════════════════════════════
    // CORE: Read file - transforms Dense → Pretty
    // ═══════════════════════════════════════════════════════════════
    
    async readFile(uri: vscode.Uri): Promise<Uint8Array> {
        const realPath = this.toRealPath(uri);
        
        try {
            // Read the DENSE content from disk
            const diskBuffer = await fs.promises.readFile(realPath);
            const diskContent = diskBuffer.toString('utf-8');
            const stat = await fs.promises.stat(realPath);
            
            // Check cache - if disk unchanged, return cached pretty
            const cached = this.cache.get(realPath);
            if (cached && cached.diskMtime === stat.mtimeMs && cached.diskContent === diskContent) {
                this.log(`Cache hit: ${path.basename(realPath)}`);
                return new TextEncoder().encode(cached.editorContent);
            }
            
            // ✨ FORMAT: Dense → Pretty
            const editorContent = this.formatter.toPretty(diskContent);
            
            // Update cache
            this.cache.set(realPath, {
                diskContent,
                editorContent,
                diskMtime: stat.mtimeMs
            });
            
            this.log(`Read & inflated: ${path.basename(realPath)} (${diskContent.length} → ${editorContent.length} bytes)`);
            
            // Return PRETTY to VS Code
            return new TextEncoder().encode(editorContent);
        } catch (error) {
            this.logError(`Read failed: ${realPath} - ${error}`);
            throw vscode.FileSystemError.FileNotFound(uri);
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // CORE: Write file - transforms Pretty → Dense
    // ═══════════════════════════════════════════════════════════════
    
    async writeFile(
        uri: vscode.Uri, 
        content: Uint8Array, 
        options: { create: boolean; overwrite: boolean }
    ): Promise<void> {
        const realPath = this.toRealPath(uri);
        
        try {
            // Ensure directory exists
            const dir = path.dirname(realPath);
            if (!fs.existsSync(dir)) {
                await fs.promises.mkdir(dir, { recursive: true });
            }
            
            // Receive PRETTY content from VS Code
            const editorContent = new TextDecoder().decode(content);
            
            // ✨ FORMAT: Pretty → Dense
            const diskContent = this.formatter.toDense(editorContent);
            
            // Write DENSE to disk
            await fs.promises.writeFile(realPath, diskContent, 'utf-8');
            
            // Update cache
            const stat = await fs.promises.stat(realPath);
            this.cache.set(realPath, {
                diskContent,
                editorContent,
                diskMtime: stat.mtimeMs
            });
            
            this.log(`Write & deflated: ${path.basename(realPath)} (${editorContent.length} → ${diskContent.length} bytes)`);
            
            // Fire and forget - trigger binary generation
            this.triggerBuild(realPath);
            
        } catch (error) {
            this.logError(`Write failed: ${realPath} - ${error}`);
            throw vscode.FileSystemError.Unavailable(uri);
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // METADATA: Must return size of PRETTY content (what editor sees)
    // ═══════════════════════════════════════════════════════════════
    
    async stat(uri: vscode.Uri): Promise<vscode.FileStat> {
        const realPath = this.toRealPath(uri);
        
        try {
            const stat = await fs.promises.stat(realPath);
            
            // Get the pretty content size (what editor will see)
            let size: number;
            const cached = this.cache.get(realPath);
            if (cached) {
                size = new TextEncoder().encode(cached.editorContent).length;
            } else {
                // Estimate: pretty is roughly 1.5-2x dense size
                size = Math.floor(stat.size * 1.5);
            }
            
            return {
                type: stat.isDirectory() ? vscode.FileType.Directory : vscode.FileType.File,
                ctime: stat.ctimeMs,
                mtime: stat.mtimeMs,
                size: size  // Size of PRETTY content
            };
        } catch (error) {
            throw vscode.FileSystemError.FileNotFound(uri);
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // FILE WATCHING: Detect external changes
    // ═══════════════════════════════════════════════════════════════
    
    watch(uri: vscode.Uri, options: { recursive: boolean; excludes: string[] }): vscode.Disposable {
        const realPath = this.toRealPath(uri);
        
        // Clean up existing watcher if any
        const existingWatcher = this.watchers.get(realPath);
        if (existingWatcher) {
            existingWatcher.close();
        }
        
        try {
            const watcher = fs.watch(realPath, (eventType, filename) => {
                if (eventType === 'change') {
                    // Invalidate cache
                    this.cache.delete(realPath);
                    // Notify VS Code
                    this._emitter.fire([{ type: vscode.FileChangeType.Changed, uri }]);
                    this.log(`External change detected: ${path.basename(realPath)}`);
                }
            });
            
            this.watchers.set(realPath, watcher);
            
            return new vscode.Disposable(() => {
                watcher.close();
                this.watchers.delete(realPath);
            });
        } catch (error) {
            // File might not exist yet
            return new vscode.Disposable(() => {});
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // DIRECTORY OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    async readDirectory(uri: vscode.Uri): Promise<[string, vscode.FileType][]> {
        const realPath = this.toRealPath(uri);
        
        try {
            const entries = await fs.promises.readdir(realPath, { withFileTypes: true });
            return entries.map(entry => [
                entry.name,
                entry.isDirectory() ? vscode.FileType.Directory : vscode.FileType.File
            ]);
        } catch (error) {
            return [];
        }
    }
    
    async createDirectory(uri: vscode.Uri): Promise<void> {
        const realPath = this.toRealPath(uri);
        await fs.promises.mkdir(realPath, { recursive: true });
    }
    
    async delete(uri: vscode.Uri, options?: { recursive: boolean }): Promise<void> {
        const realPath = this.toRealPath(uri);
        
        try {
            const stat = await fs.promises.stat(realPath);
            if (stat.isDirectory()) {
                await fs.promises.rm(realPath, { recursive: options?.recursive || false });
            } else {
                await fs.promises.unlink(realPath);
            }
            this.cache.delete(realPath);
        } catch (error) {
            throw vscode.FileSystemError.FileNotFound(uri);
        }
    }
    
    async rename(oldUri: vscode.Uri, newUri: vscode.Uri, options?: { overwrite: boolean }): Promise<void> {
        const oldPath = this.toRealPath(oldUri);
        const newPath = this.toRealPath(newUri);
        
        try {
            await fs.promises.rename(oldPath, newPath);
            
            // Migrate cache
            const cached = this.cache.get(oldPath);
            if (cached) {
                this.cache.delete(oldPath);
                this.cache.set(newPath, cached);
            }
        } catch (error) {
            throw vscode.FileSystemError.Unavailable(oldUri);
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // HELPERS
    // ═══════════════════════════════════════════════════════════════
    
    private toRealPath(uri: vscode.Uri): string {
        // dx-fs:///path/to/dx → /path/to/dx
        // Handle Windows paths correctly
        let p = uri.path;
        if (process.platform === 'win32' && p.startsWith('/')) {
            p = p.substring(1); // Remove leading /
        }
        return p;
    }
    
    private async triggerBuild(dxPath: string): Promise<void> {
        const config = vscode.workspace.getConfiguration('dx.forge');
        if (config.get('autoBuild', true)) {
            try {
                // Try to use dx-forge CLI if available
                const { exec } = require('child_process');
                exec(`dx build "${dxPath}"`, (err: Error | null, stdout: string, stderr: string) => {
                    if (!err) {
                        this.log(`Binary generated for: ${path.basename(dxPath)}`);
                    }
                    // Silently fail - forge may not be installed
                });
            } catch (error) {
                // Ignore - forge not available
            }
        }
    }
    
    /**
     * Invalidate cache for a file
     */
    invalidateCache(realPath: string): void {
        this.cache.delete(realPath);
    }
    
    /**
     * Get the dense content currently on disk
     */
    getDenseContent(realPath: string): string | undefined {
        const cached = this.cache.get(realPath);
        return cached?.diskContent;
    }
    
    /**
     * Get the pretty content being shown
     */
    getPrettyContent(realPath: string): string | undefined {
        const cached = this.cache.get(realPath);
        return cached?.editorContent;
    }
    
    private log(message: string): void {
        const timestamp = new Date().toISOString().substring(11, 19);
        this.outputChannel.appendLine(`[${timestamp}] [FS] ${message}`);
    }
    
    private logError(message: string): void {
        const timestamp = new Date().toISOString().substring(11, 19);
        this.outputChannel.appendLine(`[${timestamp}] [FS:ERR] ${message}`);
    }
    
    dispose(): void {
        // Clean up all watchers
        for (const watcher of this.watchers.values()) {
            watcher.close();
        }
        this.watchers.clear();
        this.cache.clear();
    }
}
