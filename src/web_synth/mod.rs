use wasm_bindgen::prelude::*;
use crate::web_synth::instruments::{Instrument};

pub mod random;
pub mod oscillators;
pub mod envelopes;
pub mod instruments;
pub mod keyboard;
pub mod sequencer;

pub(crate) const SAMPLE_RATE: f64 = 48000.0;
pub(crate) const SAMPLE_SIZE: usize = 128;
pub(crate) const DELTA_TIME: f64 = 1.0 / SAMPLE_RATE;

pub trait Source {
    fn get_sample_block(&self, t: f64) -> [f64; 128];
}

pub trait MutSource {
    fn get_sample_block(&mut self, t: f64) -> [f64; 128];
}

pub trait Effect {
    fn process_sample_block(&self, t: f64, block: [f64; 128]) -> [f64; 128];
}

pub struct Note {
    id: u32,
    on: f64,
    off: f64,
    active: bool,
    channel: usize
}

impl Note {
    pub fn new() -> Note {
        Note {
            id: 0,
            on: 0.0,
            off: 0.0,
            active: false,
            channel: 0
        }
    }

    pub fn new_with_params(id: u32, t_on: f64, channel: usize) -> Note {
        Note {
            id,
            on: t_on,
            off: 0.0,
            active: true,
            channel
        }
    }
}

pub fn scale(note_id: u32) -> f64 {
    8.0 * 1.0594630943592952645618252949463_f64.powi(note_id as i32)
}

pub fn piano_scale(note_id: u32) -> f64 {
    440.0 * 1.0594630943592952645618252949463_f64.powi(note_id as i32 - 49)
}

pub fn calc_offset_time(t: f64, sample_idx: usize) -> f64 {
    t + sample_idx as f64 / SAMPLE_RATE
}