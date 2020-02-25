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
    output: [f32; 128]
}

#[wasm_bindgen]
impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator {
            output: [0.0; 128]
        }
    }

    pub fn process(&self) -> *const f32 {
        // do processing here
        self.samples.as_ptr()
    }
}

#[wasm_bindgen]
pub struct Effect {
    input: [f32; 128],
    output: [f32; 128]
}

#[wasm_bindgen]
impl Effect {
    pub fn new() -> Effect {
        Effect {
            input: [0.0; 128],
            output: [0.0; 128]
        }
    }

    pub fn process(&mut self) {
        for (i, elem) in self.input.iter().enumerate() {
            self.output[i] = *elem;
        }
    }

    pub fn get_input(&mut self) -> *mut f32 {
        self.input.as_mut_ptr()
    }

    pub fn get_output(&self) -> *const f32 {
        self.input.as_ptr()
    }
}
