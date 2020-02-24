let context;
let loaded = false;

const init = async context => {
    try {
        await context.audioWorklet.addModule('wasm-worklet-processor.js');
        const worklet = new AudioWorkletNode(context, 'wasm-worklet-processor');
        const input = context.createOscillator();
        input.type = 'sine';
        input.frequency.setValueAtTime(100, context.currentTime);

        input.connect(worklet).connect(context.destination);
        // input.connect(context.destination);
        // input.start();
        // worklet.connect(context.destination);
        // loaded = true;


        fetch('/pkg/web_synth_bg.wasm')
            .then(r => r.arrayBuffer())
            .then(r => {
                worklet.port.postMessage({ type: 'load', data: r });
                input.start();
                loaded = true;
            });
    } catch (e) {
        console.error(e);
    }

    // const oscillator = context.createOscillator();
    // oscillator.type = 'sine';
    // oscillator.frequency.setValueAtTime(440, context.currentTime);
    // oscillator.connect(context.destination);
};

window.onload = function () {
    context = new AudioContext();
    init(context)
        .then(() => console.log('worklet init complete!'))
};

window.onclick = function () {
    if (loaded) {
        context.resume()
            .then(() => console.log('context resumed'));
    }
};
