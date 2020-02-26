mod utils;

use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

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
    samples: [f32; 128]
}

impl Oscillator {

    fn synth(time: f32, offset: f32, base_freq: f32, amp: f32) -> f32 {
        amp * (base_freq * 2.0 * PI * (time + offset / 44100.0)).sin()
        /*if offset < 64.0 {
            return 0.0;
        }
        1.0*/
    }
}

#[wasm_bindgen]
impl Oscillator {

    pub fn new() -> Oscillator {
        Oscillator {
            samples: [0.0; 128]
        }
    }

    pub fn process(&mut self, time: f32, base_freq: f32, amp: f32) -> *const f32 {
        // do processing here
        for i in 0..128 {
            self.samples[i] = Oscillator::synth(time, i as f32, base_freq, amp);
        }
        self.samples.as_ptr()
    }
}
