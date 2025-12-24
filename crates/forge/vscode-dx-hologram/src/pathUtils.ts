/**
 * Path Utilities for DX Cache Layer Architecture
 * 
 * Maps between:
 * - dx file (LLM-Dense source of truth)
 * - .dx/cache/dx.human (Human-Pretty editing)
 * - .dx/bin/dx.dxb (Machine binary)
 */

import * as path from 'path';
import * as fs from 'fs';

export class DxPaths {
    readonly rootDir: string;       // Directory containing dx file
    readonly dxFile: string;        // path/to/dx (LLM-Dense)
    readonly humanFile: string;     // path/to/.dx/cache/dx.human
    readonly binaryFile: string;    // path/to/.dx/bin/dx.dxb
    readonly cacheDir: string;      // path/to/.dx/cache
    readonly binDir: string;        // path/to/.dx/bin
    
    constructor(anyDxPath: string) {
        const normalized = path.normalize(anyDxPath);
        
        if (normalized.includes('.dx' + path.sep)) {
            // Path is inside .dx folder - extract root
            const parts = normalized.split(path.sep);
            const dxIdx = parts.findIndex(p => p === '.dx');
            if (dxIdx > 0) {
                this.rootDir = parts.slice(0, dxIdx).join(path.sep);
            } else {
                this.rootDir = path.dirname(normalized);
            }
        } else if (path.basename(normalized) === 'dx') {
            // Path is the root dx file
            this.rootDir = path.dirname(normalized);
        } else {
            // Assume it's a directory
            this.rootDir = normalized;
        }
        
        // Normalize rootDir
        this.rootDir = this.rootDir.replace(/[\/\\]$/, '');
        
        // Compute all paths
        this.dxFile = path.join(this.rootDir, 'dx');
        this.cacheDir = path.join(this.rootDir, '.dx', 'cache');
        this.binDir = path.join(this.rootDir, '.dx', 'bin');
        this.humanFile = path.join(this.cacheDir, 'dx.human');
        this.binaryFile = path.join(this.binDir, 'dx.dxb');
    }
    
    /** Ensure all directories exist */
    ensureDirs(): void {
        if (!fs.existsSync(this.cacheDir)) {
            fs.mkdirSync(this.cacheDir, { recursive: true });
        }
        if (!fs.existsSync(this.binDir)) {
            fs.mkdirSync(this.binDir, { recursive: true });
        }
    }
    
    /** Check if dx file exists */
    isValid(): boolean {
        return fs.existsSync(this.dxFile);
    }
    
    /** Check if human cache exists */
    hasHumanCache(): boolean {
        return fs.existsSync(this.humanFile);
    }
    
    /** Check if binary exists */
    hasBinary(): boolean {
        return fs.existsSync(this.binaryFile);
    }
    
    /** Get display path for tab label */
    getDisplayPath(workspaceRoot: string): string {
        const relative = path.relative(workspaceRoot, this.dxFile);
        return relative || 'dx';
    }
}

/**
 * Check if a file path is a dx.human file
 */
export function isHumanFile(filePath: string): boolean {
    const normalized = path.normalize(filePath);
    return normalized.includes('.dx' + path.sep + 'cache' + path.sep + 'dx.human') ||
           normalized.endsWith('.dx/cache/dx.human') || 
           normalized.endsWith('.dx\\cache\\dx.human');
}

/**
 * Check if a file path is a root dx file (not inside .dx folder)
 */
export function isDxFile(filePath: string): boolean {
    const basename = path.basename(filePath);
    const normalized = path.normalize(filePath);
    // Must be named "dx" and NOT inside .dx folder
    return basename === 'dx' && !normalized.includes('.dx' + path.sep);
}

/**
 * Find all dx files in a workspace
 */
export async function findAllDxFiles(workspaceRoot: string): Promise<DxPaths[]> {
    const results: DxPaths[] = [];
    
    async function scan(dir: string): Promise<void> {
        try {
            const entries = await fs.promises.readdir(dir, { withFileTypes: true });
            
            for (const entry of entries) {
                const fullPath = path.join(dir, entry.name);
                
                if (entry.isDirectory()) {
                    // Skip hidden folders and node_modules
                    if (!entry.name.startsWith('.') && entry.name !== 'node_modules' && entry.name !== 'target') {
                        await scan(fullPath);
                    }
                } else if (entry.name === 'dx') {
                    results.push(new DxPaths(fullPath));
                }
            }
        } catch (e) {
            // Ignore permission errors
        }
    }
    
    await scan(workspaceRoot);
    return results;
}
