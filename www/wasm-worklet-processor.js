import init, { SynthBox } from '/pkg/web_synth.js';

export class WasmWorkletProcessor extends AudioWorkletProcessor {

    static get parameterDescriptors() {
        return [
            {
                name: 'master',
                defaultValue: 0.5,
                minValue: 0,
                maxValue: 1
            }
        ];
    }

    constructor() {
        super();
        this.keysPressed = Array(17).fill(false);
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
                case 'octave':
                    this.synthbox.set_octave(e.data.octave);
                    console.log(e.data.octave);
                    break;
                case 'master':
                    this.synthbox.set_master_volume(e.data.volume);
                    break;
            }
        };
    }

    async initWasm(wasmBinary) {
        this.wasm = await init(wasmBinary);
        this.memory = this.wasm.memory;

        // TODO: set samplerate and samplesize in wasm from js

        this.synthbox = SynthBox.new();
        this.samplesPtr = this.synthbox.get_ptr();
        this.keysPtr = this.synthbox.get_keys_ptr();
        this.masterPtr = this.synthbox.get_master_vol_array_ptr();
        this.samples = new Float64Array(this.memory.buffer, this.samplesPtr, 128);
        this.keys = new Uint8Array(this.memory.buffer, this.keysPtr, 17);
        this.master = new Float64Array(this.memory.buffer, this.masterPtr, 128);

        this.synthbox.add_sequencer_channel("kickdrum",  "x...x...x...x...");
        this.synthbox.add_sequencer_channel("hihat",     "x.x.x.x.x.x.x.x.");
        this.synthbox.add_sequencer_channel("snaredrum", "..x...x...x...x.")
    }

    process(inputs, outputs, parameters) {
        let input = inputs[0];
        let output = outputs[0];
        let channelCount = input.length;

        if (parameters['master'].length > 1) {
            this.master.set(parameters['master']);
        } else {
            this.master.fill(parameters['master'][0], 0, 128);
        }

        this.keys.set(this.keysPressed);
        this.synthbox.process();
        output[0].set(this.samples);

        return true;
    }
}

registerProcessor('wasm-worklet-processor', WasmWorkletProcessor);

