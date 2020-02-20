import { Oscillator } from './node_modules/web-synth';
import { memory } from './node_modules/web-synth/web_synth_bg';

export const filename = 'wasm-worklet-processor.js';

export class WasmWorkletProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        console.log('constructing');
        this.oscillator = Oscillator.new();
        console.log('constructed');
    }

    process(inputs, outputs, parameters) {
        let input = inputs[0];
        let output = outputs[0];
        let channelCount = input.length;
        const ptr = this.oscillator.process();
        const samples = new Float32Array(memory.buffer, ptr, 128);
        output[0].set(samples);
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);