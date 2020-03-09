const fs = require('fs');
const path = require('path');
const readLine = require('readline');

const readInterface = readLine.createInterface({
    input: fs.createReadStream(path.join(__dirname, 'pkg', 'web_synth.js')),
    console: false
});

const writer = fs.createWriteStream(path.join(__dirname, 'pkg', 'test.js'), {
    flags: 'a'
});

console.log('Patching web_synth.js to run in AudioContextGlobalScope...');
readInterface.on('line', line => {
    if (line.match('cachedTextDecoder')) {
        console.log('Found line containing cachedTextEncoder, commenting');
         let commentedLine = '//' + line;
         if (line.match('return')) {
             console.log('Inserting modified return statement');
             commentedLine = commentedLine + '\n'
                 + '    return getUint8Memory0().subarray(ptr, ptr + len);';
         }
         line = commentedLine + '\n';
    }

    writer.write(line);
});

readInterface.on('close', () => {
    writer.close();
    console.log('Done, renaming files...');
    fs.unlinkSync(path.join(__dirname, 'pkg', 'web_synth.js'));
    fs.renameSync(path.join(__dirname, 'pkg', 'test.js'), path.join(__dirname, 'pkg', 'web_synth.js'));
});