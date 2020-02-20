let context;

const init = async context => {
    await context.audioWorklet.addModule('wasm-worklet-processor.js');
    const bypasser = new AudioWorkletNode(context, 'wasm-worklet-processor');
    bypasser.connect(context.destination);
};

window.onload = function () {
    context = new AudioContext();
    init(context);
};

window.onclick = function () {
    context.resume()
        .then(() => console.log('context resumed'));
};
