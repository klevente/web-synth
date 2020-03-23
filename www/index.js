let context;
let loaded = false;
let worklet;
const canvas = document.getElementById('canvas').getContext('2d');
canvas.fillStyle = 'black';
const keysPressed = Array(16).fill(false);

const init = async context => {
    try {
        await context.audioWorklet.addModule('wasm-worklet-processor.js');
        worklet = new AudioWorkletNode(context, 'wasm-worklet-processor');

        worklet.connect(context.destination);

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

        const t = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        const copied = Object.assign({}, t);
        console.log(copied);
        String.fromCodePoint()

        fetch('/pkg/web_synth_bg.wasm')
            .then(r => r.arrayBuffer())
            .then(r => {
                worklet.port.postMessage({ type: 'load', data: r, textDecoder: JSON.stringify(TextDecoder) });
                // worklet.start();
                loaded = true;
            });
    } catch (e) {
        console.error(e);
    }
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

const keyLayout = 'zsxcfvgbnjmk,l./';

function getKeyIndex(key) {
    return keyLayout.indexOf(key);
}

document.onkeydown = e => {
    if (keyLayout.includes(e.key)) {
        keysPressed[getKeyIndex(e.key)] = true;
        worklet.port.postMessage({ type: 'keys', keysPressed });
    }
};

document.onkeyup = e => {
    if (keyLayout.includes(e.key)) {
        keysPressed[getKeyIndex(e.key)] = false;
        worklet.port.postMessage({ type: 'keys', keysPressed });
    }
};
