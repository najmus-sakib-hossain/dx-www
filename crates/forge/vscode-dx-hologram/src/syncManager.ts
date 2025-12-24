/**
 * Sync Manager for DX Files
 * 
 * Orchestrates synchronization between:
 * - Human format (.dx/cache/dx.human)
 * - Dense format (dx)
 * - Binary format (.dx/bin/dx.dxb)
 */

import * as vscode from 'vscode';
import * as fs from 'fs';
import { DxPaths, isHumanFile } from './pathUtils';
import { CacheManager } from './cacheManager';
import { DxFormatter } from './formatter';

export class SyncManager {
    
    private cacheManager: CacheManager;
    private formatter: DxFormatter;
    
    constructor() {
        this.formatter = new DxFormatter(3);
        this.cacheManager = new CacheManager(this.formatter);
    }
    
    /**
     * Handle save of a human file
     * Syncs to dense format
     */
    async onHumanFileSaved(humanFilePath: string): Promise<void> {
        const dxPaths = new DxPaths(humanFilePath);
        
        try {
            // Sync human → dense
            await this.cacheManager.syncHumanToSource(dxPaths);
            
            // Show success
            vscode.window.setStatusBarMessage('$(check) DX synced', 2000);
            
        } catch (error) {
            console.error('[DX] Sync failed:', error);
            vscode.window.showErrorMessage(`DX sync failed: ${error}`);
        }
    }
    
    /**
     * Open the human version of a dx file
     */
    async openHumanVersion(dxFilePath: string): Promise<vscode.TextEditor | undefined> {
        const dxPaths = new DxPaths(dxFilePath);
        
        try {
            // Ensure human cache exists and is up-to-date
            const humanFilePath = await this.cacheManager.ensureHumanCache(dxPaths);
            
            // Open the human file
            const uri = vscode.Uri.file(humanFilePath);
            const doc = await vscode.workspace.openTextDocument(uri);
            const editor = await vscode.window.showTextDocument(doc, { preview: false });
            
            return editor;
        } catch (error) {
            console.error('[DX] Failed to open human version:', error);
            vscode.window.showErrorMessage(`Failed to open DX file: ${error}`);
            return undefined;
        }
    }
    
    /**
     * Check if source changed and prompt for reload
     */
    async checkForExternalChanges(humanFilePath: string): Promise<boolean> {
        const dxPaths = new DxPaths(humanFilePath);
        
        if (await this.cacheManager.hasSourceChanged(dxPaths)) {
            const choice = await vscode.window.showWarningMessage(
                'The dx file was modified externally. Reload?',
                'Reload', 'Ignore'
            );
            
            if (choice === 'Reload') {
                await this.cacheManager.syncSourceToHuman(dxPaths);
                return true;
            }
        }
        
        return false;
    }
    
    /**
     * Initialize a new dx file in a directory
     */
    async initDxFile(dirPath: string): Promise<void> {
        const dxPaths = new DxPaths(dirPath);
        dxPaths.ensureDirs();
        
        // Create default human file
        const defaultContent = `# DX Configuration
# This is the human-readable version
# Saves automatically sync to LLM-optimized format

name                 : my-app
version              : 1.0.0
description          : A new DX project

# Technology Stack
stack                > Lang | Runtime | Framework
stack.javascript     > javascript/typescript | bun | react
`;
        
        await fs.promises.writeFile(dxPaths.humanFile, defaultContent, 'utf-8');
        
        // Sync to create the dense file
        await this.cacheManager.syncHumanToSource(dxPaths);
        
        // Open the human file
        await this.openHumanVersion(dxPaths.dxFile);
        
        vscode.window.showInformationMessage('DX config initialized!');
    }
    
    /**
     * Get the cache manager
     */
    getCacheManager(): CacheManager {
        return this.cacheManager;
    }
}
