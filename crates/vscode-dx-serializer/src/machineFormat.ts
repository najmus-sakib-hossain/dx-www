/**
 * Machine Format for DX Serializer VS Code Extension
 * 
 * Provides efficient serialization/deserialization of DxDocument.
 * Uses JSON format for simplicity and compatibility.
 * 
 * Requirements: 1.7, 3.4
 */

import {
    DxDocument,
    DxSection,
    DxValue,
    createDocument,
    createSection,
    strValue,
    numValue,
    boolValue,
    nullValue,
    arrValue,
    refValue,
} from './llmParser';

// ============================================================================
// Types
// ============================================================================

export interface MachineDocument {
    version: number;
    context: Record<string, MachineValue>;
    refs: Record<string, string>;
    sections: Record<string, MachineSection>;
}

export interface MachineSection {
    id: string;
    schema: string[];
    rows: MachineValue[][];
}

export interface MachineValue {
    t: 's' | 'n' | 'b' | 'x' | 'a' | 'r'; // type: string, number, bool, null, array, ref
    v: unknown;
}

export interface MachineResult {
    success: boolean;
    document?: DxDocument;
    error?: string;
}

// ============================================================================
// Value Conversion
// ============================================================================

/**
 * Convert DxValue to MachineValue
 */
export function dxValueToMachine(value: DxValue): MachineValue {
    switch (value.type) {
        case 'string':
            return { t: 's', v: value.value };
        case 'number':
            return { t: 'n', v: value.value };
        case 'bool':
            return { t: 'b', v: value.value };
        case 'null':
            return { t: 'x', v: null };
        case 'array':
            const items = value.value as DxValue[];
            return { t: 'a', v: items.map(dxValueToMachine) };
        case 'ref':
            return { t: 'r', v: value.refKey || value.value };
        default:
            return { t: 's', v: String(value.value) };
    }
}

/**
 * Convert MachineValue to DxValue
 */
export function machineValueToDx(value: MachineValue): DxValue {
    switch (value.t) {
        case 's':
            return strValue(String(value.v));
        case 'n':
            return numValue(Number(value.v));
        case 'b':
            return boolValue(Boolean(value.v));
        case 'x':
            return nullValue();
        case 'a':
            const items = value.v as MachineValue[];
            return arrValue(items.map(machineValueToDx));
        case 'r':
            return refValue(String(value.v));
        default:
            return strValue(String(value.v));
    }
}

// ============================================================================
// Document Conversion
// ============================================================================

/**
 * Convert DxDocument to machine format
 * Requirement: 1.7
 */
export function documentToMachine(doc: DxDocument): MachineDocument {
    const machine: MachineDocument = {
        version: 1,
        context: {},
        refs: {},
        sections: {},
    };

    // Convert context
    for (const [key, value] of doc.context) {
        machine.context[key] = dxValueToMachine(value);
    }

    // Convert refs
    for (const [key, value] of doc.refs) {
        machine.refs[key] = value;
    }

    // Convert sections
    for (const [id, section] of doc.sections) {
        machine.sections[id] = {
            id: section.id,
            schema: section.schema,
            rows: section.rows.map(row => row.map(dxValueToMachine)),
        };
    }

    return machine;
}

/**
 * Convert machine format to DxDocument
 * Requirement: 3.4
 */
export function machineToDocument(machine: MachineDocument): DxDocument {
    const doc = createDocument();

    // Convert context
    for (const [key, value] of Object.entries(machine.context)) {
        doc.context.set(key, machineValueToDx(value));
    }

    // Convert refs
    for (const [key, value] of Object.entries(machine.refs)) {
        doc.refs.set(key, value);
    }

    // Convert sections
    for (const [id, section] of Object.entries(machine.sections)) {
        const dxSection = createSection(section.id, section.schema);
        dxSection.rows = section.rows.map(row => row.map(machineValueToDx));
        doc.sections.set(id, dxSection);
    }

    return doc;
}

// ============================================================================
// Serialization
// ============================================================================

/**
 * Serialize DxDocument to machine format string
 */
export function serializeMachine(doc: DxDocument): string {
    const machine = documentToMachine(doc);
    return JSON.stringify(machine);
}

/**
 * Deserialize machine format string to DxDocument
 */
export function deserializeMachine(content: string): MachineResult {
    try {
        const machine = JSON.parse(content) as MachineDocument;

        // Validate version
        if (machine.version !== 1) {
            return {
                success: false,
                error: `Unsupported machine format version: ${machine.version}`,
            };
        }

        const doc = machineToDocument(machine);
        return { success: true, document: doc };
    } catch (error) {
        return {
            success: false,
            error: `Machine format parse error: ${error instanceof Error ? error.message : String(error)}`,
        };
    }
}
