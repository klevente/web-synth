wasm-pack build --target web && call :prependTextDecoderImport
Rem && node fix-js-binding.js
goto:eof

:prependTextDecoderImport
@echo off
echo Adding TextDecoder/TextEncoder import to pkg\web_synth.js...
echo import { TextDecoder, TextEncoder } from '/lib/text-encoding.js'; > pkg\temp.js
type pkg\web_synth.js >> pkg\temp.js
type pkg\temp.js > pkg\web_synth.js
del pkg\temp.js
echo Done!
goto:eof