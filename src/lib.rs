// sources:
// https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html

use wasm_bindgen::prelude::*;

static mut STATE: i32 = 0;

#[wasm_bindgen]
extern {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct IntervalHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for IntervalHandle {
    fn drop(&mut self) {
        clearInterval(self.interval_id);
    }
}

#[wasm_bindgen]
pub fn wasm_main() -> IntervalHandle {
    // First up we use `Closure::new` to wrap up a Rust closure and create
    // a JS closure.
    let cb = Closure::new(|| {
        unsafe { STATE += 1 };
        log(&format!("interval elapsed {}", unsafe { STATE }));
    });

    // Next we pass this via reference to the `setInterval` function, and
    // `setInterval` gets a handle to the corresponding JS closure.
    let interval_id = setInterval(&cb, 1000);

    // If we were to drop `cb` here it would cause an exception to be raised
    // whenever the interval elapses. Instead we *return* our handle back to JS
    // so JS can decide when to cancel the interval and deallocate the closure.
    IntervalHandle {
        interval_id,
        _closure: cb,
    }
}
