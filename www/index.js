let context;
let loaded = false;
let worklet;
const canvas = document.getElementById('canvas').getContext('2d');
canvas.fillStyle = 'black';
const keysPressed = Array(16).fill(false);
const keysPressedSet = new Set();
let currentOctave = 0;

const init = async context => {
    try {
        await context.audioWorklet.addModule('wasm-worklet-processor.js');
        worklet = new AudioWorkletNode(context, 'wasm-worklet-processor');
        const masterVolumeParam = worklet.parameters.get('master');

        worklet.connect(context.destination);

        canvas.beginPath();
        canvas.moveTo(0, 0);
        canvas.lineTo(50, 50);
        canvas.lineTo(20, 70);
        canvas.stroke();

        await fetch('/pkg/web_synth_bg.wasm')
            .then(r => r.arrayBuffer())
            .then(r => {
                worklet.port.postMessage({ type: 'load', data: r });
                loaded = true;
            });

        document.getElementById('lowerOctave').onclick = () => {
            if (currentOctave > 0) {
                currentOctave--;
                worklet.port.postMessage({ type: 'octave', octave: currentOctave });
            }
        };

        document.getElementById('raiseOctave').onclick = () => {
            if (currentOctave < 8) {
                currentOctave++;
                worklet.port.postMessage({ type: 'octave', octave: currentOctave });
            }
        };

        document.getElementById('master-slider').oninput = function () {
            const volume = this.value / 100.0;
            masterVolumeParam.setValueAtTime(volume, context.currentTime);
        }

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

// const keyLayout = 'zsxcfvgbnjmk,l./';
const keyLayout = 'zsxdcvgbhnjm,l.;';

function getKeyIndex(key) {
    return keyLayout.indexOf(key);
}

document.onkeydown = e => {
    if (keyLayout.includes(e.key)) {
        keysPressed[getKeyIndex(e.key)] = true;
        worklet.port.postMessage({ type: 'keys', keysPressed });
        document.querySelectorAll('li')
            .forEach(k => {
                if (k.firstElementChild.innerHTML === e.key.toUpperCase()) {
                    // console.log(`found ${e.key.toUpperCase()}`);
                    k.classList.add('key-pressed');
                }
            });
    }
};

document.onkeyup = e => {
    if (keyLayout.includes(e.key)) {
        keysPressed[getKeyIndex(e.key)] = false;
        worklet.port.postMessage({ type: 'keys', keysPressed });
        document.querySelectorAll('li')
            .forEach(k => {
                if (k.firstElementChild.innerHTML === e.key.toUpperCase()) {
                    // console.log(`found ${e.key.toUpperCase()}`);
                    k.classList.remove('key-pressed');
                }
            });
    }
};

document.querySelectorAll('li')
    .forEach(k => {
        const key = k.firstElementChild.innerHTML.toLowerCase();
        k.onmousedown = k.ontouchstart = () => {
            if (keyLayout.includes(key)) {
                keysPressed[getKeyIndex(key)] = true;
                worklet.port.postMessage({ type: 'keys', keysPressed });
            }
        };
        k.onmouseup = k.onmouseout = k.ontouchend = () => {
            if (keyLayout.includes(key)) {
                keysPressed[getKeyIndex(key)] = false;
                worklet.port.postMessage({ type: 'keys', keysPressed });
            }
        };
    });

/*
function raiseOctave() {
    if (currentOctave < 8) {
        currentOctave++;
        worklet.port.postMessage({ type: 'octave', octave: currentOctave });
    }
}

function lowerOctave() {
    if (currentOctave > 0) {
        currentOctave--;
        worklet.port.postMessage({ type: 'octave', octave: currentOctave });
    }
}

document.querySelector('#lowerOctave').onclick = lowerOctave;
document.querySelector('#raiseOctave').onclick = raiseOctave;*/
