import {ImageImpl} from "../../index.js"
import {writeFileSync} from "node:fs"

const buffer = ImageImpl.ppmFile2PngBuffer("test.ppm")
if (!buffer) {
    console.log('Error: ppmFile2PngBuffer() failed')
} else {
    writeFileSync("ppmFile2PngBuffer.trans.png", Buffer.from(buffer))
}