import init, { Oscillator, Synthethizer } from '/pkg/web_synth.js';

export class WasmWorkletProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.noteId = 64;
        // this.baseFreq = 440.0;
        this.baseFreq = this.scale(this.noteId);
        this.volume = 0.2;
        this.distort = false;
        this.fuzz = false;
        this.gain = 0.5;
        this.mix = 0.5;
        this.initMessagePort();
    }

    initMessagePort() {
        this.port.onmessage = e => {
            if (e.data.type === 'load') {
                this.initWasm(e.data.data)
                    .then(() => console.log('loaded wasm!'));
            }
            if (e.data.type === 'higher') {
                this.noteId++;
                this.baseFreq = this.scale(this.noteId);
            }
            if (e.data.type === 'lower') {
                this.noteId--;
                this.baseFreq = this.scale(this.noteId);
            }
            if (e.data.type === 'louder') {
                this.volume += 0.05;
            }
            if (e.data.type === 'quieter') {
                this.volume -= 0.05;
            }
            if (e.data.type === 'toggleDistortion') {
                this.distort = !this.distort;
            }
            if (e.data.type === 'toggleFuzz') {
                this.fuzz = !this.fuzz;
            }
            if (e.data.type === 'increaseFuzz') {
                this.gain += 0.1;
            }
            if (e.data.type === 'decreaseFuzz') {
                this.gain -= 0.1;
            }
            if (e.data.type === 'increaseMix') {
                this.mix += 0.1;
            }
            if (e.data.type === 'decreaseMix') {
                this.mix -= 0.1;
            }
            this.oscillator.set_fuzz_params(this.gain, this.mix);
        };
    }

    async initWasm(wasmBinary) {
        this.wasm = await init(wasmBinary);
        this.memory = this.wasm.memory;
        console.log(this.memory);
        this.oscillator = Oscillator.new();
        this.outSamples = this.oscillator.get_ptr();
        this.oscillator.set_fuzz_params(this.gain, this.mix);

        this.synthetizer = Synthethizer.new();
        this.outSynthSamples = this.synthetizer.get_ptr();
    }

    scale(noteId) {
        const freq = 8 * Math.pow(1.0594630943592952645618252949463, noteId);
        console.log(freq);
        return freq;
    }

    process(inputs, outputs) {
        let input = inputs[0];
        let output = outputs[0];
        let channelCount = input.length;
        /*this.oscillator.process(currentTime, this.baseFreq, this.volume);
        if (this.distort) {
            this.oscillator.distort();
        }
        if (this.fuzz) {
            this.oscillator.fuzz();
        }
        const samples = new Float32Array(this.memory.buffer, this.outSamples, 128);*/
        // console.log(samples);
        this.synthetizer.process();
        const samples = new Float32Array(this.memory.buffer, this.outSynthSamples, 128);
        output[0].set(samples);
        return true;
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);

