import {ImageImpl} from "../../index.js"

const result = ImageImpl.ppmFile2PngFile("test.ppm", "ppmFile2PngFile.trans.png")
console.log(`transformation done | ${result ? 'success' : 'fail'}`)