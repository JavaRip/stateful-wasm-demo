import init, { wasm_main } from "./pkg/hello_wasm.js";

await init()
console.log('============MAIN============');
console.log(wasm_main());