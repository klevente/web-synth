let context;
let loaded = false;
let worklet;
const keysPressed = Array(17).fill(false);
let currentOctave = 0;

const updateOctaveDisplay = () => {
    const display = document.querySelector('.octave-display');
    display.innerHTML = display.innerHTML.replace(/\d/, currentOctave.toString());
};

const init = async context => {
    try {
        await context.audioWorklet.addModule('wasm-worklet-processor.js');
        worklet = new AudioWorkletNode(context, 'wasm-worklet-processor');
        const masterVolumeParam = worklet.parameters.get('master');

        worklet.connect(context.destination);

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
                updateOctaveDisplay();
            }
        };

        document.getElementById('raiseOctave').onclick = () => {
            if (currentOctave < 8) {
                currentOctave++;
                worklet.port.postMessage({ type: 'octave', octave: currentOctave });
                updateOctaveDisplay();
            }
        };

        document.getElementById('master-slider').oninput = function () {
            const volume = this.value / 100.0;
            masterVolumeParam.setValueAtTime(volume, context.currentTime);
        }

        setButtonListeners();

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
        // .then(() => console.log('context resumed'));
    }
};

// const keyLayout = 'zsxcfvgbnjmk,l./';
const keyLayout = 'zsxdcvgbhnjm,l.;/';

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


let beats = 4;
let subbeats = 4;
let tempo = 90.0;

const sequencerContainer = document.querySelector('.sequencer-container');
let numOfChannels = 0;

// const noteTemplate = () => `<button class="button-note"></button>`;
const noteTemplate = () => `<button class="button-note btn btn-secondary"></button>`;
const noteDividerTemplate = () => `<div class="note-divider"></div>`;

const channelTemplateStart = () => `
    <div class="sequencer-channel card card-body">
        <h5 class="channel-number text-monospace">${numOfChannels++}</h5>
        <button class="remove-button btn btn-danger">Remove</button>
        <select class="instrument-select form-control">
            <option value="kickdrum">Kick Drum</option>
            <option value="snaredrum">Snare Drum</option>
            <option value="hihat">Hihat</option>
        </select>`; // <- notes come here below

const channelTemplateEnd = () => `</div>`;

const getInputValue = id => document.querySelector(`#${id}`).value;

function updateData() {
    beats = getInputValue('beats');
    subbeats = getInputValue('subbeats')
    tempo = getInputValue('tempo');

    // TODO add validation

    worklet.port.postMessage({ type: 'update_global_data', beats, subbeats, tempo });
    document.querySelectorAll('.sequencer-channel').forEach(channel => channel.remove());
    numOfChannels = 0;
}

function updateChannelNumbers(deletedItem = 0) {
    for (let i = deletedItem; i < sequencerContainer.children.length; i++) {
        sequencerContainer.children.item(i).firstElementChild.innerHTML = i.toString();
    }
}

function updateChannelPattern(channelIndex) {
    const channelPattern = Array.from(sequencerContainer.children.item(channelIndex).children)
        .filter(child => child.classList.contains('button-note'))
        .map(note => {
                let mappedNote = note.classList
                    .toString()
                    .replace('btn', '')
                    .replace('button-note', '')
                    .replace('btn-secondary', '')
                    .trim();
                if (mappedNote === 'btn-primary') {
                    mappedNote = 'x';
                }
                if (mappedNote === '') {
                    mappedNote = '.';
                }
                return mappedNote;
            }
        ).join('');
    //console.log(channelIndex, channelPattern);
    worklet.port.postMessage({ type: 'update_pattern', index: channelIndex, pattern: channelPattern });
}

const getChannelIndex = currEvent => currEvent.currentTarget.parentNode.firstElementChild.innerHTML;

function setButtonListeners() {
    document.querySelectorAll('.button-note')
        .forEach(button => {
            button.onclick = function (event) {
                /*this.classList.contains('selected')
                    ? this.classList.remove('selected')
                    : this.classList.add('selected');*/
                if (this.classList.contains('btn-primary')) {
                    this.classList.remove('btn-primary');
                    this.classList.add('btn-secondary');
                } else {
                    this.classList.remove('btn-secondary');
                    this.classList.add('btn-primary');
                }

                updateChannelPattern(getChannelIndex(event));
            }
        });

    document.querySelectorAll('.remove-button')
        .forEach(button => {
            button.onclick = function (event) {
                console.log(getChannelIndex(event));
                worklet.port.postMessage({ type: 'remove_channel', index: getChannelIndex(event) })
                event.currentTarget.parentNode.remove();
                numOfChannels--;
                updateChannelNumbers(getChannelIndex(event));
            }
        });

    document.querySelectorAll('.instrument-select')
        .forEach(select => {
            select.onchange = function (event) {
                const channel = getChannelIndex(event);
                console.log(channel);
                worklet.port.postMessage({
                    type: 'update_instrument',
                    index: channel,
                    instrument: event.currentTarget.value
                });
            }
        });
}

function addChannel() {
    console.log(`adding channel ${numOfChannels}`);
    let channelHtml = channelTemplateStart();
    for (let i = 0; i < beats; i++) {
        for (let j = 0; j < subbeats; j++) {
            channelHtml += noteTemplate();
        }
        if (i < beats - 1) {
            channelHtml += noteDividerTemplate();
        }
    }
    channelHtml += channelTemplateEnd();
    sequencerContainer.insertAdjacentHTML('beforeend', channelHtml);
    setButtonListeners();

    const newPattern = Array(beats * subbeats).fill('.').join('');
    worklet.port.postMessage({ type: 'add_channel', instrument: 'kickdrum', pattern: newPattern })
}

document.querySelector('#update').onclick = updateData;
document.querySelector('#addChannel').onclick = addChannel;