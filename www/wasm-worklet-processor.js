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
        console.log(this.memory);

        this.keyboard = KeyboardSynthesizer.new();
        this.samplesPtr = this.keyboard.get_ptr();
        this.keysPtr = this.keyboard.get_keys_ptr();
    }

    process(inputs, outputs) {
        let input = inputs[0];
        let output = outputs[0];
        let channelCount = input.length;

        const keyboardArray = new Uint8Array(this.memory.buffer, this.keysPtr, 16);
        keyboardArray.set(this.keysPressed);
        // console.log(keyboardArray);
        this.keyboard.process();
        const samples = new Float32Array(this.memory.buffer, this.samplesPtr, 128);
        // console.log(samples);
        output[0].set(samples);
        return true;
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);

