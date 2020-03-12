let context;
let loaded = false;
let worklet;
const canvas = document.getElementById('canvas').getContext('2d');
canvas.fillStyle = 'black';
const keysPressed = new Set();

const init = async context => {
    try {
        await context.audioWorklet.addModule('wasm-worklet-processor.js');
        worklet = new AudioWorkletNode(context, 'wasm-worklet-processor');
        const input = context.createOscillator();
        input.type = 'sine';
        input.frequency.setValueAtTime(100, context.currentTime);

        input.connect(worklet).connect(context.destination);
        // input.connect(context.destination);
        // input.start();
        // worklet.connect(context.destination);
        // loaded = true;

        /*worklet.port.onmessage = function (event) {
            canvas.beginPath();
            canvas.moveTo(0, 0);
            let x = 0;
            for (const s of event.data.samples) {
                canvas.lineTo(x, s);
                x += 1 / 44100;
            }
            canvas.stroke();
        };*/

        canvas.beginPath();
        canvas.moveTo(0, 0);
        canvas.lineTo(50, 50);
        canvas.lineTo(20, 70);
        canvas.stroke();

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

window.onkeypress = function (event) {
    if (event.key === 'q') {
        worklet.port.postMessage({ type: 'higher' });
    } else if (event.key === 'a') {
        worklet.port.postMessage({ type: 'lower' });
    } else if (event.key === 'e') {
        worklet.port.postMessage({ type: 'louder' });
    } else if (event.key === 'd') {
        worklet.port.postMessage({ type: 'quieter' });
    } else if (event.key === 'c') {
        worklet.port.postMessage({ type: 'toggleDistortion' });
    } else if (event.key === 'v') {
        worklet.port.postMessage({ type: 'toggleFuzz' });
    } else if (event.key === 'i') {
        worklet.port.postMessage({ type: 'increaseFuzz' });
    } else if (event.key === 'k') {
        worklet.port.postMessage({ type: 'decreaseFuzz' });
    } else if (event.key === 'o') {
        worklet.port.postMessage({ type: 'increaseMix' });
    } else if (event.key === 'l') {
        worklet.port.postMessage({ type: 'decreaseMix' });
    }
};

document.onkeydown = e => {
    keysPressed.add(e.key);
    console.log(keysPressed);
};

document.onkeyup = e => {
    keysPressed.delete(e.key);
    console.log(keysPressed);
};
