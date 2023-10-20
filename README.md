# Stateful WASM demo

This demo shows how state can be kept on the WASM side of browser code.

The Rust code binds to the JS call to SetInterval and passes a function as a closure to SetInterval which increments the global i32 state and prints the current value, once per second

```
============MAIN============ script.js:3:13
Object { __wbg_ptr: 1114120 }
script.js:4:13
interval elapsed 1 hello_wasm.js:147:17
interval elapsed 2 hello_wasm.js:147:17
interval elapsed 3 hello_wasm.js:147:17
interval elapsed 4 hello_wasm.js:147:17
interval elapsed 5 hello_wasm.js:147:17
```