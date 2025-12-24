/**
 * Cache Manager for DX Files
 * 
 * Manages the .dx/cache/dx.human files:
 * - Creates human cache from dense source
 * - Keeps cache in sync with source
 * - Detects external changes
 */

import * as fs from 'fs';
import { DxPaths } from './pathUtils';
import { DxFormatter } from './formatter';

export class CacheManager {
    
    private formatter: DxFormatter;
    private mtimeCache: Map<string, number> = new Map();
    
    constructor(formatter: DxFormatter) {
        this.formatter = formatter;
    }
    
    /**
     * Ensure human cache exists and is up-to-date
     * Returns the path to the human file
     */
    async ensureHumanCache(dxPaths: DxPaths): Promise<string> {
        dxPaths.ensureDirs();
        
        const sourceExists = fs.existsSync(dxPaths.dxFile);
        const cacheExists = fs.existsSync(dxPaths.humanFile);
        
        if (!sourceExists && !cacheExists) {
            // Neither exists - create empty human file
            await this.createEmptyHuman(dxPaths);
            return dxPaths.humanFile;
        }
        
        if (!sourceExists && cacheExists) {
            // Only cache exists - that's fine
            return dxPaths.humanFile;
        }
        
        if (sourceExists && !cacheExists) {
            // Source exists but no cache - create cache from source
            await this.syncSourceToHuman(dxPaths);
            return dxPaths.humanFile;
        }
        
        // Both exist - check if source is newer
        const sourceStat = await fs.promises.stat(dxPaths.dxFile);
        const cacheStat = await fs.promises.stat(dxPaths.humanFile);
        
        if (sourceStat.mtimeMs > cacheStat.mtimeMs) {
            // Source is newer (external edit, git pull, etc.)
            await this.syncSourceToHuman(dxPaths);
        }
        
        return dxPaths.humanFile;
    }
    
    /**
     * Create human cache from dense source (inflate)
     */
    async syncSourceToHuman(dxPaths: DxPaths): Promise<void> {
        dxPaths.ensureDirs();
        
        const denseContent = await fs.promises.readFile(dxPaths.dxFile, 'utf-8');
        const humanContent = this.formatter.toPretty(denseContent, true);
        
        await fs.promises.writeFile(dxPaths.humanFile, humanContent, 'utf-8');
        
        // Update mtime cache
        const stat = await fs.promises.stat(dxPaths.dxFile);
        this.mtimeCache.set(dxPaths.dxFile, stat.mtimeMs);
        
        console.log(`[DX] Inflated: ${dxPaths.dxFile} → ${dxPaths.humanFile}`);
    }
    
    /**
     * Sync human cache to dense source (deflate)
     */
    async syncHumanToSource(dxPaths: DxPaths): Promise<void> {
        const humanContent = await fs.promises.readFile(dxPaths.humanFile, 'utf-8');
        const denseContent = this.formatter.toDense(humanContent);
        
        await fs.promises.writeFile(dxPaths.dxFile, denseContent, 'utf-8');
        
        // Update mtime cache
        const stat = await fs.promises.stat(dxPaths.dxFile);
        this.mtimeCache.set(dxPaths.dxFile, stat.mtimeMs);
        
        console.log(`[DX] Deflated: ${dxPaths.humanFile} → ${dxPaths.dxFile}`);
    }
    
    /**
     * Check if source file changed externally
     */
    async hasSourceChanged(dxPaths: DxPaths): Promise<boolean> {
        if (!fs.existsSync(dxPaths.dxFile)) return false;
        
        const stat = await fs.promises.stat(dxPaths.dxFile);
        const cachedMtime = this.mtimeCache.get(dxPaths.dxFile);
        
        if (cachedMtime === undefined) {
            this.mtimeCache.set(dxPaths.dxFile, stat.mtimeMs);
            return false;
        }
        
        return stat.mtimeMs > cachedMtime;
    }
    
    /**
     * Create an empty human file with default template
     */
    private async createEmptyHuman(dxPaths: DxPaths): Promise<void> {
        const template = `# DX Configuration
# Edit this file - it will sync to LLM format automatically

name                 : my-app
version              : 1.0.0
`;
        
        await fs.promises.writeFile(dxPaths.humanFile, template, 'utf-8');
    }
}
