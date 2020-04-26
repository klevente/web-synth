prependTextDecoderImport() {
  echo "import { TextDecoder, TextEncoder } from '/lib/text-encoding.js';" > pkg/temp.js
  cat pkg/web_synth.js >> pkg/temp.js
  cp pkg/temp.js pkg/web_synth.js
}

wasm-pack build --target web && prependTextDecoderImport