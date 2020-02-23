
let context;
let loaded = false;

const init = async context => {
    // await import('./wasm-worklet-processor');
    // require('./wasm-worklet-processor');
    // await context.audioWorklet.addModule('wasm-worklet-processor.js');
    try {
        // console.log(wasmWorklet);
        await context.audioWorklet.addModule('./wasm-worklet-processor.js');
        const worklet = new AudioWorkletNode(context, 'wasm-worklet-processor');
        worklet.connect(context.destination);
    } catch (e) {
        console.error(e);
    }

    /*fetch('web_synth_bg.wasm')
        .then(r => r.arrayBuffer())
        .then(r => {
            worklet.port.postMessage({ type: 'load', data: r });
            loaded = true;
        });*/

    // const oscillator = context.createOscillator();
    // oscillator.type = 'sine';
    // oscillator.frequency.setValueAtTime(440, context.currentTime);
    // oscillator.connect(context.destination);
};

window.onload = function () {
    context = new AudioContext();
    init(context);
};

window.onclick = function () {
    if (loaded) {
        context.resume()
            .then(() => console.log('context resumed'));
    }
};
