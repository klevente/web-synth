mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, web-synth!");
}

#[wasm_bindgen]
pub fn greet_num(num: i32) {
    alert(&format!("Number is {}", num));
}

#[wasm_bindgen]
pub struct Oscillator {
    test: i32,
    samples: [f32; 128]
}

#[wasm_bindgen]
impl Oscillator {

    pub fn new() -> Oscillator {
        Oscillator {
            test: 1,
            samples: [0.0; 128]
        }
    }

    pub fn test(&self) {
        greet_num(self.test);
    }

    pub fn process(&self) -> *const f32 {
        // do processing here
        self.samples.as_ptr()
    }
}
