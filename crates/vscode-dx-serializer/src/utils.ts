/**
 * Utility functions for DX Serializer VS Code Extension
 * 
 * Provides helper functions for:
 * - File type detection (isExactlyDxFile)
 * - URI conversion (getDiskUri, getLensUri)
 * - Debouncing (debounce)
 */

import * as vscode from 'vscode';

/**
 * The URI scheme for the DX Lens virtual file system
 */
export const DX_LENS_SCHEME = 'dxlens';

/**
 * Check if a URI points to a DX file:
 * - Files ending with exactly .dx (not .dx.json, .dx.yml, etc.)
 * - Files named exactly "dx" (no extension, no prefix, no suffix)
 * 
 * This function ensures we only intercept pure .dx files and not files
 * with compound extensions like .dx.json, .dx.yml, .dx.bak, etc.
 * 
 * @param uri - The URI to check
 * @returns true if the URI points to a DX file
 * 
 * Requirements: 4.1-4.7
 */
export function isExactlyDxFile(uri: vscode.Uri): boolean {
    // Must be a file scheme (not git, untitled, etc.)
    if (uri.scheme !== 'file' && uri.scheme !== DX_LENS_SCHEME) {
        return false;
    }

    const path = uri.fsPath || uri.path;

    // Must have a path
    if (!path) {
        return false;
    }

    // Get the filename from the path
    const filename = path.split(/[/\\]/).pop() || '';

    // Check if filename is exactly "dx" (no extension, no prefix, no suffix)
    if (filename === 'dx') {
        return true;
    }

    // Must end with exactly .dx
    if (!filename.endsWith('.dx')) {
        return false;
    }

    // Check for compound extensions by looking at what comes before .dx
    // If there's another dot before .dx (like file.json.dx or file.dx.json),
    // we need to check if .dx is the final extension
    const parts = filename.split('.');

    // Must have at least 2 parts (name and dx)
    if (parts.length < 2) {
        return false;
    }

    // The last part must be 'dx'
    if (parts[parts.length - 1] !== 'dx') {
        return false;
    }

    // Check that there's no compound extension pattern
    // e.g., "file.dx.json" would have parts ['file', 'dx', 'json']
    // We want to reject files where .dx is followed by another extension
    // But "file.dx" with parts ['file', 'dx'] is valid
    // And "file.name.dx" with parts ['file', 'name', 'dx'] is also valid

    // The key insight: we already checked that the filename ends with '.dx'
    // So we just need to make sure there's no extension AFTER .dx
    // which is already guaranteed by the endsWith check above

    return true;
}

/**
 * Check if a path string represents exactly a .dx file or a file named exactly "dx"
 * 
 * @param path - The file path to check
 * @returns true if the path is a pure .dx file or named exactly "dx"
 */
export function isExactlyDxPath(path: string): boolean {
    if (!path) {
        return false;
    }

    // Get the filename from the path
    const filename = path.split(/[/\\]/).pop() || '';

    // Check if filename is exactly "dx" (no extension, no prefix, no suffix)
    if (filename === 'dx') {
        return true;
    }

    // Must end with exactly .dx (case-sensitive)
    if (!filename.endsWith('.dx')) {
        return false;
    }

    // Must have a name before .dx (not just ".dx")
    const nameWithoutExt = filename.slice(0, -3);
    if (!nameWithoutExt || nameWithoutExt === '.' || nameWithoutExt.endsWith('.')) {
        return false;
    }

    return true;
}

/**
 * Convert a DX Lens URI (dxlens://) to a disk URI (file://)
 * 
 * @param lensUri - The DX Lens virtual file system URI
 * @returns The corresponding file system URI
 */
export function getDiskUri(lensUri: vscode.Uri): vscode.Uri {
    if (lensUri.scheme === 'file') {
        return lensUri;
    }

    // Convert dxlens:// to file://
    return vscode.Uri.file(lensUri.path);
}

/**
 * Convert a disk URI (file://) to a DX Lens URI (dxlens://)
 * 
 * @param diskUri - The file system URI
 * @returns The corresponding DX Lens virtual file system URI
 */
export function getLensUri(diskUri: vscode.Uri): vscode.Uri {
    if (diskUri.scheme === DX_LENS_SCHEME) {
        return diskUri;
    }

    // Convert file:// to dxlens://
    return diskUri.with({ scheme: DX_LENS_SCHEME });
}

/**
 * Create a debounced version of a function
 * 
 * The debounced function delays invoking the provided function until
 * after the specified delay has elapsed since the last time it was invoked.
 * 
 * @param fn - The function to debounce
 * @param delay - The delay in milliseconds
 * @returns A debounced version of the function
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
    fn: T,
    delay: number
): (...args: Parameters<T>) => void {
    let timeoutId: NodeJS.Timeout | undefined;

    return function debounced(...args: Parameters<T>): void {
        if (timeoutId !== undefined) {
            clearTimeout(timeoutId);
        }

        timeoutId = setTimeout(() => {
            fn(...args);
            timeoutId = undefined;
        }, delay);
    };
}

/**
 * Create a debounced function that can be cancelled
 * 
 * @param fn - The function to debounce
 * @param delay - The delay in milliseconds
 * @returns An object with the debounced function and a cancel method
 */
export function debounceCancellable<T extends (...args: unknown[]) => unknown>(
    fn: T,
    delay: number
): { call: (...args: Parameters<T>) => void; cancel: () => void } {
    let timeoutId: NodeJS.Timeout | undefined;

    return {
        call(...args: Parameters<T>): void {
            if (timeoutId !== undefined) {
                clearTimeout(timeoutId);
            }

            timeoutId = setTimeout(() => {
                fn(...args);
                timeoutId = undefined;
            }, delay);
        },

        cancel(): void {
            if (timeoutId !== undefined) {
                clearTimeout(timeoutId);
                timeoutId = undefined;
            }
        },
    };
}

/**
 * Get the document key for a URI (used for Map keys)
 * 
 * @param uri - The URI to get a key for
 * @returns A string key for the URI
 */
export function getDocumentKey(uri: vscode.Uri): string {
    // Normalize to file path for consistent keys
    const diskUri = getDiskUri(uri);
    return diskUri.fsPath.toLowerCase();
}

/**
 * Check if two URIs refer to the same file
 * 
 * @param uri1 - First URI
 * @param uri2 - Second URI
 * @returns true if both URIs refer to the same file
 */
export function isSameFile(uri1: vscode.Uri, uri2: vscode.Uri): boolean {
    return getDocumentKey(uri1) === getDocumentKey(uri2);
}
