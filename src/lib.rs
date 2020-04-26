mod utils;
mod web_synth;

use wasm_bindgen::prelude::*;
use crate::web_synth::{SAMPLE_SIZE, MutSource, DELTA_TIME};
use crate::web_synth::keyboard::Keyboard;
use crate::web_synth::sequencer::{Sequencer, MultiSequencer};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct SynthBox {
    out_samples: [f64; 128],
    keyboard: Keyboard,
    // sequencer: Sequencer,
    sequencer: MultiSequencer,

    sin_time: f64,
    master_volume: f64,
    master_volume_array: [f64; 128]
}

#[wasm_bindgen]
impl SynthBox {

    pub fn new() -> SynthBox {
        SynthBox {
            out_samples: [0.0; 128],
            keyboard: Keyboard::new(),
            // sequencer: Sequencer::new(4, 4, 90.0),
            sequencer: MultiSequencer::new(4, 4, 90.0),

            sin_time: 0.0,
            master_volume: 0.5,
            master_volume_array: [0.5; 128]
        }
    }

    pub fn add_sequencer_channel(&mut self, instrument: &str, pattern: &str) {
        self.sequencer.add_channel(instrument, pattern);
    }

    pub fn get_ptr(&self) -> *const f64 {
        self.out_samples.as_ptr()
    }

    pub fn get_keys_ptr(&self) -> *const bool {
        self.keyboard.get_keys_ptr()
    }

    pub fn get_master_vol_array_ptr(&self) -> *const f64 {
        self.master_volume_array.as_ptr()
    }

    pub fn set_octave(&mut self, new_octave: u32) {
        self.keyboard.set_octave(new_octave);
    }

    pub fn set_master_volume(&mut self, volume: f64) {
        self.master_volume = volume;
    }

    pub fn process(&mut self) {
        self.keyboard.update_notes(self.sin_time);
        let s1 = self.keyboard.get_sample_block(self.sin_time);
        let s2 = self.sequencer.get_sample_block(self.sin_time);
        self.sin_time += DELTA_TIME * SAMPLE_SIZE as f64;
        self.out_samples = [0.0; 128];
        for i in 0..SAMPLE_SIZE {
            self.out_samples[i] += s1[i] + s2[i];
            self.out_samples[i] *= self.master_volume_array[i];
        }
    }
}

/*#[wasm_bindgen]
pub struct Synthesizer {
    out_samples: [f64; 128],
    source1: Box<dyn Source>,
    source2: Box<dyn Source>,
    sources: Vec<Box<dyn Source>>,

    sin_time: f64,
    sin_delta_time: f64,
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

    pub fn get_ptr(&self) -> *const f64 {
        self.out_samples.as_ptr()
    }

    pub fn process(&mut self) {
        let s1 = self.source1.get_sample_block(self.sin_time);
        // let s2 = self.source2.get_sample_block(self.sin_time);
        let s2 = [0.0; 128];
        self.sample_count += SAMPLE_SIZE as u32;
        self.sin_time += self.sin_delta_time * SAMPLE_SIZE as f64;
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
    samples: [f64; 128],
    sin_time: f64,
    sin_delta_time: f64,
    sample_count: u32,

    gain: f64,
    mix: f64
}

impl Oscillator {

    fn synth(&mut self, time: f64, offset: f64, base_freq: f64, amp: f64) -> f64 {
        // amp * (base_freq * 2.0 * PI * (time + offset / 44100.0)).sin()
        /*if offset < 64.0 {
            return 0.0;
        }
        1.0*/
        let out = amp * (base_freq * 2.0 * PI * self.sin_delta_time * self.sample_count as f64).sin();
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

    pub fn get_ptr(&self) -> *const f64 {
        self.samples.as_ptr()
    }

    pub fn process(&mut self, time: f64, base_freq: f64, amp: f64) {
        // do processing here
        for i in 0..128 {
            self.samples[i] = self.synth(time, i as f64, base_freq, amp);
        }
    }

    pub fn distort(&mut self) {
        let threshold: f64 = 1.0/3.0;

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

    pub fn set_fuzz_params(&mut self, gain: f64, mix: f64) {
        self.gain = gain;
        self.mix = mix;
    }

    pub fn fuzz(&mut self) {
        let x = &self.samples;

        let max_abs_x = x.iter().fold(0.0, |a: f64, &b| a.abs().max(b.abs()));

        let mut z: [f64; 128] = [0.0; 128];
        for (i, x_i) in x.iter().enumerate() {
            let q = x[i] * self.gain / max_abs_x;
            let q_sign = (-1.0 * q).signum();
            z[i] = q_sign * (1.0 - (q_sign * q).exp());
        }

        let max_abs_z = z.iter().fold(0.0, |a: f64, &b| a.abs().max(b.abs()));
        let mut y: [f64; 128] = [0.0; 128];
        for (i, z_i) in z.iter().enumerate() {
            y[i] = self.mix * z[i] * max_abs_x / max_abs_z + (1.0 - self.mix) * x[i];
        }

        let max_abs_y = y.iter().fold(0.0, |a: f64, &b| a.abs().max(b.abs()));

        let mut out: [f64; 128] = [0.0; 128];
        for (i, y_i) in y.iter().enumerate() {
            out[i] = y[i] * max_abs_x / max_abs_y;
        }

        self.samples = out;
    }
}
*/