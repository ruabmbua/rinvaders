(function() {
    var wasm;
    const __exports = {};


    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    let cachedTextEncoder = new TextEncoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    let WASM_VECTOR_LEN = 0;

    function passStringToWasm(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }
    /**
    * Test exported rust function (to wasm module).
    * @param {string} arg0
    * @returns {void}
    */
    __exports.hello = function(arg0) {
        const ptr0 = passStringToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        try {
            return wasm.hello(ptr0, len0);

        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 1);

        }

    };

function getObject(idx) { return heap[idx]; }

__exports.__widl_instanceof_CanvasRenderingContext2D = function(idx) {
    return getObject(idx) instanceof CanvasRenderingContext2D ? 1 : 0;
};

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
    while (obj) {
        let desc = Object.getOwnPropertyDescriptor(obj, id);
        if (desc) return desc;
        obj = Object.getPrototypeOf(obj);
    }
return {}
}

const __widl_f_set_fill_style_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'fillStyle').set || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillStyle does not exist`);
};

__exports.__widl_f_set_fill_style_CanvasRenderingContext2D = function(arg0, arg1) {
    __widl_f_set_fill_style_CanvasRenderingContext2D_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_clear_rect_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.clearRect || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.clearRect does not exist`);
};

__exports.__widl_f_clear_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
    __widl_f_clear_rect_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2, arg3, arg4);
};

const __widl_f_fill_rect_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.fillRect || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillRect does not exist`);
};

__exports.__widl_f_fill_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
    __widl_f_fill_rect_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2, arg3, arg4);
};

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

const __widl_f_fill_text_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.fillText || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillText does not exist`);
};

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

__exports.__widl_f_fill_text_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        __widl_f_fill_text_CanvasRenderingContext2D_target.call(getObject(arg0), varg1, arg3, arg4);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_set_font_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'font').set || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.font does not exist`);
};

__exports.__widl_f_set_font_CanvasRenderingContext2D = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_font_CanvasRenderingContext2D_target.call(getObject(arg0), varg1);
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

const __widl_f_get_context_HTMLCanvasElement_target = typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype.getContext || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.getContext does not exist`);
};

__exports.__widl_f_get_context_HTMLCanvasElement = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = __widl_f_get_context_HTMLCanvasElement_target.call(getObject(arg0), varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_width_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'width').get || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.width does not exist`);
};

__exports.__widl_f_width_HTMLCanvasElement = function(arg0) {
    return __widl_f_width_HTMLCanvasElement_target.call(getObject(arg0));
};

const __widl_f_height_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'height').get || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.height does not exist`);
};

__exports.__widl_f_height_HTMLCanvasElement = function(arg0) {
    return __widl_f_height_HTMLCanvasElement_target.call(getObject(arg0));
};

const __widl_f_key_KeyboardEvent_target = GetOwnOrInheritedPropertyDescriptor(typeof KeyboardEvent === 'undefined' ? null : KeyboardEvent.prototype, 'key').get || function() {
    throw new Error(`wasm-bindgen: KeyboardEvent.key does not exist`);
};

__exports.__widl_f_key_KeyboardEvent = function(ret, arg0) {

    const retptr = passStringToWasm(__widl_f_key_KeyboardEvent_target.call(getObject(arg0)));
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_log_1__target = console.log;

__exports.__widl_f_log_1_ = function(arg0) {
    __widl_f_log_1__target(getObject(arg0));
};

__exports.__wbg_random_2cc0c8d054a5c72a = function() {
    return Math.random();
};

function freeGame(ptr) {

    wasm.__wbg_game_free(ptr);
}
/**
* Main type that holds all game state and provides functionality to update and
* render the game.
*/
class Game {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeGame(ptr);
    }

    /**
    * Create an instance of the **Game** type. Requires *canvas* parameter which should be the DOM
    * node of the canvas, into which the game should be rendered.
    *
    * Initializes the game.
    * @param {any} arg0
    * @returns {}
    */
    constructor(arg0) {
        this.ptr = wasm.game_new(addHeapObject(arg0));
    }
    /**
    * Forwards keyboard events to the **input** module.
    * @param {boolean} arg0
    * @param {any} arg1
    * @returns {void}
    */
    keyboard_event(arg0, arg1) {
        return wasm.game_keyboard_event(this.ptr, arg0, addHeapObject(arg1));
    }
    /**
    * Render the game. This will clear the canvas, and then redraw the whole scene.
    * Does not need mutable access, because it immutably reads game date, and draws
    * to the canvas. Canvas draw operations have some kind of interior mutability,
    * which allows usage through non mutable references.
    * @returns {void}
    */
    render() {
        return wasm.game_render(this.ptr);
    }
    /**
    * Update the game.
    *
    * This processes all input, and calculates the next state of the game depending on
    * previous state, current input, and the current timestamp.
    * @param {number} arg0
    * @returns {void}
    */
    update(arg0) {
        return wasm.game_update(this.ptr, arg0);
    }
    /**
    * Set the current gamepad state. Axis movement and shoot.
    *
    * This will forward the input to the **input** module.
    * @param {boolean} arg0
    * @param {boolean} arg1
    * @param {boolean} arg2
    * @returns {void}
    */
    set_gamepad_state(arg0, arg1, arg2) {
        return wasm.game_set_gamepad_state(this.ptr, arg0, arg1, arg2);
    }
}
__exports.Game = Game;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

__exports.__wbindgen_string_new = function(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
};

__exports.__wbindgen_number_get = function(n, invalid) {
    let obj = getObject(n);
    if (typeof(obj) === 'number') return obj;
    getUint8Memory()[invalid] = 1;
    return 0;
};

__exports.__wbindgen_is_null = function(idx) {
    return getObject(idx) === null ? 1 : 0;
};

__exports.__wbindgen_is_undefined = function(idx) {
    return getObject(idx) === undefined ? 1 : 0;
};

__exports.__wbindgen_boolean_get = function(i) {
    let v = getObject(i);
    if (typeof(v) === 'boolean') {
        return v ? 1 : 0;
    } else {
        return 2;
    }
};

__exports.__wbindgen_is_symbol = function(i) {
    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
};

__exports.__wbindgen_string_get = function(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string') return 0;
    const ptr = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
    return ptr;
};

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

function init(path_or_module) {
    let instantiation;
    const imports = { './rinvaders': __exports };
    if (path_or_module instanceof WebAssembly.Module) {
        instantiation = WebAssembly.instantiate(path_or_module, imports)
        .then(instance => {
        return { instance, module: path_or_module }
    });
} else {
    const data = fetch(path_or_module);
    if (typeof WebAssembly.instantiateStreaming === 'function') {
        instantiation = WebAssembly.instantiateStreaming(data, imports);
    } else {
        instantiation = data
        .then(response => response.arrayBuffer())
        .then(buffer => WebAssembly.instantiate(buffer, imports));
    }
}
return instantiation.then(({instance}) => {
    wasm = init.wasm = instance.exports;

});
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
