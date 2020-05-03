let beats = 4;
let subbeats = 4;
let tempo = 90.0;

const sequencerContainer = document.querySelector('.sequencer-container');
let numOfChannels = 0;

const noteTemplate = () => `<button class="button-note"></button>`;

const channelTemplateStart = () => `
    <div class="sequencer-channel">
        <div class="channel-number">${numOfChannels++}</div>
        <button class="remove-button">Remove</button>
        <label>
            Instrument:
            <select class="instrument-select">
                <option value="kickdrum">Kick Drum</option>
                <option value="snaredrum">Snare Drum</option>
                <option value="hihat">Hihat</option>
            </select>
        </label>`; // <- notes come here below

const channelTemplateEnd = () => `</div>`;

const getInputValue = id => document.querySelector(`#${id}`).value;

function updateData() {
    beats = getInputValue('beats');
    subbeats = getInputValue('subbeats')
    tempo = getInputValue('tempo');
}

function updateChannelNumbers(deletedItem = 0) {
    for (let i = deletedItem; i < sequencerContainer.children.length; i++) {
        sequencerContainer.children.item(i).firstElementChild.innerHTML = i.toString();
    }
}

function updateChannelPattern(channelIndex) {
    const channelPattern = Array.from(sequencerContainer.children.item(channelIndex).children)
        .filter(child => child.classList.contains('button-note'))
        .map(note => note.classList
            .toString()
            .replace('button-note', '')
            .trim() === 'selected'
        );
    console.log(channelIndex, channelPattern);
}

function setButtonListeners() {
    document.querySelectorAll('.button-note')
        .forEach(button => {
            button.onclick = function (event) {
                this.classList.contains('selected')
                    ? this.classList.remove('selected')
                    : this.classList.add('selected');

                updateChannelPattern(event.currentTarget.parentNode.firstElementChild.innerHTML);
            }
        });

    document.querySelectorAll('.remove-button')
        .forEach(button => {
            button.onclick = function (event) {
                event.currentTarget.parentNode.remove();
                numOfChannels--;
                updateChannelNumbers(event.currentTarget.previousElementSibling.innerHTML);
            }
        });

    document.querySelectorAll('.instrument-select')
        .forEach(select => {
            select.onchange = function (event) {
                console.log(event.currentTarget.value);
            }
        });
}

function addChannel() {
    console.log(`adding channel ${numOfChannels}`);
    let channelHtml = channelTemplateStart();
    for (let i = 0; i < beats * subbeats; i++) {
        channelHtml += noteTemplate();
    }
    channelHtml += channelTemplateEnd();
    sequencerContainer.innerHTML += channelHtml;
    setButtonListeners();
}

document.querySelector('#update').onclick = updateData;
document.querySelector('#addChannel').onclick = addChannel;
setButtonListeners();
addChannel();

