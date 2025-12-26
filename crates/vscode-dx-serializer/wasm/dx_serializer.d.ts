/* tslint:disable */
/* eslint-disable */

export class DeflaterJs {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create a deflater with custom configuration
   */
  static withConfig(config: HologramConfigJs): DeflaterJs;
  /**
   * Convert JSON to LLM-dense
   */
  jsonToDense(json: string): string;
  /**
   * Create a deflater with default configuration
   */
  constructor();
  /**
   * Deflate human-pretty to LLM-dense
   */
  deflate(pretty: string): string;
}

export class DxSerializer {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Check if content is complete enough to save
   *
   * Returns true if the content has no unclosed brackets or strings.
   */
  isSaveable(content: string): boolean;
  /**
   * Create a DxSerializer with custom configuration
   */
  static withConfig(config: SerializerConfig): DxSerializer;
  /**
   * Get the maximum input size limit (100 MB)
   *
   * Files larger than this will be rejected to prevent memory exhaustion.
   */
  maxInputSize(): number;
  /**
   * Get the maximum table rows limit (10 million rows)
   *
   * Tables with more rows than this will be rejected to prevent memory exhaustion.
   */
  maxTableRows(): number;
  /**
   * Get the maximum recursion depth limit (1000 levels)
   *
   * Structures nested deeper than this will be rejected to prevent stack overflow.
   */
  maxRecursionDepth(): number;
  /**
   * Create a new DxSerializer with default configuration
   */
  constructor();
  /**
   * Transform human-readable format to LLM format
   *
   * This is called when saving a .dx file in the editor.
   * Converts human-readable format back to token-optimized LLM format.
   */
  toDense(human_input: string): TransformResult;
  /**
   * Transform LLM format to human-readable format
   *
   * This is called when opening a .dx file in the editor.
   * Converts sigil-based LLM format (#c, #:, #<letter>) to beautiful
   * human-readable format with Unicode tables and expanded keys.
   */
  toHuman(llm_input: string): TransformResult;
  /**
   * Validate content syntax
   *
   * Returns detailed error information including line, column, and hints.
   */
  validate(content: string): ValidationResult;
}

export class HologramConfigJs {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Set indent size
   */
  setIndentSize(size: number): void;
  /**
   * Set whether to align values
   */
  setAlignValues(align: boolean): void;
  /**
   * Set whether to use box drawing for tables
   */
  setUseBoxDrawing(use_box: boolean): void;
  /**
   * Set whether to preserve comments
   */
  setPreserveComments(preserve: boolean): void;
  /**
   * Set whether to use Unicode symbols
   */
  setUseUnicodeSymbols(use_unicode: boolean): void;
  /**
   * Create default configuration
   */
  constructor();
  /**
   * Create ASCII-only configuration
   */
  static ascii(): HologramConfigJs;
  /**
   * Create compact configuration
   */
  static compact(): HologramConfigJs;
}

export class InflaterJs {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create an inflater with custom configuration
   */
  static withConfig(config: HologramConfigJs): InflaterJs;
  /**
   * Create an inflater with default configuration
   */
  constructor();
  /**
   * Inflate LLM-dense to human-pretty
   */
  inflate(dense: string): string;
}

export class SerializerConfig {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Set the indent size (2 or 4)
   */
  setIndentSize(size: number): void;
  /**
   * Set whether to use smart quoting
   */
  setSmartQuoting(smart: boolean): void;
  /**
   * Set whether to preserve comments
   */
  setPreserveComments(preserve: boolean): void;
  /**
   * Create a new configuration with defaults
   */
  constructor();
}

export class TransformResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Error message if transformation failed
   */
  readonly error: string | undefined;
  /**
   * The transformed content (empty if failed)
   */
  readonly content: string;
  /**
   * Whether the transformation succeeded
   */
  readonly success: boolean;
}

export class ValidationResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Actionable hint for fixing the error
   */
  readonly hint: string | undefined;
  /**
   * Line number where error occurred (1-indexed)
   */
  readonly line: number | undefined;
  /**
   * Error message if validation failed
   */
  readonly error: string | undefined;
  /**
   * Column number where error occurred (1-indexed)
   */
  readonly column: number | undefined;
  /**
   * Whether the content is valid
   */
  readonly success: boolean;
}

/**
 * Deflate human-pretty format to LLM-dense format
 *
 * This is called when saving a .dx file in the editor.
 * The beautiful format shown to the user is transformed back to
 * the token-efficient format stored on disk.
 *
 * @param pretty - The human-pretty format string from editor
 * @returns The LLM-dense format string for disk storage
 */
export function deflate(pretty: string): string;

/**
 * Get version information
 */
export function hologramVersion(): string;

/**
 * Inflate LLM-dense format to human-pretty format
 *
 * This is called when opening a .dx file in the editor.
 * The dense format stored on disk is transformed to the beautiful
 * format shown to the user.
 *
 * @param dense - The LLM-dense format string from disk
 * @returns The human-pretty format string for editor display
 */
export function inflate(dense: string): string;

/**
 * Inflate with ASCII-only output (no Unicode symbols)
 *
 * @param dense - The LLM-dense format string
 * @returns ASCII-only human-readable format
 */
export function inflate_ascii(dense: string): string;

/**
 * Inflate with compact output
 *
 * @param dense - The LLM-dense format string
 * @returns Compact human-readable format
 */
export function inflate_compact(dense: string): string;

/**
 * Initialize WASM module
 */
export function init_wasm(): void;

/**
 * Convert JSON to LLM-dense format
 *
 * @param json - A JSON string
 * @returns The LLM-dense format string
 */
export function json_to_dense(json: string): string;

/**
 * Get version information
 */
export function serializerVersion(): string;

/**
 * Verify round-trip: checks that deflate(inflate(x)) preserves data
 *
 * @param dense - The LLM-dense format string to test
 * @returns true if round-trip preserves data
 */
export function verify_round_trip(dense: string): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_deflaterjs_free: (a: number, b: number) => void;
  readonly __wbg_dxserializer_free: (a: number, b: number) => void;
  readonly __wbg_transformresult_free: (a: number, b: number) => void;
  readonly __wbg_validationresult_free: (a: number, b: number) => void;
  readonly deflate: (a: number, b: number, c: number) => void;
  readonly deflaterjs_deflate: (a: number, b: number, c: number, d: number) => void;
  readonly deflaterjs_jsonToDense: (a: number, b: number, c: number, d: number) => void;
  readonly deflaterjs_new: () => number;
  readonly deflaterjs_withConfig: (a: number) => number;
  readonly dxserializer_isSaveable: (a: number, b: number, c: number) => number;
  readonly dxserializer_maxInputSize: (a: number) => number;
  readonly dxserializer_maxRecursionDepth: (a: number) => number;
  readonly dxserializer_maxTableRows: (a: number) => number;
  readonly dxserializer_new: () => number;
  readonly dxserializer_toDense: (a: number, b: number, c: number) => number;
  readonly dxserializer_toHuman: (a: number, b: number, c: number) => number;
  readonly dxserializer_validate: (a: number, b: number, c: number) => number;
  readonly dxserializer_withConfig: (a: number) => number;
  readonly hologramVersion: (a: number) => void;
  readonly hologramconfigjs_ascii: () => number;
  readonly hologramconfigjs_compact: () => number;
  readonly hologramconfigjs_setAlignValues: (a: number, b: number) => void;
  readonly hologramconfigjs_setIndentSize: (a: number, b: number) => void;
  readonly hologramconfigjs_setPreserveComments: (a: number, b: number) => void;
  readonly hologramconfigjs_setUseBoxDrawing: (a: number, b: number) => void;
  readonly hologramconfigjs_setUseUnicodeSymbols: (a: number, b: number) => void;
  readonly inflate: (a: number, b: number, c: number) => void;
  readonly inflate_ascii: (a: number, b: number, c: number) => void;
  readonly inflate_compact: (a: number, b: number, c: number) => void;
  readonly inflaterjs_inflate: (a: number, b: number, c: number, d: number) => void;
  readonly init_wasm: () => void;
  readonly json_to_dense: (a: number, b: number, c: number) => void;
  readonly serializerVersion: (a: number) => void;
  readonly serializerconfig_new: () => number;
  readonly serializerconfig_setIndentSize: (a: number, b: number) => void;
  readonly serializerconfig_setPreserveComments: (a: number, b: number) => void;
  readonly serializerconfig_setSmartQuoting: (a: number, b: number) => void;
  readonly transformresult_content: (a: number, b: number) => void;
  readonly transformresult_error: (a: number, b: number) => void;
  readonly transformresult_success: (a: number) => number;
  readonly validationresult_column: (a: number) => number;
  readonly validationresult_error: (a: number, b: number) => void;
  readonly validationresult_hint: (a: number, b: number) => void;
  readonly validationresult_line: (a: number) => number;
  readonly validationresult_success: (a: number) => number;
  readonly verify_round_trip: (a: number, b: number) => number;
  readonly inflaterjs_withConfig: (a: number) => number;
  readonly inflaterjs_new: () => number;
  readonly hologramconfigjs_new: () => number;
  readonly __wbg_serializerconfig_free: (a: number, b: number) => void;
  readonly __wbg_hologramconfigjs_free: (a: number, b: number) => void;
  readonly __wbg_inflaterjs_free: (a: number, b: number) => void;
  readonly __wbindgen_export: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export2: (a: number, b: number) => number;
  readonly __wbindgen_export3: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
