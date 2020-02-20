import { Oscillator } from 'web-synth';
import { memory } from 'web-synth/web_synth_bg';


class WasmWorkletProcessor extends AudioWorkletProcessor {
    constructor() {
        super();

        this.oscillator = Oscillator.new();
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