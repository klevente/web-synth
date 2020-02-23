// import { Oscillator } from './node_modules/web-synth';
// import { memory } from './node_modules/web-synth/web_synth_bg';


export class WasmWorkletProcessor extends AudioWorkletProcessor {
    constructor() {
        super();

        this.initWasm();
        /*his.port.onmessage = e => {
            if (e.data.type === 'load') {
                WebAssembly.instantiate(e.data.data, {
                    module: websynth
                }).then(w => {
                    this.wasm = w.instance;
                    this.o = require('./node_modules/web-synth');
                    this.memory = this.wasm.exports.memory;
                    this.oscillator = this.o.Oscillator.new();
                })
            }
        };*/
    }

    async initWasm() {
        // await init();
    }

    process(inputs, outputs) {
        let input = inputs[0];
        let output = outputs[0];
        let channelCount = input.length;
        const ptr = this.oscillator.process();
        const samples = new Float32Array(memory.buffer, ptr, 128);
        output[0].set(samples);
        return true;
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);

