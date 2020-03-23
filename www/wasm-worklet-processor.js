import init, { KeyboardSynthesizer } from '/pkg/web_synth.js';

export class WasmWorkletProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.keysPressed = Array(16).fill(false);
        this.initMessagePort();
    }

    initMessagePort() {
        this.port.onmessage = e => {

            switch (e.data.type) {
                case 'load':
                    this.initWasm(e.data.data).then(() => console.log('loaded wasm!'));
                    break;
                case 'keys':
                    this.keysPressed = e.data.keysPressed;
                    // console.log(this.keysPressed);
                    break;
            }
        };
    }

    async initWasm(wasmBinary) {
        this.wasm = await init(wasmBinary);
        this.memory = this.wasm.memory;

        // TODO: set samplerate and samplesize in wasm from js

        this.keyboard = KeyboardSynthesizer.new();
        this.samplesPtr = this.keyboard.get_ptr();
        this.keysPtr = this.keyboard.get_keys_ptr();
        this.samples = new Float64Array(this.memory.buffer, this.samplesPtr, 128);
        this.keys = new Uint8Array(this.memory.buffer, this.keysPtr, 16);
    }

    process(inputs, outputs) {
        let input = inputs[0];
        let output = outputs[0];
        let channelCount = input.length;

        this.keys.set(this.keysPressed);
        this.keyboard.process();
        output[0].set(this.samples);

        return true;
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);

