mod utils;
mod web_synth;

use wasm_bindgen::prelude::*;
use std::f32::consts::PI;
use std::f32::INFINITY;
use crate::web_synth::{Source, SAMPLE_SIZE, MutSource};
use crate::web_synth::SAMPLE_RATE;
use crate::web_synth::oscillators::{SineOscillator, SquareOscillator};
use crate::web_synth::keyboard::Keyboard;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct KeyboardSynthesizer {
    out_samples: [f32; 128],
    keyboard: Keyboard,
    source: SineOscillator,
    sin_time: f32,
    sin_delta_time: f32
}

#[wasm_bindgen]
impl KeyboardSynthesizer {

    pub fn new() -> KeyboardSynthesizer {
        KeyboardSynthesizer {
            out_samples: [0.0; 128],
            keyboard: Keyboard::new(),
            source: SineOscillator::new(440.0, 0.01, 5.0),
            sin_time: 0.0,
            sin_delta_time: 1.0 / SAMPLE_RATE,
        }
    }

    pub fn get_ptr(&self) -> *const f32 {
        self.out_samples.as_ptr()
    }

    pub fn get_keys_ptr(&self) -> *const bool {
        self.keyboard.get_keys_ptr()
    }

    pub fn process(&mut self) {
        self.keyboard.update_notes(self.sin_time);
        let s1 = self.keyboard.get_sample_block(self.sin_time);
        // let s2 = self.source.get_sample_block(self.sin_time);
        self.sin_time += self.sin_delta_time * SAMPLE_SIZE as f32;
        self.out_samples = [0.0; 128];
        for i in 0..SAMPLE_SIZE {
            self.out_samples[i] += s1[i];
        }
    }
}

#[wasm_bindgen]
pub struct Synthesizer {
    out_samples: [f32; 128],
    source1: Box<dyn Source>,
    source2: Box<dyn Source>,
    sources: Vec<Box<dyn Source>>,

    sin_time: f32,
    sin_delta_time: f32,
    sample_count: u32
}

#[wasm_bindgen]
impl Synthesizer {

    pub fn new() -> Synthesizer {
        Synthesizer {
            out_samples: [0.0; 128],
            source1: Box::new(SineOscillator::new(440.0, 0.01, 5.0)),
            source2: Box::new(SquareOscillator::new(440.0, 0.0, 0.0)),
            sources: vec![Box::new(SineOscillator::new(440.0, 0.01, 5.0)), Box::new(SquareOscillator::new(440.0, 0.0, 0.0))],

            sin_time: 0.0,
            sin_delta_time: 1.0 / SAMPLE_RATE,
            sample_count: 0
        }
    }

    pub fn get_ptr(&self) -> *const f32 {
        self.out_samples.as_ptr()
    }

    pub fn process(&mut self) {
        let s1 = self.source1.get_sample_block(self.sin_time);
        // let s2 = self.source2.get_sample_block(self.sin_time);
        let s2 = [0.0; 128];
        self.sample_count += SAMPLE_SIZE as u32;
        self.sin_time += self.sin_delta_time * SAMPLE_SIZE as f32;
        for i in 0..SAMPLE_SIZE {
            self.out_samples[i] = s1[i] + s2[i];
        }

        // TODO: refactor to only use vectored version
        self.out_samples = [0.0; 128];
        for s in self.sources.iter() {
            let samp = s.get_sample_block(self.sin_time);
            for i in 0..SAMPLE_SIZE {
                self.out_samples[i] += samp[i];
            }
        }
        self.out_samples = [0.0; 128];
    }
}

#[wasm_bindgen]
pub struct Oscillator {
    samples: [f32; 128],
    sin_time: f32,
    sin_delta_time: f32,
    sample_count: u32,

    gain: f32,
    mix: f32
}

impl Oscillator {

    fn synth(&mut self, time: f32, offset: f32, base_freq: f32, amp: f32) -> f32 {
        // amp * (base_freq * 2.0 * PI * (time + offset / 44100.0)).sin()
        /*if offset < 64.0 {
            return 0.0;
        }
        1.0*/
        let out = amp * (base_freq * 2.0 * PI * self.sin_delta_time * self.sample_count as f32).sin();
        self.sample_count += 1;
        self.sin_time += self.sin_delta_time;

        out
    }
}

#[wasm_bindgen]
impl Oscillator {

    pub fn new() -> Oscillator {
        Oscillator {
            samples: [0.0; 128],
            sin_time: 0.0,
            sin_delta_time: 1.0 / 44100.0,
            sample_count: 0,
            gain: 0.0,
            mix: 0.0
        }
    }

    pub fn get_ptr(&self) -> *const f32 {
        self.samples.as_ptr()
    }

    pub fn process(&mut self, time: f32, base_freq: f32, amp: f32) {
        // do processing here
        for i in 0..128 {
            self.samples[i] = self.synth(time, i as f32, base_freq, amp);
        }
    }

    pub fn distort(&mut self) {
        let threshold: f32 = 1.0/3.0;

        for i in 0..128 {
            let x = self.samples[i];
            if x.abs() < threshold {
                self.samples[i] *= 2.0;
            } else if x.abs() < 2.0 * threshold {
                if x > 0.0 {
                    self.samples[i] = (3.0 - (2.0 - 3.0 * x) * (2.0 - 3.0 * x)) / 3.0;
                } else {
                    self.samples[i] = -(3.0 - (2.0 - 3.0 * x.abs()) * (2.0 - 3.0 * x.abs())) / 3.0;
                }
            } else {
                self.samples[i] = if x > 0.0 { 1.0 } else { -1.0 };
            }
        }
    }

    pub fn set_fuzz_params(&mut self, gain: f32, mix: f32) {
        self.gain = gain;
        self.mix = mix;
    }

    pub fn fuzz(&mut self) {
        let x = &self.samples;

        let max_abs_x = x.iter().fold(0.0, |a: f32, &b| a.abs().max(b.abs()));

        let mut z: [f32; 128] = [0.0; 128];
        for (i, x_i) in x.iter().enumerate() {
            let q = x[i] * self.gain / max_abs_x;
            let q_sign = (-1.0 * q).signum();
            z[i] = q_sign * (1.0 - (q_sign * q).exp());
        }

        let max_abs_z = z.iter().fold(0.0, |a: f32, &b| a.abs().max(b.abs()));
        let mut y: [f32; 128] = [0.0; 128];
        for (i, z_i) in z.iter().enumerate() {
            y[i] = self.mix * z[i] * max_abs_x / max_abs_z + (1.0 - self.mix) * x[i];
        }

        let max_abs_y = y.iter().fold(0.0, |a: f32, &b| a.abs().max(b.abs()));

        let mut out: [f32; 128] = [0.0; 128];
        for (i, y_i) in y.iter().enumerate() {
            out[i] = y[i] * max_abs_x / max_abs_y;
        }

        self.samples = out;
    }
}
