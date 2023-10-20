import init, { wasm_main } from "./pkg/hello_wasm.js";
    init().then(() => {
    console.log('============MAIN============');
    console.log(wasm_main());
});