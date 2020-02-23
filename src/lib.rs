mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {

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

    pub fn process(&self) -> *const f32 {
        // do processing here
        self.samples.as_ptr()
    }
}
