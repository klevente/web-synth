import init, { Oscillator } from '/pkg/web_synth.js';

export class WasmWorkletProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.initMessagePort();
    }

    initMessagePort() {
        this.port.onmessage = e => {
            if (e.data.type === 'load') {
                this.initWasm(e.data.data)
                    .then(() => console.log('loaded wasm!'));
            }
        };
    }

    async initWasm(wasmBinary) {
        this.wasm = await init(wasmBinary);
        this.memory = this.wasm.memory;
        console.log(this.memory);
        this.oscillator = Oscillator.new();
    }

    process(inputs, outputs) {
        let input = inputs[0];
        let output = outputs[0];
        let channelCount = input.length;
        // const ptr = this.oscillator.process();
        // const samples = new Float32Array(memory.buffer, ptr, 128);
        // output[0].set(samples);
        // output[0] = input[0].map(sample => sample * 0.1);
        return true;
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);

