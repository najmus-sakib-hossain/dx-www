/**
 * DxCore - WASM wrapper with TypeScript fallback
 * 
 * Provides transformation between dense (disk) and human (editor) formats
 * with validation support. Uses WASM for performance with a TypeScript
 * fallback for reliability.
 * 
 * Requirements: 12.2, 12.3
 */

import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

/**
 * Result of a transformation operation
 */
export interface TransformResult {
    success: boolean;
    content: string;
    error?: string;
}

/**
 * Result of a validation operation
 */
export interface ValidationResult {
    success: boolean;
    error?: string;
    line?: number;
    column?: number;
    hint?: string;
}

/**
 * DxCore interface for transformation operations
 */
export interface DxCore {
    /** Transform dense format to human-readable format */
    toHuman(dense: string): TransformResult;
    
    /** Transform human-readable format to dense format */
    toDense(human: string): TransformResult;
    
    /** Validate content syntax */
    validate(content: string): ValidationResult;
    
    /** Check if content is complete enough to save */
    isSaveable(content: string): boolean;
    
    /** Whether this is using WASM or fallback */
    readonly isWasm: boolean;
}

/**
 * WASM module interface (matches wasm-bindgen output)
 */
interface WasmModule {
    DxSerializer: new () => WasmSerializer;
}

interface WasmSerializer {
    toHuman(dense: string): WasmTransformResult;
    toDense(human: string): WasmTransformResult;
    validate(content: string): WasmValidationResult;
    isSaveable(content: string): boolean;
}


interface WasmTransformResult {
    success: boolean;
    content: string;
    error?: string;
}

interface WasmValidationResult {
    success: boolean;
    error?: string;
    line?: number;
    column?: number;
    hint?: string;
}

/**
 * WASM-based DxCore implementation
 */
class WasmDxCore implements DxCore {
    private serializer: WasmSerializer;
    readonly isWasm = true;

    constructor(serializer: WasmSerializer) {
        this.serializer = serializer;
    }

    toHuman(dense: string): TransformResult {
        const result = this.serializer.toHuman(dense);
        return {
            success: result.success,
            content: result.content,
            error: result.error,
        };
    }

    toDense(human: string): TransformResult {
        const result = this.serializer.toDense(human);
        return {
            success: result.success,
            content: result.content,
            error: result.error,
        };
    }

    validate(content: string): ValidationResult {
        const result = this.serializer.validate(content);
        return {
            success: result.success,
            error: result.error,
            line: result.line,
            column: result.column,
            hint: result.hint,
        };
    }

    isSaveable(content: string): boolean {
        return this.serializer.isSaveable(content);
    }
}

// ============================================================================
// TypeScript Fallback Implementation
// ============================================================================

/**
 * Apply smart quoting to a string value
 * 
 * - If string contains apostrophe ('), wrap in double quotes
 * - If string contains both ' and ", use double quotes with escaped "
 */
export function smartQuote(value: string): string {
    const hasSingle = value.includes("'");
    const hasDouble = value.includes('"');

    if (!hasSingle && !hasDouble) {
        // No quotes needed for simple strings without spaces/special chars
        if (!/[ #|^:]/.test(value)) {
            return value;
        }
        // Default to double quotes
        return `"${value}"`;
    }

    if (hasSingle && !hasDouble) {
        // Contains apostrophe - use double quotes
        return `"${value}"`;
    }

    if (hasDouble && !hasSingle) {
        // Contains double quotes - use single quotes
        return `'${value}'`;
    }

    // Contains both - use double quotes with escaped double quotes
    const escaped = value.replace(/"/g, '\\"');
    return `"${escaped}"`;
}


/**
 * Format DX content to human-readable format (TypeScript fallback)
 * 
 * Transforms dense format like:
 *   server#host:localhost#port:5432
 * 
 * To human-readable format like:
 *   server:
 *     host: localhost
 *     port: 5432
 */
export function formatDx(dense: string, indentSize: number = 2): string {
    if (!dense.trim()) {
        return '';
    }

    const indent = ' '.repeat(indentSize);
    const lines: string[] = [];

    // Parse the dense format
    // Format: key#field:value#field:value or key:value or key@N>item|item
    const parts = dense.split('#');
    
    for (let i = 0; i < parts.length; i++) {
        const part = parts[i];
        
        if (i === 0) {
            // First part is the root key or key:value
            if (part.includes(':')) {
                const colonIdx = part.indexOf(':');
                const key = part.substring(0, colonIdx);
                const value = part.substring(colonIdx + 1);
                
                if (parts.length === 1) {
                    // Simple key:value
                    lines.push(`${key}: ${formatValue(value)}`);
                } else {
                    // Object with fields
                    lines.push(`${key}:`);
                }
            } else if (part.includes('@')) {
                // Array: key@N>item|item
                const atIdx = part.indexOf('@');
                const key = part.substring(0, atIdx);
                const rest = part.substring(atIdx + 1);
                const gtIdx = rest.indexOf('>');
                
                if (gtIdx !== -1) {
                    const items = rest.substring(gtIdx + 1).split('|');
                    lines.push(`${key}:`);
                    for (const item of items) {
                        lines.push(`${indent}- ${formatValue(item)}`);
                    }
                } else {
                    lines.push(`${key}: []`);
                }
            } else {
                // Just a key (object follows)
                lines.push(`${part}:`);
            }
        } else {
            // Subsequent parts are field:value pairs
            const colonIdx = part.indexOf(':');
            if (colonIdx !== -1) {
                const key = part.substring(0, colonIdx);
                const value = part.substring(colonIdx + 1);
                lines.push(`${indent}${key}: ${formatValue(value)}`);
            }
        }
    }

    return lines.join('\n');
}

/**
 * Format a value for human-readable output
 */
function formatValue(value: string): string {
    // Handle special values
    if (value === '1' || value === 'true') return 'true';
    if (value === '0' || value === 'false') return 'false';
    if (value === 'null' || value === '') return 'null';
    
    // Check if value needs quoting
    if (/[ #|^:\n\t]/.test(value) || value.includes("'") || value.includes('"')) {
        return smartQuote(value);
    }
    
    return value;
}


/**
 * Minify DX content to dense format (TypeScript fallback)
 * 
 * Transforms human-readable format like:
 *   server:
 *     host: localhost
 *     port: 5432
 * 
 * To dense format like:
 *   server#host:localhost#port:5432
 */
export function minifyDx(human: string): string {
    if (!human.trim()) {
        return '';
    }

    const lines = human.split('\n').filter(line => line.trim());
    if (lines.length === 0) {
        return '';
    }

    const result: string[] = [];
    let currentKey = '';
    let inArray = false;
    const arrayItems: string[] = [];

    for (const line of lines) {
        const trimmed = line.trim();
        const indentLevel = line.search(/\S/);
        
        // Skip comments
        if (trimmed.startsWith('//') || trimmed.startsWith('#')) {
            continue;
        }

        // Handle array items
        if (trimmed.startsWith('- ')) {
            inArray = true;
            const value = trimmed.substring(2).trim();
            arrayItems.push(parseValue(value));
            continue;
        }

        // Flush array if we were in one
        if (inArray && arrayItems.length > 0) {
            result.push(`${currentKey}@${arrayItems.length}>${arrayItems.join('|')}`);
            arrayItems.length = 0;
            inArray = false;
        }

        // Parse key: value
        const colonIdx = trimmed.indexOf(':');
        if (colonIdx === -1) continue;

        const key = trimmed.substring(0, colonIdx).trim();
        const value = trimmed.substring(colonIdx + 1).trim();

        if (indentLevel === 0) {
            // Root level key
            if (value) {
                // Simple key: value
                result.push(`${key}:${parseValue(value)}`);
            } else {
                // Object or array start
                currentKey = key;
            }
        } else {
            // Nested field
            if (result.length === 0 || !result[result.length - 1].startsWith(currentKey)) {
                result.push(currentKey);
            }
            result[result.length - 1] += `#${key}:${parseValue(value)}`;
        }
    }

    // Flush any remaining array
    if (inArray && arrayItems.length > 0) {
        result.push(`${currentKey}@${arrayItems.length}>${arrayItems.join('|')}`);
    }

    return result.join('\n');
}

/**
 * Parse a value from human format to dense format
 */
function parseValue(value: string): string {
    // Handle quoted strings
    if ((value.startsWith('"') && value.endsWith('"')) ||
        (value.startsWith("'") && value.endsWith("'"))) {
        return value.slice(1, -1);
    }
    
    // Handle booleans
    if (value === 'true') return '1';
    if (value === 'false') return '0';
    if (value === 'null') return '';
    
    return value;
}


/**
 * Validate DX content syntax (TypeScript fallback)
 * 
 * Checks for:
 * - Unclosed brackets
 * - Unclosed strings
 * - Mismatched brackets
 */
export function validateDx(content: string): ValidationResult {
    const bracketStack: Array<{ char: string; line: number; column: number }> = [];
    let inString = false;
    let stringChar = '"';
    let stringStart: { line: number; column: number } | null = null;

    const lines = content.split('\n');

    for (let lineIdx = 0; lineIdx < lines.length; lineIdx++) {
        const line = lines[lineIdx];
        const lineNum = lineIdx + 1;
        let col = 0;

        for (let i = 0; i < line.length; i++) {
            const ch = line[i];
            col = i + 1;

            // Handle escape sequences in strings
            if (inString && ch === '\\' && i + 1 < line.length) {
                i++; // Skip escaped character
                continue;
            }

            // Handle string boundaries
            if (!inString && (ch === '"' || ch === "'")) {
                inString = true;
                stringChar = ch;
                stringStart = { line: lineNum, column: col };
                continue;
            }

            if (inString && ch === stringChar) {
                inString = false;
                stringStart = null;
                continue;
            }

            // Skip bracket checking inside strings
            if (inString) {
                continue;
            }

            // Track brackets
            if (ch === '{' || ch === '[' || ch === '(') {
                bracketStack.push({ char: ch, line: lineNum, column: col });
            } else if (ch === '}' || ch === ']' || ch === ')') {
                const expected = ch === '}' ? '{' : ch === ']' ? '[' : '(';
                const expectedClose = ch;

                if (bracketStack.length === 0) {
                    return {
                        success: false,
                        error: `Unexpected closing bracket '${ch}'`,
                        line: lineNum,
                        column: col,
                        hint: `No matching opening bracket for '${ch}'`,
                    };
                }

                const open = bracketStack.pop()!;
                if (open.char !== expected) {
                    const expectedCloseForOpen = open.char === '{' ? '}' : open.char === '[' ? ']' : ')';
                    return {
                        success: false,
                        error: `Mismatched bracket: expected '${expectedCloseForOpen}' but found '${ch}'`,
                        line: lineNum,
                        column: col,
                        hint: `Opening '${open.char}' at line ${open.line}, column ${open.column} expects '${expectedCloseForOpen}'`,
                    };
                }
            }
        }
    }

    // Check for unclosed strings
    if (inString && stringStart) {
        return {
            success: false,
            error: `Unclosed string starting with '${stringChar}'`,
            line: stringStart.line,
            column: stringStart.column,
            hint: `Add a closing '${stringChar}' to complete the string`,
        };
    }

    // Check for unclosed brackets
    if (bracketStack.length > 0) {
        const open = bracketStack[bracketStack.length - 1];
        const expectedClose = open.char === '{' ? '}' : open.char === '[' ? ']' : ')';
        return {
            success: false,
            error: `Unclosed bracket '${open.char}'`,
            line: open.line,
            column: open.column,
            hint: `Add a closing '${expectedClose}' to match the opening '${open.char}'`,
        };
    }

    return { success: true };
}


/**
 * TypeScript fallback DxCore implementation
 */
class FallbackDxCore implements DxCore {
    readonly isWasm = false;
    private indentSize: number;

    constructor(indentSize: number = 2) {
        this.indentSize = indentSize;
    }

    toHuman(dense: string): TransformResult {
        try {
            const content = formatDx(dense, this.indentSize);
            return { success: true, content };
        } catch (error) {
            return {
                success: false,
                content: '',
                error: error instanceof Error ? error.message : String(error),
            };
        }
    }

    toDense(human: string): TransformResult {
        try {
            const content = minifyDx(human);
            return { success: true, content };
        } catch (error) {
            return {
                success: false,
                content: '',
                error: error instanceof Error ? error.message : String(error),
            };
        }
    }

    validate(content: string): ValidationResult {
        return validateDx(content);
    }

    isSaveable(content: string): boolean {
        return this.validate(content).success;
    }
}

// ============================================================================
// DxCore Loader
// ============================================================================

let cachedCore: DxCore | null = null;

/**
 * Load the DxCore, preferring WASM with TypeScript fallback
 * 
 * @param extensionPath - Path to the extension directory
 * @param indentSize - Indent size for formatting (default: 2)
 * @returns DxCore instance
 */
export async function loadDxCore(
    extensionPath: string,
    indentSize: number = 2
): Promise<DxCore> {
    // Return cached instance if available
    if (cachedCore) {
        return cachedCore;
    }

    // Try to load WASM
    try {
        const wasmPath = path.join(extensionPath, 'wasm', 'dx_serializer.js');
        
        if (fs.existsSync(wasmPath)) {
            // Dynamic import of WASM module
            const wasmModule = await import(wasmPath) as WasmModule;
            const serializer = new wasmModule.DxSerializer();
            cachedCore = new WasmDxCore(serializer);
            console.log('DX Serializer: Using WASM core');
            return cachedCore;
        }
    } catch (error) {
        console.warn('DX Serializer: WASM load failed, using fallback:', error);
    }

    // Fall back to TypeScript implementation
    cachedCore = new FallbackDxCore(indentSize);
    console.log('DX Serializer: Using TypeScript fallback');
    return cachedCore;
}

/**
 * Get the cached DxCore instance, or create a fallback if not loaded
 */
export function getDxCore(indentSize: number = 2): DxCore {
    if (!cachedCore) {
        cachedCore = new FallbackDxCore(indentSize);
    }
    return cachedCore;
}

/**
 * Clear the cached DxCore instance (for testing)
 */
export function clearDxCoreCache(): void {
    cachedCore = null;
}

/**
 * Create a fallback DxCore instance directly (for testing)
 */
export function createFallbackCore(indentSize: number = 2): DxCore {
    return new FallbackDxCore(indentSize);
}
