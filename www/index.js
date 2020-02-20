let context;

const init = async context => {
    const filename = (await import('./wasm-worklet-processor')).filename;
    await context.audioWorklet.addModule(filename);
    const bypasser = new AudioWorkletNode(context, 'wasm-worklet-processor');
    bypasser.connect(context.destination);

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
    context.resume()
        .then(() => console.log('context resumed'));
};
