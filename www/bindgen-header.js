
let wasm;

// let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

// cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    // return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    return getUint8Memory0().subarray(ptr, ptr + len);
}