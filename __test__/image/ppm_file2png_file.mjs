import {ImageImpl} from "../../index.js"

const result = ImageImpl.ppmFile2PngFile("test.ppm", "test.png")
console.log(`transformation done | ${result ? 'success' : 'fail'}`)