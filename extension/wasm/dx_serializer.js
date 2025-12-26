let wasm;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

let heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

let WASM_VECTOR_LEN = 0;

const DeflaterJsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_deflaterjs_free(ptr >>> 0, 1));

const DxSerializerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_dxserializer_free(ptr >>> 0, 1));

const HologramConfigJsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_hologramconfigjs_free(ptr >>> 0, 1));

const InflaterJsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_inflaterjs_free(ptr >>> 0, 1));

const SerializerConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_serializerconfig_free(ptr >>> 0, 1));

const TransformResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_transformresult_free(ptr >>> 0, 1));

const ValidationResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_validationresult_free(ptr >>> 0, 1));

/**
 * Deflater for JavaScript with custom configuration
 */
export class DeflaterJs {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DeflaterJs.prototype);
        obj.__wbg_ptr = ptr;
        DeflaterJsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DeflaterJsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_deflaterjs_free(ptr, 0);
    }
    /**
     * Create a deflater with custom configuration
     * @param {HologramConfigJs} config
     * @returns {DeflaterJs}
     */
    static withConfig(config) {
        _assertClass(config, HologramConfigJs);
        const ret = wasm.deflaterjs_withConfig(config.__wbg_ptr);
        return DeflaterJs.__wrap(ret);
    }
    /**
     * Convert JSON to LLM-dense
     * @param {string} json
     * @returns {string}
     */
    jsonToDense(json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(json, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
            const len0 = WASM_VECTOR_LEN;
            wasm.deflaterjs_jsonToDense(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr2 = r0;
            var len2 = r1;
            if (r3) {
                ptr2 = 0; len2 = 0;
                throw takeObject(r2);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * Create a deflater with default configuration
     */
    constructor() {
        const ret = wasm.deflaterjs_new();
        this.__wbg_ptr = ret >>> 0;
        DeflaterJsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Deflate human-pretty to LLM-dense
     * @param {string} pretty
     * @returns {string}
     */
    deflate(pretty) {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(pretty, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
            const len0 = WASM_VECTOR_LEN;
            wasm.deflaterjs_deflate(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred2_0 = r0;
            deferred2_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export(deferred2_0, deferred2_1, 1);
        }
    }
}
if (Symbol.dispose) DeflaterJs.prototype[Symbol.dispose] = DeflaterJs.prototype.free;

/**
 * DX Serializer for VS Code extension
 *
 * Provides transformation between LLM (disk) and Human (editor) formats
 * with validation support. Uses the llm module for format conversion.
 */
export class DxSerializer {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DxSerializer.prototype);
        obj.__wbg_ptr = ptr;
        DxSerializerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DxSerializerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dxserializer_free(ptr, 0);
    }
    /**
     * Check if content is complete enough to save
     *
     * Returns true if the content has no unclosed brackets or strings.
     * @param {string} content
     * @returns {boolean}
     */
    isSaveable(content) {
        const ptr0 = passStringToWasm0(content, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dxserializer_isSaveable(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Create a DxSerializer with custom configuration
     * @param {SerializerConfig} config
     * @returns {DxSerializer}
     */
    static withConfig(config) {
        _assertClass(config, SerializerConfig);
        var ptr0 = config.__destroy_into_raw();
        const ret = wasm.dxserializer_withConfig(ptr0);
        return DxSerializer.__wrap(ret);
    }
    /**
     * Get the maximum input size limit (100 MB)
     *
     * Files larger than this will be rejected to prevent memory exhaustion.
     * @returns {number}
     */
    maxInputSize() {
        const ret = wasm.dxserializer_maxInputSize(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get the maximum table rows limit (10 million rows)
     *
     * Tables with more rows than this will be rejected to prevent memory exhaustion.
     * @returns {number}
     */
    maxTableRows() {
        const ret = wasm.dxserializer_maxTableRows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get the maximum recursion depth limit (1000 levels)
     *
     * Structures nested deeper than this will be rejected to prevent stack overflow.
     * @returns {number}
     */
    maxRecursionDepth() {
        const ret = wasm.dxserializer_maxRecursionDepth(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Create a new DxSerializer with default configuration
     */
    constructor() {
        const ret = wasm.dxserializer_new();
        this.__wbg_ptr = ret >>> 0;
        DxSerializerFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Transform human-readable format to LLM format
     *
     * This is called when saving a .dx file in the editor.
     * Converts human-readable format back to token-optimized LLM format.
     * @param {string} human_input
     * @returns {TransformResult}
     */
    toDense(human_input) {
        const ptr0 = passStringToWasm0(human_input, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dxserializer_toDense(this.__wbg_ptr, ptr0, len0);
        return TransformResult.__wrap(ret);
    }
    /**
     * Transform LLM format to human-readable format
     *
     * This is called when opening a .dx file in the editor.
     * Converts sigil-based LLM format (#c, #:, #<letter>) to beautiful
     * human-readable format with Unicode tables and expanded keys.
     * @param {string} llm_input
     * @returns {TransformResult}
     */
    toHuman(llm_input) {
        const ptr0 = passStringToWasm0(llm_input, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dxserializer_toHuman(this.__wbg_ptr, ptr0, len0);
        return TransformResult.__wrap(ret);
    }
    /**
     * Validate content syntax
     *
     * Returns detailed error information including line, column, and hints.
     * @param {string} content
     * @returns {ValidationResult}
     */
    validate(content) {
        const ptr0 = passStringToWasm0(content, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dxserializer_validate(this.__wbg_ptr, ptr0, len0);
        return ValidationResult.__wrap(ret);
    }
}
if (Symbol.dispose) DxSerializer.prototype[Symbol.dispose] = DxSerializer.prototype.free;

/**
 * Configuration object for JavaScript
 */
export class HologramConfigJs {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(HologramConfigJs.prototype);
        obj.__wbg_ptr = ptr;
        HologramConfigJsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HologramConfigJsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hologramconfigjs_free(ptr, 0);
    }
    /**
     * Set indent size
     * @param {number} size
     */
    setIndentSize(size) {
        wasm.hologramconfigjs_setIndentSize(this.__wbg_ptr, size);
    }
    /**
     * Set whether to align values
     * @param {boolean} align
     */
    setAlignValues(align) {
        wasm.hologramconfigjs_setAlignValues(this.__wbg_ptr, align);
    }
    /**
     * Set whether to use box drawing for tables
     * @param {boolean} use_box
     */
    setUseBoxDrawing(use_box) {
        wasm.hologramconfigjs_setUseBoxDrawing(this.__wbg_ptr, use_box);
    }
    /**
     * Set whether to preserve comments
     * @param {boolean} preserve
     */
    setPreserveComments(preserve) {
        wasm.hologramconfigjs_setPreserveComments(this.__wbg_ptr, preserve);
    }
    /**
     * Set whether to use Unicode symbols
     * @param {boolean} use_unicode
     */
    setUseUnicodeSymbols(use_unicode) {
        wasm.hologramconfigjs_setUseUnicodeSymbols(this.__wbg_ptr, use_unicode);
    }
    /**
     * Create default configuration
     */
    constructor() {
        const ret = wasm.deflaterjs_new();
        this.__wbg_ptr = ret >>> 0;
        HologramConfigJsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Create ASCII-only configuration
     * @returns {HologramConfigJs}
     */
    static ascii() {
        const ret = wasm.hologramconfigjs_ascii();
        return HologramConfigJs.__wrap(ret);
    }
    /**
     * Create compact configuration
     * @returns {HologramConfigJs}
     */
    static compact() {
        const ret = wasm.hologramconfigjs_compact();
        return HologramConfigJs.__wrap(ret);
    }
}
if (Symbol.dispose) HologramConfigJs.prototype[Symbol.dispose] = HologramConfigJs.prototype.free;

/**
 * Inflater for JavaScript with custom configuration
 */
export class InflaterJs {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InflaterJs.prototype);
        obj.__wbg_ptr = ptr;
        InflaterJsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InflaterJsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_inflaterjs_free(ptr, 0);
    }
    /**
     * Create an inflater with custom configuration
     * @param {HologramConfigJs} config
     * @returns {InflaterJs}
     */
    static withConfig(config) {
        _assertClass(config, HologramConfigJs);
        const ret = wasm.deflaterjs_withConfig(config.__wbg_ptr);
        return InflaterJs.__wrap(ret);
    }
    /**
     * Create an inflater with default configuration
     */
    constructor() {
        const ret = wasm.deflaterjs_new();
        this.__wbg_ptr = ret >>> 0;
        InflaterJsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Inflate LLM-dense to human-pretty
     * @param {string} dense
     * @returns {string}
     */
    inflate(dense) {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(dense, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
            const len0 = WASM_VECTOR_LEN;
            wasm.inflaterjs_inflate(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred2_0 = r0;
            deferred2_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export(deferred2_0, deferred2_1, 1);
        }
    }
}
if (Symbol.dispose) InflaterJs.prototype[Symbol.dispose] = InflaterJs.prototype.free;

/**
 * Serializer configuration for the VS Code extension
 */
export class SerializerConfig {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SerializerConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_serializerconfig_free(ptr, 0);
    }
    /**
     * Set the indent size (2 or 4)
     * @param {number} size
     */
    setIndentSize(size) {
        wasm.serializerconfig_setIndentSize(this.__wbg_ptr, size);
    }
    /**
     * Set whether to use smart quoting
     * @param {boolean} smart
     */
    setSmartQuoting(smart) {
        wasm.serializerconfig_setSmartQuoting(this.__wbg_ptr, smart);
    }
    /**
     * Set whether to preserve comments
     * @param {boolean} preserve
     */
    setPreserveComments(preserve) {
        wasm.serializerconfig_setPreserveComments(this.__wbg_ptr, preserve);
    }
    /**
     * Create a new configuration with defaults
     */
    constructor() {
        const ret = wasm.serializerconfig_new();
        this.__wbg_ptr = ret >>> 0;
        SerializerConfigFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) SerializerConfig.prototype[Symbol.dispose] = SerializerConfig.prototype.free;

/**
 * Result of a transformation operation
 */
export class TransformResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TransformResult.prototype);
        obj.__wbg_ptr = ptr;
        TransformResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TransformResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_transformresult_free(ptr, 0);
    }
    /**
     * Error message if transformation failed
     * @returns {string | undefined}
     */
    get error() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transformresult_error(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * The transformed content (empty if failed)
     * @returns {string}
     */
    get content() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.transformresult_content(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Whether the transformation succeeded
     * @returns {boolean}
     */
    get success() {
        const ret = wasm.transformresult_success(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) TransformResult.prototype[Symbol.dispose] = TransformResult.prototype.free;

/**
 * Result of a validation operation
 */
export class ValidationResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ValidationResult.prototype);
        obj.__wbg_ptr = ptr;
        ValidationResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ValidationResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_validationresult_free(ptr, 0);
    }
    /**
     * Actionable hint for fixing the error
     * @returns {string | undefined}
     */
    get hint() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.validationresult_hint(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Line number where error occurred (1-indexed)
     * @returns {number | undefined}
     */
    get line() {
        const ret = wasm.validationresult_line(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * Error message if validation failed
     * @returns {string | undefined}
     */
    get error() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.validationresult_error(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Column number where error occurred (1-indexed)
     * @returns {number | undefined}
     */
    get column() {
        const ret = wasm.validationresult_column(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * Whether the content is valid
     * @returns {boolean}
     */
    get success() {
        const ret = wasm.validationresult_success(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) ValidationResult.prototype[Symbol.dispose] = ValidationResult.prototype.free;

/**
 * Deflate human-pretty format to LLM-dense format
 *
 * This is called when saving a .dx file in the editor.
 * The beautiful format shown to the user is transformed back to
 * the token-efficient format stored on disk.
 *
 * @param pretty - The human-pretty format string from editor
 * @returns The LLM-dense format string for disk storage
 * @param {string} pretty
 * @returns {string}
 */
export function deflate(pretty) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(pretty, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        wasm.deflate(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Get version information
 * @returns {string}
 */
export function hologramVersion() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.hologramVersion(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Inflate LLM-dense format to human-pretty format
 *
 * This is called when opening a .dx file in the editor.
 * The dense format stored on disk is transformed to the beautiful
 * format shown to the user.
 *
 * @param dense - The LLM-dense format string from disk
 * @returns The human-pretty format string for editor display
 * @param {string} dense
 * @returns {string}
 */
export function inflate(dense) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(dense, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        wasm.inflate(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Inflate with ASCII-only output (no Unicode symbols)
 *
 * @param dense - The LLM-dense format string
 * @returns ASCII-only human-readable format
 * @param {string} dense
 * @returns {string}
 */
export function inflate_ascii(dense) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(dense, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        wasm.inflate_ascii(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Inflate with compact output
 *
 * @param dense - The LLM-dense format string
 * @returns Compact human-readable format
 * @param {string} dense
 * @returns {string}
 */
export function inflate_compact(dense) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(dense, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        wasm.inflate_compact(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Initialize WASM module
 */
export function init_wasm() {
    wasm.init_wasm();
}

/**
 * Convert JSON to LLM-dense format
 *
 * @param json - A JSON string
 * @returns The LLM-dense format string
 * @param {string} json
 * @returns {string}
 */
export function json_to_dense(json) {
    let deferred3_0;
    let deferred3_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        wasm.json_to_dense(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        var ptr2 = r0;
        var len2 = r1;
        if (r3) {
            ptr2 = 0; len2 = 0;
            throw takeObject(r2);
        }
        deferred3_0 = ptr2;
        deferred3_1 = len2;
        return getStringFromWasm0(ptr2, len2);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export(deferred3_0, deferred3_1, 1);
    }
}

/**
 * Get version information
 * @returns {string}
 */
export function serializerVersion() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.serializerVersion(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Verify round-trip: checks that deflate(inflate(x)) preserves data
 *
 * @param dense - The LLM-dense format string to test
 * @returns true if round-trip preserves data
 * @param {string} dense
 * @returns {boolean}
 */
export function verify_round_trip(dense) {
    const ptr0 = passStringToWasm0(dense, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.verify_round_trip(ptr0, len0);
    return ret !== 0;
}

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function (arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function (arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_export(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function () {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function (arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function (arg0, arg1) {
        // Cast intrinsic for `Ref(String) -> Externref`.
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function (arg0) {
        takeObject(arg0);
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({ module } = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({ module_or_path } = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('dx_serializer_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
