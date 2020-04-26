const utf8Encodings = [
    'utf8',
    'utf-8',
    'unicode-1-1-utf-8'
];

export class TextEncoder {
    constructor(encoding) {
        if (utf8Encodings.indexOf(encoding) < 0 && typeof encoding !== 'undefined' && encoding !== null) {
            throw new RangeError('Invalid encoding type. Only utf-8 is supported');
        }
        this.encoding = 'utf-8';
    }
        encode(str) {
            if (typeof str !== 'string') {
                throw new TypeError('passed argument must be of type string');
            }
            const binStr = unescape(encodeURIComponent(str)),
                arr = new Uint8Array(binStr.length);
            binStr.split('').forEach(function(char, i) {
                arr[i] = char.charCodeAt(0);
            });
            return arr;
        };
}

export class TextDecoder {
    constructor(encoding, options) {
        if (utf8Encodings.indexOf(encoding) < 0 && typeof encoding !== 'undefined' && encoding !== null) {
            throw new RangeError('Invalid encoding type. Only utf-8 is supported');
        }
        this.encoding = 'utf-8';
        this.ignoreBOM = false;
        this.fatal = (typeof options !== 'undefined' && 'fatal' in options) ? options.fatal : false;
        if (typeof this.fatal !== 'boolean') {
            throw new TypeError('fatal flag must be boolean');
        }
    }

    decode(view, options) {
        if (typeof view === 'undefined') {
            return '';
        }
        const stream = (typeof options !== 'undefined' && 'stream' in options) ? options.stream : false;
        if (typeof stream !== 'boolean') {
            throw new TypeError('stream option must be boolean');
        } else {
            const arr = new Uint8Array(view.buffer, view.byteOffset, view.byteLength),
                charArr = new Array(arr.length);
            arr.forEach(function (charCode, i) {
                charArr[i] = String.fromCharCode(charCode);
            });
            return decodeURIComponent(escape(charArr.join('')));
        }
    }

}
