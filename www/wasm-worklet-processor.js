import init, { Oscillator } from '/pkg/web_synth.js';

export class WasmWorkletProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.baseFreq = 440.0;
        this.volume = 0.5;
        this.initMessagePort();
    }

    initMessagePort() {
        this.port.onmessage = e => {
            if (e.data.type === 'load') {
                this.initWasm(e.data.data)
                    .then(() => console.log('loaded wasm!'));
            }
            if (e.data.type === 'higher') {
                this.baseFreq *= 2;
            }
            if (e.data.type === 'lower') {
                this.baseFreq /= 2;
            }
            if (e.data.type === 'louder') {
                this.volume += 0.1;
            }
            if (e.data.type === 'quieter') {
                this.volume -= 0.1;
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
        const ptr = this.oscillator.process(currentTime, this.baseFreq, this.volume);
        const samples = new Float32Array(this.memory.buffer, ptr, 128);
        console.log(samples);
        output[0].set(samples);
        return true;
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);

