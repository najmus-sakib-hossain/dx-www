/**
 * Format Detector for DX Serializer VS Code Extension
 * 
 * Detects input file format to enable automatic conversion:
 * - JSON: starts with { or [
 * - YAML: : patterns, ---, - at line start
 * - TOML: [section] with key = value
 * - CSV: comma-separated with consistent columns
 * - LLM: #c:, #:, #<letter>(
 * - Human V3: key = value patterns
 * 
 * Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 5.6
 */

// ============================================================================
// Types
// ============================================================================

export type DetectedFormat =
    | 'json'
    | 'yaml'
    | 'toml'
    | 'csv'
    | 'llm'
    | 'human-v3'
    | 'unknown';

export interface FormatDetectionResult {
    format: DetectedFormat;
    confidence: number; // 0-1
    hints: string[];
}

// ============================================================================
// Detection Functions
// ============================================================================

/**
 * Detect if content is JSON format
 * Requirement: 5.1
 */
export function detectJson(content: string): FormatDetectionResult {
    const trimmed = content.trim();
    const hints: string[] = [];
    let confidence = 0;

    // Check for JSON object or array start
    if (trimmed.startsWith('{') || trimmed.startsWith('[')) {
        hints.push('Starts with { or [');
        confidence += 0.5;

        // Try to parse as JSON
        try {
            JSON.parse(trimmed);
            hints.push('Valid JSON syntax');
            confidence += 0.5;
        } catch {
            hints.push('Invalid JSON syntax');
            confidence -= 0.3;
        }
    }

    return { format: 'json', confidence: Math.max(0, Math.min(1, confidence)), hints };
}

/**
 * Detect if content is YAML format
 * Requirement: 5.2
 */
export function detectYaml(content: string): FormatDetectionResult {
    const lines = content.split('\n');
    const hints: string[] = [];
    let confidence = 0;

    // Check for YAML document start
    if (content.trim().startsWith('---')) {
        hints.push('Starts with ---');
        confidence += 0.4;
    }

    // Check for key: value patterns
    let colonPatterns = 0;
    let listItems = 0;

    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('#')) continue;

        // key: value pattern (not URL)
        if (/^[a-zA-Z_][a-zA-Z0-9_]*:\s?/.test(trimmed) && !trimmed.includes('://')) {
            colonPatterns++;
        }

        // List item: - value (with or without space)
        if (/^-\s*\S/.test(trimmed)) {
            listItems++;
        }
    }

    if (colonPatterns >= 2) {
        hints.push(`Found ${colonPatterns} key: value patterns`);
        confidence += 0.3;
    }

    if (listItems >= 1) {
        hints.push(`Found ${listItems} list items`);
        confidence += 0.2;
    }

    // Combined: key: with list items is strong YAML signal
    if (colonPatterns >= 1 && listItems >= 1) {
        confidence += 0.2;
    }

    // Negative: has = signs (more likely TOML)
    const equalSigns = lines.filter(l => /^\s*[a-zA-Z_][a-zA-Z0-9_]*\s*=/.test(l)).length;
    if (equalSigns > colonPatterns) {
        confidence -= 0.3;
    }

    return { format: 'yaml', confidence: Math.max(0, Math.min(1, confidence)), hints };
}

/**
 * Detect if content is TOML format
 * Requirement: 5.3
 */
export function detectToml(content: string): FormatDetectionResult {
    const lines = content.split('\n');
    const hints: string[] = [];
    let confidence = 0;

    let sectionHeaders = 0;
    let keyValuePairs = 0;

    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('#')) continue;

        // [section] header (not array)
        if (/^\[[a-zA-Z_][a-zA-Z0-9_]*\]$/.test(trimmed)) {
            sectionHeaders++;
        }

        // key = value pattern
        if (/^[a-zA-Z_][a-zA-Z0-9_]*\s*=\s*.+$/.test(trimmed)) {
            keyValuePairs++;
        }
    }

    if (sectionHeaders >= 1) {
        hints.push(`Found ${sectionHeaders} [section] headers`);
        confidence += 0.4;
    }

    if (keyValuePairs >= 2) {
        hints.push(`Found ${keyValuePairs} key = value pairs`);
        confidence += 0.3;
    }

    // Negative: has LLM markers
    if (content.includes('#c:') || content.includes('#f(') || content.includes('#k(')) {
        confidence -= 0.5;
    }

    return { format: 'toml', confidence: Math.max(0, Math.min(1, confidence)), hints };
}

/**
 * Detect if content is CSV format
 * Requirement: 5.4
 */
export function detectCsv(content: string): FormatDetectionResult {
    const lines = content.split('\n').filter(l => l.trim());
    const hints: string[] = [];
    let confidence = 0;

    if (lines.length < 2) {
        return { format: 'csv', confidence: 0, hints: ['Too few lines'] };
    }

    // Count commas per line
    const commaCounts = lines.map(l => (l.match(/,/g) || []).length);

    // Check for consistent comma count
    const firstCount = commaCounts[0];
    if (firstCount >= 1) {
        const consistent = commaCounts.every(c => c === firstCount);
        if (consistent) {
            hints.push(`Consistent ${firstCount + 1} columns`);
            confidence += 0.5;
        } else {
            hints.push('Inconsistent column count');
            confidence += 0.2;
        }
    }

    // Check for header row (no numbers in first row, numbers in subsequent)
    const firstRow = lines[0].split(',');
    const hasHeaderLike = firstRow.every(cell => !/^\d+$/.test(cell.trim()));
    if (hasHeaderLike && lines.length > 1) {
        hints.push('Has header-like first row');
        confidence += 0.2;
    }

    // Negative: has = signs or : patterns
    if (content.includes(' = ') || /^[a-z]+:/.test(content)) {
        confidence -= 0.3;
    }

    return { format: 'csv', confidence: Math.max(0, Math.min(1, confidence)), hints };
}

/**
 * Detect if content is LLM format
 * Requirement: 5.5
 * 
 * New format: Root-level key|value pairs without #c: prefix
 * Legacy format: #c:key|value;key|value (still supported)
 */
export function detectLlm(content: string): FormatDetectionResult {
    const hints: string[] = [];
    let confidence = 0;

    // Check for legacy context marker (still supported)
    if (content.includes('#c:')) {
        hints.push('Has #c: context marker (legacy)');
        confidence += 0.4;
    }

    // Check for reference marker
    if (content.includes('#:')) {
        hints.push('Has #: reference marker');
        confidence += 0.2;
    }

    // Check for section markers #f( #k( #y( etc.
    if (/#[a-z]\(/.test(content)) {
        hints.push('Has #x( section markers');
        confidence += 0.4;
    }

    // Check for pipe-separated data rows (new format: root-level key|value)
    const lines = content.split('\n');
    const pipeRows = lines.filter(l => !l.startsWith('#') && l.includes('|')).length;
    if (pipeRows >= 1) {
        hints.push(`Has ${pipeRows} pipe-separated rows`);
        confidence += 0.2;
    }

    // Check for root-level key|value pairs (new format)
    // These are lines that start with a short key followed by |
    const rootKeyValuePattern = /^[a-z]{1,4}\|/;
    const rootKeyValueLines = lines.filter(l => rootKeyValuePattern.test(l.trim())).length;
    if (rootKeyValueLines >= 2) {
        hints.push(`Has ${rootKeyValueLines} root-level key|value pairs`);
        confidence += 0.3;
    }

    return { format: 'llm', confidence: Math.max(0, Math.min(1, confidence)), hints };
}

/**
 * Detect if content is Human V3 format
 * Requirement: 5.6
 */
export function detectHumanV3(content: string): FormatDetectionResult {
    const lines = content.split('\n');
    const hints: string[] = [];
    let confidence = 0;

    let keyValuePairs = 0;
    let sectionHeaders = 0;
    let pipeArrays = 0;

    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('#')) continue;

        // [section] header with optional = schema
        if (/^\[[a-zA-Z0-9_]+\](\s*=\s*.+)?$/.test(trimmed)) {
            sectionHeaders++;
        }

        // key = value with padding (2+ spaces before =)
        if (/^[a-zA-Z_][a-zA-Z0-9_]*\s{2,}=\s+.+$/.test(trimmed)) {
            keyValuePairs++;
        }

        // Pipe-separated arrays
        if (trimmed.includes(' | ')) {
            pipeArrays++;
        }
    }

    if (keyValuePairs >= 2) {
        hints.push(`Found ${keyValuePairs} padded key = value pairs`);
        confidence += 0.4;
    }

    if (sectionHeaders >= 1) {
        hints.push(`Found ${sectionHeaders} [section] headers`);
        confidence += 0.2;
    }

    if (pipeArrays >= 1) {
        hints.push(`Found ${pipeArrays} pipe-separated arrays`);
        confidence += 0.2;
    }

    // Section header with schema is strong Human V3 signal
    if (sectionHeaders >= 1 && pipeArrays >= 1) {
        confidence += 0.2;
    }

    // Negative: has LLM markers
    if (content.includes('#c:') || /#[a-z]\(/.test(content)) {
        confidence -= 0.5;
    }

    // Negative: has decorative borders (old format)
    if (content.includes('════') || content.includes('┌───')) {
        confidence -= 0.3;
    }

    return { format: 'human-v3', confidence: Math.max(0, Math.min(1, confidence)), hints };
}

// ============================================================================
// Main Detector
// ============================================================================

/**
 * Detect the format of input content
 * Returns the most likely format with confidence score
 */
export function detectFormat(content: string): FormatDetectionResult {
    if (!content || !content.trim()) {
        return { format: 'unknown', confidence: 0, hints: ['Empty content'] };
    }

    // Run all detectors
    const results: FormatDetectionResult[] = [
        detectLlm(content),      // Check LLM first (most specific)
        detectJson(content),
        detectYaml(content),
        detectToml(content),
        detectCsv(content),
        detectHumanV3(content),
    ];

    // Find highest confidence
    let best = results[0];
    for (const result of results) {
        if (result.confidence > best.confidence) {
            best = result;
        }
    }

    // If no clear winner, return unknown
    if (best.confidence < 0.3) {
        return { format: 'unknown', confidence: best.confidence, hints: ['No clear format detected'] };
    }

    return best;
}

/**
 * Check if format is a source format that should be converted to LLM
 */
export function isSourceFormat(format: DetectedFormat): boolean {
    return format === 'json' || format === 'yaml' || format === 'toml' || format === 'csv';
}

/**
 * Check if format is a DX format (LLM or Human)
 */
export function isDxFormat(format: DetectedFormat): boolean {
    return format === 'llm' || format === 'human-v3';
}
