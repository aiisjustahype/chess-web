let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_0.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}
/**
 * @param {Board} board
 * @param {number} depth
 * @returns {SMove}
 */
export function get_best_move(board, depth) {
    _assertClass(board, Board);
    const ret = wasm.get_best_move(board.__wbg_ptr, depth);
    return SMove.__wrap(ret);
}

/**
 * @enum {0 | 1 | 2 | 3 | 4}
 */
export const CastleOpt = Object.freeze({
    NONE: 0, "0": "NONE",
    WHITESHORT: 1, "1": "WHITESHORT",
    BLACKSHORT: 2, "2": "BLACKSHORT",
    WHITELONG: 3, "3": "WHITELONG",
    BLACKLONG: 4, "4": "BLACKLONG",
});
/**
 * @enum {0 | 1}
 */
export const Color = Object.freeze({
    WHITE: 0, "0": "WHITE",
    BLACK: 1, "1": "BLACK",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6}
 */
export const Piece = Object.freeze({
    NONE: 0, "0": "NONE",
    PAWN: 1, "1": "PAWN",
    KNIGHT: 2, "2": "KNIGHT",
    BISHOP: 3, "3": "BISHOP",
    ROOK: 4, "4": "ROOK",
    QUEEN: 5, "5": "QUEEN",
    KING: 6, "6": "KING",
});

const BoardFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_board_free(ptr >>> 0, 1));

export class Board {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Board.prototype);
        obj.__wbg_ptr = ptr;
        BoardFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BoardFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_board_free(ptr, 0);
    }
    /**
     * @returns {Color}
     */
    get turn() {
        const ret = wasm.__wbg_get_board_turn(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Color} arg0
     */
    set turn(arg0) {
        wasm.__wbg_set_board_turn(this.__wbg_ptr, arg0);
    }
    constructor() {
        const ret = wasm.board_new();
        this.__wbg_ptr = ret >>> 0;
        BoardFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Board}
     */
    get_clone() {
        const ret = wasm.board_get_clone(this.__wbg_ptr);
        return Board.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    fen() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.board_fen(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {SMove[]}
     */
    all_white_moves() {
        const ret = wasm.board_all_white_moves(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {SMove[]}
     */
    all_black_moves() {
        const ret = wasm.board_all_black_moves(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {bigint}
     */
    white_attacks() {
        const ret = wasm.board_white_attacks(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @returns {bigint}
     */
    black_attacks() {
        const ret = wasm.board_black_attacks(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @param {SMove} smove
     */
    make_move(smove) {
        _assertClass(smove, SMove);
        var ptr0 = smove.__destroy_into_raw();
        wasm.board_make_move(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {SMove} smove
     */
    play(smove) {
        _assertClass(smove, SMove);
        var ptr0 = smove.__destroy_into_raw();
        wasm.board_play(this.__wbg_ptr, ptr0);
    }
    print() {
        wasm.board_print(this.__wbg_ptr);
    }
    /**
     * @returns {string[]}
     */
    get_rows() {
        const ret = wasm.board_get_rows(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Board[]}
     */
    get_next_boards() {
        const ret = wasm.board_get_next_boards(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {MoveBoardPair[]}
     */
    get_next_move_boards() {
        const ret = wasm.board_get_next_move_boards(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {SMove[]}
     */
    get_moves() {
        const ret = wasm.board_get_moves(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {number} square
     * @returns {SMove[]}
     */
    get_moves_square(square) {
        const ret = wasm.board_get_moves_square(this.__wbg_ptr, square);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {boolean}
     */
    is_checkmate() {
        const ret = wasm.board_is_checkmate(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    is_stalemate() {
        const ret = wasm.board_is_stalemate(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    is_threefold_rep() {
        const ret = wasm.board_is_threefold_rep(this.__wbg_ptr);
        return ret !== 0;
    }
}

const MoveBoardPairFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_moveboardpair_free(ptr >>> 0, 1));

export class MoveBoardPair {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MoveBoardPair.prototype);
        obj.__wbg_ptr = ptr;
        MoveBoardPairFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MoveBoardPairFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_moveboardpair_free(ptr, 0);
    }
    /**
     * @returns {SMove}
     */
    get 0() {
        const ret = wasm.__wbg_get_moveboardpair_0(this.__wbg_ptr);
        return SMove.__wrap(ret);
    }
    /**
     * @param {SMove} arg0
     */
    set 0(arg0) {
        _assertClass(arg0, SMove);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_moveboardpair_0(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Board}
     */
    get 1() {
        const ret = wasm.__wbg_get_moveboardpair_1(this.__wbg_ptr);
        return Board.__wrap(ret);
    }
    /**
     * @param {Board} arg0
     */
    set 1(arg0) {
        _assertClass(arg0, Board);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_moveboardpair_1(this.__wbg_ptr, ptr0);
    }
}

const SMoveFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_smove_free(ptr >>> 0, 1));

export class SMove {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SMove.prototype);
        obj.__wbg_ptr = ptr;
        SMoveFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SMoveFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_smove_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get from() {
        const ret = wasm.__wbg_get_smove_from(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set from(arg0) {
        wasm.__wbg_set_smove_from(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get to() {
        const ret = wasm.__wbg_get_smove_to(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set to(arg0) {
        wasm.__wbg_set_smove_to(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Piece}
     */
    get promote_piece() {
        const ret = wasm.__wbg_get_smove_promote_piece(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Piece} arg0
     */
    set promote_piece(arg0) {
        wasm.__wbg_set_smove_promote_piece(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {CastleOpt}
     */
    get castle_move() {
        const ret = wasm.__wbg_get_smove_castle_move(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {CastleOpt} arg0
     */
    set castle_move(arg0) {
        wasm.__wbg_set_smove_castle_move(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get ep_move() {
        const ret = wasm.__wbg_get_smove_ep_move(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set ep_move(arg0) {
        wasm.__wbg_set_smove_ep_move(this.__wbg_ptr, arg0);
    }
    print() {
        wasm.smove_print(this.__wbg_ptr);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
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
    imports.wbg.__wbg_board_new = function(arg0) {
        const ret = Board.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_moveboardpair_new = function(arg0) {
        const ret = MoveBoardPair.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_smove_new = function(arg0) {
        const ret = SMove.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

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
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

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
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('rust_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
